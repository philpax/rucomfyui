//!Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
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
use crate::WorkflowInput;
/// Implemented for all typed nodes; provides the node's output and metadata.
pub trait TypedNode {
    /// The type of the node's output.
    type Output;
    /// Returns the node's output.
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output;
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
/// Converts a value to a workflow input.
pub trait ToWorkflowInput {
    /// Converts the value to a workflow input.
    fn to_workflow_input(&self) -> WorkflowInput;
}
impl ToWorkflowInput for std::string::String {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::String(self.clone())
    }
}
impl types::String for std::string::String {}
impl<'a> ToWorkflowInput for &'a str {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::String(self.to_string())
    }
}
impl<'a> types::String for &'a str {}
impl ToWorkflowInput for f32 {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::F32(*self)
    }
}
impl types::Float for f32 {}
impl ToWorkflowInput for i32 {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::I32(*self)
    }
}
impl types::Int for i32 {}
impl ToWorkflowInput for bool {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Boolean(*self)
    }
}
impl types::Boolean for bool {}
