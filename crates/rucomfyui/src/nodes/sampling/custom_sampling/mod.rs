//!`custom_sampling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod guiders;
pub mod noise;
pub mod samplers;
pub mod schedulers;
pub mod sigmas;
/// Output types for nodes.
pub mod out {
    ///Output for [`SamplerCustom`](super::SamplerCustom).
    #[derive(Clone)]
    pub struct SamplerCustomOutput {
        ///No documentation.
        pub output: crate::nodes::types::LatentOut,
        ///No documentation.
        pub denoised_output: crate::nodes::types::LatentOut,
    }
    ///Output for [`SamplerCustomAdvanced`](super::SamplerCustomAdvanced).
    #[derive(Clone)]
    pub struct SamplerCustomAdvancedOutput {
        ///No documentation.
        pub output: crate::nodes::types::LatentOut,
        ///No documentation.
        pub denoised_output: crate::nodes::types::LatentOut,
    }
}
///**SamplerCustom**: No description.
#[derive(Clone)]
pub struct SamplerCustom<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::Boolean,
    NoiseSeed: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub add_noise: AddNoise,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub noise_seed: NoiseSeed,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
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
impl<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::Boolean,
    NoiseSeed: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
> SamplerCustom<
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
    /// Create a new node.
    pub fn new(
        model: Model,
        add_noise: AddNoise,
        noise_seed: NoiseSeed,
        cfg: Cfg,
        positive: Positive,
        negative: Negative,
        sampler: Sampler,
        sigmas: Sigmas,
        latent_image: LatentImage,
    ) -> Self {
        Self {
            model,
            add_noise,
            noise_seed,
            cfg,
            positive,
            negative,
            sampler,
            sigmas,
            latent_image,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    AddNoise: crate::nodes::types::Boolean,
    NoiseSeed: crate::nodes::types::Int,
    Cfg: crate::nodes::types::Float,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
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
    type Output = out::SamplerCustomOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            output: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            denoised_output: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("add_noise".to_string(), self.add_noise.clone().into());
        output.insert("noise_seed".to_string(), self.noise_seed.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("sampler".to_string(), self.sampler.clone().into());
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("latent_image".to_string(), self.latent_image.clone().into());
        output
    }
    const NAME: &'static str = "SamplerCustom";
    const DISPLAY_NAME: &'static str = "SamplerCustom";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
///**SamplerCustomAdvanced**: No description.
#[derive(Clone)]
pub struct SamplerCustomAdvanced<
    Noise: crate::nodes::types::Noise,
    Guider: crate::nodes::types::Guider,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
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
impl<
    Noise: crate::nodes::types::Noise,
    Guider: crate::nodes::types::Guider,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
> SamplerCustomAdvanced<Noise, Guider, Sampler, Sigmas, LatentImage> {
    /// Create a new node.
    pub fn new(
        noise: Noise,
        guider: Guider,
        sampler: Sampler,
        sigmas: Sigmas,
        latent_image: LatentImage,
    ) -> Self {
        Self {
            noise,
            guider,
            sampler,
            sigmas,
            latent_image,
        }
    }
}
impl<
    Noise: crate::nodes::types::Noise,
    Guider: crate::nodes::types::Guider,
    Sampler: crate::nodes::types::Sampler,
    Sigmas: crate::nodes::types::Sigmas,
    LatentImage: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for SamplerCustomAdvanced<Noise, Guider, Sampler, Sigmas, LatentImage> {
    type Output = out::SamplerCustomAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            output: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            denoised_output: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("noise".to_string(), self.noise.clone().into());
        output.insert("guider".to_string(), self.guider.clone().into());
        output.insert("sampler".to_string(), self.sampler.clone().into());
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("latent_image".to_string(), self.latent_image.clone().into());
        output
    }
    const NAME: &'static str = "SamplerCustomAdvanced";
    const DISPLAY_NAME: &'static str = "SamplerCustomAdvanced";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
