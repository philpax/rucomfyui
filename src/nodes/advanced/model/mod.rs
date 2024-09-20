//!`model` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**ModelSamplingAuraFlow**
pub struct ModelSamplingAuraFlow<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingAuraFlow`].
#[derive(Clone)]
pub struct ModelSamplingAuraFlowOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingAuraFlow<Model, Shift> {
    type Output = ModelSamplingAuraFlowOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingAuraFlow";
    const DISPLAY_NAME: &'static str = "ModelSamplingAuraFlow";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousEDM**
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
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
///Output for [`ModelSamplingContinuousEdm`].
#[derive(Clone)]
pub struct ModelSamplingContinuousEdmOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousEdm<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = ModelSamplingContinuousEdmOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingContinuousEDM";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousEDM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousV**
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
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
///Output for [`ModelSamplingContinuousV`].
#[derive(Clone)]
pub struct ModelSamplingContinuousVOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousV<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = ModelSamplingContinuousVOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingContinuousV";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingDiscrete**
pub struct ModelSamplingDiscrete<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sampling: Sampling,
    ///No documentation.
    pub zsnr: Zsnr,
}
///Output for [`ModelSamplingDiscrete`].
#[derive(Clone)]
pub struct ModelSamplingDiscreteOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for ModelSamplingDiscrete<Model, Sampling, Zsnr> {
    type Output = ModelSamplingDiscreteOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingDiscrete";
    const DISPLAY_NAME: &'static str = "ModelSamplingDiscrete";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingFlux**
pub struct ModelSamplingFlux<
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub max_shift: MaxShift,
    ///No documentation.
    pub base_shift: BaseShift,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///Output for [`ModelSamplingFlux`].
#[derive(Clone)]
pub struct ModelSamplingFluxOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ModelSamplingFlux<Model, MaxShift, BaseShift, Width, Height> {
    type Output = ModelSamplingFluxOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingFlux";
    const DISPLAY_NAME: &'static str = "ModelSamplingFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingSD3**
pub struct ModelSamplingSd3<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingSd3`].
#[derive(Clone)]
pub struct ModelSamplingSd3Output {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingSd3<Model, Shift> {
    type Output = ModelSamplingSd3Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingSD3";
    const DISPLAY_NAME: &'static str = "ModelSamplingSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingStableCascade**
pub struct ModelSamplingStableCascade<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingStableCascade`].
#[derive(Clone)]
pub struct ModelSamplingStableCascadeOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingStableCascade<Model, Shift> {
    type Output = ModelSamplingStableCascadeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelSamplingStableCascade";
    const DISPLAY_NAME: &'static str = "ModelSamplingStableCascade";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**RescaleCFG**
pub struct RescaleCfg<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub multiplier: Multiplier,
}
///Output for [`RescaleCfg`].
#[derive(Clone)]
pub struct RescaleCfgOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for RescaleCfg<Model, Multiplier> {
    type Output = RescaleCfgOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RescaleCFG";
    const DISPLAY_NAME: &'static str = "RescaleCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
