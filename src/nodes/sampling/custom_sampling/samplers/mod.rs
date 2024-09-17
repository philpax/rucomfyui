//!samplers
///**KSamplerSelect**
pub struct KSamplerSelect {}
///**SamplerDPMAdaptative**
pub struct SamplerDpmAdaptative<
    Order: crate::nodes::Int,
    Rtol: crate::nodes::Float,
    Atol: crate::nodes::Float,
    HInit: crate::nodes::Float,
    Pcoeff: crate::nodes::Float,
    Icoeff: crate::nodes::Float,
    Dcoeff: crate::nodes::Float,
    AcceptSafety: crate::nodes::Float,
    Eta: crate::nodes::Float,
    SNoise: crate::nodes::Float,
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
///**SamplerDPMPP_2M_SDE**
pub struct SamplerDpmpp2MSde<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///**SamplerDPMPP_2S_Ancestral**
pub struct SamplerDpmpp2SAncestral<
    Eta: crate::nodes::Float,
    SNoise: crate::nodes::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///**SamplerDPMPP_3M_SDE**
pub struct SamplerDpmpp3MSde<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///**SamplerDPMPP_SDE**
pub struct SamplerDpmppSde<
    Eta: crate::nodes::Float,
    SNoise: crate::nodes::Float,
    R: crate::nodes::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
    ///No documentation.
    pub r: R,
}
///**SamplerEulerAncestral**
pub struct SamplerEulerAncestral<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///**SamplerEulerAncestralCFG++**
pub struct SamplerEulerAncestralCfgpp<
    Eta: crate::nodes::Float,
    SNoise: crate::nodes::Float,
> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///**SamplerLCMUpscale**
pub struct SamplerLcmUpscale<
    ScaleRatio: crate::nodes::Float,
    ScaleSteps: crate::nodes::Int,
> {
    ///No documentation.
    pub scale_ratio: ScaleRatio,
    ///No documentation.
    pub scale_steps: ScaleSteps,
}
///**SamplerLMS**
pub struct SamplerLms<Order: crate::nodes::Int> {
    ///No documentation.
    pub order: Order,
}
