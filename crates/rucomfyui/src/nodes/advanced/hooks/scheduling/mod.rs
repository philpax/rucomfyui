//!`scheduling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Create Hook Keyframe**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CreateHookKeyframe<
    StrengthMultParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    PrevHookKfParam: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_mult: StrengthMultParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKfParam>,
}
impl<
    StrengthMultParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> CreateHookKeyframe<StrengthMultParam, StartPercentParam, PrevHookKfParam> {
    /// Create a new node.
    pub fn new(
        strength_mult: StrengthMultParam,
        start_percent: StartPercentParam,
        prev_hook_kf: Option<PrevHookKfParam>,
    ) -> Self {
        Self {
            strength_mult,
            start_percent,
            prev_hook_kf,
        }
    }
}
impl<
    StrengthMultParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframe<StrengthMultParam, StartPercentParam, PrevHookKfParam> {
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
#[allow(non_camel_case_types)]
pub struct CreateHookKeyframesFromFloats<
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
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
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub print_keyframes: PrintKeyframesParam,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKfParam>,
}
impl<
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> CreateHookKeyframesFromFloats<
    StartPercentParam,
    EndPercentParam,
    PrintKeyframesParam,
    PrevHookKfParam,
> {
    /// Create a new node.
    pub fn new(
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        print_keyframes: PrintKeyframesParam,
        prev_hook_kf: Option<PrevHookKfParam>,
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
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframesFromFloats<
    StartPercentParam,
    EndPercentParam,
    PrintKeyframesParam,
    PrevHookKfParam,
> {
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
#[allow(non_camel_case_types)]
pub struct CreateHookKeyframesInterpolated<
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    InterpolationParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    KeyframesCountParam: crate::nodes::types::Int,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.001
*/
    pub strength_start: StrengthStartParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.001
*/
    pub strength_end: StrengthEndParam,
    ///No documentation.
    pub interpolation: InterpolationParam,
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
    /**No documentation.

**Metadata**:
  - Default: 5
  - Max: 100
  - Min: 2
  - Step: 1
*/
    pub keyframes_count: KeyframesCountParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub print_keyframes: PrintKeyframesParam,
    ///No documentation.
    pub prev_hook_kf: Option<PrevHookKfParam>,
}
impl<
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    InterpolationParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    KeyframesCountParam: crate::nodes::types::Int,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> CreateHookKeyframesInterpolated<
    StrengthStartParam,
    StrengthEndParam,
    InterpolationParam,
    StartPercentParam,
    EndPercentParam,
    KeyframesCountParam,
    PrintKeyframesParam,
    PrevHookKfParam,
> {
    /// Create a new node.
    pub fn new(
        strength_start: StrengthStartParam,
        strength_end: StrengthEndParam,
        interpolation: InterpolationParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        keyframes_count: KeyframesCountParam,
        print_keyframes: PrintKeyframesParam,
        prev_hook_kf: Option<PrevHookKfParam>,
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
    StrengthStartParam: crate::nodes::types::Float,
    StrengthEndParam: crate::nodes::types::Float,
    InterpolationParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    KeyframesCountParam: crate::nodes::types::Int,
    PrintKeyframesParam: crate::nodes::types::Boolean,
    PrevHookKfParam: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode
for CreateHookKeyframesInterpolated<
    StrengthStartParam,
    StrengthEndParam,
    InterpolationParam,
    StartPercentParam,
    EndPercentParam,
    KeyframesCountParam,
    PrintKeyframesParam,
    PrevHookKfParam,
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
#[allow(non_camel_case_types)]
pub struct SetHookKeyframes<
    HooksParam: crate::nodes::types::Hooks,
    HookKfParam: crate::nodes::types::HookKeyframes
        = crate::nodes::types::HookKeyframesOut,
> {
    ///No documentation.
    pub hooks: HooksParam,
    ///No documentation.
    pub hook_kf: Option<HookKfParam>,
}
impl<
    HooksParam: crate::nodes::types::Hooks,
    HookKfParam: crate::nodes::types::HookKeyframes,
> SetHookKeyframes<HooksParam, HookKfParam> {
    /// Create a new node.
    pub fn new(hooks: HooksParam, hook_kf: Option<HookKfParam>) -> Self {
        Self { hooks, hook_kf }
    }
}
impl<
    HooksParam: crate::nodes::types::Hooks,
    HookKfParam: crate::nodes::types::HookKeyframes,
> crate::nodes::TypedNode for SetHookKeyframes<HooksParam, HookKfParam> {
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
