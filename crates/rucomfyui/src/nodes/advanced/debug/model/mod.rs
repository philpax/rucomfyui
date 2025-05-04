//!`model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ModelComputeDtype**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelComputeDtype<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub dtype: DtypeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> ModelComputeDtype<ModelParam, DtypeParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, dtype: DtypeParam) -> Self {
        Self { model, dtype }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ModelComputeDtype<ModelParam, DtypeParam> {
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
