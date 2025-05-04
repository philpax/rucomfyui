//!`deprecated` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`DiffusersLoader`](super::DiffusersLoader).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct DiffusersLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
        ///No documentation.
        pub vae: crate::nodes::types::VaeOut,
    }
}
///**DiffusersLoader**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DiffusersLoader<ModelPathParam: crate::nodes::types::String> {
    ///No documentation.
    pub model_path: ModelPathParam,
}
impl<ModelPathParam: crate::nodes::types::String> DiffusersLoader<ModelPathParam> {
    /// Create a new node.
    pub fn new(model_path: ModelPathParam) -> Self {
        Self { model_path }
    }
}
impl<ModelPathParam: crate::nodes::types::String> crate::nodes::TypedNode
for DiffusersLoader<ModelPathParam> {
    type Output = out::DiffusersLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip: crate::nodes::types::ClipOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_path".to_string(), self.model_path.clone().into());
        output
    }
    const NAME: &'static str = "DiffusersLoader";
    const DISPLAY_NAME: &'static str = "DiffusersLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders/deprecated";
}
