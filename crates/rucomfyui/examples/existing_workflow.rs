//! Loads a workflow from JSON, queues it, and saves the output images to disk.

use anyhow::Context;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Create a client for the URL.
    let url = std::env::args()
        .nth(1)
        .context("ComfyUI URL not provided")?;
    let client = rucomfyui::Client::new(url);

    // Load the workflow from JSON and queue it.
    let workflow = rucomfyui::Workflow::from_json(include_str!("existing_workflow.json"))?;
    let result = client.easy_queue(&workflow).await?;

    // Save the output images to disk.
    for (idx, image) in result
        .iter()
        .flat_map(|(_, output)| output.images.iter())
        .enumerate()
    {
        std::fs::write(format!("output_{idx}.png"), image)?;
    }

    Ok(())
}
