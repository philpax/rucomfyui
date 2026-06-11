//!`conditioning` definitions/categories.
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
///**CLIPTextEncodeControlnet**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPTextEncodeControlnet<
    ClipParam: crate::nodes::types::Clip,
    ConditioningParam: crate::nodes::types::Conditioning,
    TextParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: ClipParam,
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: TextParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ConditioningParam: crate::nodes::types::Conditioning,
    TextParam: crate::nodes::types::String,
> CLIPTextEncodeControlnet<ClipParam, ConditioningParam, TextParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        conditioning: ConditioningParam,
        text: TextParam,
    ) -> Self {
        Self { clip, conditioning, text }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ConditioningParam: crate::nodes::types::Conditioning,
    TextParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for CLIPTextEncodeControlnet<ClipParam, ConditioningParam, TextParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeControlnet";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeControlnet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/conditioning";
}
///**T5TokenizerOptions**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct T5TokenizerOptions<
    ClipParam: crate::nodes::types::Clip,
    MinPaddingParam: crate::nodes::types::Int,
    MinLengthParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10000
  - Min: 0
  - Step: 1
*/
    pub min_padding: MinPaddingParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10000
  - Min: 0
  - Step: 1
*/
    pub min_length: MinLengthParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    MinPaddingParam: crate::nodes::types::Int,
    MinLengthParam: crate::nodes::types::Int,
> T5TokenizerOptions<ClipParam, MinPaddingParam, MinLengthParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        min_padding: MinPaddingParam,
        min_length: MinLengthParam,
    ) -> Self {
        Self {
            clip,
            min_padding,
            min_length,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    MinPaddingParam: crate::nodes::types::Int,
    MinLengthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for T5TokenizerOptions<ClipParam, MinPaddingParam, MinLengthParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("min_padding".to_string(), self.min_padding.clone().into());
        output.insert("min_length".to_string(), self.min_length.clone().into());
        output
    }
    const NAME: &'static str = "T5TokenizerOptions";
    const DISPLAY_NAME: &'static str = "T5TokenizerOptions";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/conditioning";
}
