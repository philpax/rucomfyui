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
///Output for [`StableCascadeEmptyLatentImage`].
pub struct StableCascadeEmptyLatentImageOutput {
    ///No documentation.
    pub stage_c: crate::nodes::LatentOut,
    ///No documentation.
    pub stage_b: crate::nodes::LatentOut,
}
impl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Compression: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> crate::nodes::TypedNode
for StableCascadeEmptyLatentImage<Width, Height, Compression, BatchSize> {
    type Output = StableCascadeEmptyLatentImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::LatentOut(0usize),
            stage_b: crate::nodes::LatentOut(1usize),
        }
    }
    const NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
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
///Output for [`StableCascadeStageCVaeEncode`].
pub struct StableCascadeStageCVaeEncodeOutput {
    ///No documentation.
    pub stage_c: crate::nodes::LatentOut,
    ///No documentation.
    pub stage_b: crate::nodes::LatentOut,
}
impl<
    Image: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Compression: crate::nodes::Int,
> crate::nodes::TypedNode for StableCascadeStageCVaeEncode<Image, Vae, Compression> {
    type Output = StableCascadeStageCVaeEncodeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::LatentOut(0usize),
            stage_b: crate::nodes::LatentOut(1usize),
        }
    }
    const NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DISPLAY_NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
