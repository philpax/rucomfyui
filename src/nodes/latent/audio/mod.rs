//!`audio` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**EmptyLatentAudio**: No description.
#[derive(Clone)]
pub struct EmptyLatentAudio<
    Seconds: crate::nodes::types::Float,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub seconds: Seconds,
    ///The number of latent images in the batch.
    pub batch_size: BatchSize,
}
impl<
    Seconds: crate::nodes::types::Float,
    BatchSize: crate::nodes::types::Int,
> EmptyLatentAudio<Seconds, BatchSize> {
    /// Create a new node.
    pub fn new(seconds: Seconds, batch_size: BatchSize) -> Self {
        Self { seconds, batch_size }
    }
}
impl<
    Seconds: crate::nodes::types::Float,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentAudio<Seconds, BatchSize> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
pub struct VaeDecodeAudio<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub vae: Vae,
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> VaeDecodeAudio<Samples, Vae> {
    /// Create a new node.
    pub fn new(samples: Samples, vae: Vae) -> Self {
        Self { samples, vae }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeDecodeAudio<Samples, Vae> {
    type Output = crate::nodes::types::AudioOut;
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
    const NAME: &'static str = "VAEDecodeAudio";
    const DISPLAY_NAME: &'static str = "VAEDecodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEEncodeAudio**: No description.
#[derive(Clone)]
pub struct VaeEncodeAudio<
    Audio: crate::nodes::types::Audio,
    Vae: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub vae: Vae,
}
impl<
    Audio: crate::nodes::types::Audio,
    Vae: crate::nodes::types::Vae,
> VaeEncodeAudio<Audio, Vae> {
    /// Create a new node.
    pub fn new(audio: Audio, vae: Vae) -> Self {
        Self { audio, vae }
    }
}
impl<
    Audio: crate::nodes::types::Audio,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode for VaeEncodeAudio<Audio, Vae> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
