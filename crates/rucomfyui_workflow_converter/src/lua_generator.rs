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

    /// Dedent a string by removing common leading whitespace from all lines.
    fn dedent(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let non_empty_lines: Vec<&str> = lines.iter().filter(|l| !l.trim().is_empty()).copied().collect();

        if non_empty_lines.is_empty() {
            return String::new();
        }

        // Find minimum indentation
        let min_indent = non_empty_lines
            .iter()
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        // Remove that indentation from all lines
        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line.trim()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    /// Parse Lua code and return a normalized string for comparison.
    fn normalize_lua(code: &str) -> String {
        // First dedent to remove raw string indentation
        let dedented = dedent(code);
        let ast = full_moon::parse(&dedented).expect("Failed to parse Lua code");
        // Use Display trait which preserves comments
        ast.to_string().trim().to_string()
    }

    /// Assert that two Lua code strings parse to equivalent ASTs.
    fn assert_lua_eq(actual: &str, expected: &str) {
        let actual_normalized = normalize_lua(actual);
        let expected_normalized = normalize_lua(expected);

        assert_eq!(
            actual_normalized,
            expected_normalized,
            "\n\nActual (normalized):\n{}\n\nExpected (normalized):\n{}",
            actual_normalized,
            expected_normalized
        );
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

        // Comment is generated from _meta.title
        let expected = r#"
            -- Load Checkpoint (CheckpointLoaderSimple)
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")
        "#;

        assert_lua_eq(&result, expected);
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

        // No comments since no _meta.title; fields are alphabetically ordered
        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "a cat",
            }
        "#;

        assert_lua_eq(&result, expected);
    }

    #[test]
    fn test_multi_output_node_references() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "samples": ["1", 0], "vae": ["1", 2] },
                "class_type": "VAEDecode"
            }
        }"#;

        let result = convert_to_lua(json, &object_info).unwrap();

        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local vae_decode = g:VAEDecode {
                samples = checkpoint_loader_simple.model,
                vae = checkpoint_loader_simple.vae,
            }
        "#;

        assert_lua_eq(&result, expected);
    }

    #[test]
    fn test_numeric_inputs() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": {
                    "width": 1024,
                    "height": 768,
                    "batch_size": 1
                },
                "class_type": "EmptyLatentImage"
            }
        }"#;

        let result = convert_to_lua(json, &object_info).unwrap();

        // Fields are alphabetically ordered; floats have .0 suffix
        let expected = r#"
            local empty_latent_image = g:EmptyLatentImage {
                batch_size = 1.0,
                height = 768.0,
                width = 1024.0,
            }
        "#;

        assert_lua_eq(&result, expected);
    }

    #[test]
    fn test_variable_naming_dedup() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "text": "first", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            },
            "3": {
                "inputs": { "text": "second", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            }
        }"#;

        let result = convert_to_lua(json, &object_info).unwrap();

        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "first",
            }
            local clip_text_encode_1 = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "second",
            }
        "#;

        assert_lua_eq(&result, expected);
    }

    /// Test the complete example workflow from testdata/.
    #[test]
    fn test_example_workflow() {
        let object_info = load_test_object_info();
        let json = include_str!("../testdata/example_workflow.json");

        let result = convert_to_lua(json, &object_info).unwrap();

        // Comments from _meta.title; node order by topological sort; fields alphabetically ordered
        let expected = r#"
            -- Load Checkpoint (CheckpointLoaderSimple)
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            -- Empty Latent (EmptyLatentImage)
            local empty_latent_image = g:EmptyLatentImage {
                batch_size = 1.0,
                height = 1024.0,
                width = 1024.0,
            }
            -- Positive Prompt (CLIPTextEncode)
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "a beautiful landscape",
            }
            -- Negative Prompt (CLIPTextEncode)
            local clip_text_encode_1 = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "ugly, blurry",
            }
            -- Sampler (KSampler)
            local k_sampler = g:KSampler {
                cfg = 7.5,
                denoise = 1.0,
                latent_image = empty_latent_image,
                model = checkpoint_loader_simple.model,
                negative = clip_text_encode_1,
                positive = clip_text_encode,
                sampler_name = "euler",
                scheduler = "normal",
                seed = 42.0,
                steps = 20.0,
            }
            -- VAE Decode (VAEDecode)
            local vae_decode = g:VAEDecode {
                samples = k_sampler,
                vae = checkpoint_loader_simple.vae,
            }
            -- Preview (PreviewImage)
            local preview_image = g:PreviewImage {
                images = vae_decode,
            }
        "#;

        assert_lua_eq(&result, expected);
    }
}
