//! CLI tool to convert ComfyUI API workflows to Rust or Lua code.
//!
//! Usage:
//!   rucomfyui_workflow_converter <input.json> [OPTIONS]
//!
//! Options:
//!   --rust           Output Rust code (default)
//!   --lua            Output Lua code
//!   --complete       Include boilerplate/wrapper code
//!   --object-info    Path to object_info.json for typed Rust output
//!   --output, -o     Write to file instead of stdout
//!   --help           Show this help

use anyhow::{Context, Result};
use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::{
    convert_to_lua_with_config, convert_to_rust_with_config, convert_to_rust_with_object_info,
    LuaGeneratorConfig, RustGeneratorConfig,
};
use std::fs;
use std::io::{self, Write};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return Ok(());
    }

    let input_file = &args[1];
    let use_lua = args.iter().any(|a| a == "--lua");
    let complete = args.iter().any(|a| a == "--complete");

    // Find output file if specified
    let output_file = args
        .iter()
        .position(|a| a == "--output" || a == "-o")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    // Find object_info file if specified
    let object_info_file = args
        .iter()
        .position(|a| a == "--object-info")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    // Read input
    let json = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to read input file: {}", input_file))?;

    // Convert
    let output = if use_lua {
        let config = if complete {
            LuaGeneratorConfig::complete()
        } else {
            LuaGeneratorConfig::snippet()
        };
        convert_to_lua_with_config(&json, &config)?
    } else {
        let config = if complete {
            RustGeneratorConfig::complete("workflow")
        } else {
            RustGeneratorConfig::snippet()
        };

        if let Some(oi_path) = object_info_file {
            // Load ObjectInfo and use typed conversion
            let object_info = load_object_info(oi_path)?;
            convert_to_rust_with_object_info(&json, &object_info, &config)?
        } else {
            // Use dynamic conversion
            convert_to_rust_with_config(&json, &config)?
        }
    };

    // Write output
    if let Some(output_path) = output_file {
        fs::write(output_path, &output)
            .with_context(|| format!("Failed to write output file: {}", output_path))?;
        eprintln!("Wrote output to: {}", output_path);
    } else {
        io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}

fn load_object_info(path: &str) -> Result<ObjectInfo> {
    let json = fs::read_to_string(path)
        .with_context(|| format!("Failed to read object_info file: {}", path))?;

    let objects: Vec<rucomfyui::object_info::Object> =
        serde_json::from_str(&json).with_context(|| "Failed to parse object_info.json")?;

    Ok(objects.into_iter().map(|o| (o.name.clone(), o)).collect())
}

fn print_help() {
    eprintln!(
        r#"rucomfyui_workflow_converter - Convert ComfyUI API workflows to Rust or Lua code

USAGE:
    rucomfyui_workflow_converter <input.json> [OPTIONS]

ARGUMENTS:
    <input.json>    Path to the ComfyUI API workflow JSON file

OPTIONS:
    --rust           Output Rust code (default)
    --lua            Output Lua code
    --complete       Include boilerplate/wrapper code
    --object-info    Path to object_info.json for typed Rust output
                     (enables type-aware code generation)
    --output, -o     Write to file instead of stdout
    --help, -h       Show this help

EXAMPLES:
    # Convert to Rust using dynamic nodes
    rucomfyui_workflow_converter workflow.json

    # Convert to Rust with typed nodes using object_info
    rucomfyui_workflow_converter workflow.json --object-info object_info.json --complete

    # Convert to Lua with full boilerplate
    rucomfyui_workflow_converter workflow.json --lua --complete

    # Output to file
    rucomfyui_workflow_converter workflow.json --complete -o workflow.rs
"#
    );
}
