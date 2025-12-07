//! Rust code generation from workflows using ObjectInfo.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rucomfyui::object_info::{Object, ObjectInfo, ObjectType};
use std::collections::HashMap;

/// Convert a workflow JSON to Rust code using ObjectInfo for type information.
pub fn convert_to_rust(json: &str, object_info: &ObjectInfo) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_rust_code(&analyzed, object_info)
}

/// Generate Rust code using typed nodes with ObjectInfo.
fn generate_rust_code(analyzed: &AnalyzedWorkflow, object_info: &ObjectInfo) -> Result<String> {
    let mut nodes_tokens = Vec::new();
    let mut generated_vars: HashMap<String, (&AnalyzedNode, Option<&Object>)> = HashMap::new();

    for node in &analyzed.nodes {
        let var_ident = format_ident!("{}", &node.var_name);
        let obj = object_info.get(&node.class_type);

        let comment = if let Some(display_name) = &node.display_name {
            format!("// {} ({})", display_name, node.class_type)
        } else {
            format!("// {}", node.class_type)
        };
        let comment_tokens: TokenStream = comment.parse().unwrap();

        if let Some(obj) = obj {
            // Known node - use typed node construction
            let struct_ident = format_ident!("{}", &node.class_type);

            let mut field_assignments = Vec::new();
            for (name, input) in &node.inputs {
                let field_ident = format_ident!("{}", name.to_case(Case::Snake));

                // Get expected type for this input
                let expected_type = obj
                    .all_inputs()
                    .find(|(n, _, _)| *n == name)
                    .and_then(|(_, input, _)| input.as_type().cloned());

                let value =
                    format_typed_input_value(input, expected_type.as_ref(), &generated_vars);
                let value_tokens: TokenStream = value.parse().unwrap();

                field_assignments.push(quote! {
                    #field_ident: #value_tokens
                });
            }

            nodes_tokens.push(quote! {
                #comment_tokens
                let #var_ident = g.add(#struct_ident {
                    #(#field_assignments),*
                });
            });
        } else {
            // Unknown node - use dynamic construction
            let class_type = &node.class_type;
            let mut input_calls = Vec::new();
            for (name, input) in &node.inputs {
                let value = format_dynamic_input_value(
                    input,
                    &generated_vars
                        .iter()
                        .map(|(k, (n, _))| (k.clone(), *n))
                        .collect(),
                );
                let value_tokens: TokenStream = value.parse().unwrap();
                input_calls.push(quote! {
                    .with_input(#name, #value_tokens)
                });
            }

            nodes_tokens.push(quote! {
                #comment_tokens
                let #var_ident = g.add_dynamic(
                    rucomfyui::workflow::WorkflowNode::new(#class_type)
                        #(#input_calls)*
                );
            });
        }

        generated_vars.insert(node.var_name.clone(), (node, obj));
    }

    let tokens = quote! {
        let g = WorkflowGraph::new();

        #(#nodes_tokens)*
    };

    format_tokens_as_snippet(tokens)
}

fn format_tokens_as_snippet(tokens: TokenStream) -> Result<String> {
    // For snippets, wrap in a function to parse, then extract just the body
    let wrapped = quote! {
        fn __wrapper() {
            #tokens
        }
    };
    let syntax_tree = syn::parse2::<syn::File>(wrapped)
        .map_err(|e| crate::ConversionError::InvalidNodeReference(format!("Parse error: {}", e)))?;
    let formatted = prettyplease::unparse(&syntax_tree);

    // Extract just the function body (between the first { and the last })
    if let Some(start) = formatted.find('{') {
        if let Some(end) = formatted.rfind('}') {
            let body = &formatted[start + 1..end];
            // Remove leading and trailing whitespace from each line and dedent
            return Ok(body
                .lines()
                .map(|line| {
                    // Remove exactly 4 spaces of indentation if present
                    if let Some(stripped) = line.strip_prefix("    ") {
                        stripped
                    } else {
                        line.trim_start()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string());
        }
    }
    Ok(formatted)
}

fn format_dynamic_input_value(
    input: &AnalyzedInput,
    _generated_vars: &HashMap<String, &AnalyzedNode>,
) -> String {
    match input {
        AnalyzedInput::String(s) => format!("\"{}\"", escape_string(s)),
        AnalyzedInput::Integer(i) => format!("{}i64", i),
        AnalyzedInput::Float(f) => {
            let s = f.to_string();
            if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            }
        }
        AnalyzedInput::Boolean(b) => b.to_string(),
        AnalyzedInput::NodeRef { var_name, slot } => {
            format!("{}.to_input_with_slot({})", var_name, slot)
        }
    }
}

fn format_typed_input_value(
    input: &AnalyzedInput,
    expected_type: Option<&ObjectType>,
    generated_vars: &HashMap<String, (&AnalyzedNode, Option<&Object>)>,
) -> String {
    match input {
        AnalyzedInput::String(s) => format!("\"{}\"", escape_string(s)),
        AnalyzedInput::Integer(i) => i.to_string(),
        AnalyzedInput::Float(f) => {
            // Check if expected type is Int - if so, output as integer
            if let Some(ObjectType::Int) = expected_type {
                if f.fract() == 0.0 && f.abs() < i64::MAX as f64 {
                    return format!("{}", *f as i64);
                }
            }
            // Otherwise output as float
            let s = f.to_string();
            if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            }
        }
        AnalyzedInput::Boolean(b) => b.to_string(),
        AnalyzedInput::NodeRef { var_name, slot } => {
            // Look up the referenced node to get output field name
            if let Some((_ref_node, Some(ref_obj))) = generated_vars.get(var_name) {
                let outputs: Vec<_> = ref_obj.processed_output().collect();
                if outputs.len() > 1 {
                    // Multi-output node - use field name
                    if let Some(output) = outputs.get(*slot as usize) {
                        let field_name = output.name.to_case(Case::Snake);
                        return format!("{}.{}", var_name, field_name);
                    }
                }
            }
            // Single-output or unknown - use variable directly
            if *slot == 0 {
                var_name.clone()
            } else {
                format!("{}.{}", var_name, slot)
            }
        }
    }
}

fn escape_string(s: &str) -> String {
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

        let result = convert_to_rust(json, &object_info).unwrap();
        assert!(result.contains("let g = WorkflowGraph::new()"));
        assert!(result.contains("CheckpointLoaderSimple"));
        assert!(result.contains("ckpt_name"));
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

        let result = convert_to_rust(json, &object_info).unwrap();
        assert!(result.contains("checkpoint_loader_simple.clip"));
    }
}
