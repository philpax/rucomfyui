//!sigmas
///**FlipSigmas**
pub struct FlipSigmas<Sigmas: crate::nodes::Sigmas> {
    ///No documentation.
    pub sigmas: Sigmas,
}
///**SplitSigmas**
pub struct SplitSigmas<Sigmas: crate::nodes::Sigmas, Step: crate::nodes::Int> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub step: Step,
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
