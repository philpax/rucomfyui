//!`utils` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod primitive;
pub mod string;
///**Preview Any**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewAny<SourceParam: crate::nodes::types::Wildcard> {
    ///No documentation.
    pub source: SourceParam,
}
impl<SourceParam: crate::nodes::types::Wildcard> PreviewAny<SourceParam> {
    /// Create a new node.
    pub fn new(source: SourceParam) -> Self {
        Self { source }
    }
}
impl<SourceParam: crate::nodes::types::Wildcard> crate::nodes::TypedNode
for PreviewAny<SourceParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("source".to_string(), self.source.clone().into());
        output
    }
    const NAME: &'static str = "PreviewAny";
    const DISPLAY_NAME: &'static str = "Preview Any";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils";
}
impl<SourceParam: crate::nodes::types::Wildcard> crate::nodes::TypedOutputNode
for PreviewAny<SourceParam> {}
