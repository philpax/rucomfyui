//!upscale_diffusion
///**SD_4XUpscale_Conditioning**
pub struct Sd4XUpscaleConditioning<
    Images: crate::nodes::Image,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ScaleRatio: crate::nodes::Float,
    NoiseAugmentation: crate::nodes::Float,
> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub scale_ratio: ScaleRatio,
    ///No documentation.
    pub noise_augmentation: NoiseAugmentation,
}
