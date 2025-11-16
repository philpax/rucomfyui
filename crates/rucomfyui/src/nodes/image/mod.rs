//!`image` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod animation;
pub mod batch;
pub mod postprocessing;
pub mod preprocessors;
pub mod save;
pub mod transform;
pub mod upscaling;
pub mod video;
/// Output types for nodes.
pub mod out {
    ///Output for [`GetImageSize`](super::GetImageSize).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GetImageSizeOutput {
        ///No documentation.
        pub width: crate::nodes::types::IntOut,
        ///No documentation.
        pub height: crate::nodes::types::IntOut,
        ///No documentation.
        pub batch_size: crate::nodes::types::IntOut,
    }
    ///Output for [`ImagePadForOutpaint`](super::ImagePadForOutpaint).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ImagePadForOutpaintOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`LoadImage`](super::LoadImage).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoadImageOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`LoadImageOutput`](super::LoadImageOutput).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoadImageOutputOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**EmptyImage**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ColorParam: crate::nodes::types::Int,
> {
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
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: color
  - Max: 16777215
  - Min: 0
  - Step: 1
*/
    pub color: ColorParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ColorParam: crate::nodes::types::Int,
> EmptyImage<WidthParam, HeightParam, BatchSizeParam, ColorParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
        color: ColorParam,
    ) -> Self {
        Self {
            width,
            height,
            batch_size,
            color,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ColorParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyImage<WidthParam, HeightParam, BatchSizeParam, ColorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("color".to_string(), self.color.clone().into());
        output
    }
    const NAME: &'static str = "EmptyImage";
    const DISPLAY_NAME: &'static str = "EmptyImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Get Image Size**: Returns width and height of the image, and passes it through unchanged.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GetImageSize<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> GetImageSize<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for GetImageSize<ImageParam> {
    type Output = out::GetImageSizeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            width: crate::nodes::types::IntOut::from_dynamic(node_id, 0u32),
            height: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
            batch_size: crate::nodes::types::IntOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "GetImageSize";
    const DISPLAY_NAME: &'static str = "Get Image Size";
    const DESCRIPTION: &'static str = "Returns width and height of the image, and passes it through unchanged.";
    const CATEGORY: &'static str = "image";
}
///**ImageAddNoise**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageAddNoise<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> ImageAddNoise<ImageParam, SeedParam, StrengthParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, seed: SeedParam, strength: StrengthParam) -> Self {
        Self { image, seed, strength }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageAddNoise<ImageParam, SeedParam, StrengthParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ImageAddNoise";
    const DISPLAY_NAME: &'static str = "ImageAddNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Batch Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageBatch<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> {
    ///No documentation.
    pub image_1: Image1Param,
    ///No documentation.
    pub image_2: Image2Param,
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> ImageBatch<Image1Param, Image2Param> {
    /// Create a new node.
    pub fn new(image_1: Image1Param, image_2: Image2Param) -> Self {
        Self { image_1, image_2 }
    }
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageBatch<Image1Param, Image2Param> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output.insert("image2".to_string(), self.image_2.clone().into());
        output
    }
    const NAME: &'static str = "ImageBatch";
    const DISPLAY_NAME: &'static str = "Batch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**ImageCompositeMasked**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageCompositeMasked<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    ///No documentation.
    pub destination: DestinationParam,
    ///No documentation.
    pub source: SourceParam,
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
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub resize_source: ResizeSourceParam,
    ///No documentation.
    pub mask: Option<MaskParam>,
}
impl<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> ImageCompositeMasked<
    DestinationParam,
    SourceParam,
    XParam,
    YParam,
    ResizeSourceParam,
    MaskParam,
