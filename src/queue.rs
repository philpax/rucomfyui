use super::*;

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
