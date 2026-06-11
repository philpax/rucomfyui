//!`experimental` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod attention_experiments;
#[rustfmt::skip]
pub mod conditioning;
#[rustfmt::skip]
pub mod custom_sampling;
#[rustfmt::skip]
pub mod photomaker;
#[rustfmt::skip]
pub mod stable_cascade;
///**Differential Diffusion**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DifferentialDiffusion<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: Option<StrengthParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
> DifferentialDiffusion<ModelParam, StrengthParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, strength: Option<StrengthParam>) -> Self {
        Self { model, strength }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for DifferentialDiffusion<ModelParam, StrengthParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        if let Some(v) = &self.strength {
            output.insert("strength".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "DifferentialDiffusion";
    const DISPLAY_NAME: &'static str = "Differential Diffusion";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**Flux KV Cache**: Enables KV Cache optimization for reference images on Flux family models.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxKVCache<ModelParam: crate::nodes::types::Model> {
    ///The model to use KV Cache on.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> FluxKVCache<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for FluxKVCache<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "FluxKVCache";
    const DISPLAY_NAME: &'static str = "Flux KV Cache";
    const DESCRIPTION: &'static str = "Enables KV Cache optimization for reference images on Flux family models.";
    const CATEGORY: &'static str = "experimental";
}
///**FreSca**: Applies frequency-dependent scaling to the guidance
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FreSca<
    ModelParam: crate::nodes::types::Model,
    ScaleLowParam: crate::nodes::types::Float,
    ScaleHighParam: crate::nodes::types::Float,
    FreqCutoffParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    /**Scaling factor for low-frequency components

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub scale_low: ScaleLowParam,
    /**Scaling factor for high-frequency components

**Metadata**:
  - Default: 1.25
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub scale_high: ScaleHighParam,
    /**Number of frequency indices around center to consider as low-frequency

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
  - Step: 1
*/
    pub freq_cutoff: FreqCutoffParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleLowParam: crate::nodes::types::Float,
    ScaleHighParam: crate::nodes::types::Float,
    FreqCutoffParam: crate::nodes::types::Int,
> FreSca<ModelParam, ScaleLowParam, ScaleHighParam, FreqCutoffParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        scale_low: ScaleLowParam,
        scale_high: ScaleHighParam,
        freq_cutoff: FreqCutoffParam,
    ) -> Self {
        Self {
            model,
            scale_low,
            scale_high,
            freq_cutoff,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleLowParam: crate::nodes::types::Float,
    ScaleHighParam: crate::nodes::types::Float,
    FreqCutoffParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for FreSca<ModelParam, ScaleLowParam, ScaleHighParam, FreqCutoffParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scale_low".to_string(), self.scale_low.clone().into());
        output.insert("scale_high".to_string(), self.scale_high.clone().into());
        output.insert("freq_cutoff".to_string(), self.freq_cutoff.clone().into());
        output
    }
    const NAME: &'static str = "FreSca";
    const DISPLAY_NAME: &'static str = "FreSca";
    const DESCRIPTION: &'static str = "Applies frequency-dependent scaling to the guidance";
    const CATEGORY: &'static str = "experimental";
}
///**Latent Blend**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LatentBlend<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    BlendFactorParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples_1: Samples1Param,
    ///No documentation.
    pub samples_2: Samples2Param,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blend_factor: BlendFactorParam,
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    BlendFactorParam: crate::nodes::types::Float,
> LatentBlend<Samples1Param, Samples2Param, BlendFactorParam> {
    /// Create a new node.
    pub fn new(
        samples_1: Samples1Param,
        samples_2: Samples2Param,
        blend_factor: BlendFactorParam,
    ) -> Self {
        Self {
            samples_1,
            samples_2,
            blend_factor,
        }
    }
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    BlendFactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LatentBlend<Samples1Param, Samples2Param, BlendFactorParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.clone().into());
        output.insert("samples2".to_string(), self.samples_2.clone().into());
        output.insert("blend_factor".to_string(), self.blend_factor.clone().into());
        output
    }
    const NAME: &'static str = "LatentBlend";
    const DISPLAY_NAME: &'static str = "Latent Blend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**LoadLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadLatent<LatentParam: crate::nodes::types::String> {
    ///No documentation.
    pub latent: LatentParam,
}
impl<LatentParam: crate::nodes::types::String> LoadLatent<LatentParam> {
    /// Create a new node.
    pub fn new(latent: LatentParam) -> Self {
        Self { latent }
    }
}
impl<LatentParam: crate::nodes::types::String> crate::nodes::TypedNode
for LoadLatent<LatentParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("latent".to_string(), self.latent.clone().into());
        output
    }
    const NAME: &'static str = "LoadLatent";
    const DISPLAY_NAME: &'static str = "LoadLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**Extract and Save Lora**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraSave<
    FilenamePrefixParam: crate::nodes::types::String,
    RankParam: crate::nodes::types::Int,
    BiasDiffParam: crate::nodes::types::Boolean,
    ModelDiffParam: crate::nodes::types::Model = crate::nodes::types::ModelOut,
    TextEncoderDiffParam: crate::nodes::types::Clip = crate::nodes::types::ClipOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: loras/ComfyUI_extracted_lora
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub rank: RankParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub bias_diff: BiasDiffParam,
    ///The ModelSubtract output to be converted to a lora.
    pub model_diff: Option<ModelDiffParam>,
    ///The CLIPSubtract output to be converted to a lora.
    pub text_encoder_diff: Option<TextEncoderDiffParam>,
}
impl<
    FilenamePrefixParam: crate::nodes::types::String,
    RankParam: crate::nodes::types::Int,
    BiasDiffParam: crate::nodes::types::Boolean,
    ModelDiffParam: crate::nodes::types::Model,
    TextEncoderDiffParam: crate::nodes::types::Clip,
