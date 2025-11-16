//!`Runway` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Runway Text to Image**: Generate an image from a text prompt using Runway's Gen 4 model. You can also include reference image to guide the generation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RunwayTextToImageNode<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Text prompt for the generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Optional reference image to guide the generation
    pub reference_image: Option<ReferenceImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image,
> RunwayTextToImageNode<PromptParam, ReferenceImageParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        reference_image: Option<ReferenceImageParam>,
    ) -> Self {
        Self { prompt, reference_image }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for RunwayTextToImageNode<PromptParam, ReferenceImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RunwayTextToImageNode";
    const DISPLAY_NAME: &'static str = "Runway Text to Image";
    const DESCRIPTION: &'static str = "Generate an image from a text prompt using Runway's Gen 4 model. You can also include reference image to guide the generation.";
    const CATEGORY: &'static str = "api node/image/Runway";
}
