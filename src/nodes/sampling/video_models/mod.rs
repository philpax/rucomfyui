//!video_models
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**VideoLinearCFGGuidance**
pub struct VideoLinearCfgGuidance<
    Model: crate::nodes::Model,
    MinCfg: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
}
///Output for [`VideoLinearCfgGuidance`].
#[derive(Clone)]
pub struct VideoLinearCfgGuidanceOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, MinCfg: crate::nodes::Float> crate::nodes::TypedNode
for VideoLinearCfgGuidance<Model, MinCfg> {
    type Output = VideoLinearCfgGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VideoLinearCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoLinearCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
///**VideoTriangleCFGGuidance**
pub struct VideoTriangleCfgGuidance<
    Model: crate::nodes::Model,
    MinCfg: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
}
///Output for [`VideoTriangleCfgGuidance`].
#[derive(Clone)]
pub struct VideoTriangleCfgGuidanceOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, MinCfg: crate::nodes::Float> crate::nodes::TypedNode
for VideoTriangleCfgGuidance<Model, MinCfg> {
    type Output = VideoTriangleCfgGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VideoTriangleCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoTriangleCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
