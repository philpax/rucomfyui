//!`model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ModelComputeDtype**: No description.
#[derive(Clone)]
pub struct ModelComputeDtype<
    Model: crate::nodes::types::Model,
    Dtype: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub dtype: Dtype,
}
impl<
    Model: crate::nodes::types::Model,
    Dtype: crate::nodes::types::String,
> ModelComputeDtype<Model, Dtype> {
    /// Create a new node.
    pub fn new(model: Model, dtype: Dtype) -> Self {
        Self { model, dtype }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Dtype: crate::nodes::types::String,
> crate::nodes::TypedNode for ModelComputeDtype<Model, Dtype> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("dtype".to_string(), self.dtype.clone().into());
        output
    }
    const NAME: &'static str = "ModelComputeDtype";
    const DISPLAY_NAME: &'static str = "ModelComputeDtype";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/debug/model";
}
