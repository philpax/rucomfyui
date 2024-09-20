//!`guiders` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**BasicGuider**
pub struct BasicGuider<
    Model: crate::nodes::Model,
    Conditioning: crate::nodes::Conditioning,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub conditioning: Conditioning,
}
///Output for [`BasicGuider`].
#[derive(Clone)]
pub struct BasicGuiderOutput {
    ///No documentation.
    pub guider: crate::nodes::GuiderOut,
}
impl<
    Model: crate::nodes::Model,
    Conditioning: crate::nodes::Conditioning,
> crate::nodes::TypedNode for BasicGuider<Model, Conditioning> {
    type Output = BasicGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::GuiderOut {
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
    Model: crate::nodes::Model,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Cfg: crate::nodes::Float,
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
///Output for [`CfgGuider`].
#[derive(Clone)]
pub struct CfgGuiderOutput {
    ///No documentation.
    pub guider: crate::nodes::GuiderOut,
}
impl<
    Model: crate::nodes::Model,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Cfg: crate::nodes::Float,
> crate::nodes::TypedNode for CfgGuider<Model, Positive, Negative, Cfg> {
    type Output = CfgGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::GuiderOut {
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
    Model: crate::nodes::Model,
    Cond1: crate::nodes::Conditioning,
    Cond2: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    CfgConds: crate::nodes::Float,
    CfgCond2Negative: crate::nodes::Float,
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
///Output for [`DualCfgGuider`].
#[derive(Clone)]
pub struct DualCfgGuiderOutput {
    ///No documentation.
    pub guider: crate::nodes::GuiderOut,
}
impl<
    Model: crate::nodes::Model,
    Cond1: crate::nodes::Conditioning,
    Cond2: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    CfgConds: crate::nodes::Float,
    CfgCond2Negative: crate::nodes::Float,
> crate::nodes::TypedNode
for DualCfgGuider<Model, Cond1, Cond2, Negative, CfgConds, CfgCond2Negative> {
    type Output = DualCfgGuiderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            guider: crate::nodes::GuiderOut {
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
