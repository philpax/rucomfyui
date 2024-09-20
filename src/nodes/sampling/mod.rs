//!`sampling` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod custom_sampling;
pub mod video_models;
#[doc = "**KSampler**\n\nUses the provided model, positive and negative conditioning to denoise the latent image."]
pub struct KSampler<
    Model: crate::nodes::Model,
    Seed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    SamplerName: crate::nodes::String,
    Scheduler: crate::nodes::String,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    Denoise: crate::nodes::Float,
> {
    ///The model used for denoising the input latent.
    pub model: Model,
    ///The random seed used for creating the noise.
    pub seed: Seed,
    ///The number of steps used in the denoising process.
    pub steps: Steps,
    ///The Classifier-Free Guidance scale balances creativity and adherence to the prompt. Higher values result in images more closely matching the prompt however too high values will negatively impact quality.
    pub cfg: Cfg,
    ///The algorithm used when sampling, this can affect the quality, speed, and style of the generated output.
    pub sampler_name: SamplerName,
    ///The scheduler controls how noise is gradually removed to form the image.
    pub scheduler: Scheduler,
    ///The conditioning describing the attributes you want to include in the image.
    pub positive: Positive,
    ///The conditioning describing the attributes you want to exclude from the image.
    pub negative: Negative,
    ///The latent image to denoise.
    pub latent_image: LatentImage,
    ///The amount of denoising applied, lower values will maintain the structure of the initial image allowing for image to image sampling.
    pub denoise: Denoise,
}
///Output for [`KSampler`].
#[derive(Clone)]
pub struct KSamplerOutput {
    ///The denoised latent.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Model: crate::nodes::Model,
    Seed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    SamplerName: crate::nodes::String,
    Scheduler: crate::nodes::String,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    Denoise: crate::nodes::Float,
> crate::nodes::TypedNode
for KSampler<
    Model,
    Seed,
    Steps,
    Cfg,
    SamplerName,
    Scheduler,
    Positive,
    Negative,
    LatentImage,
    Denoise,
> {
    type Output = KSamplerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "KSampler";
    const DISPLAY_NAME: &'static str = "KSampler";
    const DESCRIPTION: &'static str = "Uses the provided model, positive and negative conditioning to denoise the latent image.";
    const CATEGORY: &'static str = "sampling";
}
///**KSampler (Advanced)**
pub struct KSamplerAdvanced<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::String,
    NoiseSeed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    SamplerName: crate::nodes::String,
    Scheduler: crate::nodes::String,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    StartAtStep: crate::nodes::Int,
    EndAtStep: crate::nodes::Int,
    ReturnWithLeftoverNoise: crate::nodes::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub add_noise: AddNoise,
    ///No documentation.
    pub noise_seed: NoiseSeed,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub cfg: Cfg,
    ///No documentation.
    pub sampler_name: SamplerName,
    ///No documentation.
    pub scheduler: Scheduler,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub latent_image: LatentImage,
    ///No documentation.
    pub start_at_step: StartAtStep,
    ///No documentation.
    pub end_at_step: EndAtStep,
    ///No documentation.
    pub return_with_leftover_noise: ReturnWithLeftoverNoise,
}
///Output for [`KSamplerAdvanced`].
#[derive(Clone)]
pub struct KSamplerAdvancedOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::String,
    NoiseSeed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    SamplerName: crate::nodes::String,
    Scheduler: crate::nodes::String,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    StartAtStep: crate::nodes::Int,
    EndAtStep: crate::nodes::Int,
    ReturnWithLeftoverNoise: crate::nodes::String,
> crate::nodes::TypedNode
for KSamplerAdvanced<
    Model,
    AddNoise,
    NoiseSeed,
    Steps,
    Cfg,
    SamplerName,
    Scheduler,
    Positive,
    Negative,
    LatentImage,
    StartAtStep,
    EndAtStep,
    ReturnWithLeftoverNoise,
> {
    type Output = KSamplerAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "KSamplerAdvanced";
    const DISPLAY_NAME: &'static str = "KSampler (Advanced)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling";
}
