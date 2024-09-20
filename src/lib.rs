//! A Rust client for ComfyUI, with an emphasis on type safety and ergonomy.
#![deny(missing_docs)]

use reqwest::multipart::{Form, Part};
use thiserror::Error;

pub mod history;
pub mod object_info;
pub mod queue;
pub mod workflow;

pub use workflow::{Workflow, WorkflowGraph};

#[cfg(feature = "typed_nodes")]
pub mod nodes;

#[derive(Error, Debug)]
/// Errors that can occur when using the client.
pub enum ClientError {
    #[error("reqwest error: {0}")]
    /// Reqwest error.
    Reqwest(#[from] reqwest::Error),
}
/// Result type for the client.
pub type Result<T> = std::result::Result<T, ClientError>;

/// An alias around `Vec<u8>` for raw bytes.
pub type Bytes = Vec<u8>;

/// Client for the ComfyUI API.
pub struct Client {
    api_base: String,
    client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    pub fn new(api_base: impl Into<String>) -> Self {
        Self {
            api_base: api_base.into(),
            client: reqwest::Client::new(),
        }
    }
}
impl Client {
    /// Upload a file to the ComfyUI API.
    pub async fn upload(&self, filename: &str, bytes: Bytes) -> Result<serde_json::Value> {
        let form = Form::new()
            .part("image", Part::bytes(bytes).file_name(filename.to_string()))
            .text("type", "input")
            .text("overwrite", "true");

        Ok(self
            .client
            .post(format!("{}/upload/image", self.api_base))
            .multipart(form)
            .send()
            .await?
            .json()
            .await?)
    }
}
