//!`guiders` definitions/categories.
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
///**Basic Guider**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
    const DISPLAY_NAME: &'static str = "Basic Guider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
///**CFG Guider**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CFGGuider<
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
> CFGGuider<ModelParam, PositiveParam, NegativeParam, CfgParam> {
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
for CFGGuider<ModelParam, PositiveParam, NegativeParam, CfgParam> {
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
    const DISPLAY_NAME: &'static str = "CFG Guider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
///**Dual CFG Guider**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DualCFGGuider<
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
> DualCFGGuider<
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
for DualCFGGuider<
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
    const DISPLAY_NAME: &'static str = "Dual CFG Guider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
///**Dual Model CFG Guider**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DualModelGuider<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    ModelNegativeParam: crate::nodes::types::Model = crate::nodes::types::ModelOut,
    NegativeParam: crate::nodes::types::Conditioning
        = crate::nodes::types::ConditioningOut,
> {
    ///Model used for the positive (conditional) pass.
    pub model: ModelParam,
    ///No documentation.
    pub positive: PositiveParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.1
*/
    pub cfg: CfgParam,
    ///Model used for the negative (unconditional) pass. Use the same model for ordinary CFG.
    pub model_negative: Option<ModelNegativeParam>,
    ///Negative conditioning run on the negative model. Leave unconnected for a text-free (image-only) unconditional pass.
    pub negative: Option<NegativeParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    ModelNegativeParam: crate::nodes::types::Model,
    NegativeParam: crate::nodes::types::Conditioning,
> DualModelGuider<
    ModelParam,
    PositiveParam,
    CfgParam,
    ModelNegativeParam,
    NegativeParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        positive: PositiveParam,
        cfg: CfgParam,
        model_negative: Option<ModelNegativeParam>,
        negative: Option<NegativeParam>,
    ) -> Self {
        Self {
            model,
            positive,
            cfg,
            model_negative,
            negative,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    CfgParam: crate::nodes::types::Float,
    ModelNegativeParam: crate::nodes::types::Model,
    NegativeParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for DualModelGuider<
    ModelParam,
    PositiveParam,
    CfgParam,
    ModelNegativeParam,
    NegativeParam,
> {
    type Output = crate::nodes::types::GuiderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("cfg".to_string(), self.cfg.clone().into());
        if let Some(v) = &self.model_negative {
            output.insert("model_negative".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative {
            output.insert("negative".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "DualModelGuider";
    const DISPLAY_NAME: &'static str = "Dual Model CFG Guider";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
///**Video Linear CFG Guidance**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VideoLinearCFGGuidance<
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
> VideoLinearCFGGuidance<ModelParam, MinCfgParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, min_cfg: MinCfgParam) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoLinearCFGGuidance<ModelParam, MinCfgParam> {
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
    const DISPLAY_NAME: &'static str = "Video Linear CFG Guidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
///**Video Triangle CFG Guidance**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VideoTriangleCFGGuidance<
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
> VideoTriangleCFGGuidance<ModelParam, MinCfgParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, min_cfg: MinCfgParam) -> Self {
        Self { model, min_cfg }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    MinCfgParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VideoTriangleCFGGuidance<ModelParam, MinCfgParam> {
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
    const DISPLAY_NAME: &'static str = "Video Triangle CFG Guidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/guiders";
}
