//!`upscale_diffusion` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**SD_4XUpscale_Conditioning**
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
///Output for [`Sd4XUpscaleConditioning`].
#[derive(Clone)]
pub struct Sd4XUpscaleConditioningOutput {
    ///No documentation.
    pub positive: crate::nodes::types::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::types::ConditioningOut,
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Images: crate::nodes::types::Image,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ScaleRatio: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> crate::nodes::TypedNode
for Sd4XUpscaleConditioning<Images, Positive, Negative, ScaleRatio, NoiseAugmentation> {
    type Output = Sd4XUpscaleConditioningOutput;
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
    const NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DISPLAY_NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/upscale_diffusion";
}
