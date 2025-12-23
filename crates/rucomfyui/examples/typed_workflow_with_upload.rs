#![cfg(feature = "typed_nodes")]
//! This is a variant of the `typed_workflow` example that uploads the output as an input
//! in a cycle for several steps, demonstrating how to use the typed nodes in a more complex
//! workflow.
//!
//! Realistically, you would feed the output to the input of the next stage in the workflow itself -
//! this is just a demonstration of how to upload images.

use anyhow::Context;
use rucomfyui::nodes::all::{
    CLIPTextEncode, CheckpointLoaderSimple, EmptyLatentImage, KSampler, LoadImage, PreviewImage,
    VAEDecode, VAEEncode,
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    const INPUT_FILENAME: &str = "cycle_input.png";

    // Create a client for the URL.
    let url = std::env::args()
        .nth(1)
        .context("ComfyUI URL not provided")?;
    let client = rucomfyui::Client::new(url);

    // The image to load as an input. Populated by the output of the last step.
    let mut load_image = None;

    // Iterate over the animals and queue a workflow for each.
    for (idx, animal) in [
        "cat", "dog", "bird", "fish", "rabbit", "hamster", "turtle", "lizard",
    ]
    .iter()
    .enumerate()
    {
        // Create the workflow and queue it.
        let (workflow, preview_image) = workflow(animal, idx as u64, load_image);
        let result = client.easy_queue(&workflow).await?;

        // Save the output images to disk.
        let images = &result[&preview_image].images;
        for (image_idx, image) in images.iter().enumerate() {
            let filename = format!("output_{idx}_{image_idx}.png");
            println!("{animal}: {filename}");
            std::fs::write(filename, image)?;
        }

        client
            .upload_image(
                INPUT_FILENAME,
                images[0].clone(),
                rucomfyui::upload::UploadType::Input,
                true,
            )
            .await?;
        load_image = Some(INPUT_FILENAME);
    }

    Ok(())
}

/// Constructs a workflow using [`rucomfyui`]'s typed nodes, where the input text is constructed from
/// an input image. When `load_image` is `true`, an uploaded image is used instead of an empty latent.
fn workflow(
    animal: &str,
    seed: u64,
    load_image: Option<&str>,
) -> (rucomfyui::Workflow, rucomfyui::WorkflowNodeId) {
    let g = rucomfyui::WorkflowGraph::new();

    let c = g.add(CheckpointLoaderSimple::new("sd_xl_base_1.0.safetensors"));
    let preview_image = g.add(PreviewImage::new(g.add(VAEDecode {
        vae: c.vae,
        samples: g.add(KSampler {
            model: c.model,
            seed,
            steps: 20,
            cfg: 8.0,
            sampler_name: "euler",
            scheduler: "normal",
            positive: g.add(CLIPTextEncode::new(
                format!("a {animal} sleeping on a red chair"),
                c.clip,
            )),
            negative: g.add(CLIPTextEncode::new("text", c.clip)),
            latent_image: match load_image {
                Some(image) => g.add(VAEEncode::new(g.add(LoadImage::new(image)).image, c.vae)),
                None => g.add(EmptyLatentImage {
                    width: 1024,
                    height: 1024,
                    batch_size: 1,
                }),
            },
            denoise: if load_image.is_some() { 0.6 } else { 1.0 },
        }),
    })));

    (g.into_workflow(), preview_image)
}
