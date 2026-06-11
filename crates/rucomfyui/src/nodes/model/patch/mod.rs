//!`patch` definitions/categories.
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
pub mod chroma_radiance;
#[rustfmt::skip]
pub mod flux;
#[rustfmt::skip]
pub mod supir;
#[rustfmt::skip]
pub mod unet;
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
    FreenoiseParam: crate::nodes::types::Boolean,
    CondRetainIndexListParam: crate::nodes::types::String,
    SplitCondsToWindowsParam: crate::nodes::types::Boolean,
    CausalWindowFixParam: crate::nodes::types::Boolean,
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
    /**Whether to apply FreeNoise noise shuffling, improves window blending.

**Metadata**:
  - Default: false
*/
    pub freenoise: FreenoiseParam,
    /**List of latent indices to retain in the conditioning tensors for each window, for example setting this to '0' will use the initial start image for each window.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub cond_retain_index_list: CondRetainIndexListParam,
    /**Whether to split multiple conditionings (created by ConditionCombine) to each window based on region index.

**Metadata**:
  - Default: false
*/
    pub split_conds_to_windows: SplitCondsToWindowsParam,
    /**Whether to add a causal fix frame to non-0-indexed context windows.

**Metadata**:
  - Default: true
*/
    pub causal_window_fix: CausalWindowFixParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    DimParam: crate::nodes::types::Int,
    FreenoiseParam: crate::nodes::types::Boolean,
    CondRetainIndexListParam: crate::nodes::types::String,
    SplitCondsToWindowsParam: crate::nodes::types::Boolean,
    CausalWindowFixParam: crate::nodes::types::Boolean,
> ContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    DimParam,
    FreenoiseParam,
    CondRetainIndexListParam,
    SplitCondsToWindowsParam,
    CausalWindowFixParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        context_length: ContextLengthParam,
        context_overlap: ContextOverlapParam,
        context_stride: ContextStrideParam,
        closed_loop: ClosedLoopParam,
        dim: DimParam,
        freenoise: FreenoiseParam,
        cond_retain_index_list: CondRetainIndexListParam,
        split_conds_to_windows: SplitCondsToWindowsParam,
        causal_window_fix: CausalWindowFixParam,
    ) -> Self {
        Self {
            model,
            context_length,
            context_overlap,
            context_stride,
            closed_loop,
            dim,
            freenoise,
            cond_retain_index_list,
            split_conds_to_windows,
            causal_window_fix,
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
    FreenoiseParam: crate::nodes::types::Boolean,
    CondRetainIndexListParam: crate::nodes::types::String,
    SplitCondsToWindowsParam: crate::nodes::types::Boolean,
    CausalWindowFixParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    DimParam,
    FreenoiseParam,
    CondRetainIndexListParam,
    SplitCondsToWindowsParam,
    CausalWindowFixParam,
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
        output.insert("freenoise".to_string(), self.freenoise.clone().into());
        output
            .insert(
                "cond_retain_index_list".to_string(),
                self.cond_retain_index_list.clone().into(),
            );
        output
            .insert(
                "split_conds_to_windows".to_string(),
                self.split_conds_to_windows.clone().into(),
            );
        output
            .insert(
                "causal_window_fix".to_string(),
                self.causal_window_fix.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ContextWindowsManual";
    const DISPLAY_NAME: &'static str = "Context Windows (Manual)";
    const DESCRIPTION: &'static str = "Manually set context windows.";
    const CATEGORY: &'static str = "model/patch";
}
///**ScaleROPE**: Scale and shift the ROPE of the model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ScaleROPE<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_x: ScaleXParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_x: ShiftXParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_y: ScaleYParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_y: ShiftYParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_t: ScaleTParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_t: ShiftTParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> ScaleROPE<
    ModelParam,
    ScaleXParam,
    ShiftXParam,
    ScaleYParam,
    ShiftYParam,
    ScaleTParam,
    ShiftTParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        scale_x: ScaleXParam,
        shift_x: ShiftXParam,
        scale_y: ScaleYParam,
        shift_y: ShiftYParam,
        scale_t: ScaleTParam,
        shift_t: ShiftTParam,
    ) -> Self {
        Self {
            model,
            scale_x,
            shift_x,
            scale_y,
            shift_y,
            scale_t,
            shift_t,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ScaleROPE<
    ModelParam,
    ScaleXParam,
    ShiftXParam,
    ScaleYParam,
    ShiftYParam,
    ScaleTParam,
    ShiftTParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scale_x".to_string(), self.scale_x.clone().into());
        output.insert("shift_x".to_string(), self.shift_x.clone().into());
        output.insert("scale_y".to_string(), self.scale_y.clone().into());
        output.insert("shift_y".to_string(), self.shift_y.clone().into());
        output.insert("scale_t".to_string(), self.scale_t.clone().into());
        output.insert("shift_t".to_string(), self.shift_t.clone().into());
        output
    }
    const NAME: &'static str = "ScaleROPE";
    const DISPLAY_NAME: &'static str = "ScaleROPE";
    const DESCRIPTION: &'static str = "Scale and shift the ROPE of the model.";
    const CATEGORY: &'static str = "model/patch";
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
    FreenoiseParam: crate::nodes::types::Boolean,
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
    /**Whether to apply FreeNoise noise shuffling, improves window blending.

**Metadata**:
  - Default: false
*/
    pub freenoise: FreenoiseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    FreenoiseParam: crate::nodes::types::Boolean,
> WanContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    FreenoiseParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        context_length: ContextLengthParam,
        context_overlap: ContextOverlapParam,
        context_stride: ContextStrideParam,
        closed_loop: ClosedLoopParam,
        freenoise: FreenoiseParam,
    ) -> Self {
        Self {
            model,
            context_length,
            context_overlap,
            context_stride,
            closed_loop,
            freenoise,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ContextLengthParam: crate::nodes::types::Int,
    ContextOverlapParam: crate::nodes::types::Int,
    ContextStrideParam: crate::nodes::types::Int,
    ClosedLoopParam: crate::nodes::types::Boolean,
    FreenoiseParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanContextWindowsManual<
    ModelParam,
    ContextLengthParam,
    ContextOverlapParam,
    ContextStrideParam,
    ClosedLoopParam,
    FreenoiseParam,
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
        output.insert("freenoise".to_string(), self.freenoise.clone().into());
        output
    }
    const NAME: &'static str = "WanContextWindowsManual";
    const DISPLAY_NAME: &'static str = "WAN Context Windows (Manual)";
    const DESCRIPTION: &'static str = "Manually set context windows for WAN-like models (dim=2).";
    const CATEGORY: &'static str = "model/patch";
}
