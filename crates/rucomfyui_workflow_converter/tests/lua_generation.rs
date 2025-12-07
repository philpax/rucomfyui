//! Tests for Lua code generation.
//!
//! These tests verify the output of the Lua generator.

#![cfg(feature = "lua")]

use rucomfyui_workflow_converter::{
    convert_to_lua, convert_to_lua_with_config, LuaGeneratorConfig,
};
use std::collections::HashSet;

fn extract_lua_lines(code: &str) -> HashSet<String> {
    code.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with("--"))
        .collect()
}

#[test]
fn test_simple_checkpoint_loader() {
    let workflow = r#"{
        "1": {
            "inputs": { "ckpt_name": "model.safetensors" },
            "class_type": "CheckpointLoaderSimple"
        }
    }"#;

    let code = convert_to_lua(workflow).expect("Conversion failed");
    assert_eq!(
        code.trim(),
        r#"local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")"#
    );
}

#[test]
fn test_clip_text_encode_with_reference() {
    let workflow = r#"{
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

    let code = convert_to_lua(workflow).expect("Conversion failed");
    let lines = extract_lua_lines(&code);

    // Check checkpoint loader
    assert!(lines.contains(
        r#"local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")"#
    ));

    // Check CLIPTextEncode
    assert!(lines.contains("local clip_text_encode = g:CLIPTextEncode {"));
    assert!(lines.contains(r#"text = "a cat","#));
    assert!(lines.contains("clip = checkpoint_loader_simple.clip,"));
    assert!(lines.contains("}"));
}

#[test]
fn test_empty_latent_image() {
    let workflow = r#"{
        "1": {
            "inputs": {
                "width": 1024,
                "height": 1024,
                "batch_size": 1
            },
            "class_type": "EmptyLatentImage"
        }
    }"#;

    let code = convert_to_lua(workflow).expect("Conversion failed");
    let lines = extract_lua_lines(&code);

    assert!(lines.contains("local empty_latent_image = g:EmptyLatentImage {"));
    assert!(lines.contains("width = 1024.0,"));
    assert!(lines.contains("height = 1024.0,"));
    assert!(lines.contains("batch_size = 1.0,"));
    assert!(lines.contains("}"));
}

#[test]
fn test_ksampler() {
    let workflow = r#"{
        "1": {
            "inputs": { "ckpt_name": "model.safetensors" },
            "class_type": "CheckpointLoaderSimple"
        },
        "2": {
            "inputs": { "width": 512, "height": 512, "batch_size": 1 },
            "class_type": "EmptyLatentImage"
        },
        "3": {
            "inputs": { "text": "a cat", "clip": ["1", 1] },
            "class_type": "CLIPTextEncode"
        },
        "4": {
            "inputs": {
                "model": ["1", 0],
                "seed": 0,
                "steps": 20,
                "cfg": 8.0,
                "sampler_name": "euler",
                "scheduler": "normal",
                "positive": ["3", 0],
                "negative": ["3", 0],
                "latent_image": ["2", 0],
                "denoise": 1.0
            },
            "class_type": "KSampler"
        }
    }"#;

    let code = convert_to_lua(workflow).expect("Conversion failed");
    let lines = extract_lua_lines(&code);

    // Verify the structure
    assert!(lines.contains(
        r#"local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")"#
    ));
    assert!(lines.contains("local empty_latent_image = g:EmptyLatentImage {"));
    assert!(lines.contains("local clip_text_encode = g:CLIPTextEncode {"));
    assert!(lines.contains("local k_sampler = g:KSampler {"));

    // Verify references
    assert!(lines.contains("model = checkpoint_loader_simple.model,"));
    assert!(lines.contains("latent_image = empty_latent_image,"));
    assert!(lines.contains("positive = clip_text_encode,"));
}

#[test]
fn test_complete_workflow_with_boilerplate() {
    let workflow = r#"{
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
    let code = convert_to_lua_with_config(workflow, &config).expect("Conversion failed");

    // Check boilerplate lines
    assert!(code.contains("local object_info = client:get_object_info()"));
    assert!(code.contains("local g = comfy.graph(object_info)"));

    // Check execution
    assert!(code.contains("local result = client:easy_queue(g)"));
    assert!(code.contains("return result[preview_image]"));
}

#[test]
fn test_string_with_quotes() {
    // The JSON parser will unescape the string, so we use escaped quotes in JSON
    let workflow = r#"{
        "1": {
            "inputs": { "text": "a \"quoted\" string" },
            "class_type": "CLIPTextEncode"
        }
    }"#;

    let code = convert_to_lua(workflow).expect("Conversion failed");
    // The Lua generator should escape the quotes
    assert!(
        code.contains(r#"\"quoted\""#),
        "Code should contain escaped quotes: {}",
        code
    );
}

#[test]
fn test_multi_output_node_references() {
    let workflow = r#"{
        "1": {
            "inputs": { "ckpt_name": "model.safetensors" },
            "class_type": "CheckpointLoaderSimple"
        },
        "2": {
            "inputs": { "samples": ["1", 0], "vae": ["1", 2] },
            "class_type": "VAEDecode"
        }
    }"#;

    let code = convert_to_lua(workflow).expect("Conversion failed");
    let lines = extract_lua_lines(&code);

    // Model is slot 0, VAE is slot 2
    // Since CheckpointLoaderSimple has multiple outputs, we should use named fields
    assert!(lines.contains("samples = checkpoint_loader_simple.model,"));
    assert!(lines.contains("vae = checkpoint_loader_simple.vae,"));
}

#[test]
fn test_variable_naming() {
    let workflow = r#"{
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

    let code = convert_to_lua(workflow).expect("Conversion failed");

    // Should have two different variable names for the two CLIPTextEncode nodes
    assert!(code.contains("local clip_text_encode = g:CLIPTextEncode"));
    assert!(code.contains("local clip_text_encode_1 = g:CLIPTextEncode"));
}
