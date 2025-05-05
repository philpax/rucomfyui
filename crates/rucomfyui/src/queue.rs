//! Queuing workflows to ComfyUI.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{workflow::WorkflowNodeId, Client, OwnedBytes, Result, Workflow};

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

#[derive(Serialize, Deserialize, Debug, Default)]
/// The queue of the ComfyUI instance from [`Client::queue`].
pub struct Queue {
    /// Running entries.
    pub running: Vec<QueueEntry>,
    /// Pending entries.
    pub pending: Vec<QueueEntry>,
}
#[derive(Serialize, Deserialize, Debug)]
/// An entry in the queue from [`Queue`].
pub struct QueueEntry {
    /// The number of the entry in the queue.
    pub number: usize,
    /// The ID of the prompt.
    pub prompt_id: String,
    /// The workflow that was queued.
    pub prompt: Workflow,
    /// Extra data.
    pub extra_data: serde_json::Value,
    /// The outputs to execute.
    pub outputs_to_execute: Vec<WorkflowNodeId>,
}

/// Functions for queuing workflows to the ComfyUI API, or for retrieving the queue.
impl Client {
    /// Send a workflow to the ComfyUI API.
    pub async fn queue(&self, workflow: &Workflow) -> Result<QueueResult> {
        self.post_json(
            "prompt",
            &serde_json::json!({
                "prompt": workflow,
            }),
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
            let history = self.get_history_for_prompt(&prompt_id).await?;
            if let Some(history_data) = history.data.get(&prompt_id) {
                if history_data.status.completed {
                    break history_data.outputs.clone();
                }
            }
            futures_timer::Delay::new(web_time::Duration::from_millis(100)).await;
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

    /// Get the queue of the ComfyUI instance.
    pub async fn get_queue(&self) -> Result<Queue> {
        type ApiQueueEntry = (
            usize,
            String,
            Workflow,
            serde_json::Value,
            Vec<WorkflowNodeId>,
        );
        #[derive(Deserialize)]
        struct ApiQueue {
            queue_running: Vec<ApiQueueEntry>,
            queue_pending: Vec<ApiQueueEntry>,
        }
        fn api_queue_to_queue(api_queue: Vec<ApiQueueEntry>) -> Vec<QueueEntry> {
            api_queue
                .into_iter()
                .map(
                    |(number, prompt_id, prompt, extra_data, outputs_to_execute)| QueueEntry {
                        number,
                        prompt_id,
                        prompt,
                        extra_data,
                        outputs_to_execute,
                    },
                )
                .collect()
        }

        let api_queue: ApiQueue = self.get("queue").await?;
        Ok(Queue {
            running: api_queue_to_queue(api_queue.queue_running),
            pending: api_queue_to_queue(api_queue.queue_pending),
        })
    }

    /// Delete workflows from the queue.
    pub async fn delete_from_queue(&self, prompt_ids: Vec<String>) -> Result<()> {
        self.post_json_without_parse(
            "queue",
            &serde_json::json!({
                "delete": prompt_ids,
            }),
        )
        .await
        .map(|_| ())
    }

    /// Clear queue.
    pub async fn clear_queue(&self) -> Result<()> {
        self.post_json_without_parse(
            "queue",
            &serde_json::json!({
                "clear": true,
            }),
        )
        .await
        .map(|_| ())
    }
}
