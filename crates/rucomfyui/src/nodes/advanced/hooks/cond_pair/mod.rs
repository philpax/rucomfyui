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
    #[allow(non_camel_case_types)]
    pub struct PairConditioningCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetDefaultCombine`](super::PairConditioningSetDefaultCombine).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct PairConditioningSetDefaultCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetProperties`](super::PairConditioningSetProperties).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct PairConditioningSetPropertiesOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`PairConditioningSetPropertiesAndCombine`](super::PairConditioningSetPropertiesAndCombine).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct PairConditioningSetPropertiesAndCombineOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**Cond Pair Combine**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PairConditioningCombine<
    PositiveAParam: crate::nodes::types::Conditioning,
    NegativeAParam: crate::nodes::types::Conditioning,
    PositiveBParam: crate::nodes::types::Conditioning,
    NegativeBParam: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub positive_a: PositiveAParam,
    ///No documentation.
    pub negative_a: NegativeAParam,
    ///No documentation.
    pub positive_b: PositiveBParam,
    ///No documentation.
    pub negative_b: NegativeBParam,
}
impl<
    PositiveAParam: crate::nodes::types::Conditioning,
    NegativeAParam: crate::nodes::types::Conditioning,
    PositiveBParam: crate::nodes::types::Conditioning,
    NegativeBParam: crate::nodes::types::Conditioning,
> PairConditioningCombine<
    PositiveAParam,
    NegativeAParam,
    PositiveBParam,
    NegativeBParam,
> {
    /// Create a new node.
    pub fn new(
        positive_a: PositiveAParam,
        negative_a: NegativeAParam,
        positive_b: PositiveBParam,
        negative_b: NegativeBParam,
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
    PositiveAParam: crate::nodes::types::Conditioning,
    NegativeAParam: crate::nodes::types::Conditioning,
    PositiveBParam: crate::nodes::types::Conditioning,
    NegativeBParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for PairConditioningCombine<
    PositiveAParam,
    NegativeAParam,
    PositiveBParam,
    NegativeBParam,
> {
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
#[allow(non_camel_case_types)]
pub struct PairConditioningSetDefaultCombine<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveDefaultParam: crate::nodes::types::Conditioning,
    NegativeDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub positive_default: PositiveDefaultParam,
    ///No documentation.
    pub negative_default: NegativeDefaultParam,
    ///No documentation.
    pub hooks: Option<HooksParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveDefaultParam: crate::nodes::types::Conditioning,
    NegativeDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks,
> PairConditioningSetDefaultCombine<
    PositiveParam,
    NegativeParam,
    PositiveDefaultParam,
    NegativeDefaultParam,
    HooksParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        positive_default: PositiveDefaultParam,
        negative_default: NegativeDefaultParam,
        hooks: Option<HooksParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveDefaultParam: crate::nodes::types::Conditioning,
    NegativeDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for PairConditioningSetDefaultCombine<
    PositiveParam,
    NegativeParam,
    PositiveDefaultParam,
    NegativeDefaultParam,
    HooksParam,
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
#[allow(non_camel_case_types)]
pub struct PairConditioningSetProperties<
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    TimestepsParam: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub positive_new: PositiveNewParam,
    ///No documentation.
    pub negative_new: NegativeNewParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///No documentation.
    pub set_cond_area: SetCondAreaParam,
    ///No documentation.
    pub mask: Option<MaskParam>,
    ///No documentation.
    pub hooks: Option<HooksParam>,
    ///No documentation.
    pub timesteps: Option<TimestepsParam>,
}
impl<
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> PairConditioningSetProperties<
    PositiveNewParam,
    NegativeNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    /// Create a new node.
    pub fn new(
        positive_new: PositiveNewParam,
        negative_new: NegativeNewParam,
        strength: StrengthParam,
        set_cond_area: SetCondAreaParam,
        mask: Option<MaskParam>,
        hooks: Option<HooksParam>,
        timesteps: Option<TimestepsParam>,
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
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for PairConditioningSetProperties<
    PositiveNewParam,
    NegativeNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
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
#[allow(non_camel_case_types)]
pub struct PairConditioningSetPropertiesAndCombine<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    TimestepsParam: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub positive_new: PositiveNewParam,
    ///No documentation.
    pub negative_new: NegativeNewParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///No documentation.
    pub set_cond_area: SetCondAreaParam,
    ///No documentation.
    pub mask: Option<MaskParam>,
    ///No documentation.
    pub hooks: Option<HooksParam>,
    ///No documentation.
    pub timesteps: Option<TimestepsParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> PairConditioningSetPropertiesAndCombine<
    PositiveParam,
    NegativeParam,
    PositiveNewParam,
    NegativeNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        positive_new: PositiveNewParam,
        negative_new: NegativeNewParam,
        strength: StrengthParam,
        set_cond_area: SetCondAreaParam,
        mask: Option<MaskParam>,
        hooks: Option<HooksParam>,
        timesteps: Option<TimestepsParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    PositiveNewParam: crate::nodes::types::Conditioning,
    NegativeNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for PairConditioningSetPropertiesAndCombine<
    PositiveParam,
    NegativeParam,
    PositiveNewParam,
    NegativeNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
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
