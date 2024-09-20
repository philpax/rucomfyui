//!`style_model` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**Apply Style Model**
pub struct StyleModelApply<
    Conditioning: crate::nodes::Conditioning,
    StyleModel: crate::nodes::StyleModel,
    ClipVisionOutput: crate::nodes::ClipVisionOutput,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub style_model: StyleModel,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
}
///Output for [`StyleModelApply`].
#[derive(Clone)]
pub struct StyleModelApplyOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    StyleModel: crate::nodes::StyleModel,
    ClipVisionOutput: crate::nodes::ClipVisionOutput,
> crate::nodes::TypedNode
for StyleModelApply<Conditioning, StyleModel, ClipVisionOutput> {
    type Output = StyleModelApplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "StyleModelApply";
    const DISPLAY_NAME: &'static str = "Apply Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/style_model";
}
