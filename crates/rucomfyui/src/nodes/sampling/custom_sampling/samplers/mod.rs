//!`samplers` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**KSamplerSelect**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KSamplerSelect<SamplerNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub sampler_name: SamplerNameParam,
}
impl<SamplerNameParam: crate::nodes::types::String> KSamplerSelect<SamplerNameParam> {
    /// Create a new node.
    pub fn new(sampler_name: SamplerNameParam) -> Self {
        Self { sampler_name }
    }
}
impl<SamplerNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for KSamplerSelect<SamplerNameParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerDPMAdaptative<
    OrderParam: crate::nodes::types::Int,
    RtolParam: crate::nodes::types::Float,
    AtolParam: crate::nodes::types::Float,
    HInitParam: crate::nodes::types::Float,
    PcoeffParam: crate::nodes::types::Float,
    IcoeffParam: crate::nodes::types::Float,
    DcoeffParam: crate::nodes::types::Float,
    AcceptSafetyParam: crate::nodes::types::Float,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 3
  - Min: 2
*/
    pub order: OrderParam,
    /**No documentation.

**Metadata**:
  - Default: 0.05
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub rtol: RtolParam,
    /**No documentation.

**Metadata**:
  - Default: 0.0078
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub atol: AtolParam,
    /**No documentation.

**Metadata**:
  - Default: 0.05
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub h_init: HInitParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub pcoeff: PcoeffParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub icoeff: IcoeffParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub dcoeff: DcoeffParam,
    /**No documentation.

**Metadata**:
  - Default: 0.81
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub accept_safety: AcceptSafetyParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
}
impl<
    OrderParam: crate::nodes::types::Int,
    RtolParam: crate::nodes::types::Float,
    AtolParam: crate::nodes::types::Float,
    HInitParam: crate::nodes::types::Float,
    PcoeffParam: crate::nodes::types::Float,
    IcoeffParam: crate::nodes::types::Float,
    DcoeffParam: crate::nodes::types::Float,
    AcceptSafetyParam: crate::nodes::types::Float,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> SamplerDPMAdaptative<
    OrderParam,
    RtolParam,
    AtolParam,
    HInitParam,
    PcoeffParam,
    IcoeffParam,
    DcoeffParam,
    AcceptSafetyParam,
    EtaParam,
    SNoiseParam,
> {
    /// Create a new node.
    pub fn new(
        order: OrderParam,
        rtol: RtolParam,
        atol: AtolParam,
        h_init: HInitParam,
        pcoeff: PcoeffParam,
        icoeff: IcoeffParam,
        dcoeff: DcoeffParam,
        accept_safety: AcceptSafetyParam,
        eta: EtaParam,
        s_noise: SNoiseParam,
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
    OrderParam: crate::nodes::types::Int,
    RtolParam: crate::nodes::types::Float,
    AtolParam: crate::nodes::types::Float,
    HInitParam: crate::nodes::types::Float,
    PcoeffParam: crate::nodes::types::Float,
    IcoeffParam: crate::nodes::types::Float,
    DcoeffParam: crate::nodes::types::Float,
    AcceptSafetyParam: crate::nodes::types::Float,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SamplerDPMAdaptative<
    OrderParam,
    RtolParam,
    AtolParam,
    HInitParam,
    PcoeffParam,
    IcoeffParam,
    DcoeffParam,
    AcceptSafetyParam,
    EtaParam,
    SNoiseParam,
> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_2M_SDE<
    SolverTypeParam: crate::nodes::types::String,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub solver_type: SolverTypeParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
    ///No documentation.
    pub noise_device: NoiseDeviceParam,
}
impl<
    SolverTypeParam: crate::nodes::types::String,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> SamplerDPMPP_2M_SDE<SolverTypeParam, EtaParam, SNoiseParam, NoiseDeviceParam> {
    /// Create a new node.
    pub fn new(
        solver_type: SolverTypeParam,
        eta: EtaParam,
        s_noise: SNoiseParam,
        noise_device: NoiseDeviceParam,
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
    SolverTypeParam: crate::nodes::types::String,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SamplerDPMPP_2M_SDE<SolverTypeParam, EtaParam, SNoiseParam, NoiseDeviceParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_2S_Ancestral<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> SamplerDPMPP_2S_Ancestral<EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerDPMPP_2S_Ancestral<EtaParam, SNoiseParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_3M_SDE<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
    ///No documentation.
    pub noise_device: NoiseDeviceParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> SamplerDPMPP_3M_SDE<EtaParam, SNoiseParam, NoiseDeviceParam> {
    /// Create a new node.
    pub fn new(
        eta: EtaParam,
        s_noise: SNoiseParam,
        noise_device: NoiseDeviceParam,
    ) -> Self {
        Self { eta, s_noise, noise_device }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SamplerDPMPP_3M_SDE<EtaParam, SNoiseParam, NoiseDeviceParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_SDE<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub r: RParam,
    ///No documentation.
    pub noise_device: NoiseDeviceParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> SamplerDPMPP_SDE<EtaParam, SNoiseParam, RParam, NoiseDeviceParam> {
    /// Create a new node.
    pub fn new(
        eta: EtaParam,
        s_noise: SNoiseParam,
        r: RParam,
        noise_device: NoiseDeviceParam,
    ) -> Self {
        Self {
            eta,
            s_noise,
            r,
            noise_device,
        }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
    NoiseDeviceParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SamplerDPMPP_SDE<EtaParam, SNoiseParam, RParam, NoiseDeviceParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerEulerAncestral<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> SamplerEulerAncestral<EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerEulerAncestral<EtaParam, SNoiseParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerEulerAncestralCFGPP<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> SamplerEulerAncestralCFGPP<EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerEulerAncestralCFGPP<EtaParam, SNoiseParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerLCMUpscale<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
    UpscaleMethodParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: 0.1
  - Step: 0.01
*/
    pub scale_ratio: ScaleRatioParam,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 1000
  - Min: -1
  - Step: 1
*/
    pub scale_steps: ScaleStepsParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
}
impl<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
    UpscaleMethodParam: crate::nodes::types::String,
> SamplerLCMUpscale<ScaleRatioParam, ScaleStepsParam, UpscaleMethodParam> {
    /// Create a new node.
    pub fn new(
        scale_ratio: ScaleRatioParam,
        scale_steps: ScaleStepsParam,
        upscale_method: UpscaleMethodParam,
    ) -> Self {
        Self {
            scale_ratio,
            scale_steps,
            upscale_method,
        }
    }
}
impl<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
    UpscaleMethodParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SamplerLCMUpscale<ScaleRatioParam, ScaleStepsParam, UpscaleMethodParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct SamplerLMS<OrderParam: crate::nodes::types::Int> {
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 100
  - Min: 1
*/
    pub order: OrderParam,
}
impl<OrderParam: crate::nodes::types::Int> SamplerLMS<OrderParam> {
    /// Create a new node.
    pub fn new(order: OrderParam) -> Self {
        Self { order }
    }
}
impl<OrderParam: crate::nodes::types::Int> crate::nodes::TypedNode
for SamplerLMS<OrderParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
