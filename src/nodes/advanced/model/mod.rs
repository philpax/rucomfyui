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
///**ModelSamplingContinuousEDM**
pub struct ModelSamplingContinuousEdm<
    Model: crate::nodes::Model,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
///**ModelSamplingContinuousV**
pub struct ModelSamplingContinuousV<
    Model: crate::nodes::Model,
    SigmaMax: crate::nodes::Float,
    SigmaMin: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub sigma_max: SigmaMax,
    ///No documentation.
    pub sigma_min: SigmaMin,
}
///**ModelSamplingDiscrete**
pub struct ModelSamplingDiscrete<
    Model: crate::nodes::Model,
    Zsnr: crate::nodes::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub zsnr: Zsnr,
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
///**ModelSamplingSD3**
pub struct ModelSamplingSd3<Model: crate::nodes::Model, Shift: crate::nodes::Float> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub shift: Shift,
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
///**RescaleCFG**
pub struct RescaleCfg<Model: crate::nodes::Model, Multiplier: crate::nodes::Float> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub multiplier: Multiplier,
}
