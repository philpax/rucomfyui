//!`video_models` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**VideoLinearCFGGuidance**: No description.
pub struct VideoLinearCfgGuidance<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
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
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("min_cfg".to_string(), self.min_cfg.to_workflow_input());
        output
    }
    const NAME: &'static str = "VideoLinearCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoLinearCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
///**VideoTriangleCFGGuidance**: No description.
pub struct VideoTriangleCfgGuidance<
    Model: crate::nodes::types::Model,
    MinCfg: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
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
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("min_cfg".to_string(), self.min_cfg.to_workflow_input());
        output
    }
    const NAME: &'static str = "VideoTriangleCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoTriangleCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
