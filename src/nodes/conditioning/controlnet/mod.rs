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
///Output for [`ControlNetApply`].
#[derive(Clone)]
pub struct ControlNetApplyOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
> crate::nodes::TypedNode
for ControlNetApply<Conditioning, ControlNet, Image, Strength> {
    type Output = ControlNetApplyOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ControlNetApply";
    const DISPLAY_NAME: &'static str = "Apply ControlNet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
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
///Output for [`ControlNetApplyAdvanced`].
#[derive(Clone)]
pub struct ControlNetApplyAdvancedOutput {
    ///No documentation.
    pub positive: crate::nodes::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::ConditioningOut,
}
impl<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
    StartPercent: crate::nodes::Float,
    EndPercent: crate::nodes::Float,
> crate::nodes::TypedNode
for ControlNetApplyAdvanced<
    Positive,
    Negative,
    ControlNet,
    Image,
    Strength,
    StartPercent,
    EndPercent,
> {
    type Output = ControlNetApplyAdvancedOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut(0u32),
            negative: crate::nodes::ConditioningOut(1u32),
        }
    }
    const NAME: &'static str = "ControlNetApplyAdvanced";
    const DISPLAY_NAME: &'static str = "Apply ControlNet (Advanced)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
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
///Output for [`ControlNetApplySd3`].
#[derive(Clone)]
pub struct ControlNetApplySd3Output {
    ///No documentation.
    pub positive: crate::nodes::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::ConditioningOut,
}
impl<
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    ControlNet: crate::nodes::ControlNet,
    Vae: crate::nodes::Vae,
    Image: crate::nodes::Image,
    Strength: crate::nodes::Float,
    StartPercent: crate::nodes::Float,
    EndPercent: crate::nodes::Float,
> crate::nodes::TypedNode
for ControlNetApplySd3<
    Positive,
    Negative,
    ControlNet,
    Vae,
    Image,
    Strength,
    StartPercent,
    EndPercent,
> {
    type Output = ControlNetApplySd3Output;
    fn output(&self) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut(0u32),
            negative: crate::nodes::ConditioningOut(1u32),
        }
    }
    const NAME: &'static str = "ControlNetApplySD3";
    const DISPLAY_NAME: &'static str = "ControlNetApply SD3 and HunyuanDiT";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**SetUnionControlNetType**
pub struct SetUnionControlNetType<
    ControlNet: crate::nodes::ControlNet,
    Type: crate::nodes::String,
> {
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub type_: Type,
}
///Output for [`SetUnionControlNetType`].
#[derive(Clone)]
pub struct SetUnionControlNetTypeOutput {
    ///No documentation.
    pub control_net: crate::nodes::ControlNetOut,
}
impl<
    ControlNet: crate::nodes::ControlNet,
    Type: crate::nodes::String,
> crate::nodes::TypedNode for SetUnionControlNetType<ControlNet, Type> {
    type Output = SetUnionControlNetTypeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::ControlNetOut(0u32),
        }
    }
    const NAME: &'static str = "SetUnionControlNetType";
    const DISPLAY_NAME: &'static str = "SetUnionControlNetType";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
