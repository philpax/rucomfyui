//! Lua code generation from workflows.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use rucomfyui::object_info::{Object, ObjectInfo};
use std::collections::HashMap;
use std::fmt::Write;

/// Configuration for Lua code generation.
#[derive(Debug, Clone, Default)]
pub struct LuaGeneratorConfig {
    /// Whether to include the boilerplate code (get_object_info, graph creation).
    pub include_boilerplate: bool,
    /// Whether to include the execution code (client:easy_queue).
    pub include_execution: bool,
}

impl LuaGeneratorConfig {
    /// Create a config that generates just the workflow building code.
    pub fn snippet() -> Self {
        Self {
            include_boilerplate: false,
            include_execution: false,
        }
    }

    /// Create a config that generates a complete executable script.
    pub fn complete() -> Self {
        Self {
            include_boilerplate: true,
            include_execution: true,
        }
    }
}

/// Convert a workflow JSON to Lua code using ObjectInfo for type information.
pub fn convert_to_lua_with_object_info(
    json: &str,
    object_info: &ObjectInfo,
    config: &LuaGeneratorConfig,
) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_lua_code(&analyzed, object_info, config)
}

fn generate_lua_code(
    analyzed: &AnalyzedWorkflow,
    object_info: &ObjectInfo,
    config: &LuaGeneratorConfig,
) -> Result<String> {
    let mut output = String::new();

    if config.include_boilerplate {
        writeln!(output, "-- Generated workflow from ComfyUI API workflow").unwrap();
        writeln!(output, "-- Requires: comfy module and client to be set up").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "-- Get object info and create graph").unwrap();
        writeln!(output, "local object_info = client:get_object_info()").unwrap();
        writeln!(output, "local g = comfy.graph(object_info)").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "-- Build the workflow").unwrap();
    }

    // Track generated variables with their ObjectInfo
    let mut generated_vars: HashMap<String, (&AnalyzedNode, Option<&Object>)> = HashMap::new();

    // Generate code for each node
    for node in &analyzed.nodes {
        let obj = object_info.get(&node.class_type);

        // Add comment
        if let Some(display_name) = &node.display_name {
            writeln!(output, "-- {} ({})", display_name, node.class_type).unwrap();
        }

        // Start the node creation
        write!(output, "local {} = g:{}", node.var_name, node.class_type).unwrap();

        // Check if we can use positional or need named arguments
        let use_table = node.inputs.len() > 1
            || node
                .inputs
                .iter()
                .any(|(_, v)| matches!(v, AnalyzedInput::NodeRef { .. }));

        if node.inputs.is_empty() {
            writeln!(output, "()").unwrap();
        } else if !use_table && node.inputs.len() == 1 {
            // Single simple argument - can use positional
            let (_, input) = node.inputs.iter().next().unwrap();
            let value = format_lua_value(input, &generated_vars)?;
            writeln!(output, "({})", value).unwrap();
        } else {
            // Multiple arguments or node references - use table syntax
            writeln!(output, " {{").unwrap();
            for (name, input) in &node.inputs {
                let value = format_lua_value(input, &generated_vars)?;
                writeln!(output, "    {} = {},", name, value).unwrap();
            }
            writeln!(output, "}}").unwrap();
        }

        generated_vars.insert(node.var_name.clone(), (node, obj));
    }

    if config.include_execution {
        writeln!(output).unwrap();
        writeln!(output, "-- Queue the workflow and wait for results").unwrap();
        writeln!(output, "local result = client:easy_queue(g)").unwrap();

        // Find output nodes using ObjectInfo
        let output_nodes: Vec<_> = analyzed
            .nodes
            .iter()
            .filter(|n| {
                object_info
                    .get(&n.class_type)
                    .map(|o| o.output_node)
                    .unwrap_or(false)
            })
            .collect();

        if !output_nodes.is_empty() {
            writeln!(output).unwrap();
            writeln!(output, "-- Return results from output nodes").unwrap();
            if output_nodes.len() == 1 {
                writeln!(output, "return result[{}]", output_nodes[0].var_name).unwrap();
            } else {
                writeln!(output, "return {{").unwrap();
                for node in &output_nodes {
                    writeln!(output, "    {} = result[{}],", node.var_name, node.var_name).unwrap();
                }
                writeln!(output, "}}").unwrap();
            }
        }
    }

    Ok(output)
}

