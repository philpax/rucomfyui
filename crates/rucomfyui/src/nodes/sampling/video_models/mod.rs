//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**VideoLinearCFGGuidance**: No description.
#[derive(Clone)]
pub struct VideoLinearCfgGuidance<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.5
*/
    pub min_cfg: MinCfgParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> VideoLinearCfgGuidance<ModelParam, MinCfgParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, min_cfg: MinCfgParam) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoLinearCfgGuidance<ModelParam, MinCfgParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.5
*/
    pub min_cfg: MinCfgParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> VideoTriangleCfgGuidance<ModelParam, MinCfgParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, min_cfg: MinCfgParam) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoTriangleCfgGuidance<ModelParam, MinCfgParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
