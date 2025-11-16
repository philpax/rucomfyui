//!`flux` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**USOStyleReference**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct USOStyleReference<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub model_patch: ModelPatchParam,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutputParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> USOStyleReference<ModelParam, ModelPatchParam, ClipVisionOutputParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        model_patch: ModelPatchParam,
        clip_vision_output: ClipVisionOutputParam,
    ) -> Self {
        Self {
            model,
            model_patch,
            clip_vision_output,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for USOStyleReference<ModelParam, ModelPatchParam, ClipVisionOutputParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("model_patch".to_string(), self.model_patch.clone().into());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.clone().into(),
            );
        output
    }
    const NAME: &'static str = "USOStyleReference";
    const DISPLAY_NAME: &'static str = "USOStyleReference";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_patches/flux";
}
