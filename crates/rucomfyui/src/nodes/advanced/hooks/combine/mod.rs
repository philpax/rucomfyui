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
    HooksA: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksB: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksA>,
    ///No documentation.
    pub hooks_b: Option<HooksB>,
}
impl<
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
> CombineHooks2<HooksA, HooksB> {
    /// Create a new node.
    pub fn new(hooks_a: Option<HooksA>, hooks_b: Option<HooksB>) -> Self {
        Self { hooks_a, hooks_b }
    }
}
impl<
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
> crate::nodes::TypedNode for CombineHooks2<HooksA, HooksB> {
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
    HooksA: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksB: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksC: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksD: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksA>,
    ///No documentation.
    pub hooks_b: Option<HooksB>,
    ///No documentation.
    pub hooks_c: Option<HooksC>,
    ///No documentation.
    pub hooks_d: Option<HooksD>,
}
impl<
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
    HooksC: crate::nodes::types::Hooks,
    HooksD: crate::nodes::types::Hooks,
> CombineHooks4<HooksA, HooksB, HooksC, HooksD> {
    /// Create a new node.
    pub fn new(
        hooks_a: Option<HooksA>,
        hooks_b: Option<HooksB>,
        hooks_c: Option<HooksC>,
        hooks_d: Option<HooksD>,
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
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
    HooksC: crate::nodes::types::Hooks,
    HooksD: crate::nodes::types::Hooks,
> crate::nodes::TypedNode for CombineHooks4<HooksA, HooksB, HooksC, HooksD> {
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
    HooksA: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksB: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksC: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksD: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksE: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksF: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksG: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
    HooksH: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub hooks_a: Option<HooksA>,
    ///No documentation.
    pub hooks_b: Option<HooksB>,
    ///No documentation.
    pub hooks_c: Option<HooksC>,
    ///No documentation.
    pub hooks_d: Option<HooksD>,
    ///No documentation.
    pub hooks_e: Option<HooksE>,
    ///No documentation.
    pub hooks_f: Option<HooksF>,
    ///No documentation.
    pub hooks_g: Option<HooksG>,
    ///No documentation.
    pub hooks_h: Option<HooksH>,
}
impl<
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
    HooksC: crate::nodes::types::Hooks,
    HooksD: crate::nodes::types::Hooks,
    HooksE: crate::nodes::types::Hooks,
    HooksF: crate::nodes::types::Hooks,
    HooksG: crate::nodes::types::Hooks,
    HooksH: crate::nodes::types::Hooks,
> CombineHooks8<HooksA, HooksB, HooksC, HooksD, HooksE, HooksF, HooksG, HooksH> {
    /// Create a new node.
    pub fn new(
        hooks_a: Option<HooksA>,
        hooks_b: Option<HooksB>,
        hooks_c: Option<HooksC>,
        hooks_d: Option<HooksD>,
        hooks_e: Option<HooksE>,
        hooks_f: Option<HooksF>,
        hooks_g: Option<HooksG>,
        hooks_h: Option<HooksH>,
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
    HooksA: crate::nodes::types::Hooks,
    HooksB: crate::nodes::types::Hooks,
    HooksC: crate::nodes::types::Hooks,
    HooksD: crate::nodes::types::Hooks,
    HooksE: crate::nodes::types::Hooks,
    HooksF: crate::nodes::types::Hooks,
    HooksG: crate::nodes::types::Hooks,
    HooksH: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CombineHooks8<HooksA, HooksB, HooksC, HooksD, HooksE, HooksF, HooksG, HooksH> {
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
