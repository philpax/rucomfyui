//!`model` definitions/categories.
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
///**HiDream-O1 Patch Seam Smoothing**: Average the model output across multiple shifted patch-grid positions during the late portion of sampling. Cancels seams.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HiDreamO1PatchSeamSmoothing<
    ModelParam: crate::nodes::types::Model,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**Sampling progress (0=start, 1=end) at which the blend turns ON.

**Metadata**:
  - Default: 0.8
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub start_percent: StartPercentParam,
    /**Sampling progress at which the blend turns OFF.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub end_percent: EndPercentParam,
    /**Interpolation between the natural-grid pred (0) and the averaged result (1).

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> HiDreamO1PatchSeamSmoothing<
    ModelParam,
    StartPercentParam,
    EndPercentParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            model,
            start_percent,
            end_percent,
            strength,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for HiDreamO1PatchSeamSmoothing<
    ModelParam,
    StartPercentParam,
    EndPercentParam,
    StrengthParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "HiDreamO1PatchSeamSmoothing";
    const DISPLAY_NAME: &'static str = "HiDream-O1 Patch Seam Smoothing";
    const DESCRIPTION: &'static str = "Average the model output across multiple shifted patch-grid positions during the late portion of sampling. Cancels seams.";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelNoiseScale**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelNoiseScale<
    ModelParam: crate::nodes::types::Model,
    NoiseScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**Absolute training noise scale. For example HiDream-O1 base: 8.0, dev: 7.5.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 0
  - Step: 0.01
*/
    pub noise_scale: NoiseScaleParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    NoiseScaleParam: crate::nodes::types::Float,
