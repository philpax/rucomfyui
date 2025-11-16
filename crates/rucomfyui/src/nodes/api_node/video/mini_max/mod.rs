//!`MiniMax` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**MiniMax Hailuo Video**: Generates videos from prompt, with optional start frame using the new MiniMax Hailuo-02 model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MinimaxHailuoVideoNode<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FirstFrameImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    PromptOptimizerParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Text prompt to guide the video generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt_text: PromptTextParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    ///Optional image to use as the first frame to generate a video.
    pub first_frame_image: Option<FirstFrameImageParam>,
    /**Optimize prompt to improve generation quality when needed.

**Metadata**:
  - Default: true
*/
    pub prompt_optimizer: Option<PromptOptimizerParam>,
}
impl<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    FirstFrameImageParam: crate::nodes::types::Image,
    PromptOptimizerParam: crate::nodes::types::Boolean,
> MinimaxHailuoVideoNode<
    PromptTextParam,
    SeedParam,
    FirstFrameImageParam,
    PromptOptimizerParam,
> {
    /// Create a new node.
    pub fn new(
        prompt_text: PromptTextParam,
        seed: Option<SeedParam>,
        first_frame_image: Option<FirstFrameImageParam>,
        prompt_optimizer: Option<PromptOptimizerParam>,
    ) -> Self {
        Self {
            prompt_text,
            seed,
            first_frame_image,
            prompt_optimizer,
        }
    }
}
impl<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    FirstFrameImageParam: crate::nodes::types::Image,
    PromptOptimizerParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MinimaxHailuoVideoNode<
    PromptTextParam,
    SeedParam,
    FirstFrameImageParam,
    PromptOptimizerParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.first_frame_image {
            output.insert("first_frame_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.prompt_optimizer {
            output.insert("prompt_optimizer".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MinimaxHailuoVideoNode";
    const DISPLAY_NAME: &'static str = "MiniMax Hailuo Video";
    const DESCRIPTION: &'static str = "Generates videos from prompt, with optional start frame using the new MiniMax Hailuo-02 model.";
    const CATEGORY: &'static str = "api node/video/MiniMax";
}
///**MiniMax Image to Video**: Generates videos synchronously based on an image and prompt, and optional parameters.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MinimaxImageToVideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///Image to use as first frame of video generation
    pub image: ImageParam,
    /**Text prompt to guide the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt_text: PromptTextParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> MinimaxImageToVideoNode<ImageParam, PromptTextParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt_text: PromptTextParam,
        seed: Option<SeedParam>,
    ) -> Self {
        Self { image, prompt_text, seed }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MinimaxImageToVideoNode<ImageParam, PromptTextParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MinimaxImageToVideoNode";
    const DISPLAY_NAME: &'static str = "MiniMax Image to Video";
    const DESCRIPTION: &'static str = "Generates videos synchronously based on an image and prompt, and optional parameters.";
    const CATEGORY: &'static str = "api node/video/MiniMax";
}
///**MiniMax Text to Video**: Generates videos synchronously based on a prompt, and optional parameters.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MinimaxTextToVideoNode<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Text prompt to guide the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt_text: PromptTextParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> MinimaxTextToVideoNode<PromptTextParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt_text: PromptTextParam, seed: Option<SeedParam>) -> Self {
        Self { prompt_text, seed }
    }
}
impl<
    PromptTextParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for MinimaxTextToVideoNode<PromptTextParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MinimaxTextToVideoNode";
    const DISPLAY_NAME: &'static str = "MiniMax Text to Video";
    const DESCRIPTION: &'static str = "Generates videos synchronously based on a prompt, and optional parameters.";
    const CATEGORY: &'static str = "api node/video/MiniMax";
}