> {
    /// Create a new node.
    pub fn new(
        destination: DestinationParam,
        source: SourceParam,
        x: XParam,
        y: YParam,
        resize_source: ResizeSourceParam,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            destination,
            source,
            x,
            y,
            resize_source,
            mask,
        }
    }
}
impl<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for ImageCompositeMasked<
    DestinationParam,
    SourceParam,
    XParam,
    YParam,
    ResizeSourceParam,
    MaskParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("destination".to_string(), self.destination.clone().into());
        output.insert("source".to_string(), self.source.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("resize_source".to_string(), self.resize_source.clone().into());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ImageCompositeMasked";
    const DISPLAY_NAME: &'static str = "ImageCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Invert Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageInvert<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageInvert<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageInvert<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageInvert";
    const DISPLAY_NAME: &'static str = "Invert Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
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
    const CATEGORY: &'static str = "image";
}
///**LTXVPreprocess**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVPreprocess<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Amount of compression to apply on image.

**Metadata**:
  - Default: 35
  - Max: 100
  - Min: 0
*/
    pub img_compression: ImgCompressionParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> LTXVPreprocess<ImageParam, ImgCompressionParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, img_compression: ImgCompressionParam) -> Self {
        Self { image, img_compression }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for LTXVPreprocess<ImageParam, ImgCompressionParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert("img_compression".to_string(), self.img_compression.clone().into());
        output
    }
    const NAME: &'static str = "LTXVPreprocess";
    const DISPLAY_NAME: &'static str = "LTXVPreprocess";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Load Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImage<ImageParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Image upload: true
*/
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::String> LoadImage<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::String> crate::nodes::TypedNode
for LoadImage<ImageParam> {
    type Output = out::LoadImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "LoadImage";
    const DISPLAY_NAME: &'static str = "Load Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Load Image (from Outputs)**: Load an image from the output folder. When the refresh button is clicked, the node will update the image list and automatically select the first image, allowing for easy iteration.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageOutput {}
impl LoadImageOutput {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadImageOutput {
    type Output = out::LoadImageOutputOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadImageOutput";
    const DISPLAY_NAME: &'static str = "Load Image (from Outputs)";
    const DESCRIPTION: &'static str = "Load an image from the output folder. When the refresh button is clicked, the node will update the image list and automatically select the first image, allowing for easy iteration.";
    const CATEGORY: &'static str = "image";
}
///**Preview Image**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewImage<ImagesParam: crate::nodes::types::Image> {
    ///No documentation.
    pub images: ImagesParam,
}
impl<ImagesParam: crate::nodes::types::Image> PreviewImage<ImagesParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam) -> Self {
        Self { images }
    }
}
impl<ImagesParam: crate::nodes::types::Image> crate::nodes::TypedNode
for PreviewImage<ImagesParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
    }
    const NAME: &'static str = "PreviewImage";
    const DISPLAY_NAME: &'static str = "Preview Image";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image";
}
impl<ImagesParam: crate::nodes::types::Image> crate::nodes::TypedOutputNode
for PreviewImage<ImagesParam> {}
///**Save Image**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImage<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///The images to save.
    pub images: ImagesParam,
    /**The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveImage<ImagesParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { images, filename_prefix }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveImage<ImagesParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveImage";
    const DISPLAY_NAME: &'static str = "Save Image";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveImage<ImagesParam, FilenamePrefixParam> {}
///**Webcam Capture**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WebcamCapture<
    ImageParam: crate::nodes::types::Webcam,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CaptureOnQueueParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub capture_on_queue: CaptureOnQueueParam,
}
impl<
    ImageParam: crate::nodes::types::Webcam,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CaptureOnQueueParam: crate::nodes::types::Boolean,
> WebcamCapture<ImageParam, WidthParam, HeightParam, CaptureOnQueueParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        width: WidthParam,
        height: HeightParam,
        capture_on_queue: CaptureOnQueueParam,
    ) -> Self {
        Self {
            image,
            width,
            height,
            capture_on_queue,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Webcam,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CaptureOnQueueParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WebcamCapture<ImageParam, WidthParam, HeightParam, CaptureOnQueueParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
            .insert(
                "capture_on_queue".to_string(),
                self.capture_on_queue.clone().into(),
            );
        output
    }
    const NAME: &'static str = "WebcamCapture";
    const DISPLAY_NAME: &'static str = "Webcam Capture";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
