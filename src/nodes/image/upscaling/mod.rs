//!`upscaling` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**Upscale Image**
pub struct ImageScale<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
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
#[derive(Clone)]
pub struct ImageScaleOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageScale<Image, UpscaleMethod, Width, Height, Crop> {
    type Output = ImageScaleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageScale";
    const DISPLAY_NAME: &'static str = "Upscale Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image By**
pub struct ImageScaleBy<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub scale_by: ScaleBy,
}
///Output for [`ImageScaleBy`].
#[derive(Clone)]
pub struct ImageScaleByOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleBy<Image, UpscaleMethod, ScaleBy> {
    type Output = ImageScaleByOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageScaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Image By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**ImageScaleToTotalPixels**
pub struct ImageScaleToTotalPixels<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Megapixels: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub megapixels: Megapixels,
}
///Output for [`ImageScaleToTotalPixels`].
#[derive(Clone)]
pub struct ImageScaleToTotalPixelsOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Megapixels: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleToTotalPixels<Image, UpscaleMethod, Megapixels> {
    type Output = ImageScaleToTotalPixelsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageScaleToTotalPixels";
    const DISPLAY_NAME: &'static str = "ImageScaleToTotalPixels";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image (using Model)**
pub struct ImageUpscaleWithModel<
    UpscaleModel: crate::nodes::types::UpscaleModel,
    Image: crate::nodes::types::Image,
> {
    ///No documentation.
    pub upscale_model: UpscaleModel,
    ///No documentation.
    pub image: Image,
}
///Output for [`ImageUpscaleWithModel`].
#[derive(Clone)]
pub struct ImageUpscaleWithModelOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    UpscaleModel: crate::nodes::types::UpscaleModel,
    Image: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageUpscaleWithModel<UpscaleModel, Image> {
    type Output = ImageUpscaleWithModelOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageUpscaleWithModel";
    const DISPLAY_NAME: &'static str = "Upscale Image (using Model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
