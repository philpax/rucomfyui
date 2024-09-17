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
///Output for [`InpaintModelConditioning`].
#[derive(Clone)]
pub struct InpaintModelConditioningOutput {
    ///No documentation.
    pub positive: crate::nodes::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::ConditioningOut,
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Vae: crate::nodes::Vae,
    Pixels: crate::nodes::Image,
    Mask: crate::nodes::Mask,
> crate::nodes::TypedNode
for InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask> {
    type Output = InpaintModelConditioningOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut(0u32),
            negative: crate::nodes::ConditioningOut(1u32),
            latent: crate::nodes::LatentOut(2u32),
        }
    }
    const NAME: &'static str = "InpaintModelConditioning";
    const DISPLAY_NAME: &'static str = "InpaintModelConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/inpaint";
}
