//!`clip` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Set CLIP Hooks**: No description.
#[derive(Clone)]
pub struct SetClipHooks<
    ClipParam: crate::nodes::types::Clip,
    ApplyToCondsParam: crate::nodes::types::Boolean,
    ScheduleClipParam: crate::nodes::types::Boolean,
    HooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub apply_to_conds: ApplyToCondsParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub schedule_clip: ScheduleClipParam,
    ///No documentation.
    pub hooks: Option<HooksParam>,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ApplyToCondsParam: crate::nodes::types::Boolean,
    ScheduleClipParam: crate::nodes::types::Boolean,
    HooksParam: crate::nodes::types::Hooks,
> SetClipHooks<ClipParam, ApplyToCondsParam, ScheduleClipParam, HooksParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        apply_to_conds: ApplyToCondsParam,
        schedule_clip: ScheduleClipParam,
        hooks: Option<HooksParam>,
    ) -> Self {
        Self {
            clip,
            apply_to_conds,
            schedule_clip,
            hooks,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ApplyToCondsParam: crate::nodes::types::Boolean,
    ScheduleClipParam: crate::nodes::types::Boolean,
    HooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for SetClipHooks<ClipParam, ApplyToCondsParam, ScheduleClipParam, HooksParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("apply_to_conds".to_string(), self.apply_to_conds.clone().into());
        output.insert("schedule_clip".to_string(), self.schedule_clip.clone().into());
        if let Some(v) = &self.hooks {
            output.insert("hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SetClipHooks";
    const DISPLAY_NAME: &'static str = "Set CLIP Hooks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/clip";
}
