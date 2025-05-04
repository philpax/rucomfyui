//!`controlnet` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`ControlNetApplyAdvanced`](super::ControlNetApplyAdvanced).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ControlNetApplyAdvancedOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ControlNetApplySD3`](super::ControlNetApplySD3).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ControlNetApplySD3Output {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ControlNetInpaintingAliMamaApply`](super::ControlNetInpaintingAliMamaApply).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ControlNetInpaintingAliMamaApplyOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**Apply ControlNet (OLD)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ControlNetApply<
    ConditioningParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    ///No documentation.
    pub control_net: ControlNetParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> ControlNetApply<ConditioningParam, ControlNetParam, ImageParam, StrengthParam> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        control_net: ControlNetParam,
        image: ImageParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            conditioning,
            control_net,
            image,
            strength,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ControlNetApply<ConditioningParam, ControlNetParam, ImageParam, StrengthParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("control_net".to_string(), self.control_net.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ControlNetApply";
    const DISPLAY_NAME: &'static str = "Apply ControlNet (OLD)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**Apply ControlNet**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ControlNetApplyAdvanced<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae = crate::nodes::types::VaeOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub control_net: ControlNetParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
    ///No documentation.
    pub vae: Option<VaeParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
> ControlNetApplyAdvanced<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    ImageParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
    VaeParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        control_net: ControlNetParam,
        image: ImageParam,
        strength: StrengthParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        vae: Option<VaeParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            control_net,
            image,
            strength,
            start_percent,
            end_percent,
            vae,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode
for ControlNetApplyAdvanced<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    ImageParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
    VaeParam,
> {
    type Output = out::ControlNetApplyAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("control_net".to_string(), self.control_net.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        if let Some(v) = &self.vae {
            output.insert("vae".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ControlNetApplyAdvanced";
    const DISPLAY_NAME: &'static str = "Apply ControlNet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**Apply Controlnet with VAE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ControlNetApplySD3<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub control_net: ControlNetParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> ControlNetApplySD3<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    VaeParam,
    ImageParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        control_net: ControlNetParam,
        vae: VaeParam,
        image: ImageParam,
        strength: StrengthParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            positive,
            negative,
            control_net,
            vae,
            image,
            strength,
            start_percent,
            end_percent,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ControlNetApplySD3<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    VaeParam,
    ImageParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
> {
    type Output = out::ControlNetApplySD3Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("control_net".to_string(), self.control_net.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "ControlNetApplySD3";
    const DISPLAY_NAME: &'static str = "Apply Controlnet with VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**ControlNetInpaintingAliMamaApply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ControlNetInpaintingAliMamaApply<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub control_net: ControlNetParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> ControlNetInpaintingAliMamaApply<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    VaeParam,
    ImageParam,
    MaskParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        control_net: ControlNetParam,
        vae: VaeParam,
        image: ImageParam,
        mask: MaskParam,
        strength: StrengthParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            positive,
            negative,
            control_net,
            vae,
            image,
            mask,
            strength,
            start_percent,
            end_percent,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ControlNetParam: crate::nodes::types::ControlNet,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ControlNetInpaintingAliMamaApply<
    PositiveParam,
    NegativeParam,
    ControlNetParam,
    VaeParam,
    ImageParam,
    MaskParam,
    StrengthParam,
    StartPercentParam,
    EndPercentParam,
> {
    type Output = out::ControlNetInpaintingAliMamaApplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("control_net".to_string(), self.control_net.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "ControlNetInpaintingAliMamaApply";
    const DISPLAY_NAME: &'static str = "ControlNetInpaintingAliMamaApply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**SetUnionControlNetType**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SetUnionControlNetType<
    ControlNetParam: crate::nodes::types::ControlNet,
    TypeParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub control_net: ControlNetParam,
    ///No documentation.
    pub type_: TypeParam,
}
impl<
    ControlNetParam: crate::nodes::types::ControlNet,
    TypeParam: crate::nodes::types::String,
> SetUnionControlNetType<ControlNetParam, TypeParam> {
    /// Create a new node.
    pub fn new(control_net: ControlNetParam, type_: TypeParam) -> Self {
        Self { control_net, type_ }
    }
}
impl<
    ControlNetParam: crate::nodes::types::ControlNet,
    TypeParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SetUnionControlNetType<ControlNetParam, TypeParam> {
    type Output = crate::nodes::types::ControlNetOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("control_net".to_string(), self.control_net.clone().into());
        output.insert("type".to_string(), self.type_.clone().into());
        output
    }
    const NAME: &'static str = "SetUnionControlNetType";
    const DISPLAY_NAME: &'static str = "SetUnionControlNetType";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
