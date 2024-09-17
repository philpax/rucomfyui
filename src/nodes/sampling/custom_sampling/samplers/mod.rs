//!samplers
///**KSamplerSelect**
pub struct KSamplerSelect {}
///Output for [`KSamplerSelect`].
pub struct KSamplerSelectOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl crate::nodes::TypedNode for KSamplerSelect {
    type Output = KSamplerSelectOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "KSamplerSelect";
    const DISPLAY_NAME: &'static str = "KSamplerSelect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
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
///Output for [`SamplerDpmAdaptative`].
pub struct SamplerDpmAdaptativeOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<
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
    type Output = SamplerDpmAdaptativeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerDPMAdaptative";
    const DISPLAY_NAME: &'static str = "SamplerDPMAdaptative";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_2M_SDE**
pub struct SamplerDpmpp2MSde<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///Output for [`SamplerDpmpp2MSde`].
pub struct SamplerDpmpp2MSdeOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> crate::nodes::TypedNode
for SamplerDpmpp2MSde<Eta, SNoise> {
    type Output = SamplerDpmpp2MSdeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
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
///Output for [`SamplerDpmpp2SAncestral`].
pub struct SamplerDpmpp2SAncestralOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> crate::nodes::TypedNode
for SamplerDpmpp2SAncestral<Eta, SNoise> {
    type Output = SamplerDpmpp2SAncestralOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_2S_Ancestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerDPMPP_3M_SDE**
pub struct SamplerDpmpp3MSde<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///Output for [`SamplerDpmpp3MSde`].
pub struct SamplerDpmpp3MSdeOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> crate::nodes::TypedNode
for SamplerDpmpp3MSde<Eta, SNoise> {
    type Output = SamplerDpmpp3MSdeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_3M_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
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
///Output for [`SamplerDpmppSde`].
pub struct SamplerDpmppSdeOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<
    Eta: crate::nodes::Float,
    SNoise: crate::nodes::Float,
    R: crate::nodes::Float,
> crate::nodes::TypedNode for SamplerDpmppSde<Eta, SNoise, R> {
    type Output = SamplerDpmppSdeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerDPMPP_SDE";
    const DISPLAY_NAME: &'static str = "SamplerDPMPP_SDE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerEulerAncestral**
pub struct SamplerEulerAncestral<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> {
    ///No documentation.
    pub eta: Eta,
    ///No documentation.
    pub s_noise: SNoise,
}
///Output for [`SamplerEulerAncestral`].
pub struct SamplerEulerAncestralOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> crate::nodes::TypedNode
for SamplerEulerAncestral<Eta, SNoise> {
    type Output = SamplerEulerAncestralOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerEulerAncestral";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestral";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
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
///Output for [`SamplerEulerAncestralCfgpp`].
pub struct SamplerEulerAncestralCfgppOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Eta: crate::nodes::Float, SNoise: crate::nodes::Float> crate::nodes::TypedNode
for SamplerEulerAncestralCfgpp<Eta, SNoise> {
    type Output = SamplerEulerAncestralCfgppOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerEulerAncestralCFGPP";
    const DISPLAY_NAME: &'static str = "SamplerEulerAncestralCFG++";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
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
///Output for [`SamplerLcmUpscale`].
pub struct SamplerLcmUpscaleOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<
    ScaleRatio: crate::nodes::Float,
    ScaleSteps: crate::nodes::Int,
> crate::nodes::TypedNode for SamplerLcmUpscale<ScaleRatio, ScaleSteps> {
    type Output = SamplerLcmUpscaleOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerLCMUpscale";
    const DISPLAY_NAME: &'static str = "SamplerLCMUpscale";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
///**SamplerLMS**
pub struct SamplerLms<Order: crate::nodes::Int> {
    ///No documentation.
    pub order: Order,
}
///Output for [`SamplerLms`].
pub struct SamplerLmsOutput {
    ///No documentation.
    pub sampler: crate::nodes::SamplerOut,
}
impl<Order: crate::nodes::Int> crate::nodes::TypedNode for SamplerLms<Order> {
    type Output = SamplerLmsOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sampler: crate::nodes::SamplerOut(0u32),
        }
    }
    const NAME: &'static str = "SamplerLMS";
    const DISPLAY_NAME: &'static str = "SamplerLMS";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/samplers";
}
