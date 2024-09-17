//!sigmas
///**FlipSigmas**
pub struct FlipSigmas<Sigmas: crate::nodes::Sigmas> {
    ///No documentation.
    pub sigmas: Sigmas,
}
///Output for [`FlipSigmas`].
#[derive(Clone)]
pub struct FlipSigmasOutput {
    ///No documentation.
    pub sigmas: crate::nodes::SigmasOut,
}
impl<Sigmas: crate::nodes::Sigmas> crate::nodes::TypedNode for FlipSigmas<Sigmas> {
    type Output = FlipSigmasOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::SigmasOut(0u32),
        }
    }
    const NAME: &'static str = "FlipSigmas";
    const DISPLAY_NAME: &'static str = "FlipSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmas**
pub struct SplitSigmas<Sigmas: crate::nodes::Sigmas, Step: crate::nodes::Int> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub step: Step,
}
///Output for [`SplitSigmas`].
#[derive(Clone)]
pub struct SplitSigmasOutput {
    ///No documentation.
    pub high_sigmas: crate::nodes::SigmasOut,
    ///No documentation.
    pub low_sigmas: crate::nodes::SigmasOut,
}
impl<Sigmas: crate::nodes::Sigmas, Step: crate::nodes::Int> crate::nodes::TypedNode
for SplitSigmas<Sigmas, Step> {
    type Output = SplitSigmasOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::SigmasOut(0u32),
            low_sigmas: crate::nodes::SigmasOut(1u32),
        }
    }
    const NAME: &'static str = "SplitSigmas";
    const DISPLAY_NAME: &'static str = "SplitSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmasDenoise**
pub struct SplitSigmasDenoise<
    Sigmas: crate::nodes::Sigmas,
    Denoise: crate::nodes::Float,
> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`SplitSigmasDenoise`].
#[derive(Clone)]
pub struct SplitSigmasDenoiseOutput {
    ///No documentation.
    pub high_sigmas: crate::nodes::SigmasOut,
    ///No documentation.
    pub low_sigmas: crate::nodes::SigmasOut,
}
impl<Sigmas: crate::nodes::Sigmas, Denoise: crate::nodes::Float> crate::nodes::TypedNode
for SplitSigmasDenoise<Sigmas, Denoise> {
    type Output = SplitSigmasDenoiseOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::SigmasOut(0u32),
            low_sigmas: crate::nodes::SigmasOut(1u32),
        }
    }
    const NAME: &'static str = "SplitSigmasDenoise";
    const DISPLAY_NAME: &'static str = "SplitSigmasDenoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
