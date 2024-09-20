//!upscale_diffusion
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**SD_4XUpscale_Conditioning**
pub struct Sd4XUpscaleConditioning<
    Images: crate::nodes::Image,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ScaleRatio: crate::nodes::Float,
    NoiseAugmentation: crate::nodes::Float,
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
    pub positive: crate::nodes::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::ConditioningOut,
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Images: crate::nodes::Image,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ScaleRatio: crate::nodes::Float,
    NoiseAugmentation: crate::nodes::Float,
> crate::nodes::TypedNode
for Sd4XUpscaleConditioning<Images, Positive, Negative, ScaleRatio, NoiseAugmentation> {
    type Output = Sd4XUpscaleConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
            negative: crate::nodes::ConditioningOut {
                node_id,
                slot: 1u32,
            },
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DISPLAY_NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/upscale_diffusion";
}
