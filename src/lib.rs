//! Very basic ComfyUI API client for Rust.
#![deny(missing_docs)]

use std::collections::HashMap;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod history;
mod queue;
mod workflow;
pub use workflow::*;

#[derive(Error, Debug)]
/// Errors that can occur when using the client.
pub enum ClientError {
    #[error("reqwest error: {0}")]
    /// Reqwest error.
    Reqwest(#[from] reqwest::Error),
}
/// Result type for the client.
pub type Result<T> = std::result::Result<T, ClientError>;
/// Bytes type for the client.
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
