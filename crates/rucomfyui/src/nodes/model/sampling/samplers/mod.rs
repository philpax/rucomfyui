//!`samplers` definitions/categories.
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
///**KSamplerSelect**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KSamplerSelect {}
impl KSamplerSelect {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for KSamplerSelect {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "KSamplerSelect";
    const DISPLAY_NAME: &'static str = "KSamplerSelect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**Sampler AR Video**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerARVideo<NumFramePerBlockParam: crate::nodes::types::Int> {
    /**Frames per autoregressive block. 1 = framewise, 3 = chunkwise. Must match the checkpoint's training mode.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub num_frame_per_block: NumFramePerBlockParam,
}
impl<
    NumFramePerBlockParam: crate::nodes::types::Int,
> SamplerARVideo<NumFramePerBlockParam> {
    /// Create a new node.
    pub fn new(num_frame_per_block: NumFramePerBlockParam) -> Self {
        Self { num_frame_per_block }
    }
}
impl<NumFramePerBlockParam: crate::nodes::types::Int> crate::nodes::TypedNode
for SamplerARVideo<NumFramePerBlockParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "num_frame_per_block".to_string(),
                self.num_frame_per_block.clone().into(),
            );
        output
    }
    const NAME: &'static str = "SamplerARVideo";
    const DISPLAY_NAME: &'static str = "Sampler AR Video";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
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
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerDPMPP_2M_SDE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_2M_SDE<
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
> SamplerDPMPP_2M_SDE<EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerDPMPP_2M_SDE<EtaParam, SNoiseParam> {
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
    const NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
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
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerDPMPP_3M_SDE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_3M_SDE<
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
> SamplerDPMPP_3M_SDE<EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { eta, s_noise }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerDPMPP_3M_SDE<EtaParam, SNoiseParam> {
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
    const NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerDPMPP_SDE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerDPMPP_SDE<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
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
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
> SamplerDPMPP_SDE<EtaParam, SNoiseParam, RParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam, r: RParam) -> Self {
        Self { eta, s_noise, r }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerDPMPP_SDE<EtaParam, SNoiseParam, RParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("r".to_string(), self.r.clone().into());
        output
    }
    const NAME: &'static str = "SamplerDPMPP_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerER_SDE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerER_SDE<
    MaxStageParam: crate::nodes::types::Int,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 3
  - Min: 1
*/
    pub max_stage: MaxStageParam,
    /**Stochastic strength of reverse-time SDE.

When eta=0, it reduces to deterministic ODE. This setting doesn't apply to ER-SDE solver type.

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
    MaxStageParam: crate::nodes::types::Int,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> SamplerER_SDE<MaxStageParam, EtaParam, SNoiseParam> {
    /// Create a new node.
    pub fn new(max_stage: MaxStageParam, eta: EtaParam, s_noise: SNoiseParam) -> Self {
        Self { max_stage, eta, s_noise }
    }
}
impl<
    MaxStageParam: crate::nodes::types::Int,
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerER_SDE<MaxStageParam, EtaParam, SNoiseParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("max_stage".to_string(), self.max_stage.clone().into());
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
    }
    const NAME: &'static str = "SamplerER_SDE";
    const DISPLAY_NAME: &'static str = "SamplerER_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
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
    const CATEGORY: &'static str = "model/sampling/samplers";
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
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerLCM**: LCM sampler with tunable per-step noise. s_noise is a multiplier on the model's training noise scale
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerLCM<
    SNoiseParam: crate::nodes::types::Float,
    SNoiseEndParam: crate::nodes::types::Float,
    NoiseClipStdParam: crate::nodes::types::Float,
> {
    /**Per-step noise multiplier at the first step (1.0 = match training).

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 0
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
    /**Per-step noise multiplier at the last step. Set equal to s_noise for a constant schedule.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 0
  - Step: 0.01
*/
    pub s_noise_end: SNoiseEndParam,
    /**Clamp per-step noise to +/- N*std. 0 disables.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub noise_clip_std: NoiseClipStdParam,
}
impl<
    SNoiseParam: crate::nodes::types::Float,
    SNoiseEndParam: crate::nodes::types::Float,
    NoiseClipStdParam: crate::nodes::types::Float,
> SamplerLCM<SNoiseParam, SNoiseEndParam, NoiseClipStdParam> {
    /// Create a new node.
    pub fn new(
        s_noise: SNoiseParam,
        s_noise_end: SNoiseEndParam,
        noise_clip_std: NoiseClipStdParam,
    ) -> Self {
        Self {
            s_noise,
            s_noise_end,
            noise_clip_std,
        }
    }
}
impl<
    SNoiseParam: crate::nodes::types::Float,
    SNoiseEndParam: crate::nodes::types::Float,
    NoiseClipStdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SamplerLCM<SNoiseParam, SNoiseEndParam, NoiseClipStdParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("s_noise_end".to_string(), self.s_noise_end.clone().into());
        output.insert("noise_clip_std".to_string(), self.noise_clip_std.clone().into());
        output
    }
    const NAME: &'static str = "SamplerLCM";
    const DISPLAY_NAME: &'static str = "SamplerLCM";
    const DESCRIPTION: &'static str = "LCM sampler with tunable per-step noise. s_noise is a multiplier on the model's training noise scale";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerLCMUpscale**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerLCMUpscale<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
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
}
impl<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
> SamplerLCMUpscale<ScaleRatioParam, ScaleStepsParam> {
    /// Create a new node.
    pub fn new(scale_ratio: ScaleRatioParam, scale_steps: ScaleStepsParam) -> Self {
        Self { scale_ratio, scale_steps }
    }
}
impl<
    ScaleRatioParam: crate::nodes::types::Float,
    ScaleStepsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SamplerLCMUpscale<ScaleRatioParam, ScaleStepsParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("scale_ratio".to_string(), self.scale_ratio.clone().into());
        output.insert("scale_steps".to_string(), self.scale_steps.clone().into());
        output
    }
    const NAME: &'static str = "SamplerLCMUpscale";
    const DISPLAY_NAME: &'static str = "SamplerLCMUpscale";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
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
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**SamplerSASolver**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerSASolver<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    SdeStartPercentParam: crate::nodes::types::Float,
    SdeEndPercentParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    PredictorOrderParam: crate::nodes::types::Int,
    CorrectorOrderParam: crate::nodes::types::Int,
    UsePeceParam: crate::nodes::types::Boolean,
    SimpleOrder2Param: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**No documentation.

**Metadata**:
  - Default: 0.2
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub sde_start_percent: SdeStartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.8
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub sde_end_percent: SdeEndPercentParam,
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
  - Default: 3
  - Max: 6
  - Min: 1
*/
    pub predictor_order: PredictorOrderParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 6
  - Min: 0
*/
    pub corrector_order: CorrectorOrderParam,
    ///No documentation.
    pub use_pece: UsePeceParam,
    ///No documentation.
    pub simple_order_2: SimpleOrder2Param,
}
impl<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    SdeStartPercentParam: crate::nodes::types::Float,
    SdeEndPercentParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    PredictorOrderParam: crate::nodes::types::Int,
    CorrectorOrderParam: crate::nodes::types::Int,
    UsePeceParam: crate::nodes::types::Boolean,
    SimpleOrder2Param: crate::nodes::types::Boolean,
> SamplerSASolver<
    ModelParam,
    EtaParam,
    SdeStartPercentParam,
    SdeEndPercentParam,
    SNoiseParam,
    PredictorOrderParam,
    CorrectorOrderParam,
    UsePeceParam,
    SimpleOrder2Param,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        eta: EtaParam,
        sde_start_percent: SdeStartPercentParam,
        sde_end_percent: SdeEndPercentParam,
        s_noise: SNoiseParam,
        predictor_order: PredictorOrderParam,
        corrector_order: CorrectorOrderParam,
        use_pece: UsePeceParam,
        simple_order_2: SimpleOrder2Param,
    ) -> Self {
        Self {
            model,
            eta,
            sde_start_percent,
            sde_end_percent,
            s_noise,
            predictor_order,
            corrector_order,
            use_pece,
            simple_order_2,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    SdeStartPercentParam: crate::nodes::types::Float,
    SdeEndPercentParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    PredictorOrderParam: crate::nodes::types::Int,
    CorrectorOrderParam: crate::nodes::types::Int,
    UsePeceParam: crate::nodes::types::Boolean,
    SimpleOrder2Param: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for SamplerSASolver<
    ModelParam,
    EtaParam,
    SdeStartPercentParam,
    SdeEndPercentParam,
    SNoiseParam,
    PredictorOrderParam,
    CorrectorOrderParam,
    UsePeceParam,
    SimpleOrder2Param,
> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("eta".to_string(), self.eta.clone().into());
        output
            .insert(
                "sde_start_percent".to_string(),
                self.sde_start_percent.clone().into(),
            );
        output
            .insert("sde_end_percent".to_string(), self.sde_end_percent.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output
            .insert("predictor_order".to_string(), self.predictor_order.clone().into());
        output
            .insert("corrector_order".to_string(), self.corrector_order.clone().into());
        output.insert("use_pece".to_string(), self.use_pece.clone().into());
        output.insert("simple_order_2".to_string(), self.simple_order_2.clone().into());
        output
    }
    const NAME: &'static str = "SamplerSASolver";
    const DISPLAY_NAME: &'static str = "SamplerSASolver";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
#[doc = "**SamplerSEEDS2**: This sampler node can represent multiple samplers:\n\n\n\nseeds_2\n\n- default setting\n\n\n\nexp_heun_2_x0\n\n- solver_type=phi_2, r=1.0, eta=0.0\n\n\n\nexp_heun_2_x0_sde\n\n- solver_type=phi_2, r=1.0, eta=1.0, s_noise=1.0"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerSEEDS2<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
> {
    /**Stochastic strength

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**SDE noise multiplier

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub s_noise: SNoiseParam,
    /**Relative step size for the intermediate stage (c2 node)

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0.01
  - Round: false
  - Step: 0.01
*/
    pub r: RParam,
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
> SamplerSEEDS2<EtaParam, SNoiseParam, RParam> {
    /// Create a new node.
    pub fn new(eta: EtaParam, s_noise: SNoiseParam, r: RParam) -> Self {
        Self { eta, s_noise, r }
    }
}
impl<
    EtaParam: crate::nodes::types::Float,
    SNoiseParam: crate::nodes::types::Float,
    RParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SamplerSEEDS2<EtaParam, SNoiseParam, RParam> {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("s_noise".to_string(), self.s_noise.clone().into());
        output.insert("r".to_string(), self.r.clone().into());
        output
    }
    const NAME: &'static str = "SamplerSEEDS2";
    const DISPLAY_NAME: &'static str = "SamplerSEEDS2";
    const DESCRIPTION: &'static str = "This sampler node can represent multiple samplers:\n\nseeds_2\n- default setting\n\nexp_heun_2_x0\n- solver_type=phi_2, r=1.0, eta=0.0\n\nexp_heun_2_x0_sde\n- solver_type=phi_2, r=1.0, eta=1.0, s_noise=1.0";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
///**VOIDSampler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VOIDSampler {}
impl VOIDSampler {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for VOIDSampler {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "VOIDSampler";
    const DISPLAY_NAME: &'static str = "VOIDSampler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/samplers";
}
