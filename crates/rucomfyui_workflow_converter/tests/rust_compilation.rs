//! Integration tests that verify generated Rust code compiles correctly.
//!
//! These tests convert workflow JSON to Rust code and then verify the code
//! compiles by creating a temporary crate and running `cargo check`.

#![cfg(feature = "rust")]

use proc_macro2::TokenStream;
use quote::quote;
use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::convert_to_rust_tokens;
use std::fs;
use std::process::Command;

/// Load the ObjectInfo from the generate_nodes crate.
fn load_object_info() -> ObjectInfo {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let object_info_path = manifest_dir
        .parent()
        .unwrap()
        .join("rucomfyui")
        .join("generate_nodes")
        .join("object_info.json");

    let json = fs::read_to_string(&object_info_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read object_info.json from {:?}: {}",
            object_info_path, e
        )
    });

    let objects: Vec<rucomfyui::object_info::Object> =
        serde_json::from_str(&json).expect("Failed to parse object_info.json");

    objects.into_iter().map(|o| (o.name.clone(), o)).collect()
}

/// Format a TokenStream as a snippet string using prettyplease.
fn format_tokens_as_snippet(tokens: TokenStream) -> String {
    let wrapped = quote! {
        fn __wrapper() {
            #tokens
        }
    };
    let syntax_tree = syn::parse2::<syn::File>(wrapped).expect("Failed to parse tokens");
    let formatted = prettyplease::unparse(&syntax_tree);

    // Extract just the function body (between the first { and the last })
    if let Some(start) = formatted.find('{') {
        if let Some(end) = formatted.rfind('}') {
            let body = &formatted[start + 1..end];
            return body
                .lines()
                .map(|line| {
                    if let Some(stripped) = line.strip_prefix("    ") {
                        stripped
                    } else {
                        line.trim_start()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
        }
    }
    formatted
}

/// Wrap generated code TokenStream in a compilable module.
fn wrap_in_module(snippet: TokenStream, function_name: &str) -> String {
    let function_ident = quote::format_ident!("{}", function_name);
    let module = quote! {
        //! Generated workflow code from ComfyUI API workflow.

        use rucomfyui::{Workflow, WorkflowGraph, WorkflowNodeId};
        use rucomfyui::nodes::all::*;

        /// Constructs the workflow.
        #[allow(unused_variables)]
        pub fn #function_ident() -> (Workflow, WorkflowNodeId) {
            #snippet

            (g.into_workflow(), WorkflowNodeId(0))
        }
    };

    let syntax_tree = syn::parse2::<syn::File>(module).expect("Failed to parse module");
    prettyplease::unparse(&syntax_tree)
}

/// Example workflow JSON for testing (loaded from testdata/).
const EXAMPLE_WORKFLOW: &str = include_str!("../testdata/example_workflow.json");

/// Simple workflow with just a checkpoint loader.
const SIMPLE_WORKFLOW: &str = include_str!("../testdata/simple_workflow.json");

fn create_test_crate(code: &str) -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Get the path to rucomfyui crate relative to this test's manifest
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let rucomfyui_path = manifest_dir.parent().unwrap().join("rucomfyui");

    // Write Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "test_workflow"
version = "0.1.0"
edition = "2021"

[dependencies]
rucomfyui = {{ path = "{}", default-features = false, features = ["rustls-tls", "typed_nodes"] }}
"#,
        rucomfyui_path.display()
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    // Write lib.rs
    fs::write(src_dir.join("lib.rs"), code).expect("Failed to write lib.rs");

    temp_dir
}

fn check_compiles(temp_dir: &tempfile::TempDir) -> Result<(), String> {
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to run cargo check");

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Compilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[test]
fn test_example_workflow_compiles() {
    let object_info = load_object_info();
    let tokens = convert_to_rust_tokens(EXAMPLE_WORKFLOW, &object_info).expect("Conversion failed");

    // Verify the snippet matches expected TokenStream (with inlining)
    let actual = format_tokens_as_snippet(tokens.clone());
    let expected = format_tokens_as_snippet(quote! {
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
    assert_eq!(actual, expected);

    // Actually compile the code
    let code = wrap_in_module(tokens, "example_workflow");
    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_simple_workflow_compiles() {
    let object_info = load_object_info();
    let tokens = convert_to_rust_tokens(SIMPLE_WORKFLOW, &object_info).expect("Conversion failed");
    let code = wrap_in_module(tokens, "simple_workflow");

    // Actually compile the code
    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_existing_workflow_file_compiles() {
    let object_info = load_object_info();
    // Test with the actual example workflow file from rucomfyui
    let workflow_json = include_str!("../../rucomfyui/examples/existing_workflow.json");
    let tokens = convert_to_rust_tokens(workflow_json, &object_info).expect("Conversion failed");
    let code = wrap_in_module(tokens, "existing_workflow");

    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_snippet_generates_valid_code() {
    let object_info = load_object_info();
    let tokens = convert_to_rust_tokens(SIMPLE_WORKFLOW, &object_info).expect("Conversion failed");

    // Verify the snippet matches expected TokenStream
    let actual = format_tokens_as_snippet(tokens);
    let expected = format_tokens_as_snippet(quote! {
        let g = WorkflowGraph::new();

        let checkpoint_loader_simple = g.add(CheckpointLoaderSimple {
            ckpt_name: "model.safetensors"
        });
    });
    assert_eq!(actual, expected);
}
