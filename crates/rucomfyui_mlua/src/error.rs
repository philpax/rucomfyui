//! Error types for rucomfyui_mlua.

use mlua::prelude::*;

/// Errors that can occur in rucomfyui_mlua.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A rucomfyui client error.
    #[error("ComfyUI client error: {0}")]
    Client(#[from] rucomfyui::ClientError),

    /// An unknown node type was requested.
    #[error("Unknown node type: {0}")]
    UnknownNode(String),

    /// A required input was missing.
    #[error("Missing required input '{input}' for node '{node}'")]
    MissingInput { node: String, input: String },

    /// An input had an invalid type.
    #[error("Invalid type for input '{input}' on node '{node}': expected {expected}, got {got}")]
    InvalidInputType {
        node: String,
        input: String,
        expected: String,
        got: String,
    },

    /// An output field was not found.
    #[error("Output '{output}' not found on node '{node}'")]
    OutputNotFound { node: String, output: String },

    /// A Lua error occurred.
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),
}

impl From<Error> for mlua::Error {
    fn from(err: Error) -> Self {
        mlua::Error::external(err)
    }
}

/// Convert a rucomfyui Result to a Lua Result.
pub fn to_lua_result<T>(result: rucomfyui::Result<T>) -> LuaResult<T> {
    result.map_err(|e| mlua::Error::external(Error::Client(e)))
}
