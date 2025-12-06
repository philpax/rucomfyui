//! Lua code generation from workflows.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedWorkflow};
use crate::Result;
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

/// Mapping of node class types to their output field names for Lua.
fn get_node_output_fields() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // Loaders with multiple outputs
    map.insert("CheckpointLoaderSimple", vec!["model", "clip", "vae"]);
    map.insert("CheckpointLoader", vec!["model", "clip", "vae"]);
    map.insert(
        "unCLIPCheckpointLoader",
        vec!["model", "clip", "vae", "clip_vision"],
    );
    map.insert("LoraLoader", vec!["model", "clip"]);
    map.insert(
        "ImageOnlyCheckpointLoader",
        vec!["model", "clip_vision", "vae"],
    );
    map.insert("DualCLIPLoader", vec!["clip"]);
    map.insert("TripleCLIPLoader", vec!["clip"]);
    map.insert("QuadrupleCLIPLoader", vec!["clip"]);

    // Split nodes
    map.insert("SplitImageWithAlpha", vec!["image", "mask"]);
    map.insert("LoadImage", vec!["image", "mask"]);

    // Sampling outputs
    map.insert("SamplerCustom", vec!["output", "denoised_output"]);
    map.insert("SamplerCustomAdvanced", vec!["output", "denoised_output"]);

    map
}

/// Convert a workflow JSON to Lua code.
pub fn convert_to_lua(json: &str) -> Result<String> {
    convert_to_lua_with_config(json, &LuaGeneratorConfig::snippet())
}

/// Convert a workflow JSON to Lua code with configuration.
pub fn convert_to_lua_with_config(json: &str, config: &LuaGeneratorConfig) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_lua_code(&analyzed, config)
}

fn generate_lua_code(analyzed: &AnalyzedWorkflow, config: &LuaGeneratorConfig) -> Result<String> {
    let mut output = String::new();
    let node_outputs = get_node_output_fields();

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

    // Track generated variables
    let mut generated_vars: HashMap<String, String> = HashMap::new();

    // Generate code for each node
    for node in &analyzed.nodes {
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
            let value = format_lua_value(input, &node_outputs, &generated_vars)?;
            writeln!(output, "({})", value).unwrap();
        } else {
            // Multiple arguments or node references - use table syntax
            writeln!(output, " {{").unwrap();
            for (name, input) in &node.inputs {
                let value = format_lua_value(input, &node_outputs, &generated_vars)?;
                writeln!(output, "    {} = {},", name, value).unwrap();
            }
            writeln!(output, "}}").unwrap();
        }

        generated_vars.insert(node.var_name.clone(), node.class_type.clone());
    }

    if config.include_execution {
        writeln!(output).unwrap();
        writeln!(output, "-- Queue the workflow and wait for results").unwrap();
        writeln!(output, "local result = client:easy_queue(g)").unwrap();

        // Find likely output nodes
        let output_nodes: Vec<_> = analyzed
            .nodes
            .iter()
            .filter(|n| {
                matches!(
                    n.class_type.as_str(),
                    "PreviewImage" | "SaveImage" | "SaveVideo" | "PreviewAudio" | "SaveAudio"
                )
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
    node_outputs: &HashMap<&str, Vec<&str>>,
    generated_vars: &HashMap<String, String>,
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
            // Look up the referenced node to get its output field name
            if let Some(class_type) = generated_vars.get(var_name) {
                if let Some(fields) = node_outputs.get(class_type.as_str()) {
                    if let Some(field_name) = fields.get(*slot as usize) {
                        return Ok(format!("{}.{}", var_name, field_name));
                    }
                }
            }
            // For single-output nodes or unknown slots, just reference the variable
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

    #[test]
    fn test_convert_simple_workflow() {
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "sd_xl_base_1.0.safetensors" },
                "class_type": "CheckpointLoaderSimple",
                "_meta": { "title": "Load Checkpoint" }
            }
        }"#;

        let result = convert_to_lua(json).unwrap();
        assert!(result.contains("g:CheckpointLoaderSimple"));
        assert!(result.contains("sd_xl_base_1.0.safetensors"));
    }

    #[test]
    fn test_convert_workflow_with_dependencies() {
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

        let result = convert_to_lua(json).unwrap();
        assert!(result.contains("checkpoint_loader_simple.clip"));
    }

    #[test]
    fn test_convert_with_boilerplate() {
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
        let result = convert_to_lua_with_config(json, &config).unwrap();
        assert!(result.contains("client:get_object_info()"));
        assert!(result.contains("comfy.graph(object_info)"));
        assert!(result.contains("client:easy_queue(g)"));
    }

    #[test]
    fn test_nested_workflow() {
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

        let result = convert_to_lua(json).unwrap();
        assert!(result.contains("checkpoint_loader_simple.model"));
        assert!(result.contains("checkpoint_loader_simple.clip"));
        assert!(result.contains("clip_text_encode"));
        assert!(result.contains("k_sampler"));
    }
}
