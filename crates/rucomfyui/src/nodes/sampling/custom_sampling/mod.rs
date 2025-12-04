//!`custom_sampling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod guiders;
#[rustfmt::skip]
pub mod noise;
#[rustfmt::skip]
pub mod samplers;
#[rustfmt::skip]
pub mod schedulers;
#[rustfmt::skip]
pub mod sigmas;
/// Output types for nodes.
pub mod out {
    ///Output for [`SamplerCustom`](super::SamplerCustom).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SamplerCustomOutput {
        ///No documentation.
        pub output: crate::nodes::types::LatentOut,
        ///No documentation.
        pub denoised_output: crate::nodes::types::LatentOut,
    }
    ///Output for [`SamplerCustomAdvanced`](super::SamplerCustomAdvanced).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SamplerCustomAdvancedOutput {
        ///No documentation.
        pub output: crate::nodes::types::LatentOut,
        ///No documentation.
        pub denoised_output: crate::nodes::types::LatentOut,
    }
}
///**Adaptive Projected Guidance**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct APG<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    NormThresholdParam: crate::nodes::types::Float,
    MomentumParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**Controls the scale of the parallel guidance vector. Default CFG behavior at a setting of 1.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub eta: EtaParam,
    /**Normalize guidance vector to this value, normalization disable at a setting of 0.

**Metadata**:
  - Default: 5
  - Max: 50
  - Min: 0
  - Step: 0.1
*/
    pub norm_threshold: NormThresholdParam,
    /**Controls a running average of guidance during diffusion, disabled at a setting of 0.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: -5
  - Step: 0.01
*/
    pub momentum: MomentumParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    NormThresholdParam: crate::nodes::types::Float,
    MomentumParam: crate::nodes::types::Float,
> APG<ModelParam, EtaParam, NormThresholdParam, MomentumParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        eta: EtaParam,
        norm_threshold: NormThresholdParam,
        momentum: MomentumParam,
    ) -> Self {
        Self {
            model,
            eta,
            norm_threshold,
            momentum,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    EtaParam: crate::nodes::types::Float,
    NormThresholdParam: crate::nodes::types::Float,
    MomentumParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for APG<ModelParam, EtaParam, NormThresholdParam, MomentumParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("eta".to_string(), self.eta.clone().into());
        output.insert("norm_threshold".to_string(), self.norm_threshold.clone().into());
        output.insert("momentum".to_string(), self.momentum.clone().into());
        output
    }
    const NAME: &'static str = "APG";
    const DISPLAY_NAME: &'static str = "Adaptive Projected Guidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
///**SamplerCustom**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerCustom<
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::Boolean,
    NoiseSeedParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
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
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub sampler: SamplerParam,
    ///No documentation.
    pub sigmas: SigmasParam,
    ///No documentation.
    pub latent_image: LatentImageParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::Boolean,
    NoiseSeedParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> SamplerCustom<
    ModelParam,
    AddNoiseParam,
    NoiseSeedParam,
    CfgParam,
    PositiveParam,
    NegativeParam,
    SamplerParam,
    SigmasParam,
    LatentImageParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        add_noise: AddNoiseParam,
        noise_seed: NoiseSeedParam,
        cfg: CfgParam,
        positive: PositiveParam,
        negative: NegativeParam,
        sampler: SamplerParam,
        sigmas: SigmasParam,
        latent_image: LatentImageParam,
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
    ModelParam: crate::nodes::types::Model,
    AddNoiseParam: crate::nodes::types::Boolean,
    NoiseSeedParam: crate::nodes::types::Int,
    CfgParam: crate::nodes::types::Float,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for SamplerCustom<
    ModelParam,
    AddNoiseParam,
    NoiseSeedParam,
    CfgParam,
    PositiveParam,
    NegativeParam,
    SamplerParam,
    SigmasParam,
    LatentImageParam,
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
#[allow(non_camel_case_types)]
pub struct SamplerCustomAdvanced<
    NoiseParam: crate::nodes::types::Noise,
    GuiderParam: crate::nodes::types::Guider,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub noise: NoiseParam,
    ///No documentation.
    pub guider: GuiderParam,
    ///No documentation.
    pub sampler: SamplerParam,
    ///No documentation.
    pub sigmas: SigmasParam,
    ///No documentation.
    pub latent_image: LatentImageParam,
}
impl<
    NoiseParam: crate::nodes::types::Noise,
    GuiderParam: crate::nodes::types::Guider,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> SamplerCustomAdvanced<
    NoiseParam,
    GuiderParam,
    SamplerParam,
    SigmasParam,
    LatentImageParam,
> {
    /// Create a new node.
    pub fn new(
        noise: NoiseParam,
        guider: GuiderParam,
        sampler: SamplerParam,
        sigmas: SigmasParam,
        latent_image: LatentImageParam,
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
    NoiseParam: crate::nodes::types::Noise,
    GuiderParam: crate::nodes::types::Guider,
    SamplerParam: crate::nodes::types::Sampler,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for SamplerCustomAdvanced<
    NoiseParam,
    GuiderParam,
    SamplerParam,
    SigmasParam,
    LatentImageParam,
> {
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
