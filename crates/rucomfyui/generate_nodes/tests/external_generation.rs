//! Integration test verifying that `rucomfyui_generate_nodes` can produce code
//! that compiles in an **external** crate (i.e. outside `rucomfyui` itself).
//!
//! This test fabricates custom ComfyUI node definitions, generates typed Rust
//! bindings with `base_crate_path: "rucomfyui"`, creates a temporary crate
//! that depends on `rucomfyui`, and runs `cargo check` to verify compilation.
//! The temporary crate's `lib.rs` exercises bidirectional interoperability
//! between custom-generated nodes and `rucomfyui`'s built-in typed nodes.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use rucomfyui::object_info::{
    Object, ObjectInput, ObjectInputBundle, ObjectInputMeta, ObjectInputMetaTyped,
    ObjectInputMetaTypedNumber, ObjectInputMetaTypedNumberValue, ObjectInputMetaTypedString,
    ObjectInputType, ObjectType,
};
use rucomfyui_generate_nodes::{GenerateConfig, generate_nodes_with_config};

/// Verifies `GenerateConfig::default().base_crate_path == "crate"`. (AC.1)
#[test]
fn default_base_crate_path_is_crate() {
    assert_eq!(GenerateConfig::default().base_crate_path, "crate");
}

// ---------------------------------------------------------------------------
// Helpers for building `Object` definitions with native structs.
// ---------------------------------------------------------------------------

/// Build a typed input (e.g. `["MODEL", { tooltip }]`) with a tooltip.
fn typed_input(ty: ObjectType, tooltip: &str) -> ObjectInput {
    ObjectInput::InputWithMeta(
        ObjectInputType::Typed(ty),
        ObjectInputMeta {
            tooltip: Some(tooltip.into()),
            typed: None,
        },
    )
}

/// Build an INT input with min/max/default/step metadata.
fn int_input(tooltip: &str, default: i64, min: i64, max: i64) -> ObjectInput {
    ObjectInput::InputWithMeta(
        ObjectInputType::Typed(ObjectType::Int),
        ObjectInputMeta {
            tooltip: Some(tooltip.into()),
            typed: Some(ObjectInputMetaTyped::Number(ObjectInputMetaTypedNumber {
                default: ObjectInputMetaTypedNumberValue::I64(default),
                display: None,
                max: ObjectInputMetaTypedNumberValue::I64(max),
                min: ObjectInputMetaTypedNumberValue::I64(min),
                round: None,
                step: None,
            })),
        },
    )
}

/// Build a STRING input with a default value.
fn string_input(tooltip: &str, default: &str) -> ObjectInput {
    ObjectInput::InputWithMeta(
        ObjectInputType::Typed(ObjectType::String),
        ObjectInputMeta {
            tooltip: Some(tooltip.into()),
            typed: Some(ObjectInputMetaTyped::String(ObjectInputMetaTypedString {
                dynamic_prompts: None,
                multiline: None,
                default: Some(default.into()),
            })),
        },
    )
}

/// Build an array-of-strings input (e.g. `["model.safetensors"]` with no metadata).
fn array_input() -> ObjectInput {
    ObjectInput::Input((ObjectInputType::Array(vec![]),))
}

/// Convenience: insert a `(name, input)` pair into a `BTreeMap`.
fn ins(map: &mut BTreeMap<String, ObjectInput>, name: &str, input: ObjectInput) {
    map.insert(name.into(), input);
}

// ---------------------------------------------------------------------------
// Fabricated custom node definitions.
// ---------------------------------------------------------------------------

