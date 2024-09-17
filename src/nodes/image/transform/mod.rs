//!transform
///**ImageCrop**
pub struct ImageCrop<
    Image: crate::nodes::Image,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
///Output for [`ImageCrop`].
pub struct ImageCropOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> crate::nodes::TypedNode for ImageCrop<Image, Width, Height, X, Y> {
    type Output = ImageCropOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageCrop";
    const DISPLAY_NAME: &'static str = "ImageCrop";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
