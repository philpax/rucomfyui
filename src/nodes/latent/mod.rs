//!`latent` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd_3;
pub mod stable_cascade;
pub mod transform;
///**Empty Latent Image**: Create a new batch of empty latent images to be denoised via sampling.
#[derive(Clone)]
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
> EmptyLatentImage<Width, Height, BatchSize> {
    /// Create a new node.
    pub fn new(width: Width, height: Height, batch_size: BatchSize) -> Self {
        Self { width, height, batch_size }
    }
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentImage<Width, Height, BatchSize> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "Empty Latent Image";
    const DESCRIPTION: &'static str = "Create a new batch of empty latent images to be denoised via sampling.";
    const CATEGORY: &'static str = "latent";
}
///**Latent Composite**: No description.
#[derive(Clone)]
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
> LatentComposite<SamplesTo, SamplesFrom, X, Y, Feather> {
    /// Create a new node.
    pub fn new(
        samples_to: SamplesTo,
        samples_from: SamplesFrom,
        x: X,
        y: Y,
        feather: Feather,
    ) -> Self {
        Self {
            samples_to,
            samples_from,
            x,
            y,
            feather,
        }
    }
}
impl<
    SamplesTo: crate::nodes::types::Latent,
    SamplesFrom: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Feather: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentComposite<SamplesTo, SamplesFrom, X, Y, Feather> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples_to".to_string(), self.samples_to.clone().into());
        output.insert("samples_from".to_string(), self.samples_from.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("feather".to_string(), self.feather.clone().into());
        output
    }
    const NAME: &'static str = "LatentComposite";
    const DISPLAY_NAME: &'static str = "Latent Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**LatentCompositeMasked**: No description.
#[derive(Clone)]
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
> LatentCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
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
    Destination: crate::nodes::types::Latent,
    Source: crate::nodes::types::Latent,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    ResizeSource: crate::nodes::types::Boolean,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for LatentCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    const NAME: &'static str = "LatentCompositeMasked";
    const DISPLAY_NAME: &'static str = "LatentCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent**: No description.
#[derive(Clone)]
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
> LatentUpscale<Samples, UpscaleMethod, Width, Height, Crop> {
    /// Create a new node.
    pub fn new(
        samples: Samples,
        upscale_method: UpscaleMethod,
        width: Width,
        height: Height,
        crop: Crop,
    ) -> Self {
        Self {
            samples,
            upscale_method,
            width,
            height,
            crop,
        }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode
for LatentUpscale<Samples, UpscaleMethod, Width, Height, Crop> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("crop".to_string(), self.crop.clone().into());
        output
    }
    const NAME: &'static str = "LatentUpscale";
    const DISPLAY_NAME: &'static str = "Upscale Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent By**: No description.
#[derive(Clone)]
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
> LatentUpscaleBy<Samples, UpscaleMethod, ScaleBy> {
    /// Create a new node.
    pub fn new(
        samples: Samples,
        upscale_method: UpscaleMethod,
        scale_by: ScaleBy,
    ) -> Self {
        Self {
            samples,
            upscale_method,
            scale_by,
        }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    UpscaleMethod: crate::nodes::types::String,
    ScaleBy: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentUpscaleBy<Samples, UpscaleMethod, ScaleBy> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output.insert("scale_by".to_string(), self.scale_by.clone().into());
        output
    }
    const NAME: &'static str = "LatentUpscaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Latent By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**VAE Decode**: Decodes latent images back into pixel space images.
#[derive(Clone)]
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
> VaeDecode<Samples, Vae> {
    /// Create a new node.
    pub fn new(samples: Samples, vae: Vae) -> Self {
        Self { samples, vae }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeDecode<Samples, Vae> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "VAEDecode";
    const DISPLAY_NAME: &'static str = "VAE Decode";
    const DESCRIPTION: &'static str = "Decodes latent images back into pixel space images.";
    const CATEGORY: &'static str = "latent";
}
///**VAE Encode**: No description.
#[derive(Clone)]
pub struct VaeEncode<Pixels: crate::nodes::types::Image, Vae: crate::nodes::types::Vae> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
> VaeEncode<Pixels, Vae> {
    /// Create a new node.
    pub fn new(pixels: Pixels, vae: Vae) -> Self {
        Self { pixels, vae }
    }
}
impl<
    Pixels: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeEncode<Pixels, Vae> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("pixels".to_string(), self.pixels.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "VAEEncode";
    const DISPLAY_NAME: &'static str = "VAE Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
