//!`video_models` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`VideoLinearCfgGuidance`](super::VideoLinearCfgGuidance).
    #[derive(Clone)]
    pub struct VideoLinearCfgGuidanceOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`VideoTriangleCfgGuidance`](super::VideoTriangleCfgGuidance).
    #[derive(Clone)]
    pub struct VideoTriangleCfgGuidanceOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
}
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
    type Output = out::VideoLinearCfgGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
    type Output = out::VideoTriangleCfgGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VideoTriangleCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoTriangleCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
