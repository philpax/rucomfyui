//!`multigpu` definitions/categories.
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
#[doc = "**MultiGPU CFG Split**: Prepares model to have sampling accelerated via splitting work units.\n\n\n\nShould be placed after nodes that modify the model object itself, such as compile or attention-switch nodes.\n\n\n\nOther than those exceptions, this node can be placed in any order."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MultiGPU_WorkUnits<
    ModelParam: crate::nodes::types::Model,
    MaxGpusParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub max_gpus: MaxGpusParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxGpusParam: crate::nodes::types::Int,
> MultiGPU_WorkUnits<ModelParam, MaxGpusParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, max_gpus: MaxGpusParam) -> Self {
        Self { model, max_gpus }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxGpusParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for MultiGPU_WorkUnits<ModelParam, MaxGpusParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("max_gpus".to_string(), self.max_gpus.clone().into());
        output
    }
    const NAME: &'static str = "MultiGPU_WorkUnits";
    const DISPLAY_NAME: &'static str = "MultiGPU CFG Split";
    const DESCRIPTION: &'static str = "Prepares model to have sampling accelerated via splitting work units.\n\nShould be placed after nodes that modify the model object itself, such as compile or attention-switch nodes.\n\nOther than those exceptions, this node can be placed in any order.";
    const CATEGORY: &'static str = "advanced/multigpu";
}
#[doc = "**Select CLIP Device**: Place the CLIP text encoder on a specific device (default / cpu / gpu:N).\n\n\n\n- \"default\" restores the device assigned by the loader.\n\n- \"cpu\" pins both the load and offload device to CPU.\n\n- \"gpu:N\" pins the load device to the Nth available GPU.\n\n\n\nWhen the selected device does not exist on the current machine\n\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\n\nthe node passes the CLIP through unchanged and logs a message\n\ninstead of failing."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SelectCLIPDevice<ClipParam: crate::nodes::types::Clip> {
    ///No documentation.
    pub clip: ClipParam,
}
impl<ClipParam: crate::nodes::types::Clip> SelectCLIPDevice<ClipParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam) -> Self {
        Self { clip }
    }
}
impl<ClipParam: crate::nodes::types::Clip> crate::nodes::TypedNode
for SelectCLIPDevice<ClipParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output
    }
    const NAME: &'static str = "SelectCLIPDevice";
    const DISPLAY_NAME: &'static str = "Select CLIP Device";
    const DESCRIPTION: &'static str = "Place the CLIP text encoder on a specific device (default / cpu / gpu:N).\n\n- \"default\" restores the device assigned by the loader.\n- \"cpu\" pins both the load and offload device to CPU.\n- \"gpu:N\" pins the load device to the Nth available GPU.\n\nWhen the selected device does not exist on the current machine\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\nthe node passes the CLIP through unchanged and logs a message\ninstead of failing.";
    const CATEGORY: &'static str = "advanced/multigpu";
}
#[doc = "**Select Model Device**: Place the diffusion model on a specific device (default / cpu / gpu:N).\n\n\n\n- \"default\" restores the device assigned by the loader (even after a\n\n  prior Select Model Device call).\n\n- \"cpu\" pins both the load and offload device to CPU.\n\n- \"gpu:N\" pins the load device to the Nth available GPU; the offload\n\n  device is restored to the loader's original choice.\n\n\n\nWhen the requested device differs from the device the input model is\n\nalready on, a fresh model is spawned via the loader's reload factory\n\n(cached_patcher_init) so the new patcher owns independent weights on\n\nthe new device. Loaders that don't support multigpu (no factory) will\n\ncause the node to pass through unchanged with a warning.\n\n\n\nIf the workflow already has MultiGPU CFG Split applied and the chosen\n\nGPU collides with one of the existing multigpu clones, that clone is\n\ndropped so two patchers don't end up bound to the same device.\n\n\n\nWhen the selected device does not exist on the current machine\n\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\n\nthe node passes the model through unchanged and logs a message\n\ninstead of failing.\n\n\n\nNOTE: Placing Select Model Device *after* a node that has already\n\nconsumed the same model (e.g. a KSampler that ran on this model on\n\nthe original device) is not recommended -- any state the prior\n\nconsumer mutated on the original model will be observed when the\n\nselected device matches the original (fast path). Place Select Model\n\nDevice before any consumer of the model."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SelectModelDevice<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> SelectModelDevice<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for SelectModelDevice<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "SelectModelDevice";
    const DISPLAY_NAME: &'static str = "Select Model Device";
    const DESCRIPTION: &'static str = "Place the diffusion model on a specific device (default / cpu / gpu:N).\n\n- \"default\" restores the device assigned by the loader (even after a\n  prior Select Model Device call).\n- \"cpu\" pins both the load and offload device to CPU.\n- \"gpu:N\" pins the load device to the Nth available GPU; the offload\n  device is restored to the loader's original choice.\n\nWhen the requested device differs from the device the input model is\nalready on, a fresh model is spawned via the loader's reload factory\n(cached_patcher_init) so the new patcher owns independent weights on\nthe new device. Loaders that don't support multigpu (no factory) will\ncause the node to pass through unchanged with a warning.\n\nIf the workflow already has MultiGPU CFG Split applied and the chosen\nGPU collides with one of the existing multigpu clones, that clone is\ndropped so two patchers don't end up bound to the same device.\n\nWhen the selected device does not exist on the current machine\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\nthe node passes the model through unchanged and logs a message\ninstead of failing.\n\nNOTE: Placing Select Model Device *after* a node that has already\nconsumed the same model (e.g. a KSampler that ran on this model on\nthe original device) is not recommended -- any state the prior\nconsumer mutated on the original model will be observed when the\nselected device matches the original (fast path). Place Select Model\nDevice before any consumer of the model.";
    const CATEGORY: &'static str = "advanced/multigpu";
}
#[doc = "**Select VAE Device**: Place the VAE on a specific device (default / gpu:N).\n\n\n\n- \"default\" restores the device assigned by the loader.\n\n- \"gpu:N\" pins the load device to the Nth available GPU; the offload\n\n  device is set to the standard VAE offload device.\n\n\n\nCPU is intentionally not exposed in the UI for the VAE; if a workflow\n\nsupplies \"cpu\" anyway (e.g. opened from another machine), the request\n\nis dropped with a log message and the VAE is passed through unchanged.\n\n\n\nWhen the selected device does not exist on the current machine\n\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\n\nthe node passes the VAE through unchanged and logs a message\n\ninstead of failing."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SelectVAEDevice<VaeParam: crate::nodes::types::Vae> {
    ///No documentation.
    pub vae: VaeParam,
}
impl<VaeParam: crate::nodes::types::Vae> SelectVAEDevice<VaeParam> {
    /// Create a new node.
    pub fn new(vae: VaeParam) -> Self {
        Self { vae }
    }
}
impl<VaeParam: crate::nodes::types::Vae> crate::nodes::TypedNode
for SelectVAEDevice<VaeParam> {
    type Output = crate::nodes::types::VaeOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "SelectVAEDevice";
    const DISPLAY_NAME: &'static str = "Select VAE Device";
    const DESCRIPTION: &'static str = "Place the VAE on a specific device (default / gpu:N).\n\n- \"default\" restores the device assigned by the loader.\n- \"gpu:N\" pins the load device to the Nth available GPU; the offload\n  device is set to the standard VAE offload device.\n\nCPU is intentionally not exposed in the UI for the VAE; if a workflow\nsupplies \"cpu\" anyway (e.g. opened from another machine), the request\nis dropped with a log message and the VAE is passed through unchanged.\n\nWhen the selected device does not exist on the current machine\n(e.g. a workflow built on a 2-GPU box opened on a 1-GPU box),\nthe node passes the VAE through unchanged and logs a message\ninstead of failing.";
    const CATEGORY: &'static str = "advanced/multigpu";
}