fn format_lua_value(
    input: &AnalyzedInput,
    generated_vars: &HashMap<String, (&AnalyzedNode, Option<&Object>)>,
) -> Result<String> {
    match input {
        AnalyzedInput::String(s) => Ok(format!("\"{}\"", escape_lua_string(s))),
        AnalyzedInput::Integer(i) => Ok(i.to_string()),
        AnalyzedInput::Float(f) => {
            let s = f.to_string();
            if s.contains('.') || s.contains('e') || s.contains('E') {
                Ok(s)
            } else {
                Ok(format!("{}.0", s))
            }
        }
        AnalyzedInput::Boolean(b) => Ok(b.to_string()),
        AnalyzedInput::NodeRef { var_name, slot } => {
            // Look up the referenced node to get its output field name from ObjectInfo
            if let Some((_ref_node, Some(ref_obj))) = generated_vars.get(var_name) {
                let outputs: Vec<_> = ref_obj.processed_output().collect();
                if outputs.len() > 1 {
                    // Multi-output node - use field name
                    if let Some(output) = outputs.get(*slot as usize) {
                        let field_name = output.name.to_case(Case::Snake);
                        return Ok(format!("{}.{}", var_name, field_name));
                    }
                }
            }
            // Single-output or unknown - use variable directly
            if *slot == 0 {
                Ok(var_name.clone())
            } else {
                // For unknown multi-output nodes, use array-style access
                // Lua is 1-indexed, so we add 1
                Ok(format!("{}[{}]", var_name, slot + 1))
            }
        }
    }
}

fn escape_lua_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_object_info() -> ObjectInfo {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let object_info_path = manifest_dir
            .parent()
            .unwrap()
            .join("rucomfyui")
            .join("generate_nodes")
            .join("object_info.json");

        let json = std::fs::read_to_string(&object_info_path)
            .unwrap_or_else(|e| panic!("Failed to read object_info.json: {}", e));

        let objects: Vec<rucomfyui::object_info::Object> =
            serde_json::from_str(&json).expect("Failed to parse object_info.json");

        objects.into_iter().map(|o| (o.name.clone(), o)).collect()
    }

    #[test]
    fn test_convert_simple_workflow() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "sd_xl_base_1.0.safetensors" },
                "class_type": "CheckpointLoaderSimple",
                "_meta": { "title": "Load Checkpoint" }
            }
        }"#;

        let result =
            convert_to_lua_with_object_info(json, &object_info, &LuaGeneratorConfig::snippet())
                .unwrap();
        assert!(result.contains("g:CheckpointLoaderSimple"));
        assert!(result.contains("sd_xl_base_1.0.safetensors"));
    }

    #[test]
    fn test_convert_workflow_with_dependencies() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": {
                    "text": "a cat",
                    "clip": ["1", 1]
                },
                "class_type": "CLIPTextEncode"
            }
        }"#;

        let result =
            convert_to_lua_with_object_info(json, &object_info, &LuaGeneratorConfig::snippet())
                .unwrap();
        assert!(result.contains("checkpoint_loader_simple.clip"));
    }

    #[test]
    fn test_convert_with_boilerplate() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "images": ["1", 0] },
                "class_type": "PreviewImage"
            }
        }"#;

        let config = LuaGeneratorConfig::complete();
        let result = convert_to_lua_with_object_info(json, &object_info, &config).unwrap();
        assert!(result.contains("client:get_object_info()"));
        assert!(result.contains("comfy.graph(object_info)"));
        assert!(result.contains("client:easy_queue(g)"));
    }

    #[test]
    fn test_nested_workflow() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "width": 1024, "height": 1024, "batch_size": 1 },
                "class_type": "EmptyLatentImage"
            },
            "3": {
                "inputs": {
                    "model": ["1", 0],
                    "seed": 0,
                    "steps": 20,
                    "cfg": 8.0,
                    "sampler_name": "euler",
                    "scheduler": "normal",
                    "positive": ["4", 0],
                    "negative": ["5", 0],
                    "latent_image": ["2", 0],
                    "denoise": 1.0
                },
                "class_type": "KSampler"
            },
            "4": {
                "inputs": { "text": "a cat", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            },
            "5": {
                "inputs": { "text": "bad", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            }
        }"#;

        let result =
            convert_to_lua_with_object_info(json, &object_info, &LuaGeneratorConfig::snippet())
                .unwrap();
        assert!(result.contains("checkpoint_loader_simple.model"));
        assert!(result.contains("checkpoint_loader_simple.clip"));
        assert!(result.contains("clip_text_encode"));
        assert!(result.contains("k_sampler"));
    }
}
