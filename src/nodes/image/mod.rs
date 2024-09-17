//!image
pub mod animation;
pub mod batch;
pub mod postprocessing;
pub mod preprocessors;
pub mod transform;
pub mod upscaling;
///**EmptyImage**
pub struct EmptyImage {}
///**Batch Images**
pub struct ImageBatch {}
///**ImageCompositeMasked**
pub struct ImageCompositeMasked {}
///**Invert Image**
pub struct ImageInvert {}
///**Pad Image for Outpainting**
pub struct ImagePadForOutpaint {}
///**Load Image**
pub struct LoadImage {}
#[doc = "**Preview Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct PreviewImage {}
#[doc = "**Save Image**\n\nSaves the input images to your ComfyUI output directory."]
pub struct SaveImage {}
///**Webcam Capture**
pub struct WebcamCapture {}
