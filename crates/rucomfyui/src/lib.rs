//! A Rust client for ComfyUI with an emphasis on type safety and ergonomics.
#![deny(missing_docs)]

use reqwest::multipart::{Form, Part};

mod constructor;

pub mod workflow;
pub use workflow::{Workflow, WorkflowGraph, WorkflowNodeId};

#[cfg(feature = "typed_nodes")]
#[rustfmt::skip]
pub mod nodes;

// This order is intentional as it is the order it will be displayed in Rustdoc.
pub mod queue;

pub mod history;

pub mod models;

pub mod object_info;

pub mod system;

pub mod error;
use error::parse_response;
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
/// Functions for interacting with the ComfyUI API.
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

        self.post_multipart::<(), _>("upload/image", form).await
    }

    /// Get a resource from the ComfyUI API.
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        parse_response(
            self.client
                .get(format!("{}/{}", self.api_base, path))
                .send()
                .await?,
        )
        .await
    }

    /// Post a JSON resource to the ComfyUI API without parsing the response.
    ///
    /// This does not do any validation of the response.
    pub async fn post_json_without_parse<Req: serde::Serialize>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<String> {
        Ok(self
            .client
            .post(format!("{}/{}", self.api_base, path))
            .json(body)
            .send()
            .await?
            .text()
            .await?)
    }

    /// Post a JSON resource to the ComfyUI API.
    pub async fn post_json<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Res> {
        parse_response(
            self.client
                .post(format!("{}/{}", self.api_base, path))
                .json(body)
                .send()
                .await?,
        )
        .await
    }

    /// Post a multipart resource to the ComfyUI API.
    pub async fn post_multipart<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: reqwest::multipart::Form,
    ) -> Result<Res> {
        parse_response(
            self.client
                .post(format!("{}/{}", self.api_base, path))
                .multipart(body)
                .send()
                .await?,
        )
        .await
    }
}
