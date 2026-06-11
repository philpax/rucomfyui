//!`transform` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`ImagePadForOutpaint`](super::ImagePadForOutpaint).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ImagePadForOutpaintOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Crop Image (Center)**: Center crop an image to the specified dimensions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CenterCropImages<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Crop width.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub width: WidthParam,
    /**Crop height.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub height: HeightParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> CenterCropImages<ImagesParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, width: WidthParam, height: HeightParam) -> Self {
        Self { images, width, height }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for CenterCropImages<ImagesParam, WidthParam, HeightParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "CenterCropImages";
    const DISPLAY_NAME: &'static str = "Crop Image (Center)";
    const DESCRIPTION: &'static str = "Center crop an image to the specified dimensions.";
    const CATEGORY: &'static str = "image/transform";
}
///**Crop By Bounding Boxes**: Crop and resize regions from the input image batch based on provided bounding boxes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CropByBBoxes<
    ImageParam: crate::nodes::types::Image,
    BboxesParam: crate::nodes::types::BoundingBox,
    OutputWidthParam: crate::nodes::types::Int,
    OutputHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub bboxes: BboxesParam,
    /**Width each crop is resized to.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 8
*/
    pub output_width: OutputWidthParam,
    /**Height each crop is resized to.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 8
*/
    pub output_height: OutputHeightParam,
    /**Extra padding in pixels added on each side of the bbox before cropping.

**Metadata**:
  - Default: 0
  - Max: 1024
  - Min: 0
  - Step: 1
*/
    pub padding: PaddingParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    BboxesParam: crate::nodes::types::BoundingBox,
    OutputWidthParam: crate::nodes::types::Int,
    OutputHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> CropByBBoxes<
    ImageParam,
    BboxesParam,
    OutputWidthParam,
    OutputHeightParam,
    PaddingParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        bboxes: BboxesParam,
        output_width: OutputWidthParam,
        output_height: OutputHeightParam,
        padding: PaddingParam,
    ) -> Self {
        Self {
            image,
            bboxes,
            output_width,
            output_height,
            padding,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    BboxesParam: crate::nodes::types::BoundingBox,
    OutputWidthParam: crate::nodes::types::Int,
    OutputHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for CropByBBoxes<
    ImageParam,
    BboxesParam,
    OutputWidthParam,
    OutputHeightParam,
    PaddingParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("bboxes".to_string(), self.bboxes.clone().into());
        output.insert("output_width".to_string(), self.output_width.clone().into());
        output.insert("output_height".to_string(), self.output_height.clone().into());
        output.insert("padding".to_string(), self.padding.clone().into());
        output
    }
    const NAME: &'static str = "CropByBBoxes";
    const DISPLAY_NAME: &'static str = "Crop By Bounding Boxes";
    const DESCRIPTION: &'static str = "Crop and resize regions from the input image batch based on provided bounding boxes.";
    const CATEGORY: &'static str = "image/transform";
}
///**Crop Image (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Crop Image (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**Crop Image**: Crop an image to the specified dimensions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageCropV2<
    ImageParam: crate::nodes::types::Image,
    CropRegionParam: crate::nodes::types::BoundingBox,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub crop_region: CropRegionParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    CropRegionParam: crate::nodes::types::BoundingBox,
