//!flux
///**CLIPTextEncodeFlux**
pub struct ClipTextEncodeFlux<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    T5Xxl: crate::nodes::String,
    Guidance: crate::nodes::Float,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub clip_l: ClipL,
    ///No documentation.
    pub t5xxl: T5Xxl,
    ///No documentation.
    pub guidance: Guidance,
}
///**FluxGuidance**
pub struct FluxGuidance<
    Conditioning: crate::nodes::Conditioning,
    Guidance: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub guidance: Guidance,
}
