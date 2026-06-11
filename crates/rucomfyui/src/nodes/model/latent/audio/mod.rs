//!`audio` definitions/categories.
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
///**Empty Ace Step 1.5 Latent Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyAceStep1_5LatentAudio<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 120
  - Max: 1000
  - Min: 1
  - Step: 0.01
*/
    pub seconds: SecondsParam,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyAceStep1_5LatentAudio<SecondsParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(seconds: SecondsParam, batch_size: BatchSizeParam) -> Self {
        Self { seconds, batch_size }
    }
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyAceStep1_5LatentAudio<SecondsParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seconds".to_string(), self.seconds.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyAceStep1.5LatentAudio";
    const DISPLAY_NAME: &'static str = "Empty Ace Step 1.5 Latent Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**Empty Ace Step 1.0 Latent Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyAceStepLatentAudio<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 120
  - Max: 1000
  - Min: 1
  - Step: 0.1
*/
    pub seconds: SecondsParam,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyAceStepLatentAudio<SecondsParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(seconds: SecondsParam, batch_size: BatchSizeParam) -> Self {
        Self { seconds, batch_size }
    }
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyAceStepLatentAudio<SecondsParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seconds".to_string(), self.seconds.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyAceStepLatentAudio";
    const DISPLAY_NAME: &'static str = "Empty Ace Step 1.0 Latent Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**Empty Latent Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyLatentAudio<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 47.6
  - Max: 1000
  - Min: 1
  - Step: 0.1
*/
    pub seconds: SecondsParam,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyLatentAudio<SecondsParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(seconds: SecondsParam, batch_size: BatchSizeParam) -> Self {
        Self { seconds, batch_size }
    }
}
impl<
    SecondsParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentAudio<SecondsParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seconds".to_string(), self.seconds.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyLatentAudio";
    const DISPLAY_NAME: &'static str = "Empty Latent Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**LTXV Audio VAE Decode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVAudioVAEDecode<
    SamplesParam: crate::nodes::types::Latent,
    AudioVaeParam: crate::nodes::types::Vae,
> {
    ///The latent to be decoded.
    pub samples: SamplesParam,
    ///The Audio VAE model used for decoding the latent.
    pub audio_vae: AudioVaeParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    AudioVaeParam: crate::nodes::types::Vae,
> LTXVAudioVAEDecode<SamplesParam, AudioVaeParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, audio_vae: AudioVaeParam) -> Self {
        Self { samples, audio_vae }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    AudioVaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for LTXVAudioVAEDecode<SamplesParam, AudioVaeParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("audio_vae".to_string(), self.audio_vae.clone().into());
        output
    }
    const NAME: &'static str = "LTXVAudioVAEDecode";
    const DISPLAY_NAME: &'static str = "LTXV Audio VAE Decode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**LTXV Audio VAE Encode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVAudioVAEEncode<
    AudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
> {
    ///The audio to be encoded.
    pub audio: AudioParam,
    ///The Audio VAE model to use for encoding.
    pub audio_vae: AudioVaeParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
> LTXVAudioVAEEncode<AudioParam, AudioVaeParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, audio_vae: AudioVaeParam) -> Self {
        Self { audio, audio_vae }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for LTXVAudioVAEEncode<AudioParam, AudioVaeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("audio_vae".to_string(), self.audio_vae.clone().into());
        output
    }
    const NAME: &'static str = "LTXVAudioVAEEncode";
    const DISPLAY_NAME: &'static str = "LTXV Audio VAE Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**LTXV Empty Latent Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVEmptyLatentAudio<
    FramesNumberParam: crate::nodes::types::Int,
    FrameRateParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioVaeParam: crate::nodes::types::Vae,
> {
    /**Number of frames.

**Metadata**:
  - Default: 97
  - Display: number
  - Max: 1000
  - Min: 1
  - Step: 1
*/
    pub frames_number: FramesNumberParam,
    /**Number of frames per second.

**Metadata**:
  - Default: 25
  - Display: number
  - Max: 1000
  - Min: 1
  - Step: 1
*/
    pub frame_rate: FrameRateParam,
    /**The number of latent audio samples in the batch.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///The Audio VAE model to get configuration from.
    pub audio_vae: AudioVaeParam,
}
impl<
    FramesNumberParam: crate::nodes::types::Int,
    FrameRateParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioVaeParam: crate::nodes::types::Vae,
> LTXVEmptyLatentAudio<
    FramesNumberParam,
    FrameRateParam,
    BatchSizeParam,
    AudioVaeParam,
> {
    /// Create a new node.
    pub fn new(
        frames_number: FramesNumberParam,
        frame_rate: FrameRateParam,
        batch_size: BatchSizeParam,
        audio_vae: AudioVaeParam,
    ) -> Self {
        Self {
            frames_number,
            frame_rate,
            batch_size,
            audio_vae,
        }
    }
}
impl<
    FramesNumberParam: crate::nodes::types::Int,
    FrameRateParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioVaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode
for LTXVEmptyLatentAudio<
    FramesNumberParam,
    FrameRateParam,
    BatchSizeParam,
    AudioVaeParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("frames_number".to_string(), self.frames_number.clone().into());
        output.insert("frame_rate".to_string(), self.frame_rate.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("audio_vae".to_string(), self.audio_vae.clone().into());
        output
    }
    const NAME: &'static str = "LTXVEmptyLatentAudio";
    const DISPLAY_NAME: &'static str = "LTXV Empty Latent Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**VAE Decode Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEDecodeAudio<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub vae: VaeParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> VAEDecodeAudio<SamplesParam, VaeParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, vae: VaeParam) -> Self {
        Self { samples, vae }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VAEDecodeAudio<SamplesParam, VaeParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "VAEDecodeAudio";
    const DISPLAY_NAME: &'static str = "VAE Decode Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**VAE Decode Audio (Tiled)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEDecodeAudioTiled<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 32
  - Step: 8
*/
    pub tile_size: TileSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 1024
  - Min: 0
  - Step: 8
*/
    pub overlap: OverlapParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> VAEDecodeAudioTiled<SamplesParam, VaeParam, TileSizeParam, OverlapParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        vae: VaeParam,
        tile_size: TileSizeParam,
        overlap: OverlapParam,
    ) -> Self {
        Self {
            samples,
            vae,
            tile_size,
            overlap,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VAEDecodeAudioTiled<SamplesParam, VaeParam, TileSizeParam, OverlapParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("tile_size".to_string(), self.tile_size.clone().into());
        output.insert("overlap".to_string(), self.overlap.clone().into());
        output
    }
    const NAME: &'static str = "VAEDecodeAudioTiled";
    const DISPLAY_NAME: &'static str = "VAE Decode Audio (Tiled)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
///**VAE Encode Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEEncodeAudio<
    AudioParam: crate::nodes::types::Audio,
    VaeParam: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub audio: AudioParam,
    ///No documentation.
    pub vae: VaeParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VaeParam: crate::nodes::types::Vae,
> VAEEncodeAudio<AudioParam, VaeParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, vae: VaeParam) -> Self {
        Self { audio, vae }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VAEEncodeAudio<AudioParam, VaeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "VAEEncodeAudio";
    const DISPLAY_NAME: &'static str = "VAE Encode Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/audio";
}
