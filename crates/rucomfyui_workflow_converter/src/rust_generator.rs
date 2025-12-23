//! Rust code generation from workflows using ObjectInfo.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rucomfyui::object_info::{Object, ObjectInfo, ObjectType};
use rucomfyui::workflow::WorkflowNodeId;
use std::collections::HashMap;

// =============================================================================
// Public API
// =============================================================================

/// Convert a workflow JSON to Rust code using ObjectInfo for type information.
pub fn convert_to_rust(json: &str, object_info: &ObjectInfo) -> Result<String> {
    let tokens = convert_to_rust_tokens(json, object_info)?;
    format_tokens_as_snippet(tokens)
}

/// Convert a workflow JSON to a proc_macro2 TokenStream.
pub fn convert_to_rust_tokens(json: &str, object_info: &ObjectInfo) -> Result<TokenStream> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    Ok(generate_rust_tokens(&analyzed, object_info))
}

// =============================================================================
// Code generation
// =============================================================================

/// Context for code generation, holding references to workflow data.
struct GeneratorContext<'a> {
    object_info: &'a ObjectInfo,
    nodes_by_id: HashMap<WorkflowNodeId, &'a AnalyzedNode>,
}

impl<'a> GeneratorContext<'a> {
    fn new(analyzed: &'a AnalyzedWorkflow, object_info: &'a ObjectInfo) -> Self {
        let nodes_by_id = analyzed.nodes.iter().map(|n| (n.id, n)).collect();
        Self {
            object_info,
            nodes_by_id,
        }
    }

    fn get_node(&self, id: WorkflowNodeId) -> Option<&'a AnalyzedNode> {
        self.nodes_by_id.get(&id).copied()
    }

    fn get_object(&self, class_type: &str) -> Option<&'a Object> {
        self.object_info.get(class_type)
    }
}

/// Generate Rust TokenStream using typed nodes with ObjectInfo.
fn generate_rust_tokens(analyzed: &AnalyzedWorkflow, object_info: &ObjectInfo) -> TokenStream {
    let ctx = GeneratorContext::new(analyzed, object_info);
    let mut nodes_tokens = Vec::new();

    // Only generate statements for nodes that need variables:
    // - ref_count == 0: terminal nodes (outputs)
    // - ref_count > 1: nodes referenced multiple times
    // Nodes with ref_count == 1 will be inlined at their use site
    for node in &analyzed.nodes {
        if node.ref_count == 1 {
            continue; // Will be inlined
        }

        let var_ident = format_ident!("{}", &node.var_name);

        let comment = if let Some(display_name) = &node.display_name {
            format!("// {} ({})", display_name, node.class_type)
        } else {
            format!("// {}", node.class_type)
        };
        let comment_tokens: TokenStream = comment.parse().unwrap();

        let node_expr = build_node_expr(&ctx, node);

        nodes_tokens.push(quote! {
            #comment_tokens
            let #var_ident = g.add(#node_expr);
        });
    }

    quote! {
        let g = WorkflowGraph::new();

        #(#nodes_tokens)*
    }
}

