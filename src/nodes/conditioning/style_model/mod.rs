//!`style_model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**Apply Style Model**: No description.
pub struct StyleModelApply<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub style_model: StyleModel,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
> StyleModelApply<Conditioning, StyleModel, ClipVisionOutput> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        style_model: StyleModel,
        clip_vision_output: ClipVisionOutput,
    ) -> Self {
        Self {
            conditioning,
            style_model,
            clip_vision_output,
        }
    }
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for StyleModelApply<Conditioning, StyleModel, ClipVisionOutput> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("style_model".to_string(), self.style_model.to_workflow_input());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "StyleModelApply";
    const DISPLAY_NAME: &'static str = "Apply Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/style_model";
}
