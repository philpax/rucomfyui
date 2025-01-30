//!`hooks` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod clip;
pub mod combine;
pub mod cond_pair;
pub mod cond_single;
pub mod create;
pub mod scheduling;
/// Output types for nodes.
pub mod out {
    ///Output for [`ConditioningTimestepsRange`](super::ConditioningTimestepsRange).
    #[derive(Clone)]
    pub struct ConditioningTimestepsRangeOutput {
        ///No documentation.
        pub timesteps_range: crate::nodes::types::TimestepsRangeOut,
        ///No documentation.
        pub before_range: crate::nodes::types::TimestepsRangeOut,
        ///No documentation.
        pub after_range: crate::nodes::types::TimestepsRangeOut,
    }
}
///**Timesteps Range**: No description.
#[derive(Clone)]
pub struct ConditioningTimestepsRange<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
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
}
impl<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> ConditioningTimestepsRange<StartPercent, EndPercent> {
    /// Create a new node.
    pub fn new(start_percent: StartPercent, end_percent: EndPercent) -> Self {
        Self { start_percent, end_percent }
    }
}
impl<
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> crate::nodes::TypedNode for ConditioningTimestepsRange<StartPercent, EndPercent> {
    type Output = out::ConditioningTimestepsRangeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            timesteps_range: crate::nodes::types::TimestepsRangeOut::from_dynamic(
                node_id,
                0u32,
            ),
            before_range: crate::nodes::types::TimestepsRangeOut::from_dynamic(
                node_id,
                1u32,
            ),
            after_range: crate::nodes::types::TimestepsRangeOut::from_dynamic(
                node_id,
                2u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningTimestepsRange";
    const DISPLAY_NAME: &'static str = "Timesteps Range";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks";
}
