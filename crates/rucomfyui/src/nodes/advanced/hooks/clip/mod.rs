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
    Clip: crate::nodes::types::Clip,
    ApplyToConds: crate::nodes::types::Boolean,
    ScheduleClip: crate::nodes::types::Boolean,
    Hooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub apply_to_conds: ApplyToConds,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub schedule_clip: ScheduleClip,
    ///No documentation.
    pub hooks: Option<Hooks>,
}
impl<
    Clip: crate::nodes::types::Clip,
    ApplyToConds: crate::nodes::types::Boolean,
    ScheduleClip: crate::nodes::types::Boolean,
    Hooks: crate::nodes::types::Hooks,
> SetClipHooks<Clip, ApplyToConds, ScheduleClip, Hooks> {
    /// Create a new node.
    pub fn new(
        clip: Clip,
        apply_to_conds: ApplyToConds,
        schedule_clip: ScheduleClip,
        hooks: Option<Hooks>,
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
    Clip: crate::nodes::types::Clip,
    ApplyToConds: crate::nodes::types::Boolean,
    ScheduleClip: crate::nodes::types::Boolean,
    Hooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode for SetClipHooks<Clip, ApplyToConds, ScheduleClip, Hooks> {
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
