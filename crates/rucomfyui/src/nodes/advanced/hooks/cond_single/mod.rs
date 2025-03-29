//!`cond single` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Cond Set Default Combine**: No description.
#[derive(Clone)]
pub struct ConditioningSetDefaultCombine<
    CondParam: crate::nodes::types::Conditioning,
    CondDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub cond: CondParam,
    ///No documentation.
    pub cond_default: CondDefaultParam,
    ///No documentation.
    pub hooks: Option<HooksParam>,
}
impl<
    CondParam: crate::nodes::types::Conditioning,
    CondDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks,
> ConditioningSetDefaultCombine<CondParam, CondDefaultParam, HooksParam> {
    /// Create a new node.
    pub fn new(
        cond: CondParam,
        cond_default: CondDefaultParam,
        hooks: Option<HooksParam>,
    ) -> Self {
        Self { cond, cond_default, hooks }
    }
}
impl<
    CondParam: crate::nodes::types::Conditioning,
    CondDefaultParam: crate::nodes::types::Conditioning,
    HooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for ConditioningSetDefaultCombine<CondParam, CondDefaultParam, HooksParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("cond".to_string(), self.cond.clone().into());
        output.insert("cond_DEFAULT".to_string(), self.cond_default.clone().into());
        if let Some(v) = &self.hooks {
            output.insert("hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ConditioningSetDefaultCombine";
    const DISPLAY_NAME: &'static str = "Cond Set Default Combine";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond single";
}
///**Cond Set Props**: No description.
#[derive(Clone)]
pub struct ConditioningSetProperties<
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    TimestepsParam: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub cond_new: CondNewParam,
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
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> ConditioningSetProperties<
    CondNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    /// Create a new node.
    pub fn new(
        cond_new: CondNewParam,
        strength: StrengthParam,
        set_cond_area: SetCondAreaParam,
        mask: Option<MaskParam>,
        hooks: Option<HooksParam>,
        timesteps: Option<TimestepsParam>,
    ) -> Self {
        Self {
            cond_new,
            strength,
            set_cond_area,
            mask,
            hooks,
            timesteps,
        }
    }
}
impl<
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for ConditioningSetProperties<
    CondNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("cond_NEW".to_string(), self.cond_new.clone().into());
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
    const NAME: &'static str = "ConditioningSetProperties";
    const DISPLAY_NAME: &'static str = "Cond Set Props";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond single";
}
///**Cond Set Props Combine**: No description.
#[derive(Clone)]
pub struct ConditioningSetPropertiesAndCombine<
    CondParam: crate::nodes::types::Conditioning,
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    TimestepsParam: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub cond: CondParam,
    ///No documentation.
    pub cond_new: CondNewParam,
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
    CondParam: crate::nodes::types::Conditioning,
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> ConditioningSetPropertiesAndCombine<
    CondParam,
    CondNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    /// Create a new node.
    pub fn new(
        cond: CondParam,
        cond_new: CondNewParam,
        strength: StrengthParam,
        set_cond_area: SetCondAreaParam,
        mask: Option<MaskParam>,
        hooks: Option<HooksParam>,
        timesteps: Option<TimestepsParam>,
    ) -> Self {
        Self {
            cond,
            cond_new,
            strength,
            set_cond_area,
            mask,
            hooks,
            timesteps,
        }
    }
}
impl<
    CondParam: crate::nodes::types::Conditioning,
    CondNewParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
    MaskParam: crate::nodes::types::Mask,
    HooksParam: crate::nodes::types::Hooks,
    TimestepsParam: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for ConditioningSetPropertiesAndCombine<
    CondParam,
    CondNewParam,
    StrengthParam,
    SetCondAreaParam,
    MaskParam,
    HooksParam,
    TimestepsParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("cond".to_string(), self.cond.clone().into());
        output.insert("cond_NEW".to_string(), self.cond_new.clone().into());
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
    const NAME: &'static str = "ConditioningSetPropertiesAndCombine";
    const DISPLAY_NAME: &'static str = "Cond Set Props Combine";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/cond single";
}
