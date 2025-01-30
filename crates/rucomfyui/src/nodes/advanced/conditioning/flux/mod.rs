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
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_l: ClipL,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub t_5_xxl: T5Xxl,
    /**No documentation.

**Metadata**:
  - Default: 3.5
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub guidance: Guidance,
}
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
    /// Create a new node.
    pub fn new(clip: Clip, clip_l: ClipL, t_5_xxl: T5Xxl, guidance: Guidance) -> Self {
        Self {
            clip,
            clip_l,
            t_5_xxl,
            guidance,
        }
    }
}
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
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
pub struct FluxDisableGuidance<Conditioning: crate::nodes::types::Conditioning> {
    ///No documentation.
    pub conditioning: Conditioning,
}
impl<Conditioning: crate::nodes::types::Conditioning> FluxDisableGuidance<Conditioning> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning) -> Self {
        Self { conditioning }
    }
}
impl<Conditioning: crate::nodes::types::Conditioning> crate::nodes::TypedNode
for FluxDisableGuidance<Conditioning> {
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
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    /**No documentation.

**Metadata**:
  - Default: 3.5
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub guidance: Guidance,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> FluxGuidance<Conditioning, Guidance> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning, guidance: Guidance) -> Self {
        Self { conditioning, guidance }
    }
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for FluxGuidance<Conditioning, Guidance> {
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
