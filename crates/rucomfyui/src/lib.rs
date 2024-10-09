//! A Rust client for ComfyUI with an emphasis on type safety and ergonomics.
#![deny(missing_docs)]

use error::parse_response;
use reqwest::multipart::{Form, Part};

pub mod history;
pub mod object_info;
pub mod queue;
pub mod workflow;

pub use workflow::{Workflow, WorkflowGraph};

#[cfg(feature = "typed_nodes")]
pub mod nodes;

pub mod error;
pub use error::ClientError;

/// Result type for the client.
pub type Result<T> = std::result::Result<T, ClientError>;

/// An alias around `Vec<u8>` for raw bytes.
pub type OwnedBytes = Vec<u8>;

/// An alias around `Cow<'a, [u8]>` for borrowed bytes.
pub type BorrowedBytes<'a> = std::borrow::Cow<'a, [u8]>;

/// Client for the ComfyUI API.
pub struct Client {
    api_base: String,
    client: reqwest::Client,
}
impl Client {
    /// Create a new client with the default [`reqwest`] client.
    pub fn new(api_base: impl Into<String>) -> Self {
        Self::new_with_client(api_base, reqwest::Client::default())
    }
    /// Create a new client with a custom [`reqwest`] client.
    /// This is useful for setting custom timeouts, headers, etc.
    pub fn new_with_client(api_base: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            api_base: api_base.into(),
            client,
        }
    }
}
impl Client {
    /// Upload a file to the ComfyUI API.
    pub async fn upload(
        &self,
        filename: &str,
        bytes: impl Into<BorrowedBytes<'static>>,
    ) -> Result<serde_json::Value> {
        let form = Form::new()
            .part("image", Part::bytes(bytes).file_name(filename.to_string()))
            .text("type", "input")
            .text("overwrite", "true");

        parse_response(
            self.client
                .post(format!("{}/upload/image", self.api_base))
                .multipart(form)
                .send()
                .await?,
        )
        .await
    }
}
