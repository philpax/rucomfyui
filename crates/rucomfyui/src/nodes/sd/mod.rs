//!`sd` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`HunyuanRefinerLatent`](super::HunyuanRefinerLatent).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanRefinerLatentOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`HunyuanVideo15SuperResolution`](super::HunyuanVideo15SuperResolution).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanVideo15SuperResolutionOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**HunyuanRefinerLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanRefinerLatent<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent: LatentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentationParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> HunyuanRefinerLatent<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        latent: LatentParam,
        noise_augmentation: NoiseAugmentationParam,
    ) -> Self {
        Self {
            positive,
            negative,
            latent,
            noise_augmentation,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for HunyuanRefinerLatent<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
> {
    type Output = out::HunyuanRefinerLatentOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("latent".to_string(), self.latent.clone().into());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        output
    }
    const NAME: &'static str = "HunyuanRefinerLatent";
    const DISPLAY_NAME: &'static str = "HunyuanRefinerLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sd";
}
///**HunyuanVideo15SuperResolution**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanVideo15SuperResolution<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae = crate::nodes::types::VaeOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent: LatentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.7
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentationParam,
    ///No documentation.
    pub vae: Option<VaeParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> HunyuanVideo15SuperResolution<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
    VaeParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        latent: LatentParam,
        noise_augmentation: NoiseAugmentationParam,
        vae: Option<VaeParam>,
        start_image: Option<StartImageParam>,
        clip_vision_output: Option<ClipVisionOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            latent,
            noise_augmentation,
            vae,
            start_image,
            clip_vision_output,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for HunyuanVideo15SuperResolution<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
    VaeParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    type Output = out::HunyuanVideo15SuperResolutionOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("latent".to_string(), self.latent.clone().into());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        if let Some(v) = &self.vae {
            output.insert("vae".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "HunyuanVideo15SuperResolution";
    const DISPLAY_NAME: &'static str = "HunyuanVideo15SuperResolution";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sd";
}
