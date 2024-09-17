//!image
pub mod animation;
pub mod batch;
pub mod postprocessing;
pub mod preprocessors;
pub mod transform;
pub mod upscaling;
///**EmptyImage**
pub struct EmptyImage<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
    Color: crate::nodes::Int,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub batch_size: BatchSize,
    ///No documentation.
    pub color: Color,
}
///**Batch Images**
pub struct ImageBatch<Image1: crate::nodes::Image, Image2: crate::nodes::Image> {
    ///No documentation.
    pub image1: Image1,
    ///No documentation.
    pub image2: Image2,
}
///**ImageCompositeMasked**
pub struct ImageCompositeMasked<
    Destination: crate::nodes::Image,
    Source: crate::nodes::Image,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub resize_source: ResizeSource,
    ///No documentation.
    pub mask: Mask,
}
///**Invert Image**
pub struct ImageInvert<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
///**Pad Image for Outpainting**
pub struct ImagePadForOutpaint<
    Image: crate::nodes::Image,
    Left: crate::nodes::Int,
    Top: crate::nodes::Int,
    Right: crate::nodes::Int,
    Bottom: crate::nodes::Int,
    Feathering: crate::nodes::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub left: Left,
    ///No documentation.
    pub top: Top,
    ///No documentation.
    pub right: Right,
    ///No documentation.
    pub bottom: Bottom,
    ///No documentation.
    pub feathering: Feathering,
}
///**Load Image**
pub struct LoadImage {}
#[doc = "**Preview Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct PreviewImage<Images: crate::nodes::Image> {
    ///No documentation.
    pub images: Images,
}
#[doc = "**Save Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct SaveImage<Images: crate::nodes::Image, FilenamePrefix: crate::nodes::String> {
    ///The images to save.
    pub images: Images,
    ///The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.
    pub filename_prefix: FilenamePrefix,
}
///**Webcam Capture**
pub struct WebcamCapture<
    Image: crate::nodes::Webcam,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    CaptureOnQueue: crate::nodes::Boolean,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub capture_on_queue: CaptureOnQueue,
}
