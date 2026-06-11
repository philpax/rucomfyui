//!`schedulers` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Ideogram 4 Scheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Ideogram4Scheduler<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    MuParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 200
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 8192
  - Min: 256
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 8192
  - Min: 256
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: -10
  - Step: 0.05
*/
    pub mu: MuParam,
    /**No documentation.

**Metadata**:
  - Default: 1.75
  - Max: 5
  - Min: 0.1
  - Step: 0.05
*/
    pub std: StdParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    MuParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> Ideogram4Scheduler<StepsParam, WidthParam, HeightParam, MuParam, StdParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        width: WidthParam,
        height: HeightParam,
        mu: MuParam,
        std: StdParam,
    ) -> Self {
        Self {
            steps,
            width,
            height,
            mu,
            std,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    MuParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for Ideogram4Scheduler<StepsParam, WidthParam, HeightParam, MuParam, StdParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("mu".to_string(), self.mu.clone().into());
        output.insert("std".to_string(), self.std.clone().into());
        output
    }
    const NAME: &'static str = "Ideogram4Scheduler";
    const DISPLAY_NAME: &'static str = "Ideogram 4 Scheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
