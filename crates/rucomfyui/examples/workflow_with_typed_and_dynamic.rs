//! This is a variant of the `typed_workflow` example that uses dynamic nodes in the workflow
//! to demonstrate how typed and dynamic nodes can be used together.

use anyhow::Context;
use rucomfyui::{
    nodes::{
        all::{ClipTextEncode, EmptyLatentImage, KSampler},
        types::{ClipOut, ConditioningOut, ModelOut, Out, VaeOut},
    },
    workflow::{WorkflowMeta, WorkflowNode},
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

/// Constructs a workflow using [`rucomfyui`]'s typed nodes and dynamic nodes.
///
/// Dynamic nodes can be connected to other nodes by:
/// - typing their outputs with [`rucomfyui::WorkflowGraph::add_typed_dynamic`]
/// - wrapping a node ID with [`rucomfyui::nodes::types::Out::from_dynamic`]
/// - directly wiring outputs to inputs with [`rucomfyui::WorkflowNodeId::to_input_with_slot`]
fn workflow() -> (rucomfyui::Workflow, rucomfyui::WorkflowNodeId) {
    let g = rucomfyui::WorkflowGraph::new();

    // `add_typed_dynamic` is used to add a dynamic node and assign types to its outputs.
    let (model, clip, vae) = g.add_typed_dynamic::<(ModelOut, ClipOut, VaeOut)>(
        WorkflowNode::new("CheckpointLoaderSimple")
            .with_input("ckpt_name", "sd_xl_base_1.0.safetensors")
            .with_meta(WorkflowMeta::new("Load Checkpoint")),
    );
    // `add_dynamic` is used to add a dynamic node and return its ID.
    let negative = g.add_dynamic(
        WorkflowNode::new("CLIPTextEncode")
            .with_input("text", "text")
            .with_input("clip", clip)
            .with_meta(WorkflowMeta::new("CLIP Text Encode (Prompt)")),
    );
    let k_sampler = g.add(KSampler {
        model,
        seed: 0,
        steps: 20,
        cfg: 8.0,
        sampler_name: "euler",
        scheduler: "normal",
        positive: g.add(ClipTextEncode::new("a cat sleeping on a red chair", clip)),
        // `ConditioningOut::from_dynamic` is used to connect a dynamic node to a typed node that
        // implements `Conditioning`.
        negative: ConditioningOut::from_dynamic(negative, 0),
        latent_image: g.add(EmptyLatentImage {
            width: 1024,
            height: 1024,
            batch_size: 1,
        }),
        denoise: 1.0,
    });
    // The typed node is used as an input to another dynamic node.
    let vae_decode = g.add_dynamic(
        WorkflowNode::new("VAEDecode")
            .with_input("vae", vae)
            .with_input("samples", k_sampler)
            .with_meta(WorkflowMeta::new("VAE Decode")),
    );
    // The dynamic node is used as an input to another dynamic node.
    let preview_image = g.add_dynamic(
        WorkflowNode::new("PreviewImage")
            .with_input("images", vae_decode.to_input_with_slot(0))
            .with_meta(WorkflowMeta::new("Preview Image")),
    );

    (g.into_workflow(), preview_image)
}
