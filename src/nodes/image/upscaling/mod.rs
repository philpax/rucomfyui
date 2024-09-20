//!`upscaling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**Upscale Image**: No description.
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
> ImageScale<Image, UpscaleMethod, Width, Height, Crop> {
    /// Create a new node.
    pub fn new(
        image: Image,
        upscale_method: UpscaleMethod,
        width: Width,
        height: Height,
        crop: Crop,
    ) -> Self {
        Self {
            image,
            upscale_method,
            width,
            height,
            crop,
        }
    }
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageScale<Image, UpscaleMethod, Width, Height, Crop> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
            .insert(
                "upscale_method".to_string(),
                self.upscale_method.to_workflow_input(),
            );
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("crop".to_string(), self.crop.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageScale";
    const DISPLAY_NAME: &'static str = "Upscale Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image By**: No description.
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
> ImageScaleBy<Image, UpscaleMethod, ScaleBy> {
    /// Create a new node.
    pub fn new(image: Image, upscale_method: UpscaleMethod, scale_by: ScaleBy) -> Self {
        Self {
            image,
            upscale_method,
            scale_by,
        }
    }
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleBy<Image, UpscaleMethod, ScaleBy> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
            .insert(
                "upscale_method".to_string(),
                self.upscale_method.to_workflow_input(),
            );
        output.insert("scale_by".to_string(), self.scale_by.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageScaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Image By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**ImageScaleToTotalPixels**: No description.
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
> ImageScaleToTotalPixels<Image, UpscaleMethod, Megapixels> {
    /// Create a new node.
    pub fn new(
        image: Image,
        upscale_method: UpscaleMethod,
        megapixels: Megapixels,
    ) -> Self {
        Self {
            image,
            upscale_method,
            megapixels,
        }
    }
}
impl<
    Image: crate::nodes::types::Image,
    UpscaleMethod: crate::nodes::types::String,
    Megapixels: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageScaleToTotalPixels<Image, UpscaleMethod, Megapixels> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
            .insert(
                "upscale_method".to_string(),
                self.upscale_method.to_workflow_input(),
            );
        output.insert("megapixels".to_string(), self.megapixels.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageScaleToTotalPixels";
    const DISPLAY_NAME: &'static str = "ImageScaleToTotalPixels";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image (using Model)**: No description.
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
> ImageUpscaleWithModel<UpscaleModel, Image> {
    /// Create a new node.
    pub fn new(upscale_model: UpscaleModel, image: Image) -> Self {
        Self { upscale_model, image }
    }
}
impl<
    UpscaleModel: crate::nodes::types::UpscaleModel,
    Image: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageUpscaleWithModel<UpscaleModel, Image> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("upscale_model".to_string(), self.upscale_model.to_workflow_input());
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageUpscaleWithModel";
    const DISPLAY_NAME: &'static str = "Upscale Image (using Model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
