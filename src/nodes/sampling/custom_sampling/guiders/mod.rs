//!`guiders` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`BasicGuider`](super::BasicGuider).
    #[derive(Clone)]
    pub struct BasicGuiderOutput {
        ///No documentation.
        pub guider: crate::nodes::types::GuiderOut,
    }
    ///Output for [`CfgGuider`](super::CfgGuider).
    #[derive(Clone)]
    pub struct CfgGuiderOutput {
        ///No documentation.
        pub guider: crate::nodes::types::GuiderOut,
    }
    ///Output for [`DualCfgGuider`](super::DualCfgGuider).
    #[derive(Clone)]
    pub struct DualCfgGuiderOutput {
        ///No documentation.
        pub guider: crate::nodes::types::GuiderOut,
    }
}
///**BasicGuider**
pub struct BasicGuider<
    Model: crate::nodes::types::Model,
    Conditioning: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub conditioning: Conditioning,
}
impl<
    Model: crate::nodes::types::Model,
    Conditioning: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for BasicGuider<Model, Conditioning> {
    type Output = out::BasicGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::types::GuiderOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "BasicGuider";
    const DISPLAY_NAME: &'static str = "BasicGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**CFGGuider**
pub struct CfgGuider<
    Model: crate::nodes::types::Model,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Cfg: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub cfg: Cfg,
}
impl<
    Model: crate::nodes::types::Model,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Cfg: crate::nodes::types::Float,
> crate::nodes::TypedNode for CfgGuider<Model, Positive, Negative, Cfg> {
    type Output = out::CfgGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::types::GuiderOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CFGGuider";
    const DISPLAY_NAME: &'static str = "CFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**DualCFGGuider**
pub struct DualCfgGuider<
    Model: crate::nodes::types::Model,
    Cond1: crate::nodes::types::Conditioning,
    Cond2: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    CfgConds: crate::nodes::types::Float,
    CfgCond2Negative: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub cond_1: Cond1,
    ///No documentation.
    pub cond_2: Cond2,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub cfg_conds: CfgConds,
    ///No documentation.
    pub cfg_cond_2_negative: CfgCond2Negative,
}
impl<
    Model: crate::nodes::types::Model,
    Cond1: crate::nodes::types::Conditioning,
    Cond2: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    CfgConds: crate::nodes::types::Float,
    CfgCond2Negative: crate::nodes::types::Float,
> crate::nodes::TypedNode
for DualCfgGuider<Model, Cond1, Cond2, Negative, CfgConds, CfgCond2Negative> {
    type Output = out::DualCfgGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::types::GuiderOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "DualCFGGuider";
    const DISPLAY_NAME: &'static str = "DualCFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
