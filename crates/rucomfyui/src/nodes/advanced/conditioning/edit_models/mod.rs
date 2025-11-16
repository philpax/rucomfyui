//!`edit_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ReferenceLatent**: This node sets the guiding latent for an edit model. If the model supports it you can chain multiple to set multiple reference images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReferenceLatent<
    ConditioningParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent = crate::nodes::types::LatentOut,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    ///No documentation.
    pub latent: Option<LatentParam>,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> ReferenceLatent<ConditioningParam, LatentParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam, latent: Option<LatentParam>) -> Self {
        Self { conditioning, latent }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode for ReferenceLatent<ConditioningParam, LatentParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        if let Some(v) = &self.latent {
            output.insert("latent".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ReferenceLatent";
    const DISPLAY_NAME: &'static str = "ReferenceLatent";
    const DESCRIPTION: &'static str = "This node sets the guiding latent for an edit model. If the model supports it you can chain multiple to set multiple reference images.";
    const CATEGORY: &'static str = "advanced/conditioning/edit_models";
}
