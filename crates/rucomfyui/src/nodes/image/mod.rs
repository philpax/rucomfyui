//!`image` definitions/categories.
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
#[rustfmt::skip]
pub mod adjustments;
#[rustfmt::skip]
pub mod background_removal;
#[rustfmt::skip]
pub mod batch;
#[rustfmt::skip]
pub mod color;
#[rustfmt::skip]
pub mod compositing;
#[rustfmt::skip]
pub mod detection;
#[rustfmt::skip]
pub mod filters;
#[rustfmt::skip]
pub mod geometry_estimation;
#[rustfmt::skip]
pub mod mask;
#[rustfmt::skip]
pub mod shader;
#[rustfmt::skip]
pub mod transform;
#[rustfmt::skip]
pub mod upscaling;
#[rustfmt::skip]
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
    ///Output for [`LoadImageTextDataSetFromFolder`](super::LoadImageTextDataSetFromFolder).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoadImageTextDataSetFromFolderOutput {
        ///List of loaded images
        pub images: crate::nodes::types::ImageOut,
        ///List of text captions
        pub texts: crate::nodes::types::StringOut,
    }
    ///Output for [`Painter`](super::Painter).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct PainterOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Empty Image**: No description.
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
    const DISPLAY_NAME: &'static str = "Empty Image";
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
///**Compare Images**: Compares two images side by side with a slider.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageCompare<
    ImageAParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageBParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub image_a: Option<ImageAParam>,
    ///No documentation.
    pub image_b: Option<ImageBParam>,
}
impl<
    ImageAParam: crate::nodes::types::Image,
    ImageBParam: crate::nodes::types::Image,
> ImageCompare<ImageAParam, ImageBParam> {
    /// Create a new node.
    pub fn new(image_a: Option<ImageAParam>, image_b: Option<ImageBParam>) -> Self {
        Self { image_a, image_b }
    }
}
impl<
    ImageAParam: crate::nodes::types::Image,
    ImageBParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageCompare<ImageAParam, ImageBParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.image_a {
            output.insert("image_a".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_b {
            output.insert("image_b".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ImageCompare";
    const DISPLAY_NAME: &'static str = "Compare Images";
    const DESCRIPTION: &'static str = "Compares two images side by side with a slider.";
    const CATEGORY: &'static str = "image";
}
impl<
    ImageAParam: crate::nodes::types::Image,
    ImageBParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode for ImageCompare<ImageAParam, ImageBParam> {}
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
///**Load Image (from Folder)**: Load a dataset of images from a specified folder and return a list of images. Supported formats: PNG, JPG, JPEG, WEBP.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageDataSetFromFolder {}
impl LoadImageDataSetFromFolder {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadImageDataSetFromFolder {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadImageDataSetFromFolder";
    const DISPLAY_NAME: &'static str = "Load Image (from Folder)";
    const DESCRIPTION: &'static str = "Load a dataset of images from a specified folder and return a list of images. Supported formats: PNG, JPG, JPEG, WEBP.";
    const CATEGORY: &'static str = "image";
}
///**Load Image (as Mask)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageMask<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Image upload: true
*/
    pub image: ImageParam,
    ///No documentation.
    pub channel: ChannelParam,
}
impl<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> LoadImageMask<ImageParam, ChannelParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, channel: ChannelParam) -> Self {
        Self { image, channel }
    }
}
impl<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LoadImageMask<ImageParam, ChannelParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("channel".to_string(), self.channel.clone().into());
        output
    }
    const NAME: &'static str = "LoadImageMask";
    const DISPLAY_NAME: &'static str = "Load Image (as Mask)";
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
///**Load Image-Text (from Folder)**: Load a dataset of pairs of images and text captions from a specified folder and return them as a list. Supported formats: PNG, JPG, JPEG, WEBP.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageTextDataSetFromFolder {}
impl LoadImageTextDataSetFromFolder {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadImageTextDataSetFromFolder {
    type Output = out::LoadImageTextDataSetFromFolderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            images: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            texts: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadImageTextDataSetFromFolder";
    const DISPLAY_NAME: &'static str = "Load Image-Text (from Folder)";
    const DESCRIPTION: &'static str = "Load a dataset of pairs of images and text captions from a specified folder and return them as a list. Supported formats: PNG, JPG, JPEG, WEBP.";
    const CATEGORY: &'static str = "image";
}
///**Painter**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Painter<
    MaskParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BgColorParam: crate::nodes::types::Color,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**No documentation.

**Metadata**:
  - Image upload: true
*/
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 64
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 64
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: #000000
*/
    pub bg_color: BgColorParam,
    ///Optional base image to paint over
    pub image: Option<ImageParam>,
}
impl<
    MaskParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BgColorParam: crate::nodes::types::Color,
    ImageParam: crate::nodes::types::Image,
