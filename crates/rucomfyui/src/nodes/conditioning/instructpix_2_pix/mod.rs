//!`instructpix2pix` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`InstructPixToPixConditioning`](super::InstructPixToPixConditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct InstructPixToPixConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**InstructPixToPixConditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct InstructPixToPixConditioning<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub pixels: PixelsParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
> InstructPixToPixConditioning<PositiveParam, NegativeParam, VaeParam, PixelsParam> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        pixels: PixelsParam,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            pixels,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for InstructPixToPixConditioning<PositiveParam, NegativeParam, VaeParam, PixelsParam> {
    type Output = out::InstructPixToPixConditioningOutput;
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
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("pixels".to_string(), self.pixels.clone().into());
        output
    }
    const NAME: &'static str = "InstructPixToPixConditioning";
    const DISPLAY_NAME: &'static str = "InstructPixToPixConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/instructpix2pix";
}
