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
    Cond: crate::nodes::types::Conditioning,
    CondDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub cond: Cond,
    ///No documentation.
    pub cond_default: CondDefault,
    ///No documentation.
    pub hooks: Option<Hooks>,
}
impl<
    Cond: crate::nodes::types::Conditioning,
    CondDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks,
> ConditioningSetDefaultCombine<Cond, CondDefault, Hooks> {
    /// Create a new node.
    pub fn new(cond: Cond, cond_default: CondDefault, hooks: Option<Hooks>) -> Self {
        Self { cond, cond_default, hooks }
    }
}
impl<
    Cond: crate::nodes::types::Conditioning,
    CondDefault: crate::nodes::types::Conditioning,
    Hooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode for ConditioningSetDefaultCombine<Cond, CondDefault, Hooks> {
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
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    Timesteps: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub cond_new: CondNew,
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
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> ConditioningSetProperties<CondNew, Strength, SetCondArea, Mask, Hooks, Timesteps> {
    /// Create a new node.
    pub fn new(
        cond_new: CondNew,
        strength: Strength,
        set_cond_area: SetCondArea,
        mask: Option<Mask>,
        hooks: Option<Hooks>,
        timesteps: Option<Timesteps>,
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
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for ConditioningSetProperties<CondNew, Strength, SetCondArea, Mask, Hooks, Timesteps> {
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
    Cond: crate::nodes::types::Conditioning,
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    Timesteps: crate::nodes::types::TimestepsRange
        = crate::nodes::types::TimestepsRangeOut,
> {
    ///No documentation.
    pub cond: Cond,
    ///No documentation.
    pub cond_new: CondNew,
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
    Cond: crate::nodes::types::Conditioning,
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> ConditioningSetPropertiesAndCombine<
    Cond,
    CondNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
> {
    /// Create a new node.
    pub fn new(
        cond: Cond,
        cond_new: CondNew,
        strength: Strength,
        set_cond_area: SetCondArea,
        mask: Option<Mask>,
        hooks: Option<Hooks>,
        timesteps: Option<Timesteps>,
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
    Cond: crate::nodes::types::Conditioning,
    CondNew: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
    Mask: crate::nodes::types::Mask,
    Hooks: crate::nodes::types::Hooks,
    Timesteps: crate::nodes::types::TimestepsRange,
> crate::nodes::TypedNode
for ConditioningSetPropertiesAndCombine<
    Cond,
    CondNew,
    Strength,
    SetCondArea,
    Mask,
    Hooks,
    Timesteps,
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
