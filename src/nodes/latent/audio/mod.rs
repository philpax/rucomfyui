//!audio
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**EmptyLatentAudio**
pub struct EmptyLatentAudio<Seconds: crate::nodes::Float> {
    ///No documentation.
    pub seconds: Seconds,
}
///Output for [`EmptyLatentAudio`].
#[derive(Clone)]
pub struct EmptyLatentAudioOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<Seconds: crate::nodes::Float> crate::nodes::TypedNode
for EmptyLatentAudio<Seconds> {
    type Output = EmptyLatentAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "EmptyLatentAudio";
    const DISPLAY_NAME: &'static str = "EmptyLatentAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEDecodeAudio**
pub struct VaeDecodeAudio<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub vae: Vae,
}
///Output for [`VaeDecodeAudio`].
#[derive(Clone)]
pub struct VaeDecodeAudioOutput {
    ///No documentation.
    pub audio: crate::nodes::AudioOut,
}
impl<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> crate::nodes::TypedNode
for VaeDecodeAudio<Samples, Vae> {
    type Output = VaeDecodeAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            audio: crate::nodes::AudioOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEDecodeAudio";
    const DISPLAY_NAME: &'static str = "VAEDecodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEEncodeAudio**
pub struct VaeEncodeAudio<Audio: crate::nodes::Audio, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub vae: Vae,
}
///Output for [`VaeEncodeAudio`].
#[derive(Clone)]
pub struct VaeEncodeAudioOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<Audio: crate::nodes::Audio, Vae: crate::nodes::Vae> crate::nodes::TypedNode
for VaeEncodeAudio<Audio, Vae> {
    type Output = VaeEncodeAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEEncodeAudio";
    const DISPLAY_NAME: &'static str = "VAEEncodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
