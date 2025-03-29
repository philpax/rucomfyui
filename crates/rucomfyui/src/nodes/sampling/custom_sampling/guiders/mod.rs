//!`guiders` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**BasicGuider**: No description.
#[derive(Clone)]
pub struct BasicGuider<
    ModelParam: crate::nodes::types::Model,
    ConditioningParam: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub conditioning: ConditioningParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ConditioningParam: crate::nodes::types::Conditioning,
> BasicGuider<ModelParam, ConditioningParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, conditioning: ConditioningParam) -> Self {
        Self { model, conditioning }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ConditioningParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for BasicGuider<ModelParam, ConditioningParam> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output
    }
    const NAME: &'static str = "BasicGuider";
    const DISPLAY_NAME: &'static str = "BasicGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**CFGGuider**: No description.
#[derive(Clone)]
pub struct CfgGuider<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
> CfgGuider<ModelParam, PositiveParam, NegativeParam, CfgParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        positive: PositiveParam,
        negative: NegativeParam,
        cfg: CfgParam,
    ) -> Self {
        Self {
            model,
            positive,
            negative,
            cfg,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for CfgGuider<ModelParam, PositiveParam, NegativeParam, CfgParam> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        output
    }
    const NAME: &'static str = "CFGGuider";
    const DISPLAY_NAME: &'static str = "CFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
///**DualCFGGuider**: No description.
#[derive(Clone)]
pub struct DualCfgGuider<
    ModelParam: crate::nodes::types::Model,
    Cond1Param: crate::nodes::types::Conditioning,
    Cond2Param: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgCondsParam: crate::nodes::types::Float,
    CfgCond2NegativeParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub cond_1: Cond1Param,
    ///No documentation.
    pub cond_2: Cond2Param,
    ///No documentation.
    pub negative: NegativeParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg_conds: CfgCondsParam,
    /**No documentation.

**Metadata**:
  - Default: 8
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg_cond_2_negative: CfgCond2NegativeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    Cond1Param: crate::nodes::types::Conditioning,
    Cond2Param: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgCondsParam: crate::nodes::types::Float,
    CfgCond2NegativeParam: crate::nodes::types::Float,
> DualCfgGuider<
    ModelParam,
    Cond1Param,
    Cond2Param,
    NegativeParam,
    CfgCondsParam,
    CfgCond2NegativeParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        cond_1: Cond1Param,
        cond_2: Cond2Param,
        negative: NegativeParam,
        cfg_conds: CfgCondsParam,
        cfg_cond_2_negative: CfgCond2NegativeParam,
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
    ModelParam: crate::nodes::types::Model,
    Cond1Param: crate::nodes::types::Conditioning,
    Cond2Param: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    CfgCondsParam: crate::nodes::types::Float,
    CfgCond2NegativeParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for DualCfgGuider<
    ModelParam,
    Cond1Param,
    Cond2Param,
    NegativeParam,
    CfgCondsParam,
    CfgCond2NegativeParam,
> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("cond1".to_string(), self.cond_1.clone().into());
        output.insert("cond2".to_string(), self.cond_2.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("cfg_conds".to_string(), self.cfg_conds.clone().into());
        output
            .insert(
                "cfg_cond2_negative".to_string(),
                self.cfg_cond_2_negative.clone().into(),
            );
        output
    }
    const NAME: &'static str = "DualCFGGuider";
    const DISPLAY_NAME: &'static str = "DualCFGGuider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/guiders";
}
