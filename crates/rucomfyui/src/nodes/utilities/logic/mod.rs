//!`logic` definitions/categories.
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
///**And**: Logical AND operation. Returns true if all of the values are truthy. Uses Python's rules for truthiness.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfyAndNode {}
impl ComfyAndNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for ComfyAndNode {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "ComfyAndNode";
    const DISPLAY_NAME: &'static str = "And";
    const DESCRIPTION: &'static str = "Logical AND operation. Returns true if all of the values are truthy. Uses Python's rules for truthiness.";
    const CATEGORY: &'static str = "utilities/logic";
}
///**Not**: Logical NOT operation. Returns true if the value is falsy. Uses Python's rules for truthiness.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfyNotNode<ValueParam: crate::nodes::types::Wildcard> {
    ///No documentation.
    pub value: ValueParam,
}
impl<ValueParam: crate::nodes::types::Wildcard> ComfyNotNode<ValueParam> {
    /// Create a new node.
    pub fn new(value: ValueParam) -> Self {
        Self { value }
    }
}
impl<ValueParam: crate::nodes::types::Wildcard> crate::nodes::TypedNode
for ComfyNotNode<ValueParam> {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "ComfyNotNode";
    const DISPLAY_NAME: &'static str = "Not";
    const DESCRIPTION: &'static str = "Logical NOT operation. Returns true if the value is falsy. Uses Python's rules for truthiness.";
    const CATEGORY: &'static str = "utilities/logic";
}
///**Or**: Logical OR operation. Returns true if any of the values are truthy. Uses Python's rules for truthiness.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfyOrNode {}
impl ComfyOrNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for ComfyOrNode {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "ComfyOrNode";
    const DISPLAY_NAME: &'static str = "Or";
    const DESCRIPTION: &'static str = "Logical OR operation. Returns true if any of the values are truthy. Uses Python's rules for truthiness.";
    const CATEGORY: &'static str = "utilities/logic";
}
///**Switch**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfySwitchNode<
    SwitchParam: crate::nodes::types::Boolean,
    OnFalseParam: crate::nodes::types::ComfyMatchTypeV3,
    OnTrueParam: crate::nodes::types::ComfyMatchTypeV3,
> {
    ///No documentation.
    pub switch: SwitchParam,
    ///No documentation.
    pub on_false: OnFalseParam,
    ///No documentation.
    pub on_true: OnTrueParam,
}
impl<
    SwitchParam: crate::nodes::types::Boolean,
    OnFalseParam: crate::nodes::types::ComfyMatchTypeV3,
    OnTrueParam: crate::nodes::types::ComfyMatchTypeV3,
> ComfySwitchNode<SwitchParam, OnFalseParam, OnTrueParam> {
    /// Create a new node.
    pub fn new(
        switch: SwitchParam,
        on_false: OnFalseParam,
        on_true: OnTrueParam,
    ) -> Self {
        Self { switch, on_false, on_true }
    }
}
impl<
    SwitchParam: crate::nodes::types::Boolean,
    OnFalseParam: crate::nodes::types::ComfyMatchTypeV3,
    OnTrueParam: crate::nodes::types::ComfyMatchTypeV3,
> crate::nodes::TypedNode for ComfySwitchNode<SwitchParam, OnFalseParam, OnTrueParam> {
    type Output = crate::nodes::types::ComfyMatchTypeV3Out;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("switch".to_string(), self.switch.clone().into());
        output.insert("on_false".to_string(), self.on_false.clone().into());
        output.insert("on_true".to_string(), self.on_true.clone().into());
        output
    }
    const NAME: &'static str = "ComfySwitchNode";
    const DISPLAY_NAME: &'static str = "Switch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities/logic";
}
