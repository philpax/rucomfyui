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
///Output for [`InstructPixToPixConditioning`].
pub struct InstructPixToPixConditioningOutput {
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
> crate::nodes::TypedNode
for InstructPixToPixConditioning<Positive, Negative, Vae, Pixels> {
    type Output = InstructPixToPixConditioningOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut(0usize),
            negative: crate::nodes::ConditioningOut(1usize),
            latent: crate::nodes::LatentOut(2usize),
        }
    }
    const NAME: &'static str = "InstructPixToPixConditioning";
    const DISPLAY_NAME: &'static str = "InstructPixToPixConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/instructpix2pix";
}
