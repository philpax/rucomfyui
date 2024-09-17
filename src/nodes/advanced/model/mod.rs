//!model
///**ModelSamplingAuraFlow**
pub struct ModelSamplingAuraFlow<
    Model: crate::nodes::Model,
    Shift: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingAuraFlow`].
pub struct ModelSamplingAuraFlowOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, Shift: crate::nodes::Float> crate::nodes::TypedNode
for ModelSamplingAuraFlow<Model, Shift> {
    type Output = ModelSamplingAuraFlowOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingAuraFlow";
    const DISPLAY_NAME: &'static str = "ModelSamplingAuraFlow";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousEDM**
pub struct ModelSamplingContinuousEdm<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
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
pub struct ModelSamplingContinuousEdmOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousEdm<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = ModelSamplingContinuousEdmOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingContinuousEDM";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousEDM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingContinuousV**
pub struct ModelSamplingContinuousV<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
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
pub struct ModelSamplingContinuousVOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> crate::nodes::TypedNode
for ModelSamplingContinuousV<Model, Sampling, SigmaMax, SigmaMin> {
    type Output = ModelSamplingContinuousVOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingContinuousV";
    const DISPLAY_NAME: &'static str = "ModelSamplingContinuousV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingDiscrete**
pub struct ModelSamplingDiscrete<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    Zsnr: crate::nodes::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sampling: Sampling,
    ///No documentation.
    pub zsnr: Zsnr,
}
///Output for [`ModelSamplingDiscrete`].
pub struct ModelSamplingDiscreteOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<
    Model: crate::nodes::Model,
    Sampling: crate::nodes::String,
    Zsnr: crate::nodes::Boolean,
> crate::nodes::TypedNode for ModelSamplingDiscrete<Model, Sampling, Zsnr> {
    type Output = ModelSamplingDiscreteOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingDiscrete";
    const DISPLAY_NAME: &'static str = "ModelSamplingDiscrete";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingFlux**
pub struct ModelSamplingFlux<
    Model: crate::nodes::Model,
    MaxShift: crate::nodes::Float,
    BaseShift: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
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
pub struct ModelSamplingFluxOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<
    Model: crate::nodes::Model,
    MaxShift: crate::nodes::Float,
    BaseShift: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> crate::nodes::TypedNode
for ModelSamplingFlux<Model, MaxShift, BaseShift, Width, Height> {
    type Output = ModelSamplingFluxOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingFlux";
    const DISPLAY_NAME: &'static str = "ModelSamplingFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingSD3**
pub struct ModelSamplingSd3<Model: crate::nodes::Model, Shift: crate::nodes::Float> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingSd3`].
pub struct ModelSamplingSd3Output {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, Shift: crate::nodes::Float> crate::nodes::TypedNode
for ModelSamplingSd3<Model, Shift> {
    type Output = ModelSamplingSd3Output;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingSD3";
    const DISPLAY_NAME: &'static str = "ModelSamplingSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**ModelSamplingStableCascade**
pub struct ModelSamplingStableCascade<
    Model: crate::nodes::Model,
    Shift: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
}
///Output for [`ModelSamplingStableCascade`].
pub struct ModelSamplingStableCascadeOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, Shift: crate::nodes::Float> crate::nodes::TypedNode
for ModelSamplingStableCascade<Model, Shift> {
    type Output = ModelSamplingStableCascadeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "ModelSamplingStableCascade";
    const DISPLAY_NAME: &'static str = "ModelSamplingStableCascade";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
///**RescaleCFG**
pub struct RescaleCfg<Model: crate::nodes::Model, Multiplier: crate::nodes::Float> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub multiplier: Multiplier,
}
///Output for [`RescaleCfg`].
pub struct RescaleCfgOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, Multiplier: crate::nodes::Float> crate::nodes::TypedNode
for RescaleCfg<Model, Multiplier> {
    type Output = RescaleCfgOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "RescaleCFG";
    const DISPLAY_NAME: &'static str = "RescaleCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model";
}
