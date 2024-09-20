//!`model` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`ModelSamplingAuraFlow`](super::ModelSamplingAuraFlow).
    #[derive(Clone)]
    pub struct ModelSamplingAuraFlowOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingContinuousEdm`](super::ModelSamplingContinuousEdm).
    #[derive(Clone)]
    pub struct ModelSamplingContinuousEdmOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingContinuousV`](super::ModelSamplingContinuousV).
    #[derive(Clone)]
    pub struct ModelSamplingContinuousVOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingDiscrete`](super::ModelSamplingDiscrete).
    #[derive(Clone)]
    pub struct ModelSamplingDiscreteOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingFlux`](super::ModelSamplingFlux).
    #[derive(Clone)]
    pub struct ModelSamplingFluxOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingSd3`](super::ModelSamplingSd3).
    #[derive(Clone)]
    pub struct ModelSamplingSd3Output {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`ModelSamplingStableCascade`](super::ModelSamplingStableCascade).
    #[derive(Clone)]
    pub struct ModelSamplingStableCascadeOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`RescaleCfg`](super::RescaleCfg).
    #[derive(Clone)]
    pub struct RescaleCfgOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
}
///**ModelSamplingAuraFlow**: No description.
pub struct ModelSamplingAuraFlow<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingAuraFlow<Model, Shift> {
    type Output = out::ModelSamplingAuraFlowOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("shift".to_string(), self.shift.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingAuraFlow";
    const DISPLAY_NAME: &'static str = "ModelSamplingAuraFlow";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousEDM**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousEdm<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = out::ModelSamplingContinuousEdmOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("sampling".to_string(), self.sampling.to_workflow_input());
        output.insert("sigma_max".to_string(), self.sigma_max.to_workflow_input());
        output.insert("sigma_min".to_string(), self.sigma_min.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingContinuousEDM";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousEDM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousV**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousV<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = out::ModelSamplingContinuousVOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("sampling".to_string(), self.sampling.to_workflow_input());
        output.insert("sigma_max".to_string(), self.sigma_max.to_workflow_input());
        output.insert("sigma_min".to_string(), self.sigma_min.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingContinuousV";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingDiscrete**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    Sampling: crate::nodes::types::String,
    Zsnr: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for ModelSamplingDiscrete<Model, Sampling, Zsnr> {
    type Output = out::ModelSamplingDiscreteOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("sampling".to_string(), self.sampling.to_workflow_input());
        output.insert("zsnr".to_string(), self.zsnr.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingDiscrete";
    const DISPLAY_NAME: &'static str = "ModelSamplingDiscrete";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingFlux**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ModelSamplingFlux<Model, MaxShift, BaseShift, Width, Height> {
    type Output = out::ModelSamplingFluxOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("max_shift".to_string(), self.max_shift.to_workflow_input());
        output.insert("base_shift".to_string(), self.base_shift.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingFlux";
    const DISPLAY_NAME: &'static str = "ModelSamplingFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingSD3**: No description.
pub struct ModelSamplingSd3<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingSd3<Model, Shift> {
    type Output = out::ModelSamplingSd3Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("shift".to_string(), self.shift.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingSD3";
    const DISPLAY_NAME: &'static str = "ModelSamplingSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingStableCascade**: No description.
pub struct ModelSamplingStableCascade<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
impl<
    Model: crate::nodes::types::Model,
    Shift: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelSamplingStableCascade<Model, Shift> {
    type Output = out::ModelSamplingStableCascadeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("shift".to_string(), self.shift.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelSamplingStableCascade";
    const DISPLAY_NAME: &'static str = "ModelSamplingStableCascade";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**RescaleCFG**: No description.
pub struct RescaleCfg<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub multiplier: Multiplier,
}
impl<
    Model: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for RescaleCfg<Model, Multiplier> {
    type Output = out::RescaleCfgOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("multiplier".to_string(), self.multiplier.to_workflow_input());
        output
    }
    const NAME: &'static str = "RescaleCFG";
    const DISPLAY_NAME: &'static str = "RescaleCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
