//! Runs a workflow and follows it live, printing each event as it arrives and
//! saving any preview frames and outputs to disk.
//!
//! `Client::execute` returns an `Execution`, which is a stream of progress
//! events. Calling `.outputs()` on it (as the other examples do) runs it to the
//! end and collects the results; here we consume the stream ourselves instead.
//!
//! Run with `--features websocket` to receive sampler progress and previews over
//! a WebSocket connection. Without it, progress is synthesised from polling the
//! history endpoint, so only the start/output/completion events show up.

use anyhow::Context as _;
use futures::StreamExt as _;
use rucomfyui::{Event, execute::PreviewImageFormat};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::args()
        .nth(1)
        .context("ComfyUI URL not provided")?;
    let client = rucomfyui::Client::new(url);

    let workflow = rucomfyui::Workflow::from_json(include_str!("existing_workflow.json"))?;

    let mut execution = client.execute(&workflow).await?;
    println!("queued prompt {}", execution.prompt_id());

    let mut preview_index = 0;
    while let Some(event) = execution.next().await {
        match event? {
            Event::Status { queue_remaining } => {
                println!("queue: {queue_remaining} remaining");
            }
            Event::ExecutionStart { .. } => println!("started"),
            Event::Executing {
                node: Some(node), ..
            } => println!("executing node {}", node.0),
            Event::Executing { node: None, .. } => {}
            Event::Progress {
                node, value, max, ..
            } => {
                let node = node.map(|n| n.0.to_string()).unwrap_or_default();
                println!("progress {value}/{max} (node {node})");
            }
            Event::Preview { image, .. } => {
                let ext = match image.format {
                    PreviewImageFormat::Jpeg => "jpg",
                    PreviewImageFormat::Png => "png",
                    PreviewImageFormat::Unknown => "bin",
                };
                let path = format!("preview_{preview_index}.{ext}");
                std::fs::write(&path, &image.data)?;
                println!("preview -> {path}");
                preview_index += 1;
            }
            Event::Executed { node, output, .. } => {
                for (i, image) in output.images.iter().enumerate() {
                    let path = format!("output_{}_{i}.png", node.0);
                    std::fs::write(&path, image)?;
                    println!("output -> {path}");
                }
            }
            Event::Error { node, message, .. } => {
                let node = node.map(|n| format!(" (node {})", n.0)).unwrap_or_default();
                anyhow::bail!("execution failed{node}: {message}");
            }
            Event::Completed { .. } => println!("done"),
        }
    }

    Ok(())
}
