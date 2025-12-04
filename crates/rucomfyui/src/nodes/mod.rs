//! Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
#[rustfmt::skip]
pub mod n_3_d;
#[rustfmt::skip]
pub mod advanced;
#[rustfmt::skip]
pub mod api_node;
#[rustfmt::skip]
pub mod audio;
#[rustfmt::skip]
pub mod camera;
#[rustfmt::skip]
pub mod conditioning;
#[rustfmt::skip]
pub mod context;
#[rustfmt::skip]
pub mod dataset;
#[rustfmt::skip]
pub mod image;
#[rustfmt::skip]
pub mod latent;
#[rustfmt::skip]
pub mod loaders;
#[rustfmt::skip]
pub mod mask;
#[rustfmt::skip]
pub mod model_patches;
#[rustfmt::skip]
pub mod sampling;
#[rustfmt::skip]
pub mod sd;
#[rustfmt::skip]
pub mod training;
#[rustfmt::skip]
pub mod utils;
#[rustfmt::skip]
pub mod all;
#[rustfmt::skip]
pub mod types;
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Implemented for all typed nodes; provides the node's output and metadata.
pub trait TypedNode: Clone {
    /// The type of the node's output.
    type Output;
    /// Returns the node's output.
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output;
    /// Returns the inputs for this node after conversion to [`WorkflowInput`].
    fn inputs(&self) -> HashMap<String, WorkflowInput>;
    /// The name of the node.
    const NAME: &'static str;
    /// The display name of the node.
    const DISPLAY_NAME: &'static str;
    /// The description of the node.
    const DESCRIPTION: &'static str;
    /// The category of the node.
    const CATEGORY: &'static str;
}
/// Implemented for all output nodes (i.e. nodes at which a workflow terminates).
pub trait TypedOutputNode {}
///**wanBlockSwap**: NOP
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct wanBlockSwap<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> wanBlockSwap<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for wanBlockSwap<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "wanBlockSwap";
    const DISPLAY_NAME: &'static str = "wanBlockSwap";
    const DESCRIPTION: &'static str = "NOP";
    const CATEGORY: &'static str = "";
}
