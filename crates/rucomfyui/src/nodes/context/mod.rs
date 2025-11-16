//!`context` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Context Windows (Manual)**: Manually set context windows.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ContextWindowsManual<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    DimParam: crate::nodes::types::Int,
> {
    ///The model to apply context windows to during sampling.
    pub model: ModelParam,
    ///The length of the context window.
    pub context_length: ContextLengthParam,
    ///The overlap of the context window.
    pub context_overlap: ContextOverlapParam,
    ///The stride of the context window; only applicable to uniform schedules.
    pub context_stride: ContextStrideParam,
    /**Whether to close the context window loop; only applicable to looped schedules.

**Metadata**:
  - Default: false
*/
    pub closed_loop: ClosedLoopParam,
    /**The dimension to apply the context windows to.

**Metadata**:
  - Default: 0
  - Max: 5
  - Min: 0
*/
    pub dim: DimParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    DimParam: crate::nodes::types::Int,
> ContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    DimParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        context_length: ContextLengthParam,
        context_overlap: ContextOverlapParam,
        context_stride: ContextStrideParam,
        closed_loop: ClosedLoopParam,
        dim: DimParam,
    ) -> Self {
        Self {
            model,
            context_length,
            context_overlap,
            context_stride,
            closed_loop,
            dim,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    DimParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    DimParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("context_length".to_string(), self.context_length.clone().into());
        output
            .insert("context_overlap".to_string(), self.context_overlap.clone().into());
        output.insert("context_stride".to_string(), self.context_stride.clone().into());
        output.insert("closed_loop".to_string(), self.closed_loop.clone().into());
        output.insert("dim".to_string(), self.dim.clone().into());
        output
    }
    const NAME: &'static str = "ContextWindowsManual";
    const DISPLAY_NAME: &'static str = "Context Windows (Manual)";
    const DESCRIPTION: &'static str = "Manually set context windows.";
    const CATEGORY: &'static str = "context";
}
///**WAN Context Windows (Manual)**: Manually set context windows for WAN-like models (dim=2).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanContextWindowsManual<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
> {
    ///The model to apply context windows to during sampling.
    pub model: ModelParam,
    /**The length of the context window.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub context_length: ContextLengthParam,
    ///The overlap of the context window.
    pub context_overlap: ContextOverlapParam,
    ///The stride of the context window; only applicable to uniform schedules.
    pub context_stride: ContextStrideParam,
    /**Whether to close the context window loop; only applicable to looped schedules.

**Metadata**:
  - Default: false
*/
    pub closed_loop: ClosedLoopParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
> WanContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        context_length: ContextLengthParam,
        context_overlap: ContextOverlapParam,
        context_stride: ContextStrideParam,
        closed_loop: ClosedLoopParam,
    ) -> Self {
        Self {
            model,
            context_length,
            context_overlap,
            context_stride,
            closed_loop,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("context_length".to_string(), self.context_length.clone().into());
        output
            .insert("context_overlap".to_string(), self.context_overlap.clone().into());
        output.insert("context_stride".to_string(), self.context_stride.clone().into());
        output.insert("closed_loop".to_string(), self.closed_loop.clone().into());
        output
    }
    const NAME: &'static str = "WanContextWindowsManual";
    const DISPLAY_NAME: &'static str = "WAN Context Windows (Manual)";
    const DESCRIPTION: &'static str = "Manually set context windows for WAN-like models (dim=2).";
    const CATEGORY: &'static str = "context";
}
