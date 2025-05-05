//!`utils` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod primitive;
///**Preview Any**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewAny {}
impl PreviewAny {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for PreviewAny {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "PreviewAny";
    const DISPLAY_NAME: &'static str = "Preview Any";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils";
}
impl crate::nodes::TypedOutputNode for PreviewAny {}
