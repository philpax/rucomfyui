//!`stable_cascade` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`StableCascadeEmptyLatentImage`](super::StableCascadeEmptyLatentImage).
    #[derive(Clone)]
    pub struct StableCascadeEmptyLatentImageOutput {
        ///No documentation.
        pub stage_c: crate::nodes::types::LatentOut,
        ///No documentation.
        pub stage_b: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableCascadeStageCVaeEncode`](super::StableCascadeStageCVaeEncode).
    #[derive(Clone)]
    pub struct StableCascadeStageCVaeEncodeOutput {
        ///No documentation.
        pub stage_c: crate::nodes::types::LatentOut,
        ///No documentation.
        pub stage_b: crate::nodes::types::LatentOut,
    }
}
///**StableCascade_EmptyLatentImage**: No description.
pub struct StableCascadeEmptyLatentImage<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Compression: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub compression: Compression,
    ///No documentation.
    pub batch_size: BatchSize,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Compression: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode
for StableCascadeEmptyLatentImage<Width, Height, Compression, BatchSize> {
    type Output = out::StableCascadeEmptyLatentImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
            stage_b: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
///**StableCascade_StageC_VAEEncode**: No description.
pub struct StableCascadeStageCVaeEncode<
    Image: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Compression: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub compression: Compression,
}
impl<
    Image: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Compression: crate::nodes::types::Int,
> crate::nodes::TypedNode for StableCascadeStageCVaeEncode<Image, Vae, Compression> {
    type Output = out::StableCascadeStageCVaeEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
            stage_b: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DISPLAY_NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
