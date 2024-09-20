//!`sampling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod custom_sampling;
pub mod video_models;
///**KSampler**: Uses the provided model, positive and negative conditioning to denoise the latent image.
#[derive(Clone)]
pub struct KSampler<
    Model: crate::nodes::types::Model,
    Seed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    Denoise: crate::nodes::types::Float,
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
impl<
    Model: crate::nodes::types::Model,
    Seed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    Denoise: crate::nodes::types::Float,
> KSampler<
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
    /// Create a new node.
    pub fn new(
        model: Model,
        seed: Seed,
        steps: Steps,
        cfg: Cfg,
        sampler_name: SamplerName,
        scheduler: Scheduler,
        positive: Positive,
        negative: Negative,
        latent_image: LatentImage,
        denoise: Denoise,
    ) -> Self {
        Self {
            model,
            seed,
            steps,
            cfg,
            sampler_name,
            scheduler,
            positive,
            negative,
            latent_image,
            denoise,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Seed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    Denoise: crate::nodes::types::Float,
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
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output.insert("sampler_name".to_string(), self.sampler_name.clone().into());
        output.insert("scheduler".to_string(), self.scheduler.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("latent_image".to_string(), self.latent_image.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "KSampler";
    const DISPLAY_NAME: &'static str = "KSampler";
    const DESCRIPTION: &'static str = "Uses the provided model, positive and negative conditioning to denoise the latent image.";
    const CATEGORY: &'static str = "sampling";
}
///**KSampler (Advanced)**: No description.
#[derive(Clone)]
pub struct KSamplerAdvanced<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::String,
    NoiseSeed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    StartAtStep: crate::nodes::types::Int,
    EndAtStep: crate::nodes::types::Int,
    ReturnWithLeftoverNoise: crate::nodes::types::String,
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
impl<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::String,
    NoiseSeed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    StartAtStep: crate::nodes::types::Int,
    EndAtStep: crate::nodes::types::Int,
    ReturnWithLeftoverNoise: crate::nodes::types::String,
> KSamplerAdvanced<
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
    /// Create a new node.
    pub fn new(
        model: Model,
        add_noise: AddNoise,
        noise_seed: NoiseSeed,
        steps: Steps,
        cfg: Cfg,
        sampler_name: SamplerName,
        scheduler: Scheduler,
        positive: Positive,
        negative: Negative,
        latent_image: LatentImage,
        start_at_step: StartAtStep,
        end_at_step: EndAtStep,
        return_with_leftover_noise: ReturnWithLeftoverNoise,
    ) -> Self {
        Self {
            model,
            add_noise,
            noise_seed,
            steps,
            cfg,
            sampler_name,
            scheduler,
            positive,
            negative,
            latent_image,
            start_at_step,
            end_at_step,
            return_with_leftover_noise,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::String,
    NoiseSeed: crate::nodes::types::Int,
    Steps: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    SamplerName: crate::nodes::types::String,
    Scheduler: crate::nodes::types::String,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    LatentImage: crate::nodes::types::Latent,
    StartAtStep: crate::nodes::types::Int,
    EndAtStep: crate::nodes::types::Int,
    ReturnWithLeftoverNoise: crate::nodes::types::String,
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
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("add_noise".to_string(), self.add_noise.clone().into());
        output.insert("noise_seed".to_string(), self.noise_seed.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output.insert("sampler_name".to_string(), self.sampler_name.clone().into());
        output.insert("scheduler".to_string(), self.scheduler.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("latent_image".to_string(), self.latent_image.clone().into());
        output.insert("start_at_step".to_string(), self.start_at_step.clone().into());
        output.insert("end_at_step".to_string(), self.end_at_step.clone().into());
        output
            .insert(
                "return_with_leftover_noise".to_string(),
                self.return_with_leftover_noise.clone().into(),
            );
        output
    }
    const NAME: &'static str = "KSamplerAdvanced";
    const DISPLAY_NAME: &'static str = "KSampler (Advanced)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling";
}
