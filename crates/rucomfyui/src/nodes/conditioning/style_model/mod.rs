//!`style_model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Apply Style Model**: No description.
#[derive(Clone)]
pub struct StyleModelApply<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    StrengthType: crate::nodes::types::String,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub style_model: StyleModel,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.001
*/
    pub strength: Strength,
    ///No documentation.
    pub strength_type: StrengthType,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    StrengthType: crate::nodes::types::String,
> StyleModelApply<Conditioning, StyleModel, ClipVisionOutput, Strength, StrengthType> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        style_model: StyleModel,
        clip_vision_output: ClipVisionOutput,
        strength: Strength,
        strength_type: StrengthType,
    ) -> Self {
        Self {
            conditioning,
            style_model,
            clip_vision_output,
            strength,
            strength_type,
        }
    }
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StyleModel: crate::nodes::types::StyleModel,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    StrengthType: crate::nodes::types::String,
> crate::nodes::TypedNode
for StyleModelApply<Conditioning, StyleModel, ClipVisionOutput, Strength, StrengthType> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("style_model".to_string(), self.style_model.clone().into());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.clone().into(),
            );
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("strength_type".to_string(), self.strength_type.clone().into());
        output
    }
    const NAME: &'static str = "StyleModelApply";
    const DISPLAY_NAME: &'static str = "Apply Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/style_model";
}
