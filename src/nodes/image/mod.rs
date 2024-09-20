//!`image` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod animation;
pub mod batch;
pub mod postprocessing;
pub mod preprocessors;
pub mod transform;
pub mod upscaling;
/// Output types for nodes.
pub mod out {
    ///Output for [`EmptyImage`](super::EmptyImage).
    #[derive(Clone)]
    pub struct EmptyImageOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageBatch`](super::ImageBatch).
    #[derive(Clone)]
    pub struct ImageBatchOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageCompositeMasked`](super::ImageCompositeMasked).
    #[derive(Clone)]
    pub struct ImageCompositeMaskedOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageInvert`](super::ImageInvert).
    #[derive(Clone)]
    pub struct ImageInvertOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
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
    ///Output for [`WebcamCapture`](super::WebcamCapture).
    #[derive(Clone)]
    pub struct WebcamCaptureOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
///**EmptyImage**: No description.
pub struct EmptyImage<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Color: crate::nodes::types::Int,
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
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Color: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyImage<Width, Height, BatchSize, Color> {
    type Output = out::EmptyImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("batch_size".to_string(), self.batch_size.to_workflow_input());
        output.insert("color".to_string(), self.color.to_workflow_input());
        output
    }
    const NAME: &'static str = "EmptyImage";
    const DISPLAY_NAME: &'static str = "EmptyImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Batch Images**: No description.
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
> crate::nodes::TypedNode for ImageBatch<Image1, Image2> {
    type Output = out::ImageBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_1".to_string(), self.image_1.to_workflow_input());
        output.insert("image_2".to_string(), self.image_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageBatch";
    const DISPLAY_NAME: &'static str = "Batch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**ImageCompositeMasked**: No description.
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
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
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
> crate::nodes::TypedNode
for ImageCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    type Output = out::ImageCompositeMaskedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("destination".to_string(), self.destination.to_workflow_input());
        output.insert("source".to_string(), self.source.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output
            .insert("resize_source".to_string(), self.resize_source.to_workflow_input());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.to_workflow_input());
        }
        output
    }
    const NAME: &'static str = "ImageCompositeMasked";
    const DISPLAY_NAME: &'static str = "ImageCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Invert Image**: No description.
pub struct ImageInvert<Image: crate::nodes::types::Image> {
    ///No documentation.
    pub image: Image,
}
impl<Image: crate::nodes::types::Image> crate::nodes::TypedNode for ImageInvert<Image> {
    type Output = out::ImageInvertOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageInvert";
    const DISPLAY_NAME: &'static str = "Invert Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Pad Image for Outpainting**: No description.
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
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("left".to_string(), self.left.to_workflow_input());
        output.insert("top".to_string(), self.top.to_workflow_input());
        output.insert("right".to_string(), self.right.to_workflow_input());
        output.insert("bottom".to_string(), self.bottom.to_workflow_input());
        output.insert("feathering".to_string(), self.feathering.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImagePadForOutpaint";
    const DISPLAY_NAME: &'static str = "Pad Image for Outpainting";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Load Image**: No description.
pub struct LoadImage<Image: crate::nodes::types::String> {
    ///No documentation.
    pub image: Image,
}
impl<Image: crate::nodes::types::String> crate::nodes::TypedNode for LoadImage<Image> {
    type Output = out::LoadImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
    }
    const NAME: &'static str = "LoadImage";
    const DISPLAY_NAME: &'static str = "Load Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
///**Preview Image**: Saves the input images to your ComfyUI output directory.
pub struct PreviewImage<Images: crate::nodes::types::Image> {
    ///No documentation.
    pub images: Images,
}
impl<Images: crate::nodes::types::Image> crate::nodes::TypedNode
for PreviewImage<Images> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.to_workflow_input());
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
pub struct SaveImage<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///The images to save.
    pub images: Images,
    ///The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveImage<Images, FilenamePrefix> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.to_workflow_input());
        output
            .insert(
                "filename_prefix".to_string(),
                self.filename_prefix.to_workflow_input(),
            );
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
pub struct WebcamCapture<
    Image: crate::nodes::types::Webcam,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CaptureOnQueue: crate::nodes::types::Boolean,
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
impl<
    Image: crate::nodes::types::Webcam,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CaptureOnQueue: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for WebcamCapture<Image, Width, Height, CaptureOnQueue> {
    type Output = out::WebcamCaptureOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output
            .insert(
                "capture_on_queue".to_string(),
                self.capture_on_queue.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "WebcamCapture";
    const DISPLAY_NAME: &'static str = "Webcam Capture";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image";
}
