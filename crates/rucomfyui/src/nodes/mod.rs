//! Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
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
use crate::workflow::{WorkflowInput, WorkflowNodeId};
use std::collections::HashMap;
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