/// Fabricate custom node definitions using standard ComfyUI types.
///
/// These nodes use only standard `ObjectType` variants (MODEL, CLIP, VAE,
/// CONDITIONING, LATENT, IMAGE, INT, STRING) to avoid the `ObjectType::Other`
/// panic in `object_type_out_struct_ident`.
fn custom_nodes() -> Vec<Object> {
    // --- MyCustomLoader ---
    let mut loader_inputs = BTreeMap::new();
    ins(&mut loader_inputs, "ckpt_name", array_input());

    let loader = Object {
        name: "MyCustomLoader".into(),
        display_name: Some("My Custom Loader".into()),
        description: "A custom loader that outputs MODEL, CLIP, and VAE.".into(),
        python_module: "custom_nodes.my_custom".into(),
        category: "custom/loaders".into(),
        input: ObjectInputBundle {
            required: loader_inputs,
            optional: None,
        },
        input_order: ObjectInputBundle {
            required: vec!["ckpt_name".into()],
            optional: None,
        },
        output: vec![ObjectType::Model, ObjectType::Clip, ObjectType::Vae],
        output_is_list: vec![None, None, None],
        output_name: vec!["MODEL".into(), "CLIP".into(), "VAE".into()],
        output_node: false,
        output_tooltips: vec![
            Some("The model used for denoising.".into()),
            Some("The CLIP model for text encoding.".into()),
            Some("The VAE model for encoding/decoding.".into()),
        ],
    };

    // --- MyCustomSampler ---
    let mut sampler_inputs = BTreeMap::new();
    ins(
        &mut sampler_inputs,
        "model",
        typed_input(ObjectType::Model, "The model to use for sampling."),
    );
    ins(
        &mut sampler_inputs,
        "positive",
        typed_input(ObjectType::Conditioning, "Positive conditioning."),
    );
    ins(
        &mut sampler_inputs,
        "negative",
        typed_input(ObjectType::Conditioning, "Negative conditioning."),
    );
    ins(
        &mut sampler_inputs,
        "latent_image",
        typed_input(ObjectType::Latent, "The latent to sample."),
    );
    ins(
        &mut sampler_inputs,
        "steps",
        int_input("Number of sampling steps.", 20, 1, 100),
    );

    let sampler = Object {
        name: "MyCustomSampler".into(),
        display_name: Some("My Custom Sampler".into()),
        description: "A custom sampler that takes MODEL, two CONDITIONINGs, LATENT, and INT."
            .into(),
        python_module: "custom_nodes.my_custom".into(),
        category: "custom/sampling".into(),
        input: ObjectInputBundle {
            required: sampler_inputs,
            optional: None,
        },
        input_order: ObjectInputBundle {
            required: vec![
                "model".into(),
                "positive".into(),
                "negative".into(),
                "latent_image".into(),
                "steps".into(),
            ],
            optional: None,
        },
        output: vec![ObjectType::Latent],
        output_is_list: vec![None],
        output_name: vec!["LATENT".into()],
        output_node: false,
        output_tooltips: vec![Some("The sampled latent.".into())],
    };

    // --- MyCustomSaveImage (output node) ---
    let mut save_inputs = BTreeMap::new();
    ins(
        &mut save_inputs,
        "images",
        typed_input(ObjectType::Image, "The images to save."),
    );
    ins(
        &mut save_inputs,
        "filename_prefix",
        string_input("Prefix for the saved filename.", "custom"),
    );

    let save = Object {
        name: "MyCustomSaveImage".into(),
        display_name: Some("My Custom Save Image".into()),
        description: "A custom save image node (output node).".into(),
        python_module: "custom_nodes.my_custom".into(),
        category: "custom".into(),
        input: ObjectInputBundle {
            required: save_inputs,
            optional: None,
        },
        input_order: ObjectInputBundle {
            required: vec!["images".into(), "filename_prefix".into()],
            optional: None,
        },
        output: vec![],
        output_is_list: vec![],
        output_name: vec![],
        output_node: true,
        output_tooltips: vec![],
    };

    vec![loader, sampler, save]
}

// ---------------------------------------------------------------------------
// Temp crate creation and compilation.
// ---------------------------------------------------------------------------

/// The `lib.rs` content for the temp crate.
///
/// This exercises bidirectional interoperability between custom-generated nodes
/// and `rucomfyui`'s built-in typed nodes:
///
/// 1. Custom → builtin: `MyCustomLoader.clip` → `CLIPTextEncode`
/// 2. Builtin → custom: `EmptyLatentImage` → `MyCustomSampler`
/// 3. Custom → custom: `MyCustomLoader.model` → `MyCustomSampler`
/// 4. Custom → builtin → custom: `MyCustomLoader.vae` → `VAEDecode` → `MyCustomSaveImage`
const TEMP_CRATE_LIB_RS: &str = r#"//! Temporary crate to verify external node generation compiles.

use rucomfyui::{Workflow, WorkflowGraph, WorkflowNodeId};
// Built-in nodes from rucomfyui
use rucomfyui::nodes::all::*;
// Custom-generated nodes (external generation)
mod custom_nodes;
use custom_nodes::all::*;

