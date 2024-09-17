//!schedulers
///**AlignYourStepsScheduler**
pub struct AlignYourStepsScheduler<
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> {
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`AlignYourStepsScheduler`].
pub struct AlignYourStepsSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<Steps: crate::nodes::Int, Denoise: crate::nodes::Float> crate::nodes::TypedNode
for AlignYourStepsScheduler<Steps, Denoise> {
    type Output = AlignYourStepsSchedulerOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
///Output for [`BasicScheduler`].
pub struct BasicSchedulerOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<
    Model: crate::nodes::Model,
    Steps: crate::nodes::Int,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode for BasicScheduler<Model, Steps, Denoise> {
    type Output = BasicSchedulerOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
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
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0usize),
        }
    }
    const NAME: &'static str = "VPScheduler";
    const DISPLAY_NAME: &'static str = "VPScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
