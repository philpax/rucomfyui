//!`supir` definitions/categories.
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
///**SUPIRApply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SUPIRApply<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    RestoreCfgParam: crate::nodes::types::Float,
    RestoreCfgSTminParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub model_patch: ModelPatchParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    /**Control strength at the start of sampling (high sigma).

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength_start: StrengthStartParam,
    /**Control strength at the end of sampling (low sigma). Linearly interpolated from start.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength_end: StrengthEndParam,
    /**Pulls denoised output toward the input latent. Higher = stronger fidelity to input. 0 to disable.

**Metadata**:
  - Default: 4
  - Max: 20
  - Min: 0
  - Step: 0.1
*/
    pub restore_cfg: RestoreCfgParam,
    /**Sigma threshold below which restore_cfg is disabled.

**Metadata**:
  - Default: 0.05
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub restore_cfg_s_tmin: RestoreCfgSTminParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    RestoreCfgParam: crate::nodes::types::Float,
    RestoreCfgSTminParam: crate::nodes::types::Float,
> SUPIRApply<
    ModelParam,
    ModelPatchParam,
    VaeParam,
    ImageParam,
    StrengthStartParam,
    StrengthEndParam,
    RestoreCfgParam,
    RestoreCfgSTminParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        model_patch: ModelPatchParam,
        vae: VaeParam,
        image: ImageParam,
        strength_start: StrengthStartParam,
        strength_end: StrengthEndParam,
        restore_cfg: RestoreCfgParam,
        restore_cfg_s_tmin: RestoreCfgSTminParam,
    ) -> Self {
        Self {
            model,
            model_patch,
            vae,
            image,
            strength_start,
            strength_end,
            restore_cfg,
            restore_cfg_s_tmin,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    RestoreCfgParam: crate::nodes::types::Float,
    RestoreCfgSTminParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SUPIRApply<
    ModelParam,
    ModelPatchParam,
    VaeParam,
    ImageParam,
    StrengthStartParam,
    StrengthEndParam,
    RestoreCfgParam,
    RestoreCfgSTminParam,
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
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("strength_start".to_string(), self.strength_start.clone().into());
        output.insert("strength_end".to_string(), self.strength_end.clone().into());
        output.insert("restore_cfg".to_string(), self.restore_cfg.clone().into());
        output
            .insert(
                "restore_cfg_s_tmin".to_string(),
                self.restore_cfg_s_tmin.clone().into(),
            );
        output
    }
    const NAME: &'static str = "SUPIRApply";
    const DISPLAY_NAME: &'static str = "SUPIRApply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/patch/supir";
}
