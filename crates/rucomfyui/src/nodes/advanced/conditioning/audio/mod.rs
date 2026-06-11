//!`audio` definitions/categories.
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
///**Reference Audio**: This node sets the reference audio for ace step 1.5
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReferenceTimbreAudio<
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
> ReferenceTimbreAudio<ConditioningParam, LatentParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam, latent: Option<LatentParam>) -> Self {
        Self { conditioning, latent }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode for ReferenceTimbreAudio<ConditioningParam, LatentParam> {
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
    const NAME: &'static str = "ReferenceTimbreAudio";
    const DISPLAY_NAME: &'static str = "Reference Audio";
    const DESCRIPTION: &'static str = "This node sets the reference audio for ace step 1.5";
    const CATEGORY: &'static str = "advanced/conditioning/audio";
}
