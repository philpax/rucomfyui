//!`audio` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`EmptyLatentAudio`](super::EmptyLatentAudio).
    #[derive(Clone)]
    pub struct EmptyLatentAudioOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`VaeDecodeAudio`](super::VaeDecodeAudio).
    #[derive(Clone)]
    pub struct VaeDecodeAudioOutput {
        ///No documentation.
        pub audio: crate::nodes::types::AudioOut,
    }
    ///Output for [`VaeEncodeAudio`](super::VaeEncodeAudio).
    #[derive(Clone)]
    pub struct VaeEncodeAudioOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**EmptyLatentAudio**: No description.
pub struct EmptyLatentAudio<Seconds: crate::nodes::types::Float> {
    ///No documentation.
    pub seconds: Seconds,
}
impl<Seconds: crate::nodes::types::Float> crate::nodes::TypedNode
for EmptyLatentAudio<Seconds> {
    type Output = out::EmptyLatentAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "EmptyLatentAudio";
    const DISPLAY_NAME: &'static str = "EmptyLatentAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEDecodeAudio**: No description.
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
> crate::nodes::TypedNode for VaeDecodeAudio<Samples, Vae> {
    type Output = out::VaeDecodeAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            audio: crate::nodes::types::AudioOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEDecodeAudio";
    const DISPLAY_NAME: &'static str = "VAEDecodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
///**VAEEncodeAudio**: No description.
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
> crate::nodes::TypedNode for VaeEncodeAudio<Audio, Vae> {
    type Output = out::VaeEncodeAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAEEncodeAudio";
    const DISPLAY_NAME: &'static str = "VAEEncodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/audio";
}
