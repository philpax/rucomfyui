//!inpaint
///**InpaintModelConditioning**
pub struct InpaintModelConditioning<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Vae: crate::nodes::Vae,
    Pixels: crate::nodes::Image,
    Mask: crate::nodes::Mask,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub mask: Mask,
}
