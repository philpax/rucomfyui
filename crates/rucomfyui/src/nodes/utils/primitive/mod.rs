//!`primitive` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Boolean**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PrimitiveBoolean<ValueParam: crate::nodes::types::Boolean> {
    ///No documentation.
    pub value: ValueParam,
}
impl<ValueParam: crate::nodes::types::Boolean> PrimitiveBoolean<ValueParam> {
    /// Create a new node.
    pub fn new(value: ValueParam) -> Self {
        Self { value }
    }
}
impl<ValueParam: crate::nodes::types::Boolean> crate::nodes::TypedNode
for PrimitiveBoolean<ValueParam> {
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
#[allow(non_camel_case_types)]
pub struct PrimitiveFloat<ValueParam: crate::nodes::types::Float> {
    ///No documentation.
    pub value: ValueParam,
}
impl<ValueParam: crate::nodes::types::Float> PrimitiveFloat<ValueParam> {
    /// Create a new node.
    pub fn new(value: ValueParam) -> Self {
        Self { value }
    }
}
impl<ValueParam: crate::nodes::types::Float> crate::nodes::TypedNode
for PrimitiveFloat<ValueParam> {
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
#[allow(non_camel_case_types)]
pub struct PrimitiveInt<ValueParam: crate::nodes::types::Int> {
    ///No documentation.
    pub value: ValueParam,
}
impl<ValueParam: crate::nodes::types::Int> PrimitiveInt<ValueParam> {
    /// Create a new node.
    pub fn new(value: ValueParam) -> Self {
        Self { value }
    }
}
impl<ValueParam: crate::nodes::types::Int> crate::nodes::TypedNode
for PrimitiveInt<ValueParam> {
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
#[allow(non_camel_case_types)]
pub struct PrimitiveString<ValueParam: crate::nodes::types::String> {
    ///No documentation.
    pub value: ValueParam,
}
impl<ValueParam: crate::nodes::types::String> PrimitiveString<ValueParam> {
    /// Create a new node.
    pub fn new(value: ValueParam) -> Self {
        Self { value }
    }
}
impl<ValueParam: crate::nodes::types::String> crate::nodes::TypedNode
for PrimitiveString<ValueParam> {
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
