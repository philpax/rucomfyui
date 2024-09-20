//!`gligen` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`GligenTextBoxApply`](super::GligenTextBoxApply).
    #[derive(Clone)]
    pub struct GligenTextBoxApplyOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
}
///**GLIGENTextBoxApply**: No description.
pub struct GligenTextBoxApply<
    ConditioningTo: crate::nodes::types::Conditioning,
    Clip: crate::nodes::types::Clip,
    GligenTextboxModel: crate::nodes::types::Gligen,
    Text: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub gligen_textbox_model: GligenTextboxModel,
    ///No documentation.
    pub text: Text,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    Clip: crate::nodes::types::Clip,
    GligenTextboxModel: crate::nodes::types::Gligen,
    Text: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode
for GligenTextBoxApply<
    ConditioningTo,
    Clip,
    GligenTextboxModel,
    Text,
    Width,
    Height,
    X,
    Y,
> {
    type Output = out::GligenTextBoxApplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "conditioning_to".to_string(),
                self.conditioning_to.to_workflow_input(),
            );
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output
            .insert(
                "gligen_textbox_model".to_string(),
                self.gligen_textbox_model.to_workflow_input(),
            );
        output.insert("text".to_string(), self.text.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output
    }
    const NAME: &'static str = "GLIGENTextBoxApply";
    const DISPLAY_NAME: &'static str = "GLIGENTextBoxApply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/gligen";
}
