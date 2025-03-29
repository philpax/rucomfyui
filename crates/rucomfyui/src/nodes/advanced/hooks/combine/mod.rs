//!`combine` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Combine Hooks [2]**: No description.
#[derive(Clone)]
pub struct CombineHooks2<
    HooksAParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksBParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksAParam>,
    ///No documentation.
    pub hooks_b: Option<HooksBParam>,
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
> CombineHooks2<HooksAParam, HooksBParam> {
    /// Create a new node.
    pub fn new(hooks_a: Option<HooksAParam>, hooks_b: Option<HooksBParam>) -> Self {
        Self { hooks_a, hooks_b }
    }
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode for CombineHooks2<HooksAParam, HooksBParam> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.hooks_a {
            output.insert("hooks_A".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_b {
            output.insert("hooks_B".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CombineHooks2";
    const DISPLAY_NAME: &'static str = "Combine Hooks [2]";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/combine";
}
///**Combine Hooks [4]**: No description.
#[derive(Clone)]
pub struct CombineHooks4<
    HooksAParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksBParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksCParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksDParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksAParam>,
    ///No documentation.
    pub hooks_b: Option<HooksBParam>,
    ///No documentation.
    pub hooks_c: Option<HooksCParam>,
    ///No documentation.
    pub hooks_d: Option<HooksDParam>,
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
    HooksCParam: crate::nodes::types::Hooks,
    HooksDParam: crate::nodes::types::Hooks,
> CombineHooks4<HooksAParam, HooksBParam, HooksCParam, HooksDParam> {
    /// Create a new node.
    pub fn new(
        hooks_a: Option<HooksAParam>,
        hooks_b: Option<HooksBParam>,
        hooks_c: Option<HooksCParam>,
        hooks_d: Option<HooksDParam>,
    ) -> Self {
        Self {
            hooks_a,
            hooks_b,
            hooks_c,
            hooks_d,
        }
    }
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
    HooksCParam: crate::nodes::types::Hooks,
    HooksDParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CombineHooks4<HooksAParam, HooksBParam, HooksCParam, HooksDParam> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.hooks_a {
            output.insert("hooks_A".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_b {
            output.insert("hooks_B".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_c {
            output.insert("hooks_C".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_d {
            output.insert("hooks_D".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CombineHooks4";
    const DISPLAY_NAME: &'static str = "Combine Hooks [4]";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/combine";
}
///**Combine Hooks [8]**: No description.
#[derive(Clone)]
pub struct CombineHooks8<
    HooksAParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksBParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksCParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksDParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksEParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksFParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksGParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksHParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksAParam>,
    ///No documentation.
    pub hooks_b: Option<HooksBParam>,
    ///No documentation.
    pub hooks_c: Option<HooksCParam>,
    ///No documentation.
    pub hooks_d: Option<HooksDParam>,
    ///No documentation.
    pub hooks_e: Option<HooksEParam>,
    ///No documentation.
    pub hooks_f: Option<HooksFParam>,
    ///No documentation.
    pub hooks_g: Option<HooksGParam>,
    ///No documentation.
    pub hooks_h: Option<HooksHParam>,
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
    HooksCParam: crate::nodes::types::Hooks,
    HooksDParam: crate::nodes::types::Hooks,
    HooksEParam: crate::nodes::types::Hooks,
    HooksFParam: crate::nodes::types::Hooks,
    HooksGParam: crate::nodes::types::Hooks,
    HooksHParam: crate::nodes::types::Hooks,
> CombineHooks8<
    HooksAParam,
    HooksBParam,
    HooksCParam,
    HooksDParam,
    HooksEParam,
    HooksFParam,
    HooksGParam,
    HooksHParam,
> {
    /// Create a new node.
    pub fn new(
        hooks_a: Option<HooksAParam>,
        hooks_b: Option<HooksBParam>,
        hooks_c: Option<HooksCParam>,
        hooks_d: Option<HooksDParam>,
        hooks_e: Option<HooksEParam>,
        hooks_f: Option<HooksFParam>,
        hooks_g: Option<HooksGParam>,
        hooks_h: Option<HooksHParam>,
    ) -> Self {
        Self {
            hooks_a,
            hooks_b,
            hooks_c,
            hooks_d,
            hooks_e,
            hooks_f,
            hooks_g,
            hooks_h,
        }
    }
}
impl<
    HooksAParam: crate::nodes::types::Hooks,
    HooksBParam: crate::nodes::types::Hooks,
    HooksCParam: crate::nodes::types::Hooks,
    HooksDParam: crate::nodes::types::Hooks,
    HooksEParam: crate::nodes::types::Hooks,
    HooksFParam: crate::nodes::types::Hooks,
    HooksGParam: crate::nodes::types::Hooks,
    HooksHParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CombineHooks8<
    HooksAParam,
    HooksBParam,
    HooksCParam,
    HooksDParam,
    HooksEParam,
    HooksFParam,
    HooksGParam,
    HooksHParam,
> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.hooks_a {
            output.insert("hooks_A".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_b {
            output.insert("hooks_B".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_c {
            output.insert("hooks_C".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_d {
            output.insert("hooks_D".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_e {
            output.insert("hooks_E".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_f {
            output.insert("hooks_F".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_g {
            output.insert("hooks_G".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hooks_h {
            output.insert("hooks_H".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CombineHooks8";
    const DISPLAY_NAME: &'static str = "Combine Hooks [8]";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/combine";
}
