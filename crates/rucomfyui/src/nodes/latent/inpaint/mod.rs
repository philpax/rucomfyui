//!`inpaint` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Set Latent Noise Mask**: No description.
#[derive(Clone)]
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
> SetLatentNoiseMask<Samples, Mask> {
    /// Create a new node.
    pub fn new(samples: Samples, mask: Mask) -> Self {
        Self { samples, mask }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode for SetLatentNoiseMask<Samples, Mask> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output
    }
    const NAME: &'static str = "SetLatentNoiseMask";
    const DISPLAY_NAME: &'static str = "Set Latent Noise Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
///**VAE Encode (for Inpainting)**: No description.
#[derive(Clone)]
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
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 64
  - Min: 0
  - Step: 1
*/
    pub grow_mask_by: GrowMaskBy,
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Mask: crate::nodes::types::Mask,
    GrowMaskBy: crate::nodes::types::Int,
> VaeEncodeForInpaint<Pixels, Vae, Mask, GrowMaskBy> {
    /// Create a new node.
    pub fn new(pixels: Pixels, vae: Vae, mask: Mask, grow_mask_by: GrowMaskBy) -> Self {
        Self {
            pixels,
            vae,
            mask,
            grow_mask_by,
        }
    }
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Mask: crate::nodes::types::Mask,
    GrowMaskBy: crate::nodes::types::Int,
> crate::nodes::TypedNode for VaeEncodeForInpaint<Pixels, Vae, Mask, GrowMaskBy> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("pixels".to_string(), self.pixels.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("grow_mask_by".to_string(), self.grow_mask_by.clone().into());
        output
    }
    const NAME: &'static str = "VAEEncodeForInpaint";
    const DISPLAY_NAME: &'static str = "VAE Encode (for Inpainting)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
