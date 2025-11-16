//!`Sora` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**OpenAI Sora - Video**: OpenAI video and audio generation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIVideoSora2<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Guiding text; may be empty if an input image is present.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> OpenAIVideoSora2<PromptParam, ImageParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        image: Option<ImageParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self { prompt, image, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for OpenAIVideoSora2<PromptParam, ImageParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIVideoSora2";
    const DISPLAY_NAME: &'static str = "OpenAI Sora - Video";
    const DESCRIPTION: &'static str = "OpenAI video and audio generation.";
    const CATEGORY: &'static str = "api node/video/Sora";
}
