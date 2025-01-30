//!`scheduling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Create Hook Keyframe**: No description.
#[derive(Clone)]
pub struct CreateHookKeyframe<
    StrengthMult: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    PrevHookKf: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_mult: StrengthMult,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKf>,
}
impl<
    StrengthMult: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> CreateHookKeyframe<StrengthMult, StartPercent, PrevHookKf> {
    /// Create a new node.
    pub fn new(
        strength_mult: StrengthMult,
        start_percent: StartPercent,
        prev_hook_kf: Option<PrevHookKf>,
    ) -> Self {
        Self {
            strength_mult,
            start_percent,
            prev_hook_kf,
        }
    }
}
impl<
    StrengthMult: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframe<StrengthMult, StartPercent, PrevHookKf> {
    type Output = crate::nodes::types::HookKeyframesOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("strength_mult".to_string(), self.strength_mult.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        if let Some(v) = &self.prev_hook_kf {
            output.insert("prev_hook_kf".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookKeyframe";
    const DISPLAY_NAME: &'static str = "Create Hook Keyframe";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/scheduling";
}
///**Create Hook Keyframes From Floats**: No description.
#[derive(Clone)]
pub struct CreateHookKeyframesFromFloats<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
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
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub print_keyframes: PrintKeyframes,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKf>,
}
impl<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> CreateHookKeyframesFromFloats<StartPercent, EndPercent, PrintKeyframes, PrevHookKf> {
    /// Create a new node.
    pub fn new(
        start_percent: StartPercent,
        end_percent: EndPercent,
        print_keyframes: PrintKeyframes,
        prev_hook_kf: Option<PrevHookKf>,
    ) -> Self {
        Self {
            start_percent,
            end_percent,
            print_keyframes,
            prev_hook_kf,
        }
    }
}
impl<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframesFromFloats<StartPercent, EndPercent, PrintKeyframes, PrevHookKf> {
    type Output = crate::nodes::types::HookKeyframesOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
            .insert("print_keyframes".to_string(), self.print_keyframes.clone().into());
        if let Some(v) = &self.prev_hook_kf {
            output.insert("prev_hook_kf".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookKeyframesFromFloats";
    const DISPLAY_NAME: &'static str = "Create Hook Keyframes From Floats";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/scheduling";
}
///**Create Hook Keyframes Interp.**: No description.
#[derive(Clone)]
pub struct CreateHookKeyframesInterpolated<
    StrengthStart: crate::nodes::types::Float,
    StrengthEnd: crate::nodes::types::Float,
    Interpolation: crate::nodes::types::String,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    KeyframesCount: crate::nodes::types::Int,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.001
*/
    pub strength_start: StrengthStart,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.001
*/
    pub strength_end: StrengthEnd,
    ///No documentation.
    pub interpolation: Interpolation,
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
    /**No documentation.

**Metadata**:
  - Default: 5
  - Max: 100
  - Min: 2
  - Step: 1
*/
    pub keyframes_count: KeyframesCount,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub print_keyframes: PrintKeyframes,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKf>,
}
impl<
    StrengthStart: crate::nodes::types::Float,
    StrengthEnd: crate::nodes::types::Float,
    Interpolation: crate::nodes::types::String,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    KeyframesCount: crate::nodes::types::Int,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> CreateHookKeyframesInterpolated<
    StrengthStart,
    StrengthEnd,
    Interpolation,
    StartPercent,
    EndPercent,
    KeyframesCount,
    PrintKeyframes,
    PrevHookKf,
> {
    /// Create a new node.
    pub fn new(
        strength_start: StrengthStart,
        strength_end: StrengthEnd,
        interpolation: Interpolation,
        start_percent: StartPercent,
        end_percent: EndPercent,
        keyframes_count: KeyframesCount,
        print_keyframes: PrintKeyframes,
        prev_hook_kf: Option<PrevHookKf>,
    ) -> Self {
        Self {
            strength_start,
            strength_end,
            interpolation,
            start_percent,
            end_percent,
            keyframes_count,
            print_keyframes,
            prev_hook_kf,
        }
    }
}
impl<
    StrengthStart: crate::nodes::types::Float,
    StrengthEnd: crate::nodes::types::Float,
    Interpolation: crate::nodes::types::String,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    KeyframesCount: crate::nodes::types::Int,
    PrintKeyframes: crate::nodes::types::Boolean,
    PrevHookKf: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframesInterpolated<
    StrengthStart,
    StrengthEnd,
    Interpolation,
    StartPercent,
    EndPercent,
    KeyframesCount,
    PrintKeyframes,
    PrevHookKf,
> {
    type Output = crate::nodes::types::HookKeyframesOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("strength_start".to_string(), self.strength_start.clone().into());
        output.insert("strength_end".to_string(), self.strength_end.clone().into());
        output.insert("interpolation".to_string(), self.interpolation.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
            .insert("keyframes_count".to_string(), self.keyframes_count.clone().into());
        output
            .insert("print_keyframes".to_string(), self.print_keyframes.clone().into());
        if let Some(v) = &self.prev_hook_kf {
            output.insert("prev_hook_kf".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookKeyframesInterpolated";
    const DISPLAY_NAME: &'static str = "Create Hook Keyframes Interp.";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/scheduling";
}
///**Set Hook Keyframes**: No description.
#[derive(Clone)]
pub struct SetHookKeyframes<
    Hooks: crate::nodes::types::Hooks,
    HookKf: crate::nodes::types::HookKeyframes = crate::nodes::types::HookKeyframesOut,
> {
    ///No documentation.
    pub hooks: Hooks,
    ///No documentation.
    pub hook_kf: Option<HookKf>,
}
impl<
    Hooks: crate::nodes::types::Hooks,
    HookKf: crate::nodes::types::HookKeyframes,
> SetHookKeyframes<Hooks, HookKf> {
    /// Create a new node.
    pub fn new(hooks: Hooks, hook_kf: Option<HookKf>) -> Self {
        Self { hooks, hook_kf }
    }
}
impl<
    Hooks: crate::nodes::types::Hooks,
    HookKf: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode for SetHookKeyframes<Hooks, HookKf> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("hooks".to_string(), self.hooks.clone().into());
        if let Some(v) = &self.hook_kf {
            output.insert("hook_kf".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SetHookKeyframes";
    const DISPLAY_NAME: &'static str = "Set Hook Keyframes";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/scheduling";
}
