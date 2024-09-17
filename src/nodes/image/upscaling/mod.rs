//!upscaling
///**Upscale Image**
pub struct ImageScale<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Crop: crate::nodes::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub crop: Crop,
}
///Output for [`ImageScale`].
pub struct ImageScaleOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Crop: crate::nodes::String,
> crate::nodes::TypedNode for ImageScale<Image, UpscaleMethod, Width, Height, Crop> {
    type Output = ImageScaleOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageScale";
    const DISPLAY_NAME: &'static str = "Upscale Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image By**
pub struct ImageScaleBy<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    ScaleBy: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub scale_by: ScaleBy,
}
///Output for [`ImageScaleBy`].
pub struct ImageScaleByOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    ScaleBy: crate::nodes::Float,
> crate::nodes::TypedNode for ImageScaleBy<Image, UpscaleMethod, ScaleBy> {
    type Output = ImageScaleByOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageScaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Image By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**ImageScaleToTotalPixels**
pub struct ImageScaleToTotalPixels<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    Megapixels: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub megapixels: Megapixels,
}
///Output for [`ImageScaleToTotalPixels`].
pub struct ImageScaleToTotalPixelsOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    UpscaleMethod: crate::nodes::String,
    Megapixels: crate::nodes::Float,
> crate::nodes::TypedNode for ImageScaleToTotalPixels<Image, UpscaleMethod, Megapixels> {
    type Output = ImageScaleToTotalPixelsOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageScaleToTotalPixels";
    const DISPLAY_NAME: &'static str = "ImageScaleToTotalPixels";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
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
///Output for [`ImageUpscaleWithModel`].
pub struct ImageUpscaleWithModelOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    UpscaleModel: crate::nodes::UpscaleModel,
    Image: crate::nodes::Image,
> crate::nodes::TypedNode for ImageUpscaleWithModel<UpscaleModel, Image> {
    type Output = ImageUpscaleWithModelOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageUpscaleWithModel";
    const DISPLAY_NAME: &'static str = "Upscale Image (using Model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
