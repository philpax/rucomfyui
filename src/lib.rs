//! Very basic ComfyUI API client for Rust.
#![deny(missing_docs)]

use std::collections::HashMap;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

impl Client {
    /// Send a workflow to the ComfyUI API.
    pub async fn queue(&self, workflow: &serde_json::Value) -> Result<PromptQueueResult> {
        Ok(self
            .client
            .post(format!("{}/prompt", self.api_base))
            .json(&workflow)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Helper function that prompts with a workflow, polls for the result, and then returns all output images.
    pub async fn easy_queue(
        &self,
        workflow: &serde_json::Value,
    ) -> Result<HashMap<String, Vec<Bytes>>> {
        let payload = serde_json::json!({
            "prompt": workflow,
        });
        let output: PromptQueueResult = self
            .client
            .post(format!("{}/prompt", self.api_base))
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        // Poll for output
        let history_output = loop {
            let history = self.history(10).await?;
            if let Some(history_data) = history.data.get(&output.prompt_id) {
                if history_data.status.completed {
                    break history_data.outputs.clone();
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };

        let mut output = HashMap::new();
        for (node_name, node_output) in history_output.nodes {
            let images = futures::future::try_join_all(
                node_output.images.iter().map(|image| image.download(self)),
            )
            .await?;
            output.insert(node_name, images);
        }
        Ok(output)
    }
}
#[derive(Serialize, Deserialize, Debug)]
/// Result of the queueing of a prompt.
pub struct PromptQueueResult {
    /// Node errors.
    node_errors: serde_json::Value,
    /// Number.
    number: u32,
    /// Prompt ID.
    prompt_id: String,
}

impl Client {
    /// Get the history for this ComfyUI instance.
    ///
    /// Used by [`Self::easy_queue`] to poll and retrieve the results of a queued prompt.
    pub async fn history(&self, max_items: u32) -> Result<History> {
        Ok(self
            .client
            .get(format!("{}/history?max_items={max_items}", self.api_base))
            .send()
            .await?
            .json()
            .await?)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// History of queued prompts.
pub struct History {
    #[serde(flatten)]
    /// Data.
    pub data: HashMap<String, HistoryData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Data for a queued prompt.
pub struct HistoryData {
    /// Outputs.
    pub outputs: HistoryOutputs,
    /// Status.
    pub status: Status,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Status of a queued prompt.
pub struct Status {
    /// Completed.
    pub completed: bool,
    /// Status string.
    pub status_str: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Output of a queued prompt.
pub struct HistoryOutputs {
    #[serde(flatten)]
    /// Nodes.
    pub nodes: HashMap<String, HistoryNodeOutput>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Output of a node in the history.
pub struct HistoryNodeOutput {
    /// Images.
    pub images: Vec<HistoryImage>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Image in the history.
pub struct HistoryImage {
    /// Filename.
    pub filename: String,
    /// Subfolder.
    pub subfolder: String,
    #[serde(rename = "type")]
    /// Image type.
    pub image_type: String,
}
impl HistoryImage {
    /// Get the URL of the image.
    pub fn url(&self, client: &Client) -> String {
        format!(
            "{}/api/view?filename={}&subfolder={}&type={}",
            client.api_base, self.filename, self.subfolder, self.image_type,
        )
    }
    /// Download the image.
    pub async fn download(&self, client: &Client) -> Result<Bytes> {
        Ok(client
            .client
            .get(self.url(client))
            .send()
            .await?
            .bytes()
            .await?
            .into())
    }
}
