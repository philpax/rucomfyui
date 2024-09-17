//!inpaint
///**Set Latent Noise Mask**
pub struct SetLatentNoiseMask<Samples: crate::nodes::Latent, Mask: crate::nodes::Mask> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub mask: Mask,
}
///Output for [`SetLatentNoiseMask`].
pub struct SetLatentNoiseMaskOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<Samples: crate::nodes::Latent, Mask: crate::nodes::Mask> crate::nodes::TypedNode
for SetLatentNoiseMask<Samples, Mask> {
    type Output = SetLatentNoiseMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0usize),
        }
    }
    const NAME: &'static str = "SetLatentNoiseMask";
    const DISPLAY_NAME: &'static str = "Set Latent Noise Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
///**VAE Encode (for Inpainting)**
pub struct VaeEncodeForInpaint<
    Pixels: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Mask: crate::nodes::Mask,
    GrowMaskBy: crate::nodes::Int,
> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub grow_mask_by: GrowMaskBy,
}
///Output for [`VaeEncodeForInpaint`].
pub struct VaeEncodeForInpaintOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Pixels: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Mask: crate::nodes::Mask,
    GrowMaskBy: crate::nodes::Int,
> crate::nodes::TypedNode for VaeEncodeForInpaint<Pixels, Vae, Mask, GrowMaskBy> {
    type Output = VaeEncodeForInpaintOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0usize),
        }
    }
    const NAME: &'static str = "VAEEncodeForInpaint";
    const DISPLAY_NAME: &'static str = "VAE Encode (for Inpainting)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/inpaint";
}
