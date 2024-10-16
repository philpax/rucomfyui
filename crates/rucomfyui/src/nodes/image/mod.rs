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
pub mod transform;
pub mod upscaling;
/// Output types for nodes.
pub mod out {
    ///Output for [`ImagePadForOutpaint`](super::ImagePadForOutpaint).
    #[derive(Clone)]
    pub struct ImagePadForOutpaintOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`LoadImage`](super::LoadImage).
    #[derive(Clone)]
    pub struct LoadImageOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**EmptyImage**: No description.
#[derive(Clone)]
pub struct EmptyImage<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Color: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: color
  - Max: 16777215
  - Min: 0
  - Step: 1
*/
    pub color: Color,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Color: crate::nodes::types::Int,
> EmptyImage<Width, Height, BatchSize, Color> {
    /// Create a new node.
    pub fn new(
        width: Width,
        height: Height,
        batch_size: BatchSize,
        color: Color,
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
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Color: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyImage<Width, Height, BatchSize, Color> {
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
///**Batch Images**: No description.
#[derive(Clone)]
pub struct ImageBatch<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
> {
    ///No documentation.
    pub image_1: Image1,
    ///No documentation.
    pub image_2: Image2,
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
> ImageBatch<Image1, Image2> {
    /// Create a new node.
    pub fn new(image_1: Image1, image_2: Image2) -> Self {
        Self { image_1, image_2 }
    }
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageBatch<Image1, Image2> {
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
pub struct ImageCompositeMasked<
    Destination: crate::nodes::types::Image,
    Source: crate::nodes::types::Image,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    ResizeSource: crate::nodes::types::Boolean,
    Mask: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: X,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: Y,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub resize_source: ResizeSource,
    ///No documentation.
    pub mask: Option<Mask>,
}
impl<
    Destination: crate::nodes::types::Image,
    Source: crate::nodes::types::Image,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    ResizeSource: crate::nodes::types::Boolean,
    Mask: crate::nodes::types::Mask,
> ImageCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    /// Create a new node.
    pub fn new(
        destination: Destination,
        source: Source,
        x: X,
        y: Y,
        resize_source: ResizeSource,
        mask: Option<Mask>,
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
    Destination: crate::nodes::types::Image,
    Source: crate::nodes::types::Image,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    ResizeSource: crate::nodes::types::Boolean,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for ImageCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
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
pub struct ImageInvert<Image: crate::nodes::types::Image> {
    ///No documentation.
    pub image: Image,
}
impl<Image: crate::nodes::types::Image> ImageInvert<Image> {
    /// Create a new node.
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}
impl<Image: crate::nodes::types::Image> crate::nodes::TypedNode for ImageInvert<Image> {
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
pub struct ImagePadForOutpaint<
    Image: crate::nodes::types::Image,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
    Feathering: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub left: Left,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub top: Top,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub right: Right,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub bottom: Bottom,
    /**No documentation.

**Metadata**:
  - Default: 40
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub feathering: Feathering,
}
impl<
    Image: crate::nodes::types::Image,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
    Feathering: crate::nodes::types::Int,
> ImagePadForOutpaint<Image, Left, Top, Right, Bottom, Feathering> {
    /// Create a new node.
    pub fn new(
        image: Image,
        left: Left,
        top: Top,
        right: Right,
        bottom: Bottom,
        feathering: Feathering,
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
    Image: crate::nodes::types::Image,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
    Feathering: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImagePadForOutpaint<Image, Left, Top, Right, Bottom, Feathering> {
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
///**Load Image**: No description.
#[derive(Clone)]
pub struct LoadImage<Image: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Image upload: true
*/
    pub image: Image,
}
impl<Image: crate::nodes::types::String> LoadImage<Image> {
    /// Create a new node.
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}
impl<Image: crate::nodes::types::String> crate::nodes::TypedNode for LoadImage<Image> {
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
///**Preview Image**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
pub struct PreviewImage<Images: crate::nodes::types::Image> {
    ///No documentation.
    pub images: Images,
}
impl<Images: crate::nodes::types::Image> PreviewImage<Images> {
    /// Create a new node.
    pub fn new(images: Images) -> Self {
        Self { images }
    }
}
impl<Images: crate::nodes::types::Image> crate::nodes::TypedNode
for PreviewImage<Images> {
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
impl<Images: crate::nodes::types::Image> crate::nodes::TypedOutputNode
for PreviewImage<Images> {}
///**Save Image**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
pub struct SaveImage<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///The images to save.
    pub images: Images,
    /**The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> SaveImage<Images, FilenamePrefix> {
    /// Create a new node.
    pub fn new(images: Images, filename_prefix: FilenamePrefix) -> Self {
        Self { images, filename_prefix }
    }
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveImage<Images, FilenamePrefix> {
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
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveImage<Images, FilenamePrefix> {}
///**Webcam Capture**: No description.
#[derive(Clone)]
pub struct WebcamCapture<
    Image: crate::nodes::types::Webcam,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CaptureOnQueue: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub capture_on_queue: CaptureOnQueue,
}
impl<
    Image: crate::nodes::types::Webcam,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CaptureOnQueue: crate::nodes::types::Boolean,
> WebcamCapture<Image, Width, Height, CaptureOnQueue> {
    /// Create a new node.
    pub fn new(
        image: Image,
        width: Width,
        height: Height,
        capture_on_queue: CaptureOnQueue,
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
    Image: crate::nodes::types::Webcam,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CaptureOnQueue: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for WebcamCapture<Image, Width, Height, CaptureOnQueue> {
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
