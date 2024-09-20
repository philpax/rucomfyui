//!`samplers` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**KSamplerSelect**: No description.
#[derive(Clone)]
pub struct KSamplerSelect<SamplerName: crate::nodes::types::String> {
    ///No documentation.
    pub sampler_name: SamplerName,
}
impl<SamplerName: crate::nodes::types::String> KSamplerSelect<SamplerName> {
    /// Create a new node.
    pub fn new(sampler_name: SamplerName) -> Self {
        Self { sampler_name }
    }
}
impl<SamplerName: crate::nodes::types::String> crate::nodes::TypedNode
for KSamplerSelect<SamplerName> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sampler_name".to_string(), self.sampler_name.clone().into());
        output
    }
    const NAME: &'static str = "KSamplerSelect";
    const DISPLAY_NAME: &'static str = "KSamplerSelect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMAdaptative**: No description.
#[derive(Clone)]
pub struct SamplerDpmAdaptative<
    Order: crate::nodes::types::Int,
    Rtol: crate::nodes::types::Float,
    Atol: crate::nodes::types::Float,
    HInit: crate::nodes::types::Float,
    Pcoeff: crate::nodes::types::Float,
    Icoeff: crate::nodes::types::Float,
    Dcoeff: crate::nodes::types::Float,
    AcceptSafety: crate::nodes::types::Float,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub order: Order,
    ///No documentation.
    pub rtol: Rtol,
    ///No documentation.
    pub atol: Atol,
    ///No documentation.
    pub h_init: HInit,
    ///No documentation.
    pub pcoeff: Pcoeff,
    ///No documentation.
    pub icoeff: Icoeff,
    ///No documentation.
    pub dcoeff: Dcoeff,
    ///No documentation.
    pub accept_safety: AcceptSafety,
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
impl<
    Order: crate::nodes::types::Int,
    Rtol: crate::nodes::types::Float,
    Atol: crate::nodes::types::Float,
    HInit: crate::nodes::types::Float,
    Pcoeff: crate::nodes::types::Float,
    Icoeff: crate::nodes::types::Float,
    Dcoeff: crate::nodes::types::Float,
    AcceptSafety: crate::nodes::types::Float,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> SamplerDpmAdaptative<
    Order,
    Rtol,
    Atol,
    HInit,
    Pcoeff,
    Icoeff,
    Dcoeff,
    AcceptSafety,
    Eta,
    SNoise,
> {
    /// Create a new node.
    pub fn new(
        order: Order,
        rtol: Rtol,
        atol: Atol,
        h_init: HInit,
        pcoeff: Pcoeff,
        icoeff: Icoeff,
        dcoeff: Dcoeff,
        accept_safety: AcceptSafety,
        eta: Eta,
        s_noise: SNoise,
    ) -> Self {
        Self {
            order,
            rtol,
            atol,
            h_init,
            pcoeff,
            icoeff,
            dcoeff,
            accept_safety,
            eta,
            s_noise,
        }
    }
}
impl<
    Order: crate::nodes::types::Int,
    Rtol: crate::nodes::types::Float,
    Atol: crate::nodes::types::Float,
    HInit: crate::nodes::types::Float,
    Pcoeff: crate::nodes::types::Float,
    Icoeff: crate::nodes::types::Float,
    Dcoeff: crate::nodes::types::Float,
    AcceptSafety: crate::nodes::types::Float,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SamplerDpmAdaptative<
    Order,
    Rtol,
    Atol,
    HInit,
    Pcoeff,
    Icoeff,
    Dcoeff,
    AcceptSafety,
    Eta,
    SNoise,
> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("order".to_string(), self.order.clone().into());
        output.insert("rtol".to_string(), self.rtol.clone().into());
        output.insert("atol".to_string(), self.atol.clone().into());
        output.insert("h_init".to_string(), self.h_init.clone().into());
        output.insert("pcoeff".to_string(), self.pcoeff.clone().into());
        output.insert("icoeff".to_string(), self.icoeff.clone().into());
        output.insert("dcoeff".to_string(), self.dcoeff.clone().into());
        output.insert("accept_safety".to_string(), self.accept_safety.clone().into());
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMAdaptative";
    const DISPLAY_NAME: &'static str = "SamplerDPMAdaptative";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_2M_SDE**: No description.
