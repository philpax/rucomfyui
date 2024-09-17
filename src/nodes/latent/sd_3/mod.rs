//!sd3
///**EmptySD3LatentImage**
pub struct EmptySd3LatentImage<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub batch_size: BatchSize,
}
///Output for [`EmptySd3LatentImage`].
pub struct EmptySd3LatentImageOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> crate::nodes::TypedNode for EmptySd3LatentImage<Width, Height, BatchSize> {
    type Output = EmptySd3LatentImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "EmptySD3LatentImage";
    const DISPLAY_NAME: &'static str = "EmptySD3LatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/sd3";
}