> ModelNoiseScale<ModelParam, NoiseScaleParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, noise_scale: NoiseScaleParam) -> Self {
        Self { model, noise_scale }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    NoiseScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelNoiseScale<ModelParam, NoiseScaleParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("noise_scale".to_string(), self.noise_scale.clone().into());
        output
    }
    const NAME: &'static str = "ModelNoiseScale";
    const DISPLAY_NAME: &'static str = "ModelNoiseScale";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingAuraFlow**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingAuraFlow<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1.73
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: ShiftParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> ModelSamplingAuraFlow<ModelParam, ShiftParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, shift: ShiftParam) -> Self {
        Self { model, shift }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingAuraFlow<ModelParam, ShiftParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("shift".to_string(), self.shift.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingAuraFlow";
    const DISPLAY_NAME: &'static str = "ModelSamplingAuraFlow";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousEDM**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingContinuousEDM<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub sampling: SamplingParam,
    /**No documentation.

**Metadata**:
  - Default: 120
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.002
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_min: SigmaMinParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> ModelSamplingContinuousEDM<ModelParam, SamplingParam, SigmaMaxParam, SigmaMinParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        sampling: SamplingParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
    ) -> Self {
        Self {
            model,
            sampling,
            sigma_max,
            sigma_min,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousEDM<ModelParam, SamplingParam, SigmaMaxParam, SigmaMinParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("sampling".to_string(), self.sampling.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingContinuousEDM";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousEDM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousV**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingContinuousV<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub sampling: SamplingParam,
    /**No documentation.

**Metadata**:
  - Default: 500
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.03
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_min: SigmaMinParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> ModelSamplingContinuousV<ModelParam, SamplingParam, SigmaMaxParam, SigmaMinParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        sampling: SamplingParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
    ) -> Self {
        Self {
            model,
            sampling,
            sigma_max,
            sigma_min,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousV<ModelParam, SamplingParam, SigmaMaxParam, SigmaMinParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("sampling".to_string(), self.sampling.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingContinuousV";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingDiscrete**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingDiscrete<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    ZsnrParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub sampling: SamplingParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub zsnr: ZsnrParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    ZsnrParam: crate::nodes::types::Boolean,
> ModelSamplingDiscrete<ModelParam, SamplingParam, ZsnrParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, sampling: SamplingParam, zsnr: ZsnrParam) -> Self {
        Self { model, sampling, zsnr }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    SamplingParam: crate::nodes::types::String,
    ZsnrParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ModelSamplingDiscrete<ModelParam, SamplingParam, ZsnrParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("sampling".to_string(), self.sampling.clone().into());
        output.insert("zsnr".to_string(), self.zsnr.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingDiscrete";
    const DISPLAY_NAME: &'static str = "ModelSamplingDiscrete";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingFlux**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingFlux<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1.15
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub max_shift: MaxShiftParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub base_shift: BaseShiftParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> ModelSamplingFlux<ModelParam, MaxShiftParam, BaseShiftParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        max_shift: MaxShiftParam,
        base_shift: BaseShiftParam,
        width: WidthParam,
        height: HeightParam,
    ) -> Self {
        Self {
            model,
            max_shift,
            base_shift,
            width,
            height,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ModelSamplingFlux<
    ModelParam,
    MaxShiftParam,
    BaseShiftParam,
    WidthParam,
    HeightParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("max_shift".to_string(), self.max_shift.clone().into());
        output.insert("base_shift".to_string(), self.base_shift.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingFlux";
    const DISPLAY_NAME: &'static str = "ModelSamplingFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingLTXV**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingLTXV<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent = crate::nodes::types::LatentOut,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 2.05
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub max_shift: MaxShiftParam,
    /**No documentation.

**Metadata**:
  - Default: 0.95
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub base_shift: BaseShiftParam,
    ///No documentation.
    pub latent: Option<LatentParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent,
> ModelSamplingLTXV<ModelParam, MaxShiftParam, BaseShiftParam, LatentParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        max_shift: MaxShiftParam,
        base_shift: BaseShiftParam,
        latent: Option<LatentParam>,
    ) -> Self {
        Self {
            model,
            max_shift,
            base_shift,
            latent,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for ModelSamplingLTXV<ModelParam, MaxShiftParam, BaseShiftParam, LatentParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("max_shift".to_string(), self.max_shift.clone().into());
        output.insert("base_shift".to_string(), self.base_shift.clone().into());
        if let Some(v) = &self.latent {
            output.insert("latent".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ModelSamplingLTXV";
    const DISPLAY_NAME: &'static str = "ModelSamplingLTXV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingSD3**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingSD3<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: ShiftParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> ModelSamplingSD3<ModelParam, ShiftParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, shift: ShiftParam) -> Self {
        Self { model, shift }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingSD3<ModelParam, ShiftParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("shift".to_string(), self.shift.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingSD3";
    const DISPLAY_NAME: &'static str = "ModelSamplingSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingStableCascade**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelSamplingStableCascade<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: ShiftParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> ModelSamplingStableCascade<ModelParam, ShiftParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, shift: ShiftParam) -> Self {
        Self { model, shift }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ShiftParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingStableCascade<ModelParam, ShiftParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("shift".to_string(), self.shift.clone().into());
        output
    }
    const NAME: &'static str = "ModelSamplingStableCascade";
    const DISPLAY_NAME: &'static str = "ModelSamplingStableCascade";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**RenormCFG**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RenormCFG<
    ModelParam: crate::nodes::types::Model,
    CfgTruncParam: crate::nodes::types::Float,
    RenormCfgParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 100
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub cfg_trunc: CfgTruncParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub renorm_cfg: RenormCfgParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    CfgTruncParam: crate::nodes::types::Float,
    RenormCfgParam: crate::nodes::types::Float,
> RenormCFG<ModelParam, CfgTruncParam, RenormCfgParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        cfg_trunc: CfgTruncParam,
        renorm_cfg: RenormCfgParam,
    ) -> Self {
        Self {
            model,
            cfg_trunc,
            renorm_cfg,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    CfgTruncParam: crate::nodes::types::Float,
    RenormCfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for RenormCFG<ModelParam, CfgTruncParam, RenormCfgParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("cfg_trunc".to_string(), self.cfg_trunc.clone().into());
        output.insert("renorm_cfg".to_string(), self.renorm_cfg.clone().into());
        output
    }
    const NAME: &'static str = "RenormCFG";
    const DISPLAY_NAME: &'static str = "RenormCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**RescaleCFG**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RescaleCFG<
    ModelParam: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 0.7
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub multiplier: MultiplierParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> RescaleCFG<ModelParam, MultiplierParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, multiplier: MultiplierParam) -> Self {
        Self { model, multiplier }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for RescaleCFG<ModelParam, MultiplierParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "RescaleCFG";
    const DISPLAY_NAME: &'static str = "RescaleCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
