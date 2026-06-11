//! Queuing workflows to ComfyUI.

use serde::{Deserialize, Serialize};

use crate::{workflow::WorkflowNodeId, Client, Result, Workflow};

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
    /// Send a workflow to the ComfyUI API, returning immediately with a
    /// [`QueueResult`] containing the prompt ID.
    ///
    /// This is the low-level fire-and-forget primitive; see [`Client::execute`]
    /// for a higher-level helper that tracks the prompt's execution and yields
    /// progress events.
    pub async fn queue_prompt(&self, workflow: &Workflow) -> Result<QueueResult> {
        self.post_json(
            "prompt",
            &serde_json::json!({
                "prompt": workflow,
                "client_id": self.client_id,
            }),
        )
        .await
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

    /// Interrupt the current workflow.
    pub async fn interrupt(&self) -> Result<()> {
        self.post_json_without_parse("interrupt", &())
            .await
            .map(|_| ())
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
