//! Tests for Lua code generation.
//!
//! These tests verify the output of the Lua generator using AST comparison.

#![cfg(feature = "lua")]

use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::convert_to_lua;

fn load_object_info() -> ObjectInfo {
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
    let non_empty_lines: Vec<&str> = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .copied()
        .collect();

    if non_empty_lines.is_empty() {
        return String::new();
    }

    let min_indent = non_empty_lines
        .iter()
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

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
    let dedented = dedent(code);
    let ast = full_moon::parse(&dedented).expect("Failed to parse Lua code");
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
fn test_simple_checkpoint_loader() {
    let object_info = load_object_info();
    let workflow = r#"{
        "1": {
            "inputs": { "ckpt_name": "model.safetensors" },
            "class_type": "CheckpointLoaderSimple"
        }
    }"#;

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_clip_text_encode_with_reference() {
    let object_info = load_object_info();
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

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
        local clip_text_encode = g:CLIPTextEncode {
            clip = checkpoint_loader_simple.clip,
            text = "a cat",
        }
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_empty_latent_image() {
    let object_info = load_object_info();
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

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local empty_latent_image = g:EmptyLatentImage {
            batch_size = 1.0,
            height = 1024.0,
            width = 1024.0,
        }
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_ksampler() {
    let object_info = load_object_info();
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

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
        local empty_latent_image = g:EmptyLatentImage {
            batch_size = 1.0,
            height = 512.0,
            width = 512.0,
        }
        local clip_text_encode = g:CLIPTextEncode {
            clip = checkpoint_loader_simple.clip,
            text = "a cat",
        }
        local k_sampler = g:KSampler {
            cfg = 8.0,
            denoise = 1.0,
            latent_image = empty_latent_image,
            model = checkpoint_loader_simple.model,
            negative = clip_text_encode,
            positive = clip_text_encode,
            sampler_name = "euler",
            scheduler = "normal",
            seed = 0.0,
            steps = 20.0,
        }
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_string_with_quotes() {
    let object_info = load_object_info();
    let workflow = r#"{
        "1": {
            "inputs": { "text": "a \"quoted\" string", "clip": ["2", 1] },
            "class_type": "CLIPTextEncode"
        },
        "2": {
            "inputs": { "ckpt_name": "model.safetensors" },
            "class_type": "CheckpointLoaderSimple"
        }
    }"#;

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
        local clip_text_encode = g:CLIPTextEncode {
            clip = checkpoint_loader_simple.clip,
            text = "a \"quoted\" string",
        }
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_multi_output_node_references() {
    let object_info = load_object_info();
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

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

    let expected = r#"
        local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
        local vae_decode = g:VAEDecode {
            samples = checkpoint_loader_simple.model,
            vae = checkpoint_loader_simple.vae,
        }
    "#;

    assert_lua_eq(&code, expected);
}

#[test]
fn test_variable_naming() {
    let object_info = load_object_info();
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

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

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

    assert_lua_eq(&code, expected);
}

/// Test the complete example workflow from testdata/.
#[test]
fn test_example_workflow() {
    let object_info = load_object_info();
    let workflow = include_str!("../testdata/example_workflow.json");

    let code = convert_to_lua(workflow, &object_info).expect("Conversion failed");

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

    assert_lua_eq(&code, expected);
}
