//!`stable_cascade` definitions/categories.
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
/// Output types for nodes.
pub mod out {
    ///Output for [`StableCascade_SuperResolutionControlnet`](super::StableCascade_SuperResolutionControlnet).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct StableCascade_SuperResolutionControlnetOutput {
        ///No documentation.
        pub controlnet_input: crate::nodes::types::ImageOut,
        ///No documentation.
        pub stage_c: crate::nodes::types::LatentOut,
        ///No documentation.
        pub stage_b: crate::nodes::types::LatentOut,
    }
}
///**StableCascade_SuperResolutionControlnet**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StableCascade_SuperResolutionControlnet<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub vae: VaeParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> StableCascade_SuperResolutionControlnet<ImageParam, VaeParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, vae: VaeParam) -> Self {
        Self { image, vae }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
> crate::nodes::TypedNode
for StableCascade_SuperResolutionControlnet<ImageParam, VaeParam> {
    type Output = out::StableCascade_SuperResolutionControlnetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            controlnet_input: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            stage_c: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
            stage_b: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
    }
    const NAME: &'static str = "StableCascade_SuperResolutionControlnet";
    const DISPLAY_NAME: &'static str = "StableCascade_SuperResolutionControlnet";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/stable_cascade";
}
