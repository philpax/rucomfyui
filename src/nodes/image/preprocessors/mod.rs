//!preprocessors
///**Canny**
pub struct Canny<
    Image: crate::nodes::Image,
    LowThreshold: crate::nodes::Float,
    HighThreshold: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub low_threshold: LowThreshold,
    ///No documentation.
    pub high_threshold: HighThreshold,
}
///Output for [`Canny`].
pub struct CannyOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    LowThreshold: crate::nodes::Float,
    HighThreshold: crate::nodes::Float,
> crate::nodes::TypedNode for Canny<Image, LowThreshold, HighThreshold> {
    type Output = CannyOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "Canny";
    const DISPLAY_NAME: &'static str = "Canny";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/preprocessors";
}
