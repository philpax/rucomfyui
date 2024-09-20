//!`upscaling` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageScale`](super::ImageScale).
    #[derive(Clone)]
    pub struct ImageScaleOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageScaleBy`](super::ImageScaleBy).
    #[derive(Clone)]
    pub struct ImageScaleByOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageScaleToTotalPixels`](super::ImageScaleToTotalPixels).
    #[derive(Clone)]
    pub struct ImageScaleToTotalPixelsOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageUpscaleWithModel`](super::ImageUpscaleWithModel).
    #[derive(Clone)]
    pub struct ImageUpscaleWithModelOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
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
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageScale<Image, UpscaleMethod, Width, Height, Crop> {
    type Output = out::ImageScaleOutput;
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
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleBy<Image, UpscaleMethod, ScaleBy> {
    type Output = out::ImageScaleByOutput;
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
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Megapixels: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleToTotalPixels<Image, UpscaleMethod, Megapixels> {
    type Output = out::ImageScaleToTotalPixelsOutput;
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
impl<
    UpscaleModel: crate::nodes::types::UpscaleModel,
    Image: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageUpscaleWithModel<UpscaleModel, Image> {
    type Output = out::ImageUpscaleWithModelOutput;
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
