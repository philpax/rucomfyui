//!`audio` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**EmptyLatentAudio**: No description.
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
    const DISPLAY_NAME: &'static str = "EmptyLatentAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEDecodeAudio**: No description.
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
    const DISPLAY_NAME: &'static str = "VAEDecodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEEncodeAudio**: No description.
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
    const DISPLAY_NAME: &'static str = "VAEEncodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
