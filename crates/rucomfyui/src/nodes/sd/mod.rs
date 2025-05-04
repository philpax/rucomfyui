//!`sd` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**T5TokenizerOptions**: No description.
#[derive(Clone)]
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
    const CATEGORY: &'static str = "sd";
}
