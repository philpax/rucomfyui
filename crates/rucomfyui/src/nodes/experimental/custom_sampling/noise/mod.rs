//!`noise` definitions/categories.
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
///**AddNoise**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AddNoise<
    ModelParam: crate::nodes::types::Model,
    NoiseParam: crate::nodes::types::Noise,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub noise: NoiseParam,
    ///No documentation.
    pub sigmas: SigmasParam,
    ///No documentation.
    pub latent_image: LatentImageParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    NoiseParam: crate::nodes::types::Noise,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> AddNoise<ModelParam, NoiseParam, SigmasParam, LatentImageParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        noise: NoiseParam,
        sigmas: SigmasParam,
        latent_image: LatentImageParam,
    ) -> Self {
        Self {
            model,
            noise,
            sigmas,
            latent_image,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    NoiseParam: crate::nodes::types::Noise,
    SigmasParam: crate::nodes::types::Sigmas,
    LatentImageParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for AddNoise<ModelParam, NoiseParam, SigmasParam, LatentImageParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("noise".to_string(), self.noise.clone().into());
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("latent_image".to_string(), self.latent_image.clone().into());
        output
    }
    const NAME: &'static str = "AddNoise";
    const DISPLAY_NAME: &'static str = "AddNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/custom_sampling/noise";
}
