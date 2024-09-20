//!custom_sampling
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod guiders;
pub mod noise;
pub mod samplers;
pub mod schedulers;
pub mod sigmas;
///**SamplerCustom**
pub struct SamplerCustom<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::Boolean,
    NoiseSeed: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub add_noise: AddNoise,
    ///No documentation.
    pub noise_seed: NoiseSeed,
    ///No documentation.
    pub cfg: Cfg,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub sampler: Sampler,
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub latent_image: LatentImage,
}
///Output for [`SamplerCustom`].
#[derive(Clone)]
pub struct SamplerCustomOutput {
    ///No documentation.
    pub output: crate::nodes::LatentOut,
    ///No documentation.
    pub denoised_output: crate::nodes::LatentOut,
}
impl<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::Boolean,
    NoiseSeed: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> crate::nodes::TypedNode
for SamplerCustom<
    Model,
    AddNoise,
    NoiseSeed,
    Cfg,
    Positive,
    Negative,
    Sampler,
    Sigmas,
    LatentImage,
> {
    type Output = SamplerCustomOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            output: crate::nodes::LatentOut {
                node_id,
                slot: 0u32,
            },
            denoised_output: crate::nodes::LatentOut {
                node_id,
                slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "SamplerCustom";
    const DISPLAY_NAME: &'static str = "SamplerCustom";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
///**SamplerCustomAdvanced**
pub struct SamplerCustomAdvanced<
    Noise: crate::nodes::Noise,
    Guider: crate::nodes::Guider,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> {
    ///No documentation.
    pub noise: Noise,
    ///No documentation.
    pub guider: Guider,
    ///No documentation.
    pub sampler: Sampler,
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub latent_image: LatentImage,
}
///Output for [`SamplerCustomAdvanced`].
#[derive(Clone)]
pub struct SamplerCustomAdvancedOutput {
    ///No documentation.
    pub output: crate::nodes::LatentOut,
    ///No documentation.
    pub denoised_output: crate::nodes::LatentOut,
}
impl<
    Noise: crate::nodes::Noise,
    Guider: crate::nodes::Guider,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> crate::nodes::TypedNode
for SamplerCustomAdvanced<Noise, Guider, Sampler, Sigmas, LatentImage> {
    type Output = SamplerCustomAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            output: crate::nodes::LatentOut {
                node_id,
                slot: 0u32,
            },
            denoised_output: crate::nodes::LatentOut {
                node_id,
                slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "SamplerCustomAdvanced";
    const DISPLAY_NAME: &'static str = "SamplerCustomAdvanced";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