/// Builds a workflow exercising bidirectional interoperability
/// between custom nodes and built-in rucomfyui nodes.
pub fn build_workflow() -> (Workflow, WorkflowNodeId) {
    let g = WorkflowGraph::new();

    // Custom loader: outputs MODEL, CLIP, VAE
    let loader = g.add(MyCustomLoader {
        ckpt_name: "model.safetensors"
    });

    // 1. Custom → builtin: MyCustomLoader.clip → CLIPTextEncode
    let positive = g.add(CLIPTextEncode {
        clip: loader.clip,
        text: "a beautiful landscape"
    });
    let negative = g.add(CLIPTextEncode {
        clip: loader.clip,
        text: "ugly, blurry"
    });

    // 2. Builtin → custom: EmptyLatentImage → MyCustomSampler
    let empty_latent = g.add(EmptyLatentImage {
        width: 512,
        height: 512,
        batch_size: 1
    });

    // 3. Custom → custom: MyCustomLoader.model → MyCustomSampler
    let sampled = g.add(MyCustomSampler {
        model: loader.model,
        positive,
        negative,
        latent_image: empty_latent,
        steps: 20
    });

    // 4. Custom → builtin → custom:
    //    MyCustomLoader.vae → VAEDecode → MyCustomSaveImage
    let decoded = g.add(VAEDecode {
        samples: sampled,
        vae: loader.vae
    });

    let saved = g.add(MyCustomSaveImage {
        images: decoded,
        filename_prefix: "custom_output"
    });

    (g.into_workflow(), saved)
}
"#;

/// Creates a temporary crate depending on `rucomfyui`, with the generated
/// custom nodes copied in.
fn create_test_crate(generated_dir: &Path) -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // `generate_nodes` is nested at `crates/rucomfyui/generate_nodes`,
    // so `manifest_dir.parent()` gives us the `rucomfyui` crate root directly.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rucomfyui_path = manifest_dir.parent().unwrap();

    // Write Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "test_external_generation"
version = "0.1.0"
edition = "2021"

[dependencies]
rucomfyui = {{ path = "{}", default-features = false, features = ["typed_nodes"] }}
"#,
        rucomfyui_path.display()
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    // Copy the generated custom_nodes directory into the temp crate's src/
    let dest_nodes = src_dir.join("custom_nodes");
    copy_dir_recursive(generated_dir, &dest_nodes);

    // Write lib.rs
    fs::write(src_dir.join("lib.rs"), TEMP_CRATE_LIB_RS).expect("Failed to write lib.rs");

    temp_dir
}

/// Recursively copies a directory's contents.
fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("Failed to create destination directory");
    for entry in fs::read_dir(src).expect("Failed to read source directory") {
        let entry = entry.expect("Failed to read directory entry");
        let file_type = entry.file_type().expect("Failed to get file type");
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path);
        } else {
            fs::copy(entry.path(), dest_path).expect("Failed to copy file");
        }
    }
}

/// Runs `cargo check` in the temp crate and returns the result.
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

/// Verifies that external node generation produces compilable code that is
/// fully interoperable with `rucomfyui`'s built-in typed nodes. (AC.2–AC.5)
#[test]
fn test_external_generation_compiles() {
    let nodes = custom_nodes();

    // Generate into a temp directory with base_crate_path = "rucomfyui"
    let temp_gen = tempfile::tempdir().expect("Failed to create generation temp dir");
    let config = GenerateConfig {
        base_crate_path: "rucomfyui",
        ..Default::default()
    };
    generate_nodes_with_config(&nodes, temp_gen.path(), config).expect("Generation failed");

    // AC.2: types.rs should NOT be generated for external use
    assert!(
        !temp_gen.path().join("types.rs").exists(),
        "types.rs should not be generated when base_crate_path is 'rucomfyui'"
    );

    // Verify mod.rs was generated (it should exist, just without trait definitions)
    assert!(
        temp_gen.path().join("mod.rs").exists(),
        "mod.rs should be generated"
    );

    // Verify all.rs was generated
    assert!(
        temp_gen.path().join("all.rs").exists(),
        "all.rs should be generated"
    );

    // Verify mod.rs does not contain TypedNode trait definition (AC.2)
    let mod_rs = fs::read_to_string(temp_gen.path().join("mod.rs"))
        .expect("Failed to read generated mod.rs");
    assert!(
        !mod_rs.contains("pub trait TypedNode"),
        "mod.rs should not define TypedNode trait for external generation"
    );
    assert!(
        !mod_rs.contains("pub trait TypedOutputNode"),
        "mod.rs should not define TypedOutputNode trait for external generation"
    );

    // AC.3: Verify generated code uses rucomfyui:: paths, not crate::
    assert!(
        mod_rs.contains("rucomfyui::"),
        "mod.rs should use rucomfyui:: paths for external generation"
    );
    assert!(
        !mod_rs.contains("crate::"),
        "mod.rs should not use crate:: paths for external generation"
    );

    // Create a temp crate and compile it
    let temp_crate = create_test_crate(temp_gen.path());

    // AC.4, AC.5: cargo check verifies super:: paths work externally and
    // bidirectional interoperability type-checks
    check_compiles(&temp_crate).expect("Generated external code should compile");
}
