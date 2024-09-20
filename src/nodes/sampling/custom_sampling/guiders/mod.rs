//!`guiders` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**BasicGuider**: No description.
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
> BasicGuider<Model, Conditioning> {
    /// Create a new node.
    pub fn new(model: Model, conditioning: Conditioning) -> Self {
        Self { model, conditioning }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Conditioning: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for BasicGuider<Model, Conditioning> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output
    }
    const NAME: &'static str = "BasicGuider";
    const DISPLAY_NAME: &'static str = "BasicGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**CFGGuider**: No description.
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
> CfgGuider<Model, Positive, Negative, Cfg> {
    /// Create a new node.
    pub fn new(model: Model, positive: Positive, negative: Negative, cfg: Cfg) -> Self {
        Self {
            model,
            positive,
            negative,
            cfg,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Cfg: crate::nodes::types::Float,
> crate::nodes::TypedNode for CfgGuider<Model, Positive, Negative, Cfg> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("positive".to_string(), self.positive.to_workflow_input());
        output.insert("negative".to_string(), self.negative.to_workflow_input());
        output.insert("cfg".to_string(), self.cfg.to_workflow_input());
        output
    }
    const NAME: &'static str = "CFGGuider";
    const DISPLAY_NAME: &'static str = "CFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**DualCFGGuider**: No description.
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
> DualCfgGuider<Model, Cond1, Cond2, Negative, CfgConds, CfgCond2Negative> {
    /// Create a new node.
    pub fn new(
        model: Model,
        cond_1: Cond1,
        cond_2: Cond2,
        negative: Negative,
        cfg_conds: CfgConds,
        cfg_cond_2_negative: CfgCond2Negative,
    ) -> Self {
        Self {
            model,
            cond_1,
            cond_2,
            negative,
            cfg_conds,
            cfg_cond_2_negative,
        }
    }
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
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("cond1".to_string(), self.cond_1.to_workflow_input());
        output.insert("cond2".to_string(), self.cond_2.to_workflow_input());
        output.insert("negative".to_string(), self.negative.to_workflow_input());
        output.insert("cfg_conds".to_string(), self.cfg_conds.to_workflow_input());
        output
            .insert(
                "cfg_cond2_negative".to_string(),
                self.cfg_cond_2_negative.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "DualCFGGuider";
    const DISPLAY_NAME: &'static str = "DualCFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
