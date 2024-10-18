//!`inpaint` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`InpaintModelConditioning`](super::InpaintModelConditioning).
    #[derive(Clone)]
    pub struct InpaintModelConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**InpaintModelConditioning**: No description.
#[derive(Clone)]
pub struct InpaintModelConditioning<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub mask: Mask,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
> InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        pixels: Pixels,
        mask: Mask,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            pixels,
            mask,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask> {
    type Output = out::InpaintModelConditioningOutput;
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
        output.insert("mask".to_string(), self.mask.clone().into());
        output
    }
    const NAME: &'static str = "InpaintModelConditioning";
    const DISPLAY_NAME: &'static str = "InpaintModelConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/inpaint";
}