#[derive(Clone)]
pub struct SamplerDpmpp2MSde<
    SolverType: crate::nodes::types::String,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> {
    ///No documentation.
    pub solver_type: SolverType,
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
    ///No documentation.
    pub noise_device: NoiseDevice,
}
impl<
    SolverType: crate::nodes::types::String,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> SamplerDpmpp2MSde<SolverType, Eta, SNoise, NoiseDevice> {
    /// Create a new node.
    pub fn new(
        solver_type: SolverType,
        eta: Eta,
        s_noise: SNoise,
        noise_device: NoiseDevice,
    ) -> Self {
        Self {
            solver_type,
            eta,
            s_noise,
            noise_device,
        }
    }
}
impl<
    SolverType: crate::nodes::types::String,
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> crate::nodes::TypedNode for SamplerDpmpp2MSde<SolverType, Eta, SNoise, NoiseDevice> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("solver_type".to_string(), self.solver_type.clone().into());
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("noise_device".to_string(), self.noise_device.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_2S_Ancestral**: No description.
#[derive(Clone)]
pub struct SamplerDpmpp2SAncestral<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> SamplerDpmpp2SAncestral<Eta, SNoise> {
    /// Create a new node.
    pub fn new(eta: Eta, s_noise: SNoise) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerDpmpp2SAncestral<Eta, SNoise> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_3M_SDE**: No description.
#[derive(Clone)]
pub struct SamplerDpmpp3MSde<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
    ///No documentation.
    pub noise_device: NoiseDevice,
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> SamplerDpmpp3MSde<Eta, SNoise, NoiseDevice> {
    /// Create a new node.
    pub fn new(eta: Eta, s_noise: SNoise, noise_device: NoiseDevice) -> Self {
        Self { eta, s_noise, noise_device }
    }
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> crate::nodes::TypedNode for SamplerDpmpp3MSde<Eta, SNoise, NoiseDevice> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("noise_device".to_string(), self.noise_device.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_SDE**: No description.
#[derive(Clone)]
pub struct SamplerDpmppSde<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    R: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
    ///No documentation.
    pub r: R,
    ///No documentation.
    pub noise_device: NoiseDevice,
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    R: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> SamplerDpmppSde<Eta, SNoise, R, NoiseDevice> {
    /// Create a new node.
    pub fn new(eta: Eta, s_noise: SNoise, r: R, noise_device: NoiseDevice) -> Self {
        Self {
            eta,
            s_noise,
            r,
            noise_device,
        }
    }
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
    R: crate::nodes::types::Float,
    NoiseDevice: crate::nodes::types::String,
> crate::nodes::TypedNode for SamplerDpmppSde<Eta, SNoise, R, NoiseDevice> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("r".to_string(), self.r.clone().into());
        output.insert("noise_device".to_string(), self.noise_device.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMPP_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerEulerAncestral**: No description.
#[derive(Clone)]
pub struct SamplerEulerAncestral<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> SamplerEulerAncestral<Eta, SNoise> {
    /// Create a new node.
    pub fn new(eta: Eta, s_noise: SNoise) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerEulerAncestral<Eta, SNoise> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
    }
    const NAME: &'static str = "SamplerEulerAncestral";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerEulerAncestralCFG++**: No description.
#[derive(Clone)]
pub struct SamplerEulerAncestralCfgpp<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> SamplerEulerAncestralCfgpp<Eta, SNoise> {
    /// Create a new node.
    pub fn new(eta: Eta, s_noise: SNoise) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    Eta: crate::nodes::types::Float,
    SNoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerEulerAncestralCfgpp<Eta, SNoise> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
    }
    const NAME: &'static str = "SamplerEulerAncestralCFGPP";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestralCFG++";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerLCMUpscale**: No description.
#[derive(Clone)]
pub struct SamplerLcmUpscale<
    ScaleRatio: crate::nodes::types::Float,
    ScaleSteps: crate::nodes::types::Int,
    UpscaleMethod: crate::nodes::types::String,
> {
    ///No documentation.
    pub scale_ratio: ScaleRatio,
    ///No documentation.
    pub scale_steps: ScaleSteps,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
}
impl<
    ScaleRatio: crate::nodes::types::Float,
    ScaleSteps: crate::nodes::types::Int,
    UpscaleMethod: crate::nodes::types::String,
> SamplerLcmUpscale<ScaleRatio, ScaleSteps, UpscaleMethod> {
    /// Create a new node.
    pub fn new(
        scale_ratio: ScaleRatio,
        scale_steps: ScaleSteps,
        upscale_method: UpscaleMethod,
    ) -> Self {
        Self {
            scale_ratio,
            scale_steps,
            upscale_method,
        }
    }
}
impl<
    ScaleRatio: crate::nodes::types::Float,
    ScaleSteps: crate::nodes::types::Int,
    UpscaleMethod: crate::nodes::types::String,
> crate::nodes::TypedNode for SamplerLcmUpscale<ScaleRatio, ScaleSteps, UpscaleMethod> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("scale_ratio".to_string(), self.scale_ratio.clone().into());
        output.insert("scale_steps".to_string(), self.scale_steps.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output
    }
    const NAME: &'static str = "SamplerLCMUpscale";
    const DISPLAY_NAME: &'static str = "SamplerLCMUpscale";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerLMS**: No description.
#[derive(Clone)]
pub struct SamplerLms<Order: crate::nodes::types::Int> {
    ///No documentation.
    pub order: Order,
}
impl<Order: crate::nodes::types::Int> SamplerLms<Order> {
    /// Create a new node.
    pub fn new(order: Order) -> Self {
        Self { order }
    }
}
impl<Order: crate::nodes::types::Int> crate::nodes::TypedNode for SamplerLms<Order> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("order".to_string(), self.order.clone().into());
        output
    }
    const NAME: &'static str = "SamplerLMS";
    const DISPLAY_NAME: &'static str = "SamplerLMS";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
