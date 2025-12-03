//!`latent` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod n_3_d;
#[rustfmt::skip]
pub mod advanced;
#[rustfmt::skip]
pub mod audio;
#[rustfmt::skip]
pub mod batch;
#[rustfmt::skip]
pub mod chroma_radiance;
#[rustfmt::skip]
pub mod inpaint;
#[rustfmt::skip]
pub mod sd_3;
#[rustfmt::skip]
pub mod stable_cascade;
#[rustfmt::skip]
pub mod transform;
#[rustfmt::skip]
pub mod video;
///**EmptyHunyuanImageLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyHunyuanImageLatent<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 2048
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 2048
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyHunyuanImageLatent<WidthParam, HeightParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self { width, height, batch_size }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyHunyuanImageLatent<WidthParam, HeightParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyHunyuanImageLatent";
    const DISPLAY_NAME: &'static str = "EmptyHunyuanImageLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Empty Latent Image**: Create a new batch of empty latent images to be denoised via sampling.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyLatentImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**The width of the latent images in pixels.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**The height of the latent images in pixels.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyLatentImage<WidthParam, HeightParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self { width, height, batch_size }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentImage<WidthParam, HeightParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct LatentComposite<
    SamplesToParam: crate::nodes::types::Latent,
    SamplesFromParam: crate::nodes::types::Latent,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    FeatherParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples_to: SamplesToParam,
    ///No documentation.
    pub samples_from: SamplesFromParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub feather: FeatherParam,
}
impl<
    SamplesToParam: crate::nodes::types::Latent,
    SamplesFromParam: crate::nodes::types::Latent,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    FeatherParam: crate::nodes::types::Int,
> LatentComposite<SamplesToParam, SamplesFromParam, XParam, YParam, FeatherParam> {
    /// Create a new node.
    pub fn new(
        samples_to: SamplesToParam,
        samples_from: SamplesFromParam,
        x: XParam,
        y: YParam,
        feather: FeatherParam,
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
    SamplesToParam: crate::nodes::types::Latent,
    SamplesFromParam: crate::nodes::types::Latent,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    FeatherParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for LatentComposite<SamplesToParam, SamplesFromParam, XParam, YParam, FeatherParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct LatentCompositeMasked<
    DestinationParam: crate::nodes::types::Latent,
    SourceParam: crate::nodes::types::Latent,
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
  - Step: 8
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
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
    DestinationParam: crate::nodes::types::Latent,
    SourceParam: crate::nodes::types::Latent,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> LatentCompositeMasked<
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
    DestinationParam: crate::nodes::types::Latent,
    SourceParam: crate::nodes::types::Latent,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for LatentCompositeMasked<
    DestinationParam,
    SourceParam,
    XParam,
    YParam,
    ResizeSourceParam,
    MaskParam,
> {
    type Output = crate::nodes::types::LatentOut;
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
    const NAME: &'static str = "LatentCompositeMasked";
    const DISPLAY_NAME: &'static str = "LatentCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LatentUpscale<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub height: HeightParam,
    ///No documentation.
    pub crop: CropParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> LatentUpscale<SamplesParam, UpscaleMethodParam, WidthParam, HeightParam, CropParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        upscale_method: UpscaleMethodParam,
        width: WidthParam,
        height: HeightParam,
        crop: CropParam,
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
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for LatentUpscale<SamplesParam, UpscaleMethodParam, WidthParam, HeightParam, CropParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct LatentUpscaleBy<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
    /**No documentation.

**Metadata**:
  - Default: 1.5
  - Max: 8
  - Min: 0.01
  - Step: 0.01
*/
    pub scale_by: ScaleByParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> LatentUpscaleBy<SamplesParam, UpscaleMethodParam, ScaleByParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        upscale_method: UpscaleMethodParam,
        scale_by: ScaleByParam,
    ) -> Self {
        Self {
            samples,
            upscale_method,
            scale_by,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleMethodParam: crate::nodes::types::String,
    ScaleByParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LatentUpscaleBy<SamplesParam, UpscaleMethodParam, ScaleByParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct VAEDecode<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> {
    ///The latent to be decoded.
    pub samples: SamplesParam,
    ///The VAE model used for decoding the latent.
    pub vae: VaeParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> VAEDecode<SamplesParam, VaeParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, vae: VaeParam) -> Self {
        Self { samples, vae }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VAEDecode<SamplesParam, VaeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct VAEEncode<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub pixels: PixelsParam,
    ///No documentation.
    pub vae: VaeParam,
}
impl<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> VAEEncode<PixelsParam, VaeParam> {
    /// Create a new node.
    pub fn new(pixels: PixelsParam, vae: VaeParam) -> Self {
        Self { pixels, vae }
    }
}
impl<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VAEEncode<PixelsParam, VaeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
