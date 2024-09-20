//!`schedulers` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`AlignYourStepsScheduler`](super::AlignYourStepsScheduler).
    #[derive(Clone)]
    pub struct AlignYourStepsSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`BasicScheduler`](super::BasicScheduler).
    #[derive(Clone)]
    pub struct BasicSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`BetaSamplingScheduler`](super::BetaSamplingScheduler).
    #[derive(Clone)]
    pub struct BetaSamplingSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`ExponentialScheduler`](super::ExponentialScheduler).
    #[derive(Clone)]
    pub struct ExponentialSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`GitsScheduler`](super::GitsScheduler).
    #[derive(Clone)]
    pub struct GitsSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`KarrasScheduler`](super::KarrasScheduler).
    #[derive(Clone)]
    pub struct KarrasSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`PolyexponentialScheduler`](super::PolyexponentialScheduler).
    #[derive(Clone)]
    pub struct PolyexponentialSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`SdTurboScheduler`](super::SdTurboScheduler).
    #[derive(Clone)]
    pub struct SdTurboSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`VpScheduler`](super::VpScheduler).
    #[derive(Clone)]
    pub struct VpSchedulerOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
}
///**AlignYourStepsScheduler**
pub struct AlignYourStepsScheduler<
    ModelType: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_type: ModelType,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
impl<
    ModelType: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for AlignYourStepsScheduler<ModelType, Steps, Denoise> {
    type Output = out::AlignYourStepsSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "AlignYourStepsScheduler";
    const DISPLAY_NAME: &'static str = "AlignYourStepsScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**BasicScheduler**
pub struct BasicScheduler<
    Model: crate::nodes::types::Model,
    Scheduler: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub scheduler: Scheduler,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
impl<
    Model: crate::nodes::types::Model,
    Scheduler: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for BasicScheduler<Model, Scheduler, Steps, Denoise> {
    type Output = out::BasicSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "BasicScheduler";
    const DISPLAY_NAME: &'static str = "BasicScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**BetaSamplingScheduler**
pub struct BetaSamplingScheduler<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Alpha: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub alpha: Alpha,
    ///No documentation.
    pub beta: Beta,
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Alpha: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> crate::nodes::TypedNode for BetaSamplingScheduler<Model, Steps, Alpha, Beta> {
    type Output = out::BetaSamplingSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "BetaSamplingScheduler";
    const DISPLAY_NAME: &'static str = "BetaSamplingScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**ExponentialScheduler**
pub struct ExponentialScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode for ExponentialScheduler<Steps, SigmaMax, SigmaMin> {
    type Output = out::ExponentialSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ExponentialScheduler";
    const DISPLAY_NAME: &'static str = "ExponentialScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**GITSScheduler**
pub struct GitsScheduler<
    Coeff: crate::nodes::types::Float,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub coeff: Coeff,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
impl<
    Coeff: crate::nodes::types::Float,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for GitsScheduler<Coeff, Steps, Denoise> {
    type Output = out::GitsSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "GITSScheduler";
    const DISPLAY_NAME: &'static str = "GITSScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**KarrasScheduler**
pub struct KarrasScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
    ///No documentation.
    pub rho: Rho,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> crate::nodes::TypedNode for KarrasScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    type Output = out::KarrasSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "KarrasScheduler";
    const DISPLAY_NAME: &'static str = "KarrasScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**PolyexponentialScheduler**
pub struct PolyexponentialScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
    ///No documentation.
    pub rho: Rho,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> crate::nodes::TypedNode for PolyexponentialScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    type Output = out::PolyexponentialSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "PolyexponentialScheduler";
    const DISPLAY_NAME: &'static str = "PolyexponentialScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**SDTurboScheduler**
pub struct SdTurboScheduler<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SdTurboScheduler<Model, Steps, Denoise> {
    type Output = out::SdTurboSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SDTurboScheduler";
    const DISPLAY_NAME: &'static str = "SDTurboScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**VPScheduler**
pub struct VpScheduler<
    Steps: crate::nodes::types::Int,
    BetaD: crate::nodes::types::Float,
    BetaMin: crate::nodes::types::Float,
    EpsS: crate::nodes::types::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub beta_d: BetaD,
    ///No documentation.
    pub beta_min: BetaMin,
    ///No documentation.
    pub eps_s: EpsS,
}
impl<
    Steps: crate::nodes::types::Int,
    BetaD: crate::nodes::types::Float,
    BetaMin: crate::nodes::types::Float,
    EpsS: crate::nodes::types::Float,
> crate::nodes::TypedNode for VpScheduler<Steps, BetaD, BetaMin, EpsS> {
    type Output = out::VpSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VPScheduler";
    const DISPLAY_NAME: &'static str = "VPScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
