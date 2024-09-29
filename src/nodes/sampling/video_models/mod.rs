//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**VideoLinearCFGGuidance**: No description.
#[derive(Clone)]
pub struct VideoLinearCfgGuidance<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.5
*/
    pub min_cfg: MinCfg,
}
impl<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> VideoLinearCfgGuidance<Model, MinCfg> {
    /// Create a new node.
    pub fn new(model: Model, min_cfg: MinCfg) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoLinearCfgGuidance<Model, MinCfg> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("min_cfg".to_string(), self.min_cfg.clone().into());
        output
    }
    const NAME: &'static str = "VideoLinearCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoLinearCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
///**VideoTriangleCFGGuidance**: No description.
#[derive(Clone)]
pub struct VideoTriangleCfgGuidance<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.5
*/
    pub min_cfg: MinCfg,
}
impl<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> VideoTriangleCfgGuidance<Model, MinCfg> {
    /// Create a new node.
    pub fn new(model: Model, min_cfg: MinCfg) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoTriangleCfgGuidance<Model, MinCfg> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("min_cfg".to_string(), self.min_cfg.clone().into());
        output
    }
    const NAME: &'static str = "VideoTriangleCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoTriangleCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
