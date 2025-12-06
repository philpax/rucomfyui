//! Integration tests that verify generated Rust code compiles correctly.
//!
//! These tests convert workflow JSON to Rust code and then verify the code
//! compiles by creating a temporary crate and running `cargo check`.

use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::{convert_to_rust_with_object_info, RustGeneratorConfig};
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

/// Example workflow JSON for testing.
const EXAMPLE_WORKFLOW: &str = r#"{
    "1": {
        "inputs": { "ckpt_name": "model.safetensors" },
        "class_type": "CheckpointLoaderSimple",
        "_meta": { "title": "Load Checkpoint" }
    },
    "2": {
        "inputs": {
            "text": "a beautiful landscape",
            "clip": ["1", 1]
        },
        "class_type": "CLIPTextEncode",
        "_meta": { "title": "Positive Prompt" }
    },
    "3": {
        "inputs": {
            "text": "ugly, blurry",
            "clip": ["1", 1]
        },
        "class_type": "CLIPTextEncode",
        "_meta": { "title": "Negative Prompt" }
    },
    "4": {
        "inputs": {
            "width": 1024,
            "height": 1024,
            "batch_size": 1
        },
        "class_type": "EmptyLatentImage",
        "_meta": { "title": "Empty Latent" }
    },
    "5": {
        "inputs": {
            "model": ["1", 0],
            "seed": 42,
            "steps": 20,
            "cfg": 7.5,
            "sampler_name": "euler",
            "scheduler": "normal",
            "positive": ["2", 0],
            "negative": ["3", 0],
            "latent_image": ["4", 0],
            "denoise": 1.0
        },
        "class_type": "KSampler",
        "_meta": { "title": "Sampler" }
    },
    "6": {
        "inputs": {
            "samples": ["5", 0],
            "vae": ["1", 2]
        },
        "class_type": "VAEDecode",
        "_meta": { "title": "VAE Decode" }
    },
    "7": {
        "inputs": {
            "images": ["6", 0]
        },
        "class_type": "PreviewImage",
        "_meta": { "title": "Preview" }
    }
}"#;

/// Simple workflow with just a checkpoint loader.
const SIMPLE_WORKFLOW: &str = r#"{
    "1": {
        "inputs": { "ckpt_name": "model.safetensors" },
        "class_type": "CheckpointLoaderSimple"
    }
}"#;

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
    let config = RustGeneratorConfig::complete("example_workflow");
    let code = convert_to_rust_with_object_info(EXAMPLE_WORKFLOW, &object_info, &config)
        .expect("Conversion failed");

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
    let config = RustGeneratorConfig::complete("simple_workflow");
    let code = convert_to_rust_with_object_info(SIMPLE_WORKFLOW, &object_info, &config)
        .expect("Conversion failed");

    // Actually compile the code
    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_existing_workflow_file_compiles() {
    let object_info = load_object_info();
    // Test with the actual example workflow file from rucomfyui
    let workflow_json = include_str!("../../rucomfyui/examples/existing_workflow.json");
    let config = RustGeneratorConfig::complete("existing_workflow");
    let code = convert_to_rust_with_object_info(workflow_json, &object_info, &config)
        .expect("Conversion failed");

    let temp_dir = create_test_crate(&code);
    check_compiles(&temp_dir).expect("Generated code should compile");
}

#[test]
fn test_snippet_mode_generates_valid_code() {
    let object_info = load_object_info();
    let config = RustGeneratorConfig::snippet();
    let code = convert_to_rust_with_object_info(SIMPLE_WORKFLOW, &object_info, &config)
        .expect("Conversion failed");

    // Should not contain function wrapper
    assert!(!code.contains("pub fn"));
    // Should contain the workflow construction
    assert!(code.contains("let g = WorkflowGraph::new()"));
}
