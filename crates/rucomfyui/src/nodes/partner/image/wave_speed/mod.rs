//!`WaveSpeed` definitions/categories.
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
///**WaveSpeed Image Upscale**: Boost image resolution and quality, upscaling photos to 4K or 8K for sharp, detailed results.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WavespeedImageUpscaleNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> WavespeedImageUpscaleNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for WavespeedImageUpscaleNode<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "WavespeedImageUpscaleNode";
    const DISPLAY_NAME: &'static str = "WaveSpeed Image Upscale";
    const DESCRIPTION: &'static str = "Boost image resolution and quality, upscaling photos to 4K or 8K for sharp, detailed results.";
    const CATEGORY: &'static str = "partner/image/WaveSpeed";
}
