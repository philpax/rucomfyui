//!`latent` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd_3;
pub mod stable_cascade;
pub mod transform;
#[doc = "**Empty Latent Image**\n\nCreate a new batch of empty latent images to be denoised via sampling."]
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
///Output for [`EmptyLatentImage`].
#[derive(Clone)]
pub struct EmptyLatentImageOutput {
    ///The empty latent image batch.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentImage<Width, Height, BatchSize> {
    type Output = EmptyLatentImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "Empty Latent Image";
    const DESCRIPTION: &'static str = "Create a new batch of empty latent images to be denoised via sampling.";
    const CATEGORY: &'static str = "latent";
}
///**Latent Composite**
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
///Output for [`LatentComposite`].
#[derive(Clone)]
pub struct LatentCompositeOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    SamplesTo: crate::nodes::types::Latent,
    SamplesFrom: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Feather: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentComposite<SamplesTo, SamplesFrom, X, Y, Feather> {
    type Output = LatentCompositeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentComposite";
    const DISPLAY_NAME: &'static str = "Latent Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**LatentCompositeMasked**
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
///Output for [`LatentCompositeMasked`].
#[derive(Clone)]
pub struct LatentCompositeMaskedOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
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
    type Output = LatentCompositeMaskedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentCompositeMasked";
    const DISPLAY_NAME: &'static str = "LatentCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent**
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
///Output for [`LatentUpscale`].
#[derive(Clone)]
pub struct LatentUpscaleOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode
for LatentUpscale<Samples, UpscaleMethod, Width, Height, Crop> {
    type Output = LatentUpscaleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentUpscale";
    const DISPLAY_NAME: &'static str = "Upscale Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent By**
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
///Output for [`LatentUpscaleBy`].
#[derive(Clone)]
pub struct LatentUpscaleByOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentUpscaleBy<Samples, UpscaleMethod, ScaleBy> {
    type Output = LatentUpscaleByOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentUpscaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Latent By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
#[doc = "**VAE Decode**\n\nDecodes latent images back into pixel space images."]
pub struct VaeDecode<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> {
    ///The latent to be decoded.
    pub samples: Samples,
    ///The VAE model used for decoding the latent.
    pub vae: Vae,
}
///Output for [`VaeDecode`].
#[derive(Clone)]
pub struct VaeDecodeOutput {
    ///The decoded image.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeDecode<Samples, Vae> {
    type Output = VaeDecodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEDecode";
    const DISPLAY_NAME: &'static str = "VAE Decode";
    const DESCRIPTION: &'static str = "Decodes latent images back into pixel space images.";
    const CATEGORY: &'static str = "latent";
}
///**VAE Encode**
pub struct VaeEncode<Pixels: crate::nodes::types::Image, Vae: crate::nodes::types::Vae> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
}
///Output for [`VaeEncode`].
#[derive(Clone)]
pub struct VaeEncodeOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeEncode<Pixels, Vae> {
    type Output = VaeEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEEncode";
    const DISPLAY_NAME: &'static str = "VAE Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
