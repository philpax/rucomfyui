#![cfg(feature = "typed_nodes")]
//! A typed equivalent of the `existing_workflow` example.

use anyhow::Context;
use rucomfyui::nodes::all::{
    CheckpointLoaderSimple, ClipTextEncode, EmptyLatentImage, KSampler, PreviewImage, VaeDecode,
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Create a client for the URL.
    let url = std::env::args()
        .nth(1)
        .context("ComfyUI URL not provided")?;
    let client = rucomfyui::Client::new(url);

    // Create the workflow and queue it.
    let (workflow, preview_image) = workflow();
    let result = client.easy_queue(&workflow).await?;

    // Save the output images to disk.
    for (idx, image) in result[&preview_image].images.iter().enumerate() {
        std::fs::write(format!("output_{idx}.png"), image)?;
    }

    Ok(())
}

/// Constructs a workflow using [`rucomfyui`]'s typed nodes. A [`rucomfyui::WorkflowGraph`]
/// is constructed, populated, and then converted to a [`rucomfyui::Workflow`]. The output node
/// ID is returned to allow for easy access to the output images.
fn workflow() -> (rucomfyui::Workflow, rucomfyui::WorkflowNodeId) {
    let g = rucomfyui::WorkflowGraph::new();

    let c = g.add(CheckpointLoaderSimple::new("sd_xl_base_1.0.safetensors"));
    let preview_image = g.add(PreviewImage::new(g.add(VaeDecode {
        vae: c.vae,
        samples: g.add(KSampler {
            model: c.model,
            seed: 0,
            steps: 20,
            cfg: 8.0,
            sampler_name: "euler",
            scheduler: "normal",
            positive: g.add(ClipTextEncode::new("a cat sleeping on a red chair", c.clip)),
            negative: g.add(ClipTextEncode::new("text", c.clip)),
            latent_image: g.add(EmptyLatentImage {
                width: 1024,
                height: 1024,
                batch_size: 1,
            }),
            denoise: 1.0,
        }),
    })));

    (g.into_workflow(), preview_image)
}
