//! Lua code generation from workflows.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use rucomfyui::object_info::{Object, ObjectInfo};
use std::collections::HashMap;
use std::fmt::Write;

/// Convert a workflow JSON to Lua code using ObjectInfo for type information.
pub fn convert_to_lua(json: &str, object_info: &ObjectInfo) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_lua_code(&analyzed, object_info)
}

fn generate_lua_code(analyzed: &AnalyzedWorkflow, object_info: &ObjectInfo) -> Result<String> {
    let mut output = String::new();

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

        let result = convert_to_lua(json, &object_info).unwrap();
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

        let result = convert_to_lua(json, &object_info).unwrap();
        assert!(result.contains("checkpoint_loader_simple.clip"));
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

        let result = convert_to_lua(json, &object_info).unwrap();
        assert!(result.contains("checkpoint_loader_simple.model"));
        assert!(result.contains("checkpoint_loader_simple.clip"));
        assert!(result.contains("clip_text_encode"));
        assert!(result.contains("k_sampler"));
    }

    /// Test the complete example workflow from testdata/.
    #[test]
    fn test_example_workflow() {
        let object_info = load_test_object_info();
        let json = include_str!("../testdata/example_workflow.json");

        let result = convert_to_lua(json, &object_info).unwrap();

        // Verify all nodes are present
        assert!(result.contains("local checkpoint_loader_simple = g:CheckpointLoaderSimple"));
        assert!(result.contains("local clip_text_encode = g:CLIPTextEncode"));
        assert!(result.contains("local clip_text_encode_1 = g:CLIPTextEncode"));
        assert!(result.contains("local empty_latent_image = g:EmptyLatentImage"));
        assert!(result.contains("local k_sampler = g:KSampler"));
        assert!(result.contains("local vae_decode = g:VAEDecode"));
        assert!(result.contains("local preview_image = g:PreviewImage"));

        // Verify key references
        assert!(result.contains("checkpoint_loader_simple.model"));
        assert!(result.contains("checkpoint_loader_simple.clip"));
        assert!(result.contains("checkpoint_loader_simple.vae"));
        assert!(result.contains("positive = clip_text_encode,"));
        assert!(result.contains("negative = clip_text_encode_1,"));
    }
}
