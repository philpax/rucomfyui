//!`flux` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**CLIPTextEncodeFlux**: No description.
pub struct ClipTextEncodeFlux<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub clip_l: ClipL,
    ///No documentation.
    pub t_5_xxl: T5Xxl,
    ///No documentation.
    pub guidance: Guidance,
}
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("clip_l".to_string(), self.clip_l.to_workflow_input());
        output.insert("t5xxl".to_string(), self.t_5_xxl.to_workflow_input());
        output.insert("guidance".to_string(), self.guidance.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeFlux";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
///**FluxGuidance**: No description.
pub struct FluxGuidance<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub guidance: Guidance,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for FluxGuidance<Conditioning, Guidance> {
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
        output.insert("guidance".to_string(), self.guidance.to_workflow_input());
        output
    }
    const NAME: &'static str = "FluxGuidance";
    const DISPLAY_NAME: &'static str = "FluxGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
