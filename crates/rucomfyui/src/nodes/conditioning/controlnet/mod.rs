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
    pub struct ControlNetApplyAdvancedOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ControlNetApplySd3`](super::ControlNetApplySd3).
    #[derive(Clone)]
    pub struct ControlNetApplySd3Output {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ControlNetInpaintingAliMamaApply`](super::ControlNetInpaintingAliMamaApply).
    #[derive(Clone)]
    pub struct ControlNetInpaintingAliMamaApplyOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**Apply ControlNet (OLD)**: No description.
#[derive(Clone)]
pub struct ControlNetApply<
    Conditioning: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
> ControlNetApply<Conditioning, ControlNet, Image, Strength> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        control_net: ControlNet,
        image: Image,
        strength: Strength,
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
    Conditioning: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ControlNetApply<Conditioning, ControlNet, Image, Strength> {
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
pub struct ControlNetApplyAdvanced<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    Vae: crate::nodes::types::Vae = crate::nodes::types::VaeOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
    ///No documentation.
    pub vae: Option<Vae>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    Vae: crate::nodes::types::Vae,
> ControlNetApplyAdvanced<
    Positive,
    Negative,
    ControlNet,
    Image,
    Strength,
    StartPercent,
    EndPercent,
    Vae,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        control_net: ControlNet,
        image: Image,
        strength: Strength,
        start_percent: StartPercent,
        end_percent: EndPercent,
        vae: Option<Vae>,
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
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    Vae: crate::nodes::types::Vae,
> crate::nodes::TypedNode
for ControlNetApplyAdvanced<
    Positive,
    Negative,
    ControlNet,
    Image,
    Strength,
    StartPercent,
    EndPercent,
    Vae,
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
pub struct ControlNetApplySd3<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
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
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> ControlNetApplySd3<
    Positive,
    Negative,
    ControlNet,
    Vae,
    Image,
    Strength,
    StartPercent,
    EndPercent,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        control_net: ControlNet,
        vae: Vae,
        image: Image,
        strength: Strength,
        start_percent: StartPercent,
        end_percent: EndPercent,
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
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
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
    type Output = out::ControlNetApplySd3Output;
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
pub struct ControlNetInpaintingAliMamaApply<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
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
    pub mask: Mask,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> ControlNetInpaintingAliMamaApply<
    Positive,
    Negative,
    ControlNet,
    Vae,
    Image,
    Mask,
    Strength,
    StartPercent,
    EndPercent,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        control_net: ControlNet,
        vae: Vae,
        image: Image,
        mask: Mask,
        strength: Strength,
        start_percent: StartPercent,
        end_percent: EndPercent,
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
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ControlNetInpaintingAliMamaApply<
    Positive,
    Negative,
    ControlNet,
    Vae,
    Image,
    Mask,
    Strength,
    StartPercent,
    EndPercent,
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
pub struct SetUnionControlNetType<
    ControlNet: crate::nodes::types::ControlNet,
    Type: crate::nodes::types::String,
> {
    ///No documentation.
    pub control_net: ControlNet,
    ///No documentation.
    pub type_: Type,
}
impl<
    ControlNet: crate::nodes::types::ControlNet,
    Type: crate::nodes::types::String,
> SetUnionControlNetType<ControlNet, Type> {
    /// Create a new node.
    pub fn new(control_net: ControlNet, type_: Type) -> Self {
        Self { control_net, type_ }
    }
}
impl<
    ControlNet: crate::nodes::types::ControlNet,
    Type: crate::nodes::types::String,
> crate::nodes::TypedNode for SetUnionControlNetType<ControlNet, Type> {
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