> LoraSave<
    FilenamePrefixParam,
    RankParam,
    BiasDiffParam,
    ModelDiffParam,
    TextEncoderDiffParam,
> {
    /// Create a new node.
    pub fn new(
        filename_prefix: FilenamePrefixParam,
        rank: RankParam,
        bias_diff: BiasDiffParam,
        model_diff: Option<ModelDiffParam>,
        text_encoder_diff: Option<TextEncoderDiffParam>,
    ) -> Self {
        Self {
            filename_prefix,
            rank,
            bias_diff,
            model_diff,
            text_encoder_diff,
        }
    }
}
impl<
    FilenamePrefixParam: crate::nodes::types::String,
    RankParam: crate::nodes::types::Int,
    BiasDiffParam: crate::nodes::types::Boolean,
    ModelDiffParam: crate::nodes::types::Model,
    TextEncoderDiffParam: crate::nodes::types::Clip,
> crate::nodes::TypedNode
for LoraSave<
    FilenamePrefixParam,
    RankParam,
    BiasDiffParam,
    ModelDiffParam,
    TextEncoderDiffParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("rank".to_string(), self.rank.clone().into());
        output.insert("bias_diff".to_string(), self.bias_diff.clone().into());
        if let Some(v) = &self.model_diff {
            output.insert("model_diff".to_string(), v.clone().into());
        }
        if let Some(v) = &self.text_encoder_diff {
            output.insert("text_encoder_diff".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LoraSave";
    const DISPLAY_NAME: &'static str = "Extract and Save Lora";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
impl<
    FilenamePrefixParam: crate::nodes::types::String,
    RankParam: crate::nodes::types::Int,
    BiasDiffParam: crate::nodes::types::Boolean,
    ModelDiffParam: crate::nodes::types::Model,
    TextEncoderDiffParam: crate::nodes::types::Clip,
> crate::nodes::TypedOutputNode
for LoraSave<
    FilenamePrefixParam,
    RankParam,
    BiasDiffParam,
    ModelDiffParam,
    TextEncoderDiffParam,
> {}
///**Positive-Biased Guidance**: Modify the guidance to scale more on the 'direction' of the positive prompt rather than the difference between the negative prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Mahiro<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> Mahiro<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for Mahiro<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "Mahiro";
    const DISPLAY_NAME: &'static str = "Positive-Biased Guidance";
    const DESCRIPTION: &'static str = "Modify the guidance to scale more on the 'direction' of the positive prompt rather than the difference between the negative prompt.";
    const CATEGORY: &'static str = "experimental";
}
///**Perp-Neg (DEPRECATED by Perp-Neg Guider)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PerpNeg<
    ModelParam: crate::nodes::types::Model,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    NegScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub empty_conditioning: EmptyConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub neg_scale: NegScaleParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    NegScaleParam: crate::nodes::types::Float,
> PerpNeg<ModelParam, EmptyConditioningParam, NegScaleParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        empty_conditioning: EmptyConditioningParam,
        neg_scale: NegScaleParam,
    ) -> Self {
        Self {
            model,
            empty_conditioning,
            neg_scale,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    NegScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for PerpNeg<ModelParam, EmptyConditioningParam, NegScaleParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert(
                "empty_conditioning".to_string(),
                self.empty_conditioning.clone().into(),
            );
        output.insert("neg_scale".to_string(), self.neg_scale.clone().into());
        output
    }
    const NAME: &'static str = "PerpNeg";
    const DISPLAY_NAME: &'static str = "Perp-Neg (DEPRECATED by Perp-Neg Guider)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**Perp-Neg Guider**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PerpNegGuider<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    NegScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub empty_conditioning: EmptyConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub neg_scale: NegScaleParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    NegScaleParam: crate::nodes::types::Float,
> PerpNegGuider<
    ModelParam,
    PositiveParam,
    NegativeParam,
    EmptyConditioningParam,
    CfgParam,
    NegScaleParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        positive: PositiveParam,
        negative: NegativeParam,
        empty_conditioning: EmptyConditioningParam,
        cfg: CfgParam,
        neg_scale: NegScaleParam,
    ) -> Self {
        Self {
            model,
            positive,
            negative,
            empty_conditioning,
            cfg,
            neg_scale,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    EmptyConditioningParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    NegScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for PerpNegGuider<
    ModelParam,
    PositiveParam,
    NegativeParam,
    EmptyConditioningParam,
    CfgParam,
    NegScaleParam,
> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output
            .insert(
                "empty_conditioning".to_string(),
                self.empty_conditioning.clone().into(),
            );
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output.insert("neg_scale".to_string(), self.neg_scale.clone().into());
        output
    }
    const NAME: &'static str = "PerpNegGuider";
    const DISPLAY_NAME: &'static str = "Perp-Neg Guider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**SamplerEulerCFG++**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SamplerEulerCFGpp {}
impl SamplerEulerCFGpp {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for SamplerEulerCFGpp {
    type Output = crate::nodes::types::SamplerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "SamplerEulerCFGpp";
    const DISPLAY_NAME: &'static str = "SamplerEulerCFG++";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**SaveLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveLatent<
    SamplesParam: crate::nodes::types::Latent,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: latents/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveLatent<SamplesParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { samples, filename_prefix }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveLatent<SamplesParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveLatent";
    const DISPLAY_NAME: &'static str = "SaveLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveLatent<SamplesParam, FilenamePrefixParam> {}
///**Self-Attention Guidance**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SelfAttentionGuidance<
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
    BlurSigmaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 5
  - Min: -2
  - Step: 0.01
*/
    pub scale: ScaleParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub blur_sigma: BlurSigmaParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
    BlurSigmaParam: crate::nodes::types::Float,
> SelfAttentionGuidance<ModelParam, ScaleParam, BlurSigmaParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        scale: ScaleParam,
        blur_sigma: BlurSigmaParam,
    ) -> Self {
        Self { model, scale, blur_sigma }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
    BlurSigmaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SelfAttentionGuidance<ModelParam, ScaleParam, BlurSigmaParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scale".to_string(), self.scale.clone().into());
        output.insert("blur_sigma".to_string(), self.blur_sigma.clone().into());
        output
    }
    const NAME: &'static str = "SelfAttentionGuidance";
    const DISPLAY_NAME: &'static str = "Self-Attention Guidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**TorchCompileModel**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TorchCompileModel<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> TorchCompileModel<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for TorchCompileModel<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "TorchCompileModel";
    const DISPLAY_NAME: &'static str = "TorchCompileModel";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**VAE Decode (Tiled)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEDecodeTiled<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 32
*/
    pub tile_size: TileSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 4096
  - Min: 0
  - Step: 32
*/
    pub overlap: OverlapParam,
    /**Only used for video VAEs: Amount of frames to decode at a time.

**Metadata**:
  - Default: 64
  - Max: 4096
  - Min: 8
  - Step: 4
*/
    pub temporal_size: TemporalSizeParam,
    /**Only used for video VAEs: Amount of frames to overlap.

**Metadata**:
  - Default: 8
  - Max: 4096
  - Min: 4
  - Step: 4
*/
    pub temporal_overlap: TemporalOverlapParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> VAEDecodeTiled<
    SamplesParam,
    VaeParam,
    TileSizeParam,
    OverlapParam,
    TemporalSizeParam,
    TemporalOverlapParam,
> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        vae: VaeParam,
        tile_size: TileSizeParam,
        overlap: OverlapParam,
        temporal_size: TemporalSizeParam,
        temporal_overlap: TemporalOverlapParam,
    ) -> Self {
        Self {
            samples,
            vae,
            tile_size,
            overlap,
            temporal_size,
            temporal_overlap,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VAEDecodeTiled<
    SamplesParam,
    VaeParam,
    TileSizeParam,
    OverlapParam,
    TemporalSizeParam,
    TemporalOverlapParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("tile_size".to_string(), self.tile_size.clone().into());
        output.insert("overlap".to_string(), self.overlap.clone().into());
        output.insert("temporal_size".to_string(), self.temporal_size.clone().into());
        output
            .insert(
                "temporal_overlap".to_string(),
                self.temporal_overlap.clone().into(),
            );
        output
    }
    const NAME: &'static str = "VAEDecodeTiled";
    const DISPLAY_NAME: &'static str = "VAE Decode (Tiled)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
///**VAE Encode (Tiled)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEEncodeTiled<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub pixels: PixelsParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 4096
  - Min: 64
  - Step: 64
*/
    pub tile_size: TileSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 4096
  - Min: 0
  - Step: 32
*/
    pub overlap: OverlapParam,
    /**Only used for video VAEs: Amount of frames to encode at a time.

**Metadata**:
  - Default: 64
  - Max: 4096
  - Min: 8
  - Step: 4
*/
    pub temporal_size: TemporalSizeParam,
    /**Only used for video VAEs: Amount of frames to overlap.

**Metadata**:
  - Default: 8
  - Max: 4096
  - Min: 4
  - Step: 4
*/
    pub temporal_overlap: TemporalOverlapParam,
}
impl<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> VAEEncodeTiled<
    PixelsParam,
    VaeParam,
    TileSizeParam,
    OverlapParam,
    TemporalSizeParam,
    TemporalOverlapParam,
> {
    /// Create a new node.
    pub fn new(
        pixels: PixelsParam,
        vae: VaeParam,
        tile_size: TileSizeParam,
        overlap: OverlapParam,
        temporal_size: TemporalSizeParam,
        temporal_overlap: TemporalOverlapParam,
    ) -> Self {
        Self {
            pixels,
            vae,
            tile_size,
            overlap,
            temporal_size,
            temporal_overlap,
        }
    }
}
impl<
    PixelsParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    TileSizeParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
    TemporalSizeParam: crate::nodes::types::Int,
    TemporalOverlapParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VAEEncodeTiled<
    PixelsParam,
    VaeParam,
    TileSizeParam,
    OverlapParam,
    TemporalSizeParam,
    TemporalOverlapParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("pixels".to_string(), self.pixels.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("tile_size".to_string(), self.tile_size.clone().into());
        output.insert("overlap".to_string(), self.overlap.clone().into());
        output.insert("temporal_size".to_string(), self.temporal_size.clone().into());
        output
            .insert(
                "temporal_overlap".to_string(),
                self.temporal_overlap.clone().into(),
            );
        output
    }
    const NAME: &'static str = "VAEEncodeTiled";
    const DISPLAY_NAME: &'static str = "VAE Encode (Tiled)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental";
}
