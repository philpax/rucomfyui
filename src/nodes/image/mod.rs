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
///Output for [`EmptyImage`].
pub struct EmptyImageOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
    Color: crate::nodes::Int,
> crate::nodes::TypedNode for EmptyImage<Width, Height, BatchSize, Color> {
    type Output = EmptyImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "EmptyImage";
    const DISPLAY_NAME: &'static str = "EmptyImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Batch Images**
pub struct ImageBatch<Image1: crate::nodes::Image, Image2: crate::nodes::Image> {
    ///No documentation.
    pub image_1: Image1,
    ///No documentation.
    pub image_2: Image2,
}
///Output for [`ImageBatch`].
pub struct ImageBatchOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image1: crate::nodes::Image, Image2: crate::nodes::Image> crate::nodes::TypedNode
for ImageBatch<Image1, Image2> {
    type Output = ImageBatchOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageBatch";
    const DISPLAY_NAME: &'static str = "Batch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**ImageCompositeMasked**
pub struct ImageCompositeMasked<
    Destination: crate::nodes::Image,
    Source: crate::nodes::Image,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask = crate::nodes::MaskOut,
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
    pub mask: Option<Mask>,
}
///Output for [`ImageCompositeMasked`].
pub struct ImageCompositeMaskedOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Destination: crate::nodes::Image,
    Source: crate::nodes::Image,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask,
> crate::nodes::TypedNode
for ImageCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    type Output = ImageCompositeMaskedOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageCompositeMasked";
    const DISPLAY_NAME: &'static str = "ImageCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Invert Image**
pub struct ImageInvert<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
///Output for [`ImageInvert`].
pub struct ImageInvertOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image: crate::nodes::Image> crate::nodes::TypedNode for ImageInvert<Image> {
    type Output = ImageInvertOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageInvert";
    const DISPLAY_NAME: &'static str = "Invert Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
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
///Output for [`ImagePadForOutpaint`].
pub struct ImagePadForOutpaintOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Image: crate::nodes::Image,
    Left: crate::nodes::Int,
    Top: crate::nodes::Int,
    Right: crate::nodes::Int,
    Bottom: crate::nodes::Int,
    Feathering: crate::nodes::Int,
> crate::nodes::TypedNode
for ImagePadForOutpaint<Image, Left, Top, Right, Bottom, Feathering> {
    type Output = ImagePadForOutpaintOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
            mask: crate::nodes::MaskOut(1usize),
        }
    }
    const NAME: &'static str = "ImagePadForOutpaint";
    const DISPLAY_NAME: &'static str = "Pad Image for Outpainting";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Load Image**
pub struct LoadImage {}
///Output for [`LoadImage`].
pub struct LoadImageOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl crate::nodes::TypedNode for LoadImage {
    type Output = LoadImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
            mask: crate::nodes::MaskOut(1usize),
        }
    }
    const NAME: &'static str = "LoadImage";
    const DISPLAY_NAME: &'static str = "Load Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
#[doc = "**Preview Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct PreviewImage<Images: crate::nodes::Image> {
    ///No documentation.
    pub images: Images,
}
///Output for [`PreviewImage`].
pub struct PreviewImageOutput {}
impl<Images: crate::nodes::Image> crate::nodes::TypedNode for PreviewImage<Images> {
    type Output = PreviewImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "PreviewImage";
    const DISPLAY_NAME: &'static str = "Preview Image";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image";
}
#[doc = "**Save Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct SaveImage<Images: crate::nodes::Image, FilenamePrefix: crate::nodes::String> {
    ///The images to save.
    pub images: Images,
    ///The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.
    pub filename_prefix: FilenamePrefix,
}
///Output for [`SaveImage`].
pub struct SaveImageOutput {}
impl<
    Images: crate::nodes::Image,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for SaveImage<Images, FilenamePrefix> {
    type Output = SaveImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "SaveImage";
    const DISPLAY_NAME: &'static str = "Save Image";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image";
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
///Output for [`WebcamCapture`].
pub struct WebcamCaptureOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Webcam,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    CaptureOnQueue: crate::nodes::Boolean,
> crate::nodes::TypedNode for WebcamCapture<Image, Width, Height, CaptureOnQueue> {
    type Output = WebcamCaptureOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "WebcamCapture";
    const DISPLAY_NAME: &'static str = "Webcam Capture";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
