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
