//!`cond pair` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`PairConditioningCombine`](super::PairConditioningCombine).
    #[derive(Clone)]
    pub struct PairConditioningCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetDefaultCombine`](super::PairConditioningSetDefaultCombine).
    #[derive(Clone)]
    pub struct PairConditioningSetDefaultCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetProperties`](super::PairConditioningSetProperties).
    #[derive(Clone)]
    pub struct PairConditioningSetPropertiesOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetPropertiesAndCombine`](super::PairConditioningSetPropertiesAndCombine).
    #[derive(Clone)]
    pub struct PairConditioningSetPropertiesAndCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**Cond Pair Combine**: No description.
#[derive(Clone)]
pub struct PairConditioningCombine<
    PositiveA: crate::nodes::types::Conditioning,
    NegativeA: crate::nodes::types::Conditioning,
    PositiveB: crate::nodes::types::Conditioning,
    NegativeB: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub positive_a: PositiveA,
    ///No documentation.
    pub negative_a: NegativeA,
    ///No documentation.
    pub positive_b: PositiveB,
    ///No documentation.
    pub negative_b: NegativeB,
}
impl<
    PositiveA: crate::nodes::types::Conditioning,
    NegativeA: crate::nodes::types::Conditioning,
    PositiveB: crate::nodes::types::Conditioning,
    NegativeB: crate::nodes::types::Conditioning,
> PairConditioningCombine<PositiveA, NegativeA, PositiveB, NegativeB> {
    /// Create a new node.
    pub fn new(
        positive_a: PositiveA,
        negative_a: NegativeA,
        positive_b: PositiveB,
        negative_b: NegativeB,
    ) -> Self {
        Self {
            positive_a,
            negative_a,
            positive_b,
            negative_b,
        }
    }
}
impl<
    PositiveA: crate::nodes::types::Conditioning,
    NegativeA: crate::nodes::types::Conditioning,
    PositiveB: crate::nodes::types::Conditioning,
    NegativeB: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for PairConditioningCombine<PositiveA, NegativeA, PositiveB, NegativeB> {
    type Output = out::PairConditioningCombineOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive_A".to_string(), self.positive_a.clone().into());
        output.insert("negative_A".to_string(), self.negative_a.clone().into());
        output.insert("positive_B".to_string(), self.positive_b.clone().into());
        output.insert("negative_B".to_string(), self.negative_b.clone().into());
        output
    }
    const NAME: &'static str = "PairConditioningCombine";
    const DISPLAY_NAME: &'static str = "Cond Pair Combine";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond pair";
}
///**Cond Pair Set Default Combine**: No description.
#[derive(Clone)]
pub struct PairConditioningSetDefaultCombine<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveDefault: crate::nodes::types::Conditioning,
    NegativeDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub positive_default: PositiveDefault,
    ///No documentation.
    pub negative_default: NegativeDefault,
    ///No documentation.
    pub hooks: Option<Hooks>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveDefault: crate::nodes::types::Conditioning,
    NegativeDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks,
> PairConditioningSetDefaultCombine<
    Positive,
    Negative,
    PositiveDefault,
    NegativeDefault,
    Hooks,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        positive_default: PositiveDefault,
        negative_default: NegativeDefault,
        hooks: Option<Hooks>,
    ) -> Self {
        Self {
            positive,
            negative,
            positive_default,
            negative_default,
            hooks,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveDefault: crate::nodes::types::Conditioning,
    NegativeDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for PairConditioningSetDefaultCombine<
    Positive,
    Negative,
    PositiveDefault,
    NegativeDefault,
    Hooks,
> {
    type Output = out::PairConditioningSetDefaultCombineOutput;
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
        output
            .insert(
                "positive_DEFAULT".to_string(),
                self.positive_default.clone().into(),
            );
        output
            .insert(
                "negative_DEFAULT".to_string(),
                self.negative_default.clone().into(),
            );
        if let Some(v) = &self.hooks {
            output.insert("hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PairConditioningSetDefaultCombine";
    const DISPLAY_NAME: &'static str = "Cond Pair Set Default Combine";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond pair";
}
///**Cond Pair Set Props**: No description.
#[derive(Clone)]
pub struct PairConditioningSetProperties<
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    Timesteps: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub positive_new: PositiveNew,
    ///No documentation.
    pub negative_new: NegativeNew,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    ///No documentation.
    pub set_cond_area: SetCondArea,
    ///No documentation.
    pub mask: Option<Mask>,
    ///No documentation.
    pub hooks: Option<Hooks>,
    ///No documentation.
    pub timesteps: Option<Timesteps>,
}
impl<
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> PairConditioningSetProperties<
    PositiveNew,
    NegativeNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
> {
    /// Create a new node.
    pub fn new(
        positive_new: PositiveNew,
        negative_new: NegativeNew,
        strength: Strength,
        set_cond_area: SetCondArea,
        mask: Option<Mask>,
        hooks: Option<Hooks>,
        timesteps: Option<Timesteps>,
    ) -> Self {
        Self {
            positive_new,
            negative_new,
            strength,
            set_cond_area,
            mask,
            hooks,
            timesteps,
        }
    }
}
impl<
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for PairConditioningSetProperties<
    PositiveNew,
    NegativeNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
> {
    type Output = out::PairConditioningSetPropertiesOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive_NEW".to_string(), self.positive_new.clone().into());
        output.insert("negative_NEW".to_string(), self.negative_new.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("set_cond_area".to_string(), self.set_cond_area.clone().into());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks {
            output.insert("hooks".to_string(), v.clone().into());
        }
        if let Some(v) = &self.timesteps {
            output.insert("timesteps".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PairConditioningSetProperties";
    const DISPLAY_NAME: &'static str = "Cond Pair Set Props";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond pair";
}
///**Cond Pair Set Props Combine**: No description.
#[derive(Clone)]
pub struct PairConditioningSetPropertiesAndCombine<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    Timesteps: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub positive_new: PositiveNew,
    ///No documentation.
    pub negative_new: NegativeNew,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    ///No documentation.
    pub set_cond_area: SetCondArea,
    ///No documentation.
    pub mask: Option<Mask>,
    ///No documentation.
    pub hooks: Option<Hooks>,
    ///No documentation.
    pub timesteps: Option<Timesteps>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> PairConditioningSetPropertiesAndCombine<
    Positive,
    Negative,
    PositiveNew,
    NegativeNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        positive_new: PositiveNew,
        negative_new: NegativeNew,
        strength: Strength,
        set_cond_area: SetCondArea,
        mask: Option<Mask>,
        hooks: Option<Hooks>,
        timesteps: Option<Timesteps>,
    ) -> Self {
        Self {
            positive,
            negative,
            positive_new,
            negative_new,
            strength,
            set_cond_area,
            mask,
            hooks,
            timesteps,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    PositiveNew: crate::nodes::types::Conditioning,
    NegativeNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for PairConditioningSetPropertiesAndCombine<
    Positive,
    Negative,
    PositiveNew,
    NegativeNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
> {
    type Output = out::PairConditioningSetPropertiesAndCombineOutput;
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
        output.insert("positive_NEW".to_string(), self.positive_new.clone().into());
        output.insert("negative_NEW".to_string(), self.negative_new.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("set_cond_area".to_string(), self.set_cond_area.clone().into());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks {
            output.insert("hooks".to_string(), v.clone().into());
        }
        if let Some(v) = &self.timesteps {
            output.insert("timesteps".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PairConditioningSetPropertiesAndCombine";
    const DISPLAY_NAME: &'static str = "Cond Pair Set Props Combine";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond pair";
}