> ImageCropV2<ImageParam, CropRegionParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, crop_region: CropRegionParam) -> Self {
        Self { image, crop_region }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    CropRegionParam: crate::nodes::types::BoundingBox,
> crate::nodes::TypedNode for ImageCropV2<ImageParam, CropRegionParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("crop_region".to_string(), self.crop_region.clone().into());
        output
    }
    const NAME: &'static str = "ImageCropV2";
    const DISPLAY_NAME: &'static str = "Crop Image";
    const DESCRIPTION: &'static str = "Crop an image to the specified dimensions.";
    const CATEGORY: &'static str = "image/transform";
}
///**Flip Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageFlip<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageFlip<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageFlip<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageFlip";
    const DISPLAY_NAME: &'static str = "Flip Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**Pad Image for Outpainting**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImagePadForOutpaint<
    ImageParam: crate::nodes::types::Image,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    FeatheringParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub left: LeftParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub top: TopParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub right: RightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub bottom: BottomParam,
    /**No documentation.

**Metadata**:
  - Default: 40
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub feathering: FeatheringParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    FeatheringParam: crate::nodes::types::Int,
> ImagePadForOutpaint<
    ImageParam,
    LeftParam,
    TopParam,
    RightParam,
    BottomParam,
    FeatheringParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        left: LeftParam,
        top: TopParam,
        right: RightParam,
        bottom: BottomParam,
        feathering: FeatheringParam,
    ) -> Self {
        Self {
            image,
            left,
            top,
            right,
            bottom,
            feathering,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    FeatheringParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImagePadForOutpaint<
    ImageParam,
    LeftParam,
    TopParam,
    RightParam,
    BottomParam,
    FeatheringParam,
> {
    type Output = out::ImagePadForOutpaintOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("left".to_string(), self.left.clone().into());
        output.insert("top".to_string(), self.top.clone().into());
        output.insert("right".to_string(), self.right.clone().into());
        output.insert("bottom".to_string(), self.bottom.clone().into());
        output.insert("feathering".to_string(), self.feathering.clone().into());
        output
    }
    const NAME: &'static str = "ImagePadForOutpaint";
    const DISPLAY_NAME: &'static str = "Pad Image for Outpainting";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**Rotate Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageRotate<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageRotate<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageRotate<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageRotate";
    const DISPLAY_NAME: &'static str = "Rotate Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
#[doc = "**Stitch Images**: Stitches image2 to image1 in the specified direction.\n\nIf image2 is not provided, returns image1 unchanged.\n\nOptional spacing can be added between images."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageStitch<
    Image1Param: crate::nodes::types::Image,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    Image2Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub image_1: Image1Param,
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
    ///No documentation.
    pub image_2: Option<Image2Param>,
}
impl<
    Image1Param: crate::nodes::types::Image,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    Image2Param: crate::nodes::types::Image,
> ImageStitch<Image1Param, MatchImageSizeParam, SpacingWidthParam, Image2Param> {
    /// Create a new node.
    pub fn new(
        image_1: Image1Param,
        match_image_size: MatchImageSizeParam,
        spacing_width: SpacingWidthParam,
        image_2: Option<Image2Param>,
    ) -> Self {
        Self {
            image_1,
            match_image_size,
            spacing_width,
            image_2,
        }
    }
}
impl<
    Image1Param: crate::nodes::types::Image,
    MatchImageSizeParam: crate::nodes::types::Boolean,
    SpacingWidthParam: crate::nodes::types::Int,
    Image2Param: crate::nodes::types::Image,
> crate::nodes::TypedNode
for ImageStitch<Image1Param, MatchImageSizeParam, SpacingWidthParam, Image2Param> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output
            .insert(
                "match_image_size".to_string(),
                self.match_image_size.clone().into(),
            );
        output.insert("spacing_width".to_string(), self.spacing_width.clone().into());
        if let Some(v) = &self.image_2 {
            output.insert("image2".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ImageStitch";
    const DISPLAY_NAME: &'static str = "Stitch Images";
    const DESCRIPTION: &'static str = "Stitches image2 to image1 in the specified direction.\nIf image2 is not provided, returns image1 unchanged.\nOptional spacing can be added between images.";
    const CATEGORY: &'static str = "image/transform";
}
///**Crop Image (Random)**: Randomly crop an image to the specified dimensions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RandomCropImages<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Crop width.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub width: WidthParam,
    /**Crop height.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub height: HeightParam,
    /**Random seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> RandomCropImages<ImagesParam, WidthParam, HeightParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        width: WidthParam,
        height: HeightParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            images,
            width,
            height,
            seed,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RandomCropImages<ImagesParam, WidthParam, HeightParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "RandomCropImages";
    const DISPLAY_NAME: &'static str = "Crop Image (Random)";
    const DESCRIPTION: &'static str = "Randomly crop an image to the specified dimensions.";
    const CATEGORY: &'static str = "image/transform";
}
///**Resize And Pad Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeAndPadImage<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
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
}
impl<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
> ResizeAndPadImage<ImageParam, TargetWidthParam, TargetHeightParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        target_width: TargetWidthParam,
        target_height: TargetHeightParam,
    ) -> Self {
        Self {
            image,
            target_width,
            target_height,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ResizeAndPadImage<ImageParam, TargetWidthParam, TargetHeightParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("target_width".to_string(), self.target_width.clone().into());
        output.insert("target_height".to_string(), self.target_height.clone().into());
        output
    }
    const NAME: &'static str = "ResizeAndPadImage";
    const DISPLAY_NAME: &'static str = "Resize And Pad Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
///**Resize Image/Mask**: Resize an image or mask using various scaling methods.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeImageMaskNode<InputParam: crate::nodes::types::ComfyMatchTypeV3> {
    ///No documentation.
    pub input: InputParam,
}
impl<InputParam: crate::nodes::types::ComfyMatchTypeV3> ResizeImageMaskNode<InputParam> {
    /// Create a new node.
    pub fn new(input: InputParam) -> Self {
        Self { input }
    }
}
impl<InputParam: crate::nodes::types::ComfyMatchTypeV3> crate::nodes::TypedNode
for ResizeImageMaskNode<InputParam> {
    type Output = crate::nodes::types::ComfyMatchTypeV3Out;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("input".to_string(), self.input.clone().into());
        output
    }
    const NAME: &'static str = "ResizeImageMaskNode";
    const DISPLAY_NAME: &'static str = "Resize Image/Mask";
    const DESCRIPTION: &'static str = "Resize an image or mask using various scaling methods.";
    const CATEGORY: &'static str = "image/transform";
}
///**Resize Images by Longer Edge (DEPRECATED)**: Resize images so that the longer edge matches the specified dimension while preserving aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeImagesByLongerEdge<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Target dimension for the longer edge.

**Metadata**:
  - Default: 1024
  - Max: 8192
  - Min: 1
*/
    pub longer_edge: LongerEdgeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> ResizeImagesByLongerEdge<ImagesParam, LongerEdgeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, longer_edge: LongerEdgeParam) -> Self {
        Self { images, longer_edge }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ResizeImagesByLongerEdge<ImagesParam, LongerEdgeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("longer_edge".to_string(), self.longer_edge.clone().into());
        output
    }
    const NAME: &'static str = "ResizeImagesByLongerEdge";
    const DISPLAY_NAME: &'static str = "Resize Images by Longer Edge (DEPRECATED)";
    const DESCRIPTION: &'static str = "Resize images so that the longer edge matches the specified dimension while preserving aspect ratio.";
    const CATEGORY: &'static str = "image/transform";
}
///**Resize Images by Shorter Edge (DEPRECATED)**: Resize images so that the shorter edge matches the specified dimension while preserving aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeImagesByShorterEdge<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Target dimension for the shorter edge.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub shorter_edge: ShorterEdgeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> ResizeImagesByShorterEdge<ImagesParam, ShorterEdgeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, shorter_edge: ShorterEdgeParam) -> Self {
        Self { images, shorter_edge }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ResizeImagesByShorterEdge<ImagesParam, ShorterEdgeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("shorter_edge".to_string(), self.shorter_edge.clone().into());
        output
    }
    const NAME: &'static str = "ResizeImagesByShorterEdge";
    const DISPLAY_NAME: &'static str = "Resize Images by Shorter Edge (DEPRECATED)";
    const DESCRIPTION: &'static str = "Resize images so that the shorter edge matches the specified dimension while preserving aspect ratio.";
    const CATEGORY: &'static str = "image/transform";
}
