//!controlnet
///**Apply ControlNet**
pub struct ControlNetApply<
    Conditioning: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub strength: Strength,
}
///**Apply ControlNet (Advanced)**
pub struct ControlNetApplyAdvanced<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
    StartPercent: crate::nodes::Float,
    EndPercent: crate::nodes::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub strength: Strength,
    ///No documentation.
    pub start_percent: StartPercent,
    ///No documentation.
    pub end_percent: EndPercent,
}
///**ControlNetApply SD3 and HunyuanDiT**
pub struct ControlNetApplySd3<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Vae: crate::nodes::Vae,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
    StartPercent: crate::nodes::Float,
    EndPercent: crate::nodes::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub strength: Strength,
    ///No documentation.
    pub start_percent: StartPercent,
    ///No documentation.
    pub end_percent: EndPercent,
}
///**SetUnionControlNetType**
pub struct SetUnionControlNetType<ControlNet: crate::nodes::ControlNet> {
    ///No documentation.
    pub control_net: ControlNet,
}
