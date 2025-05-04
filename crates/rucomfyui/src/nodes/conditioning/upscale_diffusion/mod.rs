//!`upscale_diffusion` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`SD_4XUpscale_Conditioning`](super::SD_4XUpscale_Conditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SD_4XUpscale_ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**SD_4XUpscale_Conditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SD_4XUpscale_Conditioning<
    ImagesParam: crate::nodes::types::Image,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ScaleRatioParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub images: ImagesParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub scale_ratio: ScaleRatioParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub noise_augmentation: NoiseAugmentationParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ScaleRatioParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> SD_4XUpscale_Conditioning<
    ImagesParam,
    PositiveParam,
    NegativeParam,
    ScaleRatioParam,
    NoiseAugmentationParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        positive: PositiveParam,
        negative: NegativeParam,
        scale_ratio: ScaleRatioParam,
        noise_augmentation: NoiseAugmentationParam,
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
    ImagesParam: crate::nodes::types::Image,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ScaleRatioParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SD_4XUpscale_Conditioning<
    ImagesParam,
    PositiveParam,
    NegativeParam,
    ScaleRatioParam,
    NoiseAugmentationParam,
> {
    type Output = out::SD_4XUpscale_ConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("scale_ratio".to_string(), self.scale_ratio.clone().into());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        output
    }
    const NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DISPLAY_NAME: &'static str = "SD_4XUpscale_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/upscale_diffusion";
}
