//!stable_cascade
///**StableCascade_EmptyLatentImage**
pub struct StableCascadeEmptyLatentImage<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Compression: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub compression: Compression,
    ///No documentation.
    pub batch_size: BatchSize,
}
///**StableCascade_StageC_VAEEncode**
pub struct StableCascadeStageCVaeEncode<
    Image: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Compression: crate::nodes::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub compression: Compression,
}