/// Build an expression for a node: `NodeType { field: value, ... }`
fn build_node_expr(ctx: &GeneratorContext, node: &AnalyzedNode) -> TokenStream {
    let obj = ctx.get_object(&node.class_type);

    if let Some(obj) = obj {
        // Known node - use typed node construction
        let struct_ident = format_ident!("{}", &node.class_type);

        let field_assignments: Vec<TokenStream> = node
            .inputs
            .iter()
            .map(|(name, input)| {
                let field_ident = format_ident!("{}", name.to_case(Case::Snake));

                // Get expected type for this input
                let expected_type = obj
                    .all_inputs()
                    .find(|(n, _, _)| *n == name)
                    .and_then(|(_, input, _)| input.as_type().cloned());

                let value_tokens = build_input_expr(ctx, input, expected_type.as_ref());

                quote! {
                    #field_ident: #value_tokens
                }
            })
            .collect();

        quote! {
            #struct_ident {
                #(#field_assignments),*
            }
        }
    } else {
        // Unknown node - use dynamic construction
        // Note: This doesn't support inlining for dynamic nodes
        let class_type = &node.class_type;
        let input_calls: Vec<TokenStream> = node
            .inputs
            .iter()
            .map(|(name, input)| {
                let value_tokens = build_input_expr(ctx, input, None);
                quote! {
                    .with_input(#name, #value_tokens)
                }
            })
            .collect();

        quote! {
            rucomfyui::workflow::WorkflowNode::new(#class_type)
                #(#input_calls)*
        }
    }
}

/// Build an expression for an input value, inlining single-use node references.
fn build_input_expr(
    ctx: &GeneratorContext,
    input: &AnalyzedInput,
    expected_type: Option<&ObjectType>,
) -> TokenStream {
    match input {
        AnalyzedInput::String(s) => {
            let escaped = escape_string(s);
            quote! { #escaped }
        }
        AnalyzedInput::Integer(i) => {
            // Check if expected type is Float - if so, output as float
            if let Some(ObjectType::Float) = expected_type {
                let lit = proc_macro2::Literal::f64_unsuffixed(*i as f64);
                return quote! { #lit };
            }
            let lit = proc_macro2::Literal::i64_unsuffixed(*i);
            quote! { #lit }
        }
        AnalyzedInput::Float(f) => {
            // Check if expected type is Int - if so, output as integer
            if let Some(ObjectType::Int) = expected_type {
                if f.fract() == 0.0 && f.abs() < i64::MAX as f64 {
                    let lit = proc_macro2::Literal::i64_unsuffixed(*f as i64);
                    return quote! { #lit };
                }
            }
            let lit = proc_macro2::Literal::f64_unsuffixed(*f);
            quote! { #lit }
        }
        AnalyzedInput::Boolean(b) => {
            quote! { #b }
        }
        AnalyzedInput::NodeRef {
            node_id,
            var_name,
            slot,
        } => {
            // Look up the referenced node
            let ref_node = ctx.get_node(*node_id);
            let ref_obj = ref_node.and_then(|n| ctx.get_object(&n.class_type));

            // Check if we should inline this node
            if let Some(ref_node) = ref_node {
                if ref_node.ref_count == 1 {
                    // Inline the node - build its full expression
                    let node_expr = build_node_expr(ctx, ref_node);

                    // If this is a multi-output node and we need a specific output, add field access
                    if let Some(ref_obj) = ref_obj {
                        let outputs: Vec<_> = ref_obj.processed_output().collect();
                        if outputs.len() > 1 {
                            if let Some(output) = outputs.get(*slot as usize) {
                                let field_ident =
                                    format_ident!("{}", output.name.to_case(Case::Snake));
                                return quote! { g.add(#node_expr).#field_ident };
                            }
                        }
                    }

                    // Single output - just the add expression
                    return quote! { g.add(#node_expr) };
                }
            }

            // Not inlining - use variable reference
            let var_ident = format_ident!("{}", var_name);

            // Check if we need field access for multi-output nodes
            if let Some(ref_obj) = ref_obj {
                let outputs: Vec<_> = ref_obj.processed_output().collect();
                if outputs.len() > 1 {
                    if let Some(output) = outputs.get(*slot as usize) {
                        let field_ident = format_ident!("{}", output.name.to_case(Case::Snake));
                        return quote! { #var_ident.#field_ident };
                    }
                }
            }

            // Single-output or unknown - use variable directly
            if *slot == 0 {
                quote! { #var_ident }
            } else {
                let slot_lit = proc_macro2::Literal::u32_unsuffixed(*slot);
                quote! { #var_ident.#slot_lit }
            }
        }
    }
}

// =============================================================================
// Formatting
// =============================================================================

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

// =============================================================================
// String utilities
// =============================================================================

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// =============================================================================
// Tests
// =============================================================================

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

    /// Format expected TokenStream the same way as the generator.
    fn format_expected(tokens: TokenStream) -> String {
        format_tokens_as_snippet(tokens).unwrap()
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

        // Single terminal node - still gets a variable
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
                ckpt_name: "sd_xl_base_1.0.safetensors"
            });
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_use_node_inlined() {
        let object_info = load_test_object_info();
        // checkpoint_loader_simple is referenced only once by clip_text_encode
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

        // checkpoint_loader_simple should be inlined
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let clip_text_encode = g.add(CLIPTextEncode {
                clip: g.add(CheckpointLoaderSimple {
                    ckpt_name: "model.safetensors"
                }).clip,
                text: "a cat"
            });
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_use_node_not_inlined() {
        let object_info = load_test_object_info();
        // checkpoint_loader_simple is referenced twice (by two CLIPTextEncode nodes)
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

        let result = convert_to_rust(json, &object_info).unwrap();

        // checkpoint_loader_simple should NOT be inlined (used twice)
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
                ckpt_name: "model.safetensors"
            });

            let clip_text_encode = g.add(CLIPTextEncode {
                clip: checkpoint_loader_simple.clip,
                text: "first"
            });

            let clip_text_encode_1 = g.add(CLIPTextEncode {
                clip: checkpoint_loader_simple.clip,
                text: "second"
            });
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_output_node_not_inlined_when_multiple_refs() {
        let object_info = load_test_object_info();
        // checkpoint_loader_simple is referenced twice (samples and vae)
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

        let result = convert_to_rust(json, &object_info).unwrap();

        // checkpoint_loader_simple is referenced twice, so NOT inlined
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
                ckpt_name: "model.safetensors"
            });

            let vae_decode = g.add(VAEDecode {
                samples: checkpoint_loader_simple.model,
                vae: checkpoint_loader_simple.vae
            });
        });

        assert_eq!(result, expected);
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

        let result = convert_to_rust(json, &object_info).unwrap();

        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let empty_latent_image = g.add(EmptyLatentImage {
                batch_size: 1,
                height: 768,
                width: 1024
            });
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deeply_nested_inlining() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "text": "test", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            },
            "3": {
                "inputs": { "width": 512, "height": 512, "batch_size": 1 },
                "class_type": "EmptyLatentImage"
            },
            "4": {
                "inputs": { "samples": ["3", 0], "vae": ["1", 2] },
                "class_type": "VAEDecode"
            }
        }"#;

        let result = convert_to_rust(json, &object_info).unwrap();

        // CheckpointLoaderSimple is referenced twice (by CLIPTextEncode and VAEDecode)
        // CLIPTextEncode is referenced 0 times (terminal) - gets a variable
        // EmptyLatentImage is referenced once - inlined
        // VAEDecode is referenced 0 times (terminal) - gets a variable
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
                ckpt_name: "model.safetensors"
            });

            let clip_text_encode = g.add(CLIPTextEncode {
                clip: checkpoint_loader_simple.clip,
                text: "test"
            });

            let vae_decode = g.add(VAEDecode {
                samples: g.add(EmptyLatentImage {
                    batch_size: 1,
                    height: 512,
                    width: 512
                }),
                vae: checkpoint_loader_simple.vae
            });
        });

        assert_eq!(result, expected);
    }

    /// Test the complete example workflow from testdata/.
    #[test]
    fn test_example_workflow() {
        let object_info = load_test_object_info();
        let json = include_str!("../testdata/example_workflow.json");

        let result = convert_to_rust(json, &object_info).unwrap();

        // In the example workflow:
        // - CheckpointLoaderSimple: ref_count > 1 (model, clip x2, vae) -> variable
        // - EmptyLatentImage: ref_count = 1 (latent_image) -> inlined
        // - CLIPTextEncode (positive): ref_count = 1 -> inlined
        // - CLIPTextEncode (negative): ref_count = 1 -> inlined
        // - KSampler: ref_count = 1 (samples) -> inlined
        // - VAEDecode: ref_count = 1 (images) -> inlined
        // - PreviewImage: ref_count = 0 (terminal) -> variable
        let expected = format_expected(quote! {
            let g = WorkflowGraph::new();

            let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
                ckpt_name: "model.safetensors"
            });

            let preview_image = g.add(PreviewImage {
                images: g.add(VAEDecode {
                    samples: g.add(KSampler {
                        cfg: 7.5,
                        denoise: 1.0,
                        latent_image: g.add(EmptyLatentImage {
                            batch_size: 1,
                            height: 1024,
                            width: 1024
                        }),
                        model: checkpoint_loader_simple.model,
                        negative: g.add(CLIPTextEncode {
                            clip: checkpoint_loader_simple.clip,
                            text: "ugly, blurry"
                        }),
                        positive: g.add(CLIPTextEncode {
                            clip: checkpoint_loader_simple.clip,
                            text: "a beautiful landscape"
                        }),
                        sampler_name: "euler",
                        scheduler: "normal",
                        seed: 42,
                        steps: 20
                    }),
                    vae: checkpoint_loader_simple.vae
                })
            });
        });

        assert_eq!(result, expected);
    }
}
