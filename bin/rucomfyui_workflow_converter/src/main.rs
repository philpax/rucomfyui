//! CLI tool to convert ComfyUI API workflows to Rust or Lua code.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::{
    convert_to_lua_with_object_info, convert_to_rust_with_object_info, LuaGeneratorConfig,
    RustGeneratorConfig,
};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Bundled object_info.json from generate_nodes
const BUNDLED_OBJECT_INFO: &str =
    include_str!("../../../crates/rucomfyui/generate_nodes/object_info.json");

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum OutputFormat {
    #[default]
    Rust,
    Lua,
}

/// Convert ComfyUI API workflows to Rust or Lua code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(after_help = "EXAMPLES:
    # Convert to Rust with typed nodes (uses bundled object_info)
    rucomfyui_workflow_converter workflow.json

    # Convert to Rust with full wrapper function
    rucomfyui_workflow_converter workflow.json --complete

    # Convert to Lua with full boilerplate
    rucomfyui_workflow_converter workflow.json --format lua --complete

    # Use custom object_info.json
    rucomfyui_workflow_converter workflow.json --object-info custom_object_info.json

    # Output to file
    rucomfyui_workflow_converter workflow.json --complete -o workflow.rs")]
struct Args {
    /// Path to the ComfyUI API workflow JSON file
    input: PathBuf,

    /// Path to object_info.json (uses bundled version if not specified)
    #[arg(long, value_name = "FILE")]
    object_info: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Rust)]
    format: OutputFormat,

    /// Include boilerplate/wrapper code
    #[arg(short, long)]
    complete: bool,

    /// Write to file instead of stdout
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read input
    let json = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input.display()))?;

    // Load ObjectInfo (from file or bundled)
    let object_info = match &args.object_info {
        Some(path) => load_object_info_from_file(path)?,
        None => load_bundled_object_info()?,
    };

    // Convert
    let output = match args.format {
        OutputFormat::Lua => {
            let config = if args.complete {
                LuaGeneratorConfig::complete()
            } else {
                LuaGeneratorConfig::snippet()
            };
            convert_to_lua_with_object_info(&json, &object_info, &config)?
        }
        OutputFormat::Rust => {
            let config = if args.complete {
                RustGeneratorConfig::complete("workflow")
            } else {
                RustGeneratorConfig::snippet()
            };
            convert_to_rust_with_object_info(&json, &object_info, &config)?
        }
    };

    // Write output
    if let Some(output_path) = &args.output {
        fs::write(output_path, &output)
            .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;
        eprintln!("Wrote output to: {}", output_path.display());
    } else {
        io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}

fn load_bundled_object_info() -> Result<ObjectInfo> {
    let objects: Vec<rucomfyui::object_info::Object> =
        serde_json::from_str(BUNDLED_OBJECT_INFO).context("Failed to parse bundled object_info")?;

    Ok(objects.into_iter().map(|o| (o.name.clone(), o)).collect())
}

fn load_object_info_from_file(path: &PathBuf) -> Result<ObjectInfo> {
    let json = fs::read_to_string(path)
        .with_context(|| format!("Failed to read object_info file: {}", path.display()))?;

    let objects: Vec<rucomfyui::object_info::Object> =
        serde_json::from_str(&json).with_context(|| "Failed to parse object_info.json")?;

    Ok(objects.into_iter().map(|o| (o.name.clone(), o)).collect())
}
