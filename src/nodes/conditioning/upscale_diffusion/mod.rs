//!`upscale_diffusion` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`Sd4XUpscaleConditioning`](super::Sd4XUpscaleConditioning).
    #[derive(Clone)]
    pub struct Sd4XUpscaleConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**SD_4XUpscale_Conditioning**: No description.
pub struct Sd4XUpscaleConditioning<
    Images: crate::nodes::types::Image,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ScaleRatio: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub scale_ratio: ScaleRatio,
    ///No documentation.
    pub noise_augmentation: NoiseAugmentation,
}
impl<
    Images: crate::nodes::types::Image,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ScaleRatio: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> Sd4XUpscaleConditioning<Images, Positive, Negative, ScaleRatio, NoiseAugmentation> {
    /// Create a new node.
    pub fn new(
        images: Images,
        positive: Positive,
        negative: Negative,
        scale_ratio: ScaleRatio,
        noise_augmentation: NoiseAugmentation,
    ) -> Self {
        Self {
            images,
            positive,
            negative,
            scale_ratio,
            noise_augmentation,
        }
    }
}
impl<
    Images: crate::nodes::types::Image,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ScaleRatio: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> crate::nodes::TypedNode
for Sd4XUpscaleConditioning<Images, Positive, Negative, ScaleRatio, NoiseAugmentation> {
    type Output = out::Sd4XUpscaleConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
            negative: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 1u32,
            },
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.to_workflow_input());
        output.insert("positive".to_string(), self.positive.to_workflow_input());
        output.insert("negative".to_string(), self.negative.to_workflow_input());
        output.insert("scale_ratio".to_string(), self.scale_ratio.to_workflow_input());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DISPLAY_NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/upscale_diffusion";
}
