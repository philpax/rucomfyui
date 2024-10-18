//!`model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ModelSamplingAuraFlow**: No description.
#[derive(Clone)]
pub struct ModelSamplingAuraFlow<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1.73
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> ModelSamplingAuraFlow<Model, Shift> {
    /// Create a new node.
    pub fn new(model: Model, shift: Shift) -> Self {
        Self { model, shift }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingAuraFlow<Model, Shift> {
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
pub struct ModelSamplingContinuousEdm<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sampling: Sampling,
    /**No documentation.

**Metadata**:
  - Default: 120
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.002
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_min: SigmaMin,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> ModelSamplingContinuousEdm<Model, Sampling, SigmaMax, SigmaMin> {
    /// Create a new node.
    pub fn new(
        model: Model,
        sampling: Sampling,
        sigma_max: SigmaMax,
        sigma_min: SigmaMin,
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
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousEdm<Model, Sampling, SigmaMax, SigmaMin> {
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
pub struct ModelSamplingContinuousV<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sampling: Sampling,
    /**No documentation.

**Metadata**:
  - Default: 500
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.03
  - Max: 1000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma_min: SigmaMin,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> ModelSamplingContinuousV<Model, Sampling, SigmaMax, SigmaMin> {
    /// Create a new node.
    pub fn new(
        model: Model,
        sampling: Sampling,
        sigma_max: SigmaMax,
        sigma_min: SigmaMin,
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
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousV<Model, Sampling, SigmaMax, SigmaMin> {
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
pub struct ModelSamplingDiscrete<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sampling: Sampling,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub zsnr: Zsnr,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> ModelSamplingDiscrete<Model, Sampling, Zsnr> {
    /// Create a new node.
    pub fn new(model: Model, sampling: Sampling, zsnr: Zsnr) -> Self {
        Self { model, sampling, zsnr }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for ModelSamplingDiscrete<Model, Sampling, Zsnr> {
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
pub struct ModelSamplingFlux<
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1.15
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub max_shift: MaxShift,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub base_shift: BaseShift,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: Height,
}
impl<
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> ModelSamplingFlux<Model, MaxShift, BaseShift, Width, Height> {
    /// Create a new node.
    pub fn new(
        model: Model,
        max_shift: MaxShift,
        base_shift: BaseShift,
        width: Width,
        height: Height,
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
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ModelSamplingFlux<Model, MaxShift, BaseShift, Width, Height> {
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
///**ModelSamplingSD3**: No description.
#[derive(Clone)]
pub struct ModelSamplingSd3<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> ModelSamplingSd3<Model, Shift> {
    /// Create a new node.
    pub fn new(model: Model, shift: Shift) -> Self {
        Self { model, shift }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingSd3<Model, Shift> {
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
pub struct ModelSamplingStableCascade<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> ModelSamplingStableCascade<Model, Shift> {
    /// Create a new node.
    pub fn new(model: Model, shift: Shift) -> Self {
        Self { model, shift }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingStableCascade<Model, Shift> {
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
///**RescaleCFG**: No description.
#[derive(Clone)]
pub struct RescaleCfg<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 0.7
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub multiplier: Multiplier,
}
impl<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> RescaleCfg<Model, Multiplier> {
    /// Create a new node.
    pub fn new(model: Model, multiplier: Multiplier) -> Self {
        Self { model, multiplier }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for RescaleCfg<Model, Multiplier> {
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
