//!`latent` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd_3;
pub mod stable_cascade;
pub mod transform;
/// Output types for nodes.
pub mod out {
    ///Output for [`EmptyLatentImage`](super::EmptyLatentImage).
    #[derive(Clone)]
    pub struct EmptyLatentImageOutput {
        ///The empty latent image batch.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentComposite`](super::LatentComposite).
    #[derive(Clone)]
    pub struct LatentCompositeOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentCompositeMasked`](super::LatentCompositeMasked).
    #[derive(Clone)]
    pub struct LatentCompositeMaskedOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentUpscale`](super::LatentUpscale).
    #[derive(Clone)]
    pub struct LatentUpscaleOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentUpscaleBy`](super::LatentUpscaleBy).
    #[derive(Clone)]
    pub struct LatentUpscaleByOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`VaeDecode`](super::VaeDecode).
    #[derive(Clone)]
    pub struct VaeDecodeOutput {
        ///The decoded image.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`VaeEncode`](super::VaeEncode).
    #[derive(Clone)]
    pub struct VaeEncodeOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**Empty Latent Image**: Create a new batch of empty latent images to be denoised via sampling.
pub struct EmptyLatentImage<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    ///The width of the latent images in pixels.
    pub width: Width,
    ///The height of the latent images in pixels.
    pub height: Height,
    ///The number of latent images in the batch.
    pub batch_size: BatchSize,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentImage<Width, Height, BatchSize> {
    type Output = out::EmptyLatentImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
        output
    }
    const NAME: &'static str = "EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "Empty Latent Image";
    const DESCRIPTION: &'static str = "Create a new batch of empty latent images to be denoised via sampling.";
    const CATEGORY: &'static str = "latent";
}
///**Latent Composite**: No description.
pub struct LatentComposite<
    SamplesTo: crate::nodes::types::Latent,
    SamplesFrom: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Feather: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples_to: SamplesTo,
    ///No documentation.
    pub samples_from: SamplesFrom,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub feather: Feather,
}
impl<
    SamplesTo: crate::nodes::types::Latent,
    SamplesFrom: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Feather: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentComposite<SamplesTo, SamplesFrom, X, Y, Feather> {
    type Output = out::LatentCompositeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples_to".to_string(), self.samples_to.to_workflow_input());
        output.insert("samples_from".to_string(), self.samples_from.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output.insert("feather".to_string(), self.feather.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentComposite";
    const DISPLAY_NAME: &'static str = "Latent Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**LatentCompositeMasked**: No description.
pub struct LatentCompositeMasked<
    Destination: crate::nodes::types::Latent,
    Source: crate::nodes::types::Latent,
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
    Destination: crate::nodes::types::Latent,
    Source: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    ResizeSource: crate::nodes::types::Boolean,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for LatentCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    type Output = out::LatentCompositeMaskedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
    const NAME: &'static str = "LatentCompositeMasked";
    const DISPLAY_NAME: &'static str = "LatentCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent**: No description.
pub struct LatentUpscale<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub crop: Crop,
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode
for LatentUpscale<Samples, UpscaleMethod, Width, Height, Crop> {
    type Output = out::LatentUpscaleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output
            .insert(
                "upscale_method".to_string(),
                self.upscale_method.to_workflow_input(),
            );
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("crop".to_string(), self.crop.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentUpscale";
    const DISPLAY_NAME: &'static str = "Upscale Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent By**: No description.
pub struct LatentUpscaleBy<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub scale_by: ScaleBy,
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentUpscaleBy<Samples, UpscaleMethod, ScaleBy> {
    type Output = out::LatentUpscaleByOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output
            .insert(
                "upscale_method".to_string(),
                self.upscale_method.to_workflow_input(),
            );
        output.insert("scale_by".to_string(), self.scale_by.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentUpscaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Latent By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**VAE Decode**: Decodes latent images back into pixel space images.
pub struct VaeDecode<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> {
    ///The latent to be decoded.
    pub samples: Samples,
    ///The VAE model used for decoding the latent.
    pub vae: Vae,
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeDecode<Samples, Vae> {
    type Output = out::VaeDecodeOutput;
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
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output
    }
    const NAME: &'static str = "VAEDecode";
    const DISPLAY_NAME: &'static str = "VAE Decode";
    const DESCRIPTION: &'static str = "Decodes latent images back into pixel space images.";
    const CATEGORY: &'static str = "latent";
}
///**VAE Encode**: No description.
pub struct VaeEncode<Pixels: crate::nodes::types::Image, Vae: crate::nodes::types::Vae> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeEncode<Pixels, Vae> {
    type Output = out::VaeEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("pixels".to_string(), self.pixels.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output
    }
    const NAME: &'static str = "VAEEncode";
    const DISPLAY_NAME: &'static str = "VAE Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
