//!`custom_sampling` definitions/categories.
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
#[rustfmt::skip]
pub mod noise;
///**ManualSigmas**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ManualSigmas<SigmasParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 1, 0.5
*/
    pub sigmas: SigmasParam,
}
impl<SigmasParam: crate::nodes::types::String> ManualSigmas<SigmasParam> {
    /// Create a new node.
    pub fn new(sigmas: SigmasParam) -> Self {
        Self { sigmas }
    }
}
impl<SigmasParam: crate::nodes::types::String> crate::nodes::TypedNode
for ManualSigmas<SigmasParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output
    }
    const NAME: &'static str = "ManualSigmas";
    const DISPLAY_NAME: &'static str = "ManualSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/custom_sampling";
}
