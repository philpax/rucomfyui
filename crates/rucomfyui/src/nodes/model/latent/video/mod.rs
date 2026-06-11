//!`video` definitions/categories.
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
pub mod ltxv;
///**EmptyARVideoLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyARVideoLatent<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 1024
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyARVideoLatent<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyARVideoLatent<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyARVideoLatent";
    const DISPLAY_NAME: &'static str = "EmptyARVideoLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**EmptyCosmosLatentVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyCosmosLatentVideo<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1280
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 704
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 121
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyCosmosLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyCosmosLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyCosmosLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyCosmosLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**Empty HunyuanVideo 1.0 Latent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyHunyuanLatentVideo<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyHunyuanLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyHunyuanLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyHunyuanLatentVideo";
    const DISPLAY_NAME: &'static str = "Empty HunyuanVideo 1.0 Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**Empty HunyuanVideo 1.5 Latent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyHunyuanVideo15Latent<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyHunyuanVideo15Latent<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyHunyuanVideo15Latent<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyHunyuanVideo15Latent";
    const DISPLAY_NAME: &'static str = "Empty HunyuanVideo 1.5 Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**EmptyMochiLatentVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyMochiLatentVideo<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 7
  - Step: 6
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyMochiLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyMochiLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyMochiLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyMochiLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**LTXVLatentUpsampler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVLatentUpsampler<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleModelParam: crate::nodes::types::LatentUpscaleModel,
    VaeParam: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub upscale_model: UpscaleModelParam,
    ///No documentation.
    pub vae: VaeParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleModelParam: crate::nodes::types::LatentUpscaleModel,
    VaeParam: crate::nodes::types::Vae,
> LTXVLatentUpsampler<SamplesParam, UpscaleModelParam, VaeParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        upscale_model: UpscaleModelParam,
        vae: VaeParam,
    ) -> Self {
        Self {
            samples,
            upscale_model,
            vae,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    UpscaleModelParam: crate::nodes::types::LatentUpscaleModel,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode
for LTXVLatentUpsampler<SamplesParam, UpscaleModelParam, VaeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("upscale_model".to_string(), self.upscale_model.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "LTXVLatentUpsampler";
    const DISPLAY_NAME: &'static str = "LTXVLatentUpsampler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**TrimVideoLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TrimVideoLatent<
    SamplesParam: crate::nodes::types::Latent,
    TrimAmountParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 99999
  - Min: 0
*/
    pub trim_amount: TrimAmountParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    TrimAmountParam: crate::nodes::types::Int,
> TrimVideoLatent<SamplesParam, TrimAmountParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, trim_amount: TrimAmountParam) -> Self {
        Self { samples, trim_amount }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    TrimAmountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for TrimVideoLatent<SamplesParam, TrimAmountParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("trim_amount".to_string(), self.trim_amount.clone().into());
        output
    }
    const NAME: &'static str = "TrimVideoLatent";
    const DISPLAY_NAME: &'static str = "TrimVideoLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
///**VOIDWarpedNoise**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VOIDWarpedNoise<
    OpticalFlowParam: crate::nodes::types::OpticalFlow,
    VideoParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///Optical flow model from OpticalFlowLoader (RAFT-large).
    pub optical_flow: OpticalFlowParam,
    ///Pass 1 output video frames \[T, H, W, 3\]
    pub video: VideoParam,
    /**No documentation.

**Metadata**:
  - Default: 672
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 384
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**Number of pixel frames. Rounded down to make latent_t even (patch_size_t=2 requirement), e.g. 49 → 45.

**Metadata**:
  - Default: 45
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    OpticalFlowParam: crate::nodes::types::OpticalFlow,
    VideoParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> VOIDWarpedNoise<
    OpticalFlowParam,
    VideoParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        optical_flow: OpticalFlowParam,
        video: VideoParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            optical_flow,
            video,
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    OpticalFlowParam: crate::nodes::types::OpticalFlow,
    VideoParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VOIDWarpedNoise<
    OpticalFlowParam,
    VideoParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("optical_flow".to_string(), self.optical_flow.clone().into());
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "VOIDWarpedNoise";
    const DISPLAY_NAME: &'static str = "VOIDWarpedNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video";
}
