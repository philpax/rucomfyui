//!`controlnet` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
}
///**Apply ControlNet**: No description.
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
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    const DISPLAY_NAME: &'static str = "Apply ControlNet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**Apply ControlNet (Advanced)**: No description.
#[derive(Clone)]
pub struct ControlNetApplyAdvanced<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
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
    pub image: Image,
    ///No documentation.
    pub strength: Strength,
    ///No documentation.
    pub start_percent: StartPercent,
    ///No documentation.
    pub end_percent: EndPercent,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    ControlNet: crate::nodes::types::ControlNet,
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> ControlNetApplyAdvanced<
    Positive,
    Negative,
    ControlNet,
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
        image: Image,
        strength: Strength,
        start_percent: StartPercent,
        end_percent: EndPercent,
    ) -> Self {
        Self {
            positive,
            negative,
            control_net,
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
    Image: crate::nodes::types::Image,
    Strength: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
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
    type Output = out::ControlNetApplyAdvancedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
            negative: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 1u32,
            },
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
        output
    }
    const NAME: &'static str = "ControlNetApplyAdvanced";
    const DISPLAY_NAME: &'static str = "Apply ControlNet (Advanced)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/controlnet";
}
///**ControlNetApply SD3 and HunyuanDiT**: No description.
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
    ///No documentation.
    pub strength: Strength,
    ///No documentation.
    pub start_percent: StartPercent,
    ///No documentation.
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
            positive: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
            negative: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 1u32,
            },
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
    const DISPLAY_NAME: &'static str = "ControlNetApply SD3 and HunyuanDiT";
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
