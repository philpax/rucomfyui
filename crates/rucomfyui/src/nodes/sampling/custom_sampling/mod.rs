//!`custom_sampling` definitions/categories.
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
#[rustfmt::skip]
pub mod schedulers;
///**CFG Override**: Override cfg to a fixed value over a \[start, end\] percent (sigma) range. With multiple overrides, the one nearest the sampler wins on overlap.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CFGOverride<
    ModelParam: crate::nodes::types::Model,
    CfgParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
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
}
impl<
    ModelParam: crate::nodes::types::Model,
    CfgParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> CFGOverride<ModelParam, CfgParam, StartPercentParam, EndPercentParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        cfg: CfgParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            model,
            cfg,
            start_percent,
            end_percent,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    CfgParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for CFGOverride<ModelParam, CfgParam, StartPercentParam, EndPercentParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "CFGOverride";
    const DISPLAY_NAME: &'static str = "CFG Override";
    const DESCRIPTION: &'static str = "Override cfg to a fixed value over a [start, end] percent (sigma) range. With multiple overrides, the one nearest the sampler wins on overlap.";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
