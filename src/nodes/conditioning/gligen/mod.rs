//!gligen
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**GLIGENTextBoxApply**
pub struct GligenTextBoxApply<
    ConditioningTo: crate::nodes::Conditioning,
    Clip: crate::nodes::Clip,
    GligenTextboxModel: crate::nodes::Gligen,
    Text: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
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
///Output for [`GligenTextBoxApply`].
#[derive(Clone)]
pub struct GligenTextBoxApplyOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    ConditioningTo: crate::nodes::Conditioning,
    Clip: crate::nodes::Clip,
    GligenTextboxModel: crate::nodes::Gligen,
    Text: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
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
    type Output = GligenTextBoxApplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "GLIGENTextBoxApply";
    const DISPLAY_NAME: &'static str = "GLIGENTextBoxApply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/gligen";
}
