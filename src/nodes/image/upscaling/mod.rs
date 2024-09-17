//!upscaling
///**Upscale Image**
pub struct ImageScale<
    Image: crate::nodes::Image,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///**Upscale Image By**
pub struct ImageScaleBy<Image: crate::nodes::Image, ScaleBy: crate::nodes::Float> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub scale_by: ScaleBy,
}
///**ImageScaleToTotalPixels**
pub struct ImageScaleToTotalPixels<
    Image: crate::nodes::Image,
    Megapixels: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub megapixels: Megapixels,
}
///**Upscale Image (using Model)**
pub struct ImageUpscaleWithModel<
    UpscaleModel: crate::nodes::UpscaleModel,
    Image: crate::nodes::Image,
> {
    ///No documentation.
    pub upscale_model: UpscaleModel,
    ///No documentation.
    pub image: Image,
}
