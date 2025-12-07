//! Integration tests that verify generated Rust code compiles correctly.
//!
//! These tests convert workflow JSON to Rust code and then verify the code
//! compiles by creating a temporary crate and running `cargo check`.

#![cfg(feature = "rust")]

use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::convert_to_rust;
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

/// Wrap generated code snippet in a compilable module.
fn wrap_in_module(snippet: &str, function_name: &str) -> String {
    format!(
        r#"//! Generated workflow code from ComfyUI API workflow.

use rucomfyui::{{Workflow, WorkflowGraph, WorkflowNodeId}};
use rucomfyui::nodes::all::*;

/// Constructs the workflow.
#[allow(unused_variables)]
pub fn {function_name}() -> (Workflow, WorkflowNodeId) {{
    {snippet}

    (g.into_workflow(), WorkflowNodeId(0))
}}
"#
    )
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
rucomfyui = {{ path = "{}" }}
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
    let snippet = convert_to_rust(EXAMPLE_WORKFLOW, &object_info).expect("Conversion failed");
    let code = wrap_in_module(&snippet, "example_workflow");

    // Verify it contains expected elements
    assert!(code.contains("pub fn example_workflow()"));
    assert!(code.contains("CheckpointLoaderSimple"));
    assert!(code.contains("CLIPTextEncode"));
    assert!(code.contains("EmptyLatentImage"));
    assert!(code.contains("KSampler"));
    assert!(code.contains("VAEDecode"));
    assert!(code.contains("PreviewImage"));

    // Actually compile the code
    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_simple_workflow_compiles() {
    let object_info = load_object_info();
    let snippet = convert_to_rust(SIMPLE_WORKFLOW, &object_info).expect("Conversion failed");
    let code = wrap_in_module(&snippet, "simple_workflow");

    // Actually compile the code
    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_existing_workflow_file_compiles() {
    let object_info = load_object_info();
    // Test with the actual example workflow file from rucomfyui
    let workflow_json = include_str!("../../rucomfyui/examples/existing_workflow.json");
    let snippet = convert_to_rust(workflow_json, &object_info).expect("Conversion failed");
    let code = wrap_in_module(&snippet, "existing_workflow");

    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_snippet_generates_valid_code() {
    let object_info = load_object_info();
    let code = convert_to_rust(SIMPLE_WORKFLOW, &object_info).expect("Conversion failed");

    // Should not contain function wrapper
    assert!(!code.contains("pub fn"));
    // Should contain the workflow construction
    assert!(code.contains("let g = WorkflowGraph::new()"));
}
