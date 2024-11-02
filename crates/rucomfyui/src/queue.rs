//! Queuing workflows to ComfyUI.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    error::parse_response, workflow::WorkflowNodeId, Client, OwnedBytes, Result, Workflow,
};

impl Client {
    /// Send a workflow to the ComfyUI API.
    pub async fn queue(&self, workflow: &Workflow) -> Result<QueueResult> {
        let payload = serde_json::json!({
            "prompt": workflow,
        });
        parse_response(
            self.client
                .post(format!("{}/prompt", self.api_base))
                .json(&payload)
                .send()
                .await?,
        )
        .await
    }

    /// Helper function that prompts with a workflow, polls for the result, and then returns all output images.
    pub async fn easy_queue(
        &self,
        workflow: &Workflow,
    ) -> Result<HashMap<WorkflowNodeId, EasyQueueNodeOutput>> {
        let output = self.queue(workflow).await?;

        // Poll for the prompt's completion.
        let prompt_id = output.prompt_id;
        let history_output = loop {
            let history = self.history_for_prompt(&prompt_id).await?;
            if let Some(history_data) = history.data.get(&prompt_id) {
                if history_data.status.completed {
                    break history_data.outputs.clone();
                }
            }
            futures_timer::Delay::new(std::time::Duration::from_millis(100)).await;
        };

        // Convert output to `EasyQueueNodeOutput`.
        let mut output = HashMap::new();
        for (node_name, node_output) in history_output.nodes {
            let images = futures::future::try_join_all(
                node_output.images.iter().map(|out| out.download(self)),
            )
            .await?;
            let texts = node_output.text;
            output.insert(
                node_name.parse::<WorkflowNodeId>()?,
                EasyQueueNodeOutput { images, texts },
            );
        }
        Ok(output)
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Result of the queueing of a prompt.
pub struct QueueResult {
    /// Node errors.
    pub node_errors: serde_json::Value,
    /// Number.
    pub number: u32,
    /// Prompt ID.
    pub prompt_id: String,
}

#[derive(Debug)]
/// Output of a node in [`Client::easy_queue`].
pub struct EasyQueueNodeOutput {
    /// Images.
    pub images: Vec<OwnedBytes>,
    /// Texts.
    pub texts: Vec<String>,
}
