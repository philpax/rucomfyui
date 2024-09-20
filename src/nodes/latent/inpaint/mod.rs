//!`inpaint` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**Set Latent Noise Mask**: No description.
pub struct SetLatentNoiseMask<
    Samples: crate::nodes::types::Latent,
    Mask: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub mask: Mask,
}
impl<
    Samples: crate::nodes::types::Latent,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode for SetLatentNoiseMask<Samples, Mask> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("mask".to_string(), self.mask.to_workflow_input());
        output
    }
    const NAME: &'static str = "SetLatentNoiseMask";
    const DISPLAY_NAME: &'static str = "Set Latent Noise Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
///**VAE Encode (for Inpainting)**: No description.
pub struct VaeEncodeForInpaint<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Mask: crate::nodes::types::Mask,
    GrowMaskBy: crate::nodes::types::Int,
> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub grow_mask_by: GrowMaskBy,
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Mask: crate::nodes::types::Mask,
    GrowMaskBy: crate::nodes::types::Int,
> crate::nodes::TypedNode for VaeEncodeForInpaint<Pixels, Vae, Mask, GrowMaskBy> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("pixels".to_string(), self.pixels.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output.insert("mask".to_string(), self.mask.to_workflow_input());
        output.insert("grow_mask_by".to_string(), self.grow_mask_by.to_workflow_input());
        output
    }
    const NAME: &'static str = "VAEEncodeForInpaint";
    const DISPLAY_NAME: &'static str = "VAE Encode (for Inpainting)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
