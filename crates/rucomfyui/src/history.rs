//! History of queued prompts.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Client, OwnedBytes, Result};

/// Functions for interacting with the history of queued prompts.
impl Client {
    /// Get the history for this ComfyUI instance.
    pub async fn get_history(&self, max_items: u32) -> Result<History> {
        self.get(&format!("history?max_items={max_items}")).await
    }

    /// Get the history for a given prompt.
    ///
    /// Used by [`Self::easy_queue`] to poll and retrieve the results of a queued prompt.
    pub async fn get_history_for_prompt(&self, prompt_id: &str) -> Result<History> {
        self.get(&format!("history/{prompt_id}")).await
    }

    /// Delete entries from the history.
    pub async fn delete_from_history(&self, prompt_ids: Vec<String>) -> Result<()> {
        self.post_json_without_parse(
            "history",
            &serde_json::json!({
                "delete": prompt_ids,
            }),
        )
        .await
        .map(|_| ())
    }

    /// Clear the history.
    pub async fn clear_history(&self) -> Result<()> {
        self.post_json_without_parse("history", &serde_json::json!({ "clear": true }))
            .await
            .map(|_| ())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// History of queued prompts.
pub struct History {
    #[serde(flatten)]
    /// Data.
    pub data: indexmap::IndexMap<String, HistoryData>,
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
    pub nodes: BTreeMap<String, HistoryNodeOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Output of a node in the history.
pub struct HistoryNodeOutput {
    /// Images.
    #[serde(default)]
    pub images: Vec<HistoryImage>,
    /// Text.
    #[serde(default)]
    pub text: Vec<String>,
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
    pub async fn download(&self, client: &Client) -> Result<OwnedBytes> {
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
