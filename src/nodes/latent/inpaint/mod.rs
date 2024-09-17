//!inpaint
///**Set Latent Noise Mask**
pub struct SetLatentNoiseMask<Samples: crate::nodes::Latent, Mask: crate::nodes::Mask> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub mask: Mask,
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
