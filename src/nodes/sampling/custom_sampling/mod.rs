//!`custom_sampling` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
            output: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
            denoised_output: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("add_noise".to_string(), self.add_noise.to_workflow_input());
        output.insert("noise_seed".to_string(), self.noise_seed.to_workflow_input());
        output.insert("cfg".to_string(), self.cfg.to_workflow_input());
        output.insert("positive".to_string(), self.positive.to_workflow_input());
        output.insert("negative".to_string(), self.negative.to_workflow_input());
        output.insert("sampler".to_string(), self.sampler.to_workflow_input());
        output.insert("sigmas".to_string(), self.sigmas.to_workflow_input());
        output.insert("latent_image".to_string(), self.latent_image.to_workflow_input());
        output
    }
    const NAME: &'static str = "SamplerCustom";
    const DISPLAY_NAME: &'static str = "SamplerCustom";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
///**SamplerCustomAdvanced**: No description.
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
> crate::nodes::TypedNode
for SamplerCustomAdvanced<Noise, Guider, Sampler, Sigmas, LatentImage> {
    type Output = out::SamplerCustomAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            output: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
            denoised_output: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("noise".to_string(), self.noise.to_workflow_input());
        output.insert("guider".to_string(), self.guider.to_workflow_input());
        output.insert("sampler".to_string(), self.sampler.to_workflow_input());
        output.insert("sigmas".to_string(), self.sigmas.to_workflow_input());
        output.insert("latent_image".to_string(), self.latent_image.to_workflow_input());
        output
    }
    const NAME: &'static str = "SamplerCustomAdvanced";
    const DISPLAY_NAME: &'static str = "SamplerCustomAdvanced";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
