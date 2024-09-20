//!`stable_cascade` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
> StableCascadeEmptyLatentImage<Width, Height, Compression, BatchSize> {
    /// Create a new node.
    pub fn new(
        width: Width,
        height: Height,
        compression: Compression,
        batch_size: BatchSize,
    ) -> Self {
        Self {
            width,
            height,
            compression,
            batch_size,
        }
    }
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("compression".to_string(), self.compression.to_workflow_input());
        output.insert("batch_size".to_string(), self.batch_size.to_workflow_input());
        output
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
> StableCascadeStageCVaeEncode<Image, Vae, Compression> {
    /// Create a new node.
    pub fn new(image: Image, vae: Vae, compression: Compression) -> Self {
        Self { image, vae, compression }
    }
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output.insert("compression".to_string(), self.compression.to_workflow_input());
        output
    }
    const NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DISPLAY_NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
