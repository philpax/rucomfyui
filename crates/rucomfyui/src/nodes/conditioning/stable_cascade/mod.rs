//!`stable_cascade` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**StableCascade_StageB_Conditioning**: No description.
#[derive(Clone)]
pub struct StableCascadeStageBConditioning<
    Conditioning: crate::nodes::types::Conditioning,
    StageC: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub stage_c: StageC,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StageC: crate::nodes::types::Latent,
> StableCascadeStageBConditioning<Conditioning, StageC> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning, stage_c: StageC) -> Self {
        Self { conditioning, stage_c }
    }
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StageC: crate::nodes::types::Latent,
> crate::nodes::TypedNode for StableCascadeStageBConditioning<Conditioning, StageC> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("stage_c".to_string(), self.stage_c.clone().into());
        output
    }
    const NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DISPLAY_NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/stable_cascade";
}
