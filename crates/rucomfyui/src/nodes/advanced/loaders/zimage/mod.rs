//!`zimage` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ZImageFunControlnet**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ZImageFunControlnet<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    InpaintImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub model_patch: ModelPatchParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    ///No documentation.
    pub inpaint_image: Option<InpaintImageParam>,
    ///No documentation.
    pub mask: Option<MaskParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    ImageParam: crate::nodes::types::Image,
    InpaintImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> ZImageFunControlnet<
    ModelParam,
    ModelPatchParam,
    VaeParam,
    StrengthParam,
    ImageParam,
    InpaintImageParam,
    MaskParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        model_patch: ModelPatchParam,
        vae: VaeParam,
        strength: StrengthParam,
        image: Option<ImageParam>,
        inpaint_image: Option<InpaintImageParam>,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            model,
            model_patch,
            vae,
            strength,
            image,
            inpaint_image,
            mask,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    ImageParam: crate::nodes::types::Image,
    InpaintImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for ZImageFunControlnet<
    ModelParam,
    ModelPatchParam,
    VaeParam,
    StrengthParam,
    ImageParam,
    InpaintImageParam,
    MaskParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("model_patch".to_string(), self.model_patch.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.inpaint_image {
            output.insert("inpaint_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ZImageFunControlnet";
    const DISPLAY_NAME: &'static str = "ZImageFunControlnet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders/zimage";
}
