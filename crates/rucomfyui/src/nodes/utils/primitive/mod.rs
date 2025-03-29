//!`primitive` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Boolean**: No description.
#[derive(Clone)]
pub struct PrimitiveBoolean<Value: crate::nodes::types::Boolean> {
    ///No documentation.
    pub value: Value,
}
impl<Value: crate::nodes::types::Boolean> PrimitiveBoolean<Value> {
    /// Create a new node.
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
impl<Value: crate::nodes::types::Boolean> crate::nodes::TypedNode
for PrimitiveBoolean<Value> {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "PrimitiveBoolean";
    const DISPLAY_NAME: &'static str = "Boolean";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/primitive";
}
///**Float**: No description.
#[derive(Clone)]
pub struct PrimitiveFloat<Value: crate::nodes::types::Float> {
    ///No documentation.
    pub value: Value,
}
impl<Value: crate::nodes::types::Float> PrimitiveFloat<Value> {
    /// Create a new node.
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
impl<Value: crate::nodes::types::Float> crate::nodes::TypedNode
for PrimitiveFloat<Value> {
    type Output = crate::nodes::types::FloatOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "PrimitiveFloat";
    const DISPLAY_NAME: &'static str = "Float";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/primitive";
}
///**Int**: No description.
#[derive(Clone)]
pub struct PrimitiveInt<Value: crate::nodes::types::Int> {
    ///No documentation.
    pub value: Value,
}
impl<Value: crate::nodes::types::Int> PrimitiveInt<Value> {
    /// Create a new node.
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
impl<Value: crate::nodes::types::Int> crate::nodes::TypedNode for PrimitiveInt<Value> {
    type Output = crate::nodes::types::IntOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "PrimitiveInt";
    const DISPLAY_NAME: &'static str = "Int";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/primitive";
}
///**String**: No description.
#[derive(Clone)]
pub struct PrimitiveString<Value: crate::nodes::types::String> {
    ///No documentation.
    pub value: Value,
}
impl<Value: crate::nodes::types::String> PrimitiveString<Value> {
    /// Create a new node.
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
impl<Value: crate::nodes::types::String> crate::nodes::TypedNode
for PrimitiveString<Value> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "PrimitiveString";
    const DISPLAY_NAME: &'static str = "String";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/primitive";
}
