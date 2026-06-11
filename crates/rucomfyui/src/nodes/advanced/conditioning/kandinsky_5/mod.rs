//!`kandinsky5` definitions/categories.
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
///**CLIPTextEncodeKandinsky5**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPTextEncodeKandinsky5<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    Qwen257BParam: crate::nodes::types::String,
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
    pub qwen_25_7_b: Qwen257BParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    Qwen257BParam: crate::nodes::types::String,
> CLIPTextEncodeKandinsky5<ClipParam, ClipLParam, Qwen257BParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam, clip_l: ClipLParam, qwen_25_7_b: Qwen257BParam) -> Self {
        Self { clip, clip_l, qwen_25_7_b }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    Qwen257BParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for CLIPTextEncodeKandinsky5<ClipParam, ClipLParam, Qwen257BParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("clip_l".to_string(), self.clip_l.clone().into());
        output.insert("qwen25_7b".to_string(), self.qwen_25_7_b.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeKandinsky5";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeKandinsky5";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/kandinsky5";
}