> Painter<MaskParam, WidthParam, HeightParam, BgColorParam, ImageParam> {
    /// Create a new node.
    pub fn new(
        mask: MaskParam,
        width: WidthParam,
        height: HeightParam,
        bg_color: BgColorParam,
        image: Option<ImageParam>,
    ) -> Self {
        Self {
            mask,
            width,
            height,
            bg_color,
            image,
        }
    }
}
impl<
    MaskParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BgColorParam: crate::nodes::types::Color,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for Painter<MaskParam, WidthParam, HeightParam, BgColorParam, ImageParam> {
    type Output = out::PainterOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("bg_color".to_string(), self.bg_color.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Painter";
    const DISPLAY_NAME: &'static str = "Painter";
    const DESCRIPTION: &'static str = "";
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
///**Save Animated PNG**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAnimatedPNG<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 9
  - Min: 0
*/
    pub compress_level: CompressLevelParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        fps: FpsParam,
        compress_level: CompressLevelParam,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            fps,
            compress_level,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("compress_level".to_string(), self.compress_level.clone().into());
        output
    }
    const NAME: &'static str = "SaveAnimatedPNG";
    const DISPLAY_NAME: &'static str = "Save Animated PNG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {}
///**Save Animated WEBP**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAnimatedWEBP<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub lossless: LosslessParam,
    /**No documentation.

**Metadata**:
  - Default: 80
  - Max: 100
  - Min: 0
*/
    pub quality: QualityParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
> SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        fps: FpsParam,
        lossless: LosslessParam,
        quality: QualityParam,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            fps,
            lossless,
            quality,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("lossless".to_string(), self.lossless.clone().into());
        output.insert("quality".to_string(), self.quality.clone().into());
        output
    }
    const NAME: &'static str = "SaveAnimatedWEBP";
    const DISPLAY_NAME: &'static str = "Save Animated WEBP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
> {}
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
///**Save Image (Advanced)**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImageAdvanced<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///The images to save.
    pub images: ImagesParam,
    /**The prefix for the file to save. May include formatting tokens such as %date:yyyy-MM-dd% or %Empty Latent Image.width%.

**Metadata**:
  - Multiline: false
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveImageAdvanced<ImagesParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { images, filename_prefix }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveImageAdvanced<ImagesParam, FilenamePrefixParam> {
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
    const NAME: &'static str = "SaveImageAdvanced";
    const DISPLAY_NAME: &'static str = "Save Image (Advanced)";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveImageAdvanced<ImagesParam, FilenamePrefixParam> {}
///**Save Image (to Folder) (DEPRECATED)**: Save a dataset of images to a specified folder. Supported formats: PNG.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImageDataSetToFolder<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///List of images to save.
    pub images: ImagesParam,
    /**Name of the folder to save images to (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: dataset
*/
    pub folder_name: FolderNameParam,
    /**Prefix for saved image filenames.

**Metadata**:
  - Multiline: false
  - Default: image
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        folder_name: FolderNameParam,
        filename_prefix: FilenamePrefixParam,
    ) -> Self {
        Self {
            images,
            folder_name,
            filename_prefix,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveImageDataSetToFolder";
    const DISPLAY_NAME: &'static str = "Save Image (to Folder) (DEPRECATED)";
    const DESCRIPTION: &'static str = "Save a dataset of images to a specified folder. Supported formats: PNG.";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {}
///**Save Image-Text (to Folder)**: Save a dataset of pairs of images and text captions to a specified folder. Images are saved as PNG files and captions are saved as TXT files with the same filename_prefix.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImageTextDataSetToFolder<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
    TextsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///List of images to save.
    pub images: ImagesParam,
    /**Name of the folder to save images to (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: dataset
*/
    pub folder_name: FolderNameParam,
    /**Prefix for saved image filenames.

**Metadata**:
  - Multiline: false
  - Default: image
*/
    pub filename_prefix: FilenamePrefixParam,
    /**List of text captions to save.

**Metadata**:
  - Multiline: false
*/
    pub texts: Option<TextsParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
    TextsParam: crate::nodes::types::String,
> SaveImageTextDataSetToFolder<
    ImagesParam,
    FolderNameParam,
    FilenamePrefixParam,
    TextsParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        folder_name: FolderNameParam,
        filename_prefix: FilenamePrefixParam,
        texts: Option<TextsParam>,
    ) -> Self {
        Self {
            images,
            folder_name,
            filename_prefix,
            texts,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
    TextsParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveImageTextDataSetToFolder<
    ImagesParam,
    FolderNameParam,
    FilenamePrefixParam,
    TextsParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        if let Some(v) = &self.texts {
            output.insert("texts".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SaveImageTextDataSetToFolder";
    const DISPLAY_NAME: &'static str = "Save Image-Text (to Folder)";
    const DESCRIPTION: &'static str = "Save a dataset of pairs of images and text captions to a specified folder. Images are saved as PNG files and captions are saved as TXT files with the same filename_prefix.";
    const CATEGORY: &'static str = "image";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
    TextsParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveImageTextDataSetToFolder<
    ImagesParam,
    FolderNameParam,
    FilenamePrefixParam,
    TextsParam,
> {}
///**Save SVG**: Save SVG files on disk.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveSVGNode<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub svg: SvgParam,
    /**The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.

**Metadata**:
  - Multiline: false
  - Default: svg/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveSVGNode<SvgParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(svg: SvgParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { svg, filename_prefix }
    }
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveSVGNode<SvgParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("svg".to_string(), self.svg.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveSVGNode";
    const DISPLAY_NAME: &'static str = "Save SVG";
    const DESCRIPTION: &'static str = "Save SVG files on disk.";
    const CATEGORY: &'static str = "image";
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveSVGNode<SvgParam, FilenamePrefixParam> {}
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
