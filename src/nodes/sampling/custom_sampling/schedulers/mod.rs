//!`schedulers` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**AlignYourStepsScheduler**
pub struct AlignYourStepsScheduler<
    ModelType: crate::nodes::String,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> {
    ///No documentation.
    pub model_type: ModelType,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`AlignYourStepsScheduler`].
#[derive(Clone)]
pub struct AlignYourStepsSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    ModelType: crate::nodes::String,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode for AlignYourStepsScheduler<ModelType, Steps, Denoise> {
    type Output = AlignYourStepsSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Model: crate::nodes::Model,
    Scheduler: crate::nodes::String,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
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
///Output for [`BasicScheduler`].
#[derive(Clone)]
pub struct BasicSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Model: crate::nodes::Model,
    Scheduler: crate::nodes::String,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode for BasicScheduler<Model, Scheduler, Steps, Denoise> {
    type Output = BasicSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Model: crate::nodes::Model,
    Steps: crate::nodes::Int,
    Alpha: crate::nodes::Float,
    Beta: crate::nodes::Float,
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
///Output for [`BetaSamplingScheduler`].
#[derive(Clone)]
pub struct BetaSamplingSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Model: crate::nodes::Model,
    Steps: crate::nodes::Int,
    Alpha: crate::nodes::Float,
    Beta: crate::nodes::Float,
> crate::nodes::TypedNode for BetaSamplingScheduler<Model, Steps, Alpha, Beta> {
    type Output = BetaSamplingSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
///Output for [`ExponentialScheduler`].
#[derive(Clone)]
pub struct ExponentialSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> crate::nodes::TypedNode for ExponentialScheduler<Steps, SigmaMax, SigmaMin> {
    type Output = ExponentialSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Coeff: crate::nodes::Float,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> {
    ///No documentation.
    pub coeff: Coeff,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`GitsScheduler`].
#[derive(Clone)]
pub struct GitsSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Coeff: crate::nodes::Float,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode for GitsScheduler<Coeff, Steps, Denoise> {
    type Output = GitsSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
    Rho: crate::nodes::Float,
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
///Output for [`KarrasScheduler`].
#[derive(Clone)]
pub struct KarrasSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
    Rho: crate::nodes::Float,
> crate::nodes::TypedNode for KarrasScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    type Output = KarrasSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
    Rho: crate::nodes::Float,
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
///Output for [`PolyexponentialScheduler`].
#[derive(Clone)]
pub struct PolyexponentialSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Steps: crate::nodes::Int,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
    Rho: crate::nodes::Float,
> crate::nodes::TypedNode for PolyexponentialScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    type Output = PolyexponentialSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Model: crate::nodes::Model,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`SdTurboScheduler`].
#[derive(Clone)]
pub struct SdTurboSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Model: crate::nodes::Model,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode for SdTurboScheduler<Model, Steps, Denoise> {
    type Output = SdTurboSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
    Steps: crate::nodes::Int,
    BetaD: crate::nodes::Float,
    BetaMin: crate::nodes::Float,
    EpsS: crate::nodes::Float,
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
///Output for [`VpScheduler`].
#[derive(Clone)]
pub struct VpSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Steps: crate::nodes::Int,
    BetaD: crate::nodes::Float,
    BetaMin: crate::nodes::Float,
    EpsS: crate::nodes::Float,
> crate::nodes::TypedNode for VpScheduler<Steps, BetaD, BetaMin, EpsS> {
    type Output = VpSchedulerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut {
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
