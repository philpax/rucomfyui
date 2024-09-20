//! Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
pub mod advanced;
pub mod audio;
pub mod conditioning;
pub mod image;
pub mod latent;
pub mod loaders;
pub mod mask;
pub mod model_patches;
pub mod sampling;
pub mod all;
pub mod types;
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
