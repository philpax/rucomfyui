//!`transform` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Image Crop**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageCrop<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: YParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> ImageCrop<ImageParam, WidthParam, HeightParam, XParam, YParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
    ) -> Self {
        Self { image, width, height, x, y }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImageCrop<ImageParam, WidthParam, HeightParam, XParam, YParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output
    }
    const NAME: &'static str = "ImageCrop";
    const DISPLAY_NAME: &'static str = "Image Crop";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**ImageFlip**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageFlip<
    ImageParam: crate::nodes::types::Image,
    FlipMethodParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub flip_method: FlipMethodParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    FlipMethodParam: crate::nodes::types::String,
> ImageFlip<ImageParam, FlipMethodParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, flip_method: FlipMethodParam) -> Self {
        Self { image, flip_method }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    FlipMethodParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageFlip<ImageParam, FlipMethodParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("flip_method".to_string(), self.flip_method.clone().into());
        output
    }
    const NAME: &'static str = "ImageFlip";
    const DISPLAY_NAME: &'static str = "ImageFlip";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**ImageRotate**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageRotate<
    ImageParam: crate::nodes::types::Image,
    RotationParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub rotation: RotationParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    RotationParam: crate::nodes::types::String,
> ImageRotate<ImageParam, RotationParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, rotation: RotationParam) -> Self {
        Self { image, rotation }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    RotationParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageRotate<ImageParam, RotationParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("rotation".to_string(), self.rotation.clone().into());
        output
    }
    const NAME: &'static str = "ImageRotate";
    const DISPLAY_NAME: &'static str = "ImageRotate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
#[doc = "**Image Stitch**: \n\nStitches image2 to image1 in the specified direction.\n\nIf image2 is not provided, returns image1 unchanged.\n\nOptional spacing can be added between images.\n\n"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageStitch<
    Image1Param: crate::nodes::types::Image,
    DirectionParam: crate::nodes::types::String,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    SpacingColorParam: crate::nodes::types::String,
    Image2Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub image_1: Image1Param,
    /**No documentation.

**Metadata**:
  - Default: right
*/
    pub direction: DirectionParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub match_image_size: MatchImageSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1024
  - Min: 0
  - Step: 2
*/
    pub spacing_width: SpacingWidthParam,
    /**No documentation.

**Metadata**:
  - Default: white
*/
    pub spacing_color: SpacingColorParam,
    ///No documentation.
    pub image_2: Option<Image2Param>,
}
impl<
    Image1Param: crate::nodes::types::Image,
    DirectionParam: crate::nodes::types::String,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    SpacingColorParam: crate::nodes::types::String,
    Image2Param: crate::nodes::types::Image,
> ImageStitch<
    Image1Param,
    DirectionParam,
    MatchImageSizeParam,
    SpacingWidthParam,
    SpacingColorParam,
    Image2Param,
> {
    /// Create a new node.
    pub fn new(
        image_1: Image1Param,
        direction: DirectionParam,
        match_image_size: MatchImageSizeParam,
        spacing_width: SpacingWidthParam,
        spacing_color: SpacingColorParam,
        image_2: Option<Image2Param>,
    ) -> Self {
        Self {
            image_1,
            direction,
            match_image_size,
            spacing_width,
            spacing_color,
            image_2,
        }
    }
}
impl<
    Image1Param: crate::nodes::types::Image,
    DirectionParam: crate::nodes::types::String,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    SpacingColorParam: crate::nodes::types::String,
    Image2Param: crate::nodes::types::Image,
> crate::nodes::TypedNode
for ImageStitch<
    Image1Param,
    DirectionParam,
    MatchImageSizeParam,
    SpacingWidthParam,
    SpacingColorParam,
    Image2Param,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output.insert("direction".to_string(), self.direction.clone().into());
        output
            .insert(
                "match_image_size".to_string(),
                self.match_image_size.clone().into(),
            );
        output.insert("spacing_width".to_string(), self.spacing_width.clone().into());
        output.insert("spacing_color".to_string(), self.spacing_color.clone().into());
        if let Some(v) = &self.image_2 {
            output.insert("image2".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ImageStitch";
    const DISPLAY_NAME: &'static str = "Image Stitch";
    const DESCRIPTION: &'static str = "\nStitches image2 to image1 in the specified direction.\nIf image2 is not provided, returns image1 unchanged.\nOptional spacing can be added between images.\n";
    const CATEGORY: &'static str = "image/transform";
}
///**ResizeAndPadImage**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeAndPadImage<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    PaddingColorParam: crate::nodes::types::String,
    InterpolationParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub target_width: TargetWidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub target_height: TargetHeightParam,
    ///No documentation.
    pub padding_color: PaddingColorParam,
    ///No documentation.
    pub interpolation: InterpolationParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    PaddingColorParam: crate::nodes::types::String,
    InterpolationParam: crate::nodes::types::String,
> ResizeAndPadImage<
    ImageParam,
    TargetWidthParam,
    TargetHeightParam,
    PaddingColorParam,
    InterpolationParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        target_width: TargetWidthParam,
        target_height: TargetHeightParam,
        padding_color: PaddingColorParam,
        interpolation: InterpolationParam,
    ) -> Self {
        Self {
            image,
            target_width,
            target_height,
            padding_color,
            interpolation,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    PaddingColorParam: crate::nodes::types::String,
    InterpolationParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ResizeAndPadImage<
    ImageParam,
    TargetWidthParam,
    TargetHeightParam,
    PaddingColorParam,
    InterpolationParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("target_width".to_string(), self.target_width.clone().into());
        output.insert("target_height".to_string(), self.target_height.clone().into());
        output.insert("padding_color".to_string(), self.padding_color.clone().into());
        output.insert("interpolation".to_string(), self.interpolation.clone().into());
        output
    }
    const NAME: &'static str = "ResizeAndPadImage";
    const DISPLAY_NAME: &'static str = "ResizeAndPadImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
