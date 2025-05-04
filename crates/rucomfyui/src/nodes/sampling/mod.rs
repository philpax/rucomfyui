//!`sampling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod custom_sampling;
pub mod video_models;
///**KSampler**: Uses the provided model, positive and negative conditioning to denoise the latent image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KSampler<
    ModelParam: crate::nodes::types::Model,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    DenoiseParam: crate::nodes::types::Float,
> {
    ///The model used for denoising the input latent.
    pub model: ModelParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**The number of steps used in the denoising process.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**The Classifier-Free Guidance scale balances creativity and adherence to the prompt. Higher values result in images more closely matching the prompt however too high values will negatively impact quality.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
    ///The algorithm used when sampling, this can affect the quality, speed, and style of the generated output.
    pub sampler_name: SamplerNameParam,
    ///The scheduler controls how noise is gradually removed to form the image.
    pub scheduler: SchedulerParam,
    ///The conditioning describing the attributes you want to include in the image.
    pub positive: PositiveParam,
    ///The conditioning describing the attributes you want to exclude from the image.
    pub negative: NegativeParam,
    ///The latent image to denoise.
    pub latent_image: LatentImageParam,
    /**The amount of denoising applied, lower values will maintain the structure of the initial image allowing for image to image sampling.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    DenoiseParam: crate::nodes::types::Float,
> KSampler<
    ModelParam,
    SeedParam,
    StepsParam,
    CfgParam,
    SamplerNameParam,
    SchedulerParam,
    PositiveParam,
    NegativeParam,
    LatentImageParam,
    DenoiseParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        seed: SeedParam,
        steps: StepsParam,
        cfg: CfgParam,
        sampler_name: SamplerNameParam,
        scheduler: SchedulerParam,
        positive: PositiveParam,
        negative: NegativeParam,
        latent_image: LatentImageParam,
        denoise: DenoiseParam,
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
    ModelParam: crate::nodes::types::Model,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KSampler<
    ModelParam,
    SeedParam,
    StepsParam,
    CfgParam,
    SamplerNameParam,
    SchedulerParam,
    PositiveParam,
    NegativeParam,
    LatentImageParam,
    DenoiseParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct KSamplerAdvanced<
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::String,
    NoiseSeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    StartAtStepParam: crate::nodes::types::Int,
    EndAtStepParam: crate::nodes::types::Int,
    ReturnWithLeftoverNoiseParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub add_noise: AddNoiseParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub noise_seed: NoiseSeedParam,
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
    ///No documentation.
    pub sampler_name: SamplerNameParam,
    ///No documentation.
    pub scheduler: SchedulerParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent_image: LatentImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10000
  - Min: 0
*/
    pub start_at_step: StartAtStepParam,
    /**No documentation.

**Metadata**:
  - Default: 10000
  - Max: 10000
  - Min: 0
*/
    pub end_at_step: EndAtStepParam,
    ///No documentation.
    pub return_with_leftover_noise: ReturnWithLeftoverNoiseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::String,
    NoiseSeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    StartAtStepParam: crate::nodes::types::Int,
    EndAtStepParam: crate::nodes::types::Int,
    ReturnWithLeftoverNoiseParam: crate::nodes::types::String,
> KSamplerAdvanced<
    ModelParam,
    AddNoiseParam,
    NoiseSeedParam,
    StepsParam,
    CfgParam,
    SamplerNameParam,
    SchedulerParam,
    PositiveParam,
    NegativeParam,
    LatentImageParam,
    StartAtStepParam,
    EndAtStepParam,
    ReturnWithLeftoverNoiseParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        add_noise: AddNoiseParam,
        noise_seed: NoiseSeedParam,
        steps: StepsParam,
        cfg: CfgParam,
        sampler_name: SamplerNameParam,
        scheduler: SchedulerParam,
        positive: PositiveParam,
        negative: NegativeParam,
        latent_image: LatentImageParam,
        start_at_step: StartAtStepParam,
        end_at_step: EndAtStepParam,
        return_with_leftover_noise: ReturnWithLeftoverNoiseParam,
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
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::String,
    NoiseSeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    SamplerNameParam: crate::nodes::types::String,
    SchedulerParam: crate::nodes::types::String,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentImageParam: crate::nodes::types::Latent,
    StartAtStepParam: crate::nodes::types::Int,
    EndAtStepParam: crate::nodes::types::Int,
    ReturnWithLeftoverNoiseParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for KSamplerAdvanced<
    ModelParam,
    AddNoiseParam,
    NoiseSeedParam,
    StepsParam,
    CfgParam,
    SamplerNameParam,
    SchedulerParam,
    PositiveParam,
    NegativeParam,
    LatentImageParam,
    StartAtStepParam,
    EndAtStepParam,
    ReturnWithLeftoverNoiseParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
