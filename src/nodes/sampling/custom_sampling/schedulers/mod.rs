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
