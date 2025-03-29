//!`upscaling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Upscale Image**: No description.
#[derive(Clone)]
pub struct ImageScale<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub height: HeightParam,
    ///No documentation.
    pub crop: CropParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> ImageScale<ImageParam, UpscaleMethodParam, WidthParam, HeightParam, CropParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        upscale_method: UpscaleMethodParam,
        width: WidthParam,
        height: HeightParam,
        crop: CropParam,
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
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ImageScale<ImageParam, UpscaleMethodParam, WidthParam, HeightParam, CropParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("crop".to_string(), self.crop.clone().into());
        output
    }
    const NAME: &'static str = "ImageScale";
    const DISPLAY_NAME: &'static str = "Upscale Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image By**: No description.
#[derive(Clone)]
pub struct ImageScaleBy<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 8
  - Min: 0.01
  - Step: 0.01
*/
    pub scale_by: ScaleByParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> ImageScaleBy<ImageParam, UpscaleMethodParam, ScaleByParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        upscale_method: UpscaleMethodParam,
        scale_by: ScaleByParam,
    ) -> Self {
        Self {
            image,
            upscale_method,
            scale_by,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ImageScaleBy<ImageParam, UpscaleMethodParam, ScaleByParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output.insert("scale_by".to_string(), self.scale_by.clone().into());
        output
    }
    const NAME: &'static str = "ImageScaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Image By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Scale Image to Total Pixels**: No description.
#[derive(Clone)]
pub struct ImageScaleToTotalPixels<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    MegapixelsParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 16
  - Min: 0.01
  - Step: 0.01
*/
    pub megapixels: MegapixelsParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    MegapixelsParam: crate::nodes::types::Float,
> ImageScaleToTotalPixels<ImageParam, UpscaleMethodParam, MegapixelsParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        upscale_method: UpscaleMethodParam,
        megapixels: MegapixelsParam,
    ) -> Self {
        Self {
            image,
            upscale_method,
            megapixels,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    UpscaleMethodParam: crate::nodes::types::String,
    MegapixelsParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ImageScaleToTotalPixels<ImageParam, UpscaleMethodParam, MegapixelsParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output.insert("megapixels".to_string(), self.megapixels.clone().into());
        output
    }
    const NAME: &'static str = "ImageScaleToTotalPixels";
    const DISPLAY_NAME: &'static str = "Scale Image to Total Pixels";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
///**Upscale Image (using Model)**: No description.
#[derive(Clone)]
pub struct ImageUpscaleWithModel<
    UpscaleModelParam: crate::nodes::types::UpscaleModel,
    ImageParam: crate::nodes::types::Image,
> {
    ///No documentation.
    pub upscale_model: UpscaleModelParam,
    ///No documentation.
    pub image: ImageParam,
}
impl<
    UpscaleModelParam: crate::nodes::types::UpscaleModel,
    ImageParam: crate::nodes::types::Image,
> ImageUpscaleWithModel<UpscaleModelParam, ImageParam> {
    /// Create a new node.
    pub fn new(upscale_model: UpscaleModelParam, image: ImageParam) -> Self {
        Self { upscale_model, image }
    }
}
impl<
    UpscaleModelParam: crate::nodes::types::UpscaleModel,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageUpscaleWithModel<UpscaleModelParam, ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("upscale_model".to_string(), self.upscale_model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageUpscaleWithModel";
    const DISPLAY_NAME: &'static str = "Upscale Image (using Model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/upscaling";
}
