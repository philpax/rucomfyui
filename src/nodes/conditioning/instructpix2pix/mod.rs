//!instructpix2pix
///**InstructPixToPixConditioning**
pub struct InstructPixToPixConditioning<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Vae: crate::nodes::Vae,
    Pixels: crate::nodes::Image,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub pixels: Pixels,
}
