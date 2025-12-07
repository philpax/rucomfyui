//! CLI tool to convert ComfyUI API workflows to Rust or Lua code.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use rucomfyui::object_info::ObjectInfo;
use rucomfyui_workflow_converter::{
    convert_to_lua_with_config, convert_to_rust_with_config, convert_to_rust_with_object_info,
    LuaGeneratorConfig, RustGeneratorConfig,
};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

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
    # Convert to Rust using dynamic nodes
    rucomfyui_workflow_converter workflow.json

    # Convert to Rust with typed nodes using object_info
    rucomfyui_workflow_converter workflow.json --object-info object_info.json --complete

    # Convert to Lua with full boilerplate
    rucomfyui_workflow_converter workflow.json --format lua --complete

    # Output to file
    rucomfyui_workflow_converter workflow.json --complete -o workflow.rs")]
struct Args {
    /// Path to the ComfyUI API workflow JSON file
    input: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Rust)]
    format: OutputFormat,

    /// Include boilerplate/wrapper code
    #[arg(short, long)]
    complete: bool,

    /// Path to object_info.json for typed Rust output (enables type-aware code generation)
    #[arg(long, value_name = "FILE")]
    object_info: Option<PathBuf>,

    /// Write to file instead of stdout
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read input
    let json = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input.display()))?;

    // Convert
    let output = match args.format {
        OutputFormat::Lua => {
            let config = if args.complete {
                LuaGeneratorConfig::complete()
            } else {
                LuaGeneratorConfig::snippet()
            };
            convert_to_lua_with_config(&json, &config)?
        }
        OutputFormat::Rust => {
            let config = if args.complete {
                RustGeneratorConfig::complete("workflow")
            } else {
                RustGeneratorConfig::snippet()
            };

            if let Some(oi_path) = &args.object_info {
                let object_info = load_object_info(oi_path)?;
                convert_to_rust_with_object_info(&json, &object_info, &config)?
            } else {
                convert_to_rust_with_config(&json, &config)?
            }
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

fn load_object_info(path: &PathBuf) -> Result<ObjectInfo> {
    let json = fs::read_to_string(path)
        .with_context(|| format!("Failed to read object_info file: {}", path.display()))?;

    let objects: Vec<rucomfyui::object_info::Object> =
        serde_json::from_str(&json).with_context(|| "Failed to parse object_info.json")?;

    Ok(objects.into_iter().map(|o| (o.name.clone(), o)).collect())
}
