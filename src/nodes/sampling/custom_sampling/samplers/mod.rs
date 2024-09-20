//!`samplers` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`KSamplerSelect`](super::KSamplerSelect).
    #[derive(Clone)]
    pub struct KSamplerSelectOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerDpmAdaptative`](super::SamplerDpmAdaptative).
    #[derive(Clone)]
    pub struct SamplerDpmAdaptativeOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerDpmpp2MSde`](super::SamplerDpmpp2MSde).
    #[derive(Clone)]
    pub struct SamplerDpmpp2MSdeOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerDpmpp2SAncestral`](super::SamplerDpmpp2SAncestral).
    #[derive(Clone)]
    pub struct SamplerDpmpp2SAncestralOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerDpmpp3MSde`](super::SamplerDpmpp3MSde).
    #[derive(Clone)]
    pub struct SamplerDpmpp3MSdeOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerDpmppSde`](super::SamplerDpmppSde).
    #[derive(Clone)]
    pub struct SamplerDpmppSdeOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerEulerAncestral`](super::SamplerEulerAncestral).
    #[derive(Clone)]
    pub struct SamplerEulerAncestralOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerEulerAncestralCfgpp`](super::SamplerEulerAncestralCfgpp).
    #[derive(Clone)]
    pub struct SamplerEulerAncestralCfgppOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerLcmUpscale`](super::SamplerLcmUpscale).
    #[derive(Clone)]
    pub struct SamplerLcmUpscaleOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
    ///Output for [`SamplerLms`](super::SamplerLms).
    #[derive(Clone)]
    pub struct SamplerLmsOutput {
        ///No documentation.
        pub sampler: crate::nodes::types::SamplerOut,
    }
}
///**KSamplerSelect**: No description.
pub struct KSamplerSelect<SamplerName: crate::nodes::types::String> {
    ///No documentation.
    pub sampler_name: SamplerName,
}
impl<SamplerName: crate::nodes::types::String> crate::nodes::TypedNode
for KSamplerSelect<SamplerName> {
    type Output = out::KSamplerSelectOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "KSamplerSelect";
    const DISPLAY_NAME: &'static str = "KSamplerSelect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMAdaptative**: No description.
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
    type Output = out::SamplerDpmAdaptativeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerDPMAdaptative";
    const DISPLAY_NAME: &'static str = "SamplerDPMAdaptative";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_2M_SDE**: No description.
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
> crate::nodes::TypedNode for SamplerDpmpp2MSde<SolverType, Eta, SNoise, NoiseDevice> {
    type Output = out::SamplerDpmpp2MSdeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_2S_Ancestral**: No description.
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
> crate::nodes::TypedNode for SamplerDpmpp2SAncestral<Eta, SNoise> {
    type Output = out::SamplerDpmpp2SAncestralOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_3M_SDE**: No description.
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
> crate::nodes::TypedNode for SamplerDpmpp3MSde<Eta, SNoise, NoiseDevice> {
    type Output = out::SamplerDpmpp3MSdeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_SDE**: No description.
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
> crate::nodes::TypedNode for SamplerDpmppSde<Eta, SNoise, R, NoiseDevice> {
    type Output = out::SamplerDpmppSdeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerDPMPP_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerEulerAncestral**: No description.
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
> crate::nodes::TypedNode for SamplerEulerAncestral<Eta, SNoise> {
    type Output = out::SamplerEulerAncestralOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerEulerAncestral";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerEulerAncestralCFG++**: No description.
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
> crate::nodes::TypedNode for SamplerEulerAncestralCfgpp<Eta, SNoise> {
    type Output = out::SamplerEulerAncestralCfgppOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerEulerAncestralCFGPP";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestralCFG++";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerLCMUpscale**: No description.
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
> crate::nodes::TypedNode for SamplerLcmUpscale<ScaleRatio, ScaleSteps, UpscaleMethod> {
    type Output = out::SamplerLcmUpscaleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerLCMUpscale";
    const DISPLAY_NAME: &'static str = "SamplerLCMUpscale";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerLMS**: No description.
pub struct SamplerLms<Order: crate::nodes::types::Int> {
    ///No documentation.
    pub order: Order,
}
impl<Order: crate::nodes::types::Int> crate::nodes::TypedNode for SamplerLms<Order> {
    type Output = out::SamplerLmsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::types::SamplerOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SamplerLMS";
    const DISPLAY_NAME: &'static str = "SamplerLMS";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
