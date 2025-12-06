//! Convert ComfyUI API workflows to typed Rust or Lua code.
//!
//! This library takes a ComfyUI API workflow JSON and converts it to either:
//! - Typed Rust code compatible with `rucomfyui`'s `typed_nodes` feature
//! - Lua code compatible with `rucomfyui_mlua`
//!
//! # Example
//!
//! ```no_run
//! use rucomfyui_workflow_converter::{convert_to_rust, convert_to_lua};
//!
//! let workflow_json = r#"{
//!     "1": {
//!         "inputs": { "ckpt_name": "model.safetensors" },
//!         "class_type": "CheckpointLoaderSimple"
//!     }
//! }"#;
//!
//! let rust_code = convert_to_rust(workflow_json).unwrap();
//! let lua_code = convert_to_lua(workflow_json).unwrap();
//! ```

mod lua_generator;
mod rust_generator;
mod workflow_analyzer;

pub use lua_generator::{convert_to_lua, convert_to_lua_with_config, LuaGeneratorConfig};
pub use rust_generator::{
    convert_to_rust, convert_to_rust_with_config, convert_to_rust_with_object_info,
    RustGeneratorConfig,
};
pub use workflow_analyzer::{AnalyzedNode, AnalyzedWorkflow};

use thiserror::Error;

/// Errors that can occur during workflow conversion.
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to parse workflow JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Workflow contains a cycle")]
    CyclicWorkflow,

    #[error("Invalid node reference: {0}")]
    InvalidNodeReference(String),

    #[error("Node {0} not found in workflow")]
    NodeNotFound(String),
}

pub type Result<T> = std::result::Result<T, ConversionError>;
