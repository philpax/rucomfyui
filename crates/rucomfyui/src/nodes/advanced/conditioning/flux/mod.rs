//!`flux` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**CLIPTextEncodeFlux**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeFlux<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_l: ClipLParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub t_5_xxl: T5XxlParam,
    /**No documentation.

**Metadata**:
  - Default: 3.5
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub guidance: GuidanceParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
> ClipTextEncodeFlux<ClipParam, ClipLParam, T5XxlParam, GuidanceParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        clip_l: ClipLParam,
        t_5_xxl: T5XxlParam,
        guidance: GuidanceParam,
    ) -> Self {
        Self {
            clip,
            clip_l,
            t_5_xxl,
            guidance,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ClipTextEncodeFlux<ClipParam, ClipLParam, T5XxlParam, GuidanceParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("clip_l".to_string(), self.clip_l.clone().into());
        output.insert("t5xxl".to_string(), self.t_5_xxl.clone().into());
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeFlux";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
///**FluxDisableGuidance**: This node completely disables the guidance embed on Flux and Flux like models
#[derive(Clone)]
pub struct FluxDisableGuidance<ConditioningParam: crate::nodes::types::Conditioning> {
    ///No documentation.
    pub conditioning: ConditioningParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
> FluxDisableGuidance<ConditioningParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam) -> Self {
        Self { conditioning }
    }
}
impl<ConditioningParam: crate::nodes::types::Conditioning> crate::nodes::TypedNode
for FluxDisableGuidance<ConditioningParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output
    }
    const NAME: &'static str = "FluxDisableGuidance";
    const DISPLAY_NAME: &'static str = "FluxDisableGuidance";
    const DESCRIPTION: &'static str = "This node completely disables the guidance embed on Flux and Flux like models";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
///**FluxGuidance**: No description.
#[derive(Clone)]
pub struct FluxGuidance<
    ConditioningParam: crate::nodes::types::Conditioning,
    GuidanceParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 3.5
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub guidance: GuidanceParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    GuidanceParam: crate::nodes::types::Float,
> FluxGuidance<ConditioningParam, GuidanceParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam, guidance: GuidanceParam) -> Self {
        Self { conditioning, guidance }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    GuidanceParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for FluxGuidance<ConditioningParam, GuidanceParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output
    }
    const NAME: &'static str = "FluxGuidance";
    const DISPLAY_NAME: &'static str = "FluxGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
