//! Rust code generation from workflows using ObjectInfo.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rucomfyui::object_info::{Object, ObjectInfo, ObjectType};
use std::collections::HashMap;

/// Configuration for Rust code generation.
#[derive(Debug, Clone, Default)]
pub struct RustGeneratorConfig {
    /// Whether to generate a complete compilable file with imports and function wrapper.
    /// If false, generates just the workflow construction code.
    pub include_wrapper: bool,
    /// The function name to use when generating a complete file.
    pub function_name: String,
}

impl RustGeneratorConfig {
    /// Create a config that generates just the workflow code snippet.
    pub fn snippet() -> Self {
        Self {
            include_wrapper: false,
            function_name: String::new(),
        }
    }

    /// Create a config that generates a complete file with the given function name.
    pub fn complete(function_name: impl Into<String>) -> Self {
        Self {
            include_wrapper: true,
            function_name: function_name.into(),
        }
    }
}

/// Convert a workflow JSON to Rust code without ObjectInfo (uses dynamic nodes).
pub fn convert_to_rust(json: &str) -> Result<String> {
    convert_to_rust_with_config(json, &RustGeneratorConfig::snippet())
}

/// Convert a workflow JSON to Rust code with configuration but without ObjectInfo.
pub fn convert_to_rust_with_config(json: &str, config: &RustGeneratorConfig) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_rust_code_dynamic(&analyzed, config)
}

/// Convert a workflow JSON to Rust code using ObjectInfo for type information.
pub fn convert_to_rust_with_object_info(
    json: &str,
    object_info: &ObjectInfo,
    config: &RustGeneratorConfig,
) -> Result<String> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_rust_code_typed(&analyzed, object_info, config)
}

/// Generate Rust code using dynamic nodes (without ObjectInfo).
fn generate_rust_code_dynamic(
    analyzed: &AnalyzedWorkflow,
    config: &RustGeneratorConfig,
) -> Result<String> {
    let mut nodes_tokens = Vec::new();
    let mut generated_vars: HashMap<String, &AnalyzedNode> = HashMap::new();

    for node in &analyzed.nodes {
        let var_ident = format_ident!("{}", &node.var_name);
        let class_type = &node.class_type;

        let comment = if let Some(display_name) = &node.display_name {
            format!("// {} ({})", display_name, class_type)
        } else {
            format!("// {}", class_type)
        };
        let comment_tokens: TokenStream = comment.parse().unwrap();

        let mut input_calls = Vec::new();
        for (name, input) in &node.inputs {
            let value = format_dynamic_input_value(input, &generated_vars);
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

        generated_vars.insert(node.var_name.clone(), node);
    }

    let output_node = analyzed
        .nodes
        .iter()
        .rev()
        .find(|n| is_likely_output_node(&n.class_type));

    let output_ident = output_node
        .map(|n| format_ident!("{}", &n.var_name))
        .unwrap_or_else(|| format_ident!("WorkflowNodeId"));

    let output_expr = if output_node.is_some() {
        quote! { #output_ident }
    } else {
        quote! { WorkflowNodeId(0) }
    };

    let tokens = if config.include_wrapper {
        let func_name = format_ident!(
            "{}",
            if config.function_name.is_empty() {
                "workflow"
            } else {
                &config.function_name
            }
        );

        quote! {
            //! Generated workflow code from ComfyUI API workflow.

            use rucomfyui::{Workflow, WorkflowGraph, WorkflowNodeId};

            /// Constructs the workflow.
            pub fn #func_name() -> (Workflow, WorkflowNodeId) {
                let g = WorkflowGraph::new();

                #(#nodes_tokens)*

                (g.into_workflow(), #output_expr)
            }
        }
    } else {
        quote! {
            let g = WorkflowGraph::new();

            #(#nodes_tokens)*
        }
    };

    if config.include_wrapper {
        format_tokens_as_file(tokens)
    } else {
        format_tokens_as_snippet(tokens)
    }
}

/// Generate Rust code using typed nodes with ObjectInfo.
fn generate_rust_code_typed(
    analyzed: &AnalyzedWorkflow,
    object_info: &ObjectInfo,
    config: &RustGeneratorConfig,
) -> Result<String> {
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

    // Find output node using ObjectInfo
    let output_node = analyzed.nodes.iter().rev().find(|n| {
        object_info
            .get(&n.class_type)
            .map(|o| o.output_node)
            .unwrap_or(false)
    });

    let output_ident = output_node
        .map(|n| format_ident!("{}", &n.var_name))
        .unwrap_or_else(|| format_ident!("WorkflowNodeId"));

    let output_expr = if output_node.is_some() {
        quote! { #output_ident }
    } else {
        quote! { WorkflowNodeId(0) }
    };

    let tokens = if config.include_wrapper {
        let func_name = format_ident!(
            "{}",
            if config.function_name.is_empty() {
                "workflow"
            } else {
                &config.function_name
            }
        );

        quote! {
            //! Generated workflow code from ComfyUI API workflow.

            use rucomfyui::{Workflow, WorkflowGraph, WorkflowNodeId};
            use rucomfyui::nodes::all::*;

            /// Constructs the workflow.
            pub fn #func_name() -> (Workflow, WorkflowNodeId) {
                let g = WorkflowGraph::new();

                #(#nodes_tokens)*

                (g.into_workflow(), #output_expr)
            }
        }
    } else {
        quote! {
            let g = WorkflowGraph::new();

            #(#nodes_tokens)*
        }
    };

    if config.include_wrapper {
        format_tokens_as_file(tokens)
    } else {
        format_tokens_as_snippet(tokens)
    }
}

fn format_tokens_as_file(tokens: TokenStream) -> Result<String> {
    let syntax_tree = syn::parse2::<syn::File>(tokens.clone())
        .map_err(|e| crate::ConversionError::InvalidNodeReference(format!("Parse error: {}", e)))?;
    Ok(prettyplease::unparse(&syntax_tree))
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

/// Check if a node is likely an output node (used when ObjectInfo is not available).
fn is_likely_output_node(class_type: &str) -> bool {
    matches!(
        class_type,
        "PreviewImage"
            | "SaveImage"
            | "PreviewAudio"
            | "SaveAudio"
            | "SaveAudioMP3"
            | "SaveAudioOpus"
            | "SaveVideo"
            | "SaveAnimatedPNG"
            | "SaveAnimatedWEBP"
            | "SaveWEBM"
            | "SaveGLB"
            | "SaveSVGNode"
    )
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

        let result = convert_to_rust(json).unwrap();
        assert!(result.contains("let g = WorkflowGraph::new()"));
        assert!(result.contains("CheckpointLoaderSimple"));
        assert!(result.contains("ckpt_name"));
    }

    #[test]
    fn test_convert_with_wrapper() {
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            }
        }"#;

        let config = RustGeneratorConfig::complete("my_workflow");
        let result = convert_to_rust_with_config(json, &config).unwrap();
        assert!(result.contains("pub fn my_workflow()"));
        assert!(result.contains("use rucomfyui::"));
    }
}
