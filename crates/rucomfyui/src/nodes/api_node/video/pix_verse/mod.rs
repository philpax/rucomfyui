//!`PixVerse` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**PixVerse Image to Video**: Generates videos based on prompt and output_size.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PixverseImageToVideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate
        = crate::nodes::types::PixverseTemplateOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt for the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed for video generation.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///An optional template to influence style of generation, created by the PixVerse Template node.
    pub pixverse_template: Option<PixverseTemplateParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate,
> PixverseImageToVideoNode<
    ImageParam,
    PromptParam,
    SeedParam,
    NegativePromptParam,
    PixverseTemplateParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
        pixverse_template: Option<PixverseTemplateParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            seed,
            negative_prompt,
            pixverse_template,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate,
> crate::nodes::TypedNode
for PixverseImageToVideoNode<
    ImageParam,
    PromptParam,
    SeedParam,
    NegativePromptParam,
    PixverseTemplateParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pixverse_template {
            output.insert("pixverse_template".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PixverseImageToVideoNode";
    const DISPLAY_NAME: &'static str = "PixVerse Image to Video";
    const DESCRIPTION: &'static str = "Generates videos based on prompt and output_size.";
    const CATEGORY: &'static str = "api node/video/PixVerse";
}
///**PixVerse Template**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PixverseTemplateNode {}
impl PixverseTemplateNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for PixverseTemplateNode {
    type Output = crate::nodes::types::PixverseTemplateOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "PixverseTemplateNode";
    const DISPLAY_NAME: &'static str = "PixVerse Template";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/video/PixVerse";
}
///**PixVerse Text to Video**: Generates videos based on prompt and output_size.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PixverseTextToVideoNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate
        = crate::nodes::types::PixverseTemplateOut,
> {
    /**Prompt for the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed for video generation.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///An optional template to influence style of generation, created by the PixVerse Template node.
    pub pixverse_template: Option<PixverseTemplateParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate,
> PixverseTextToVideoNode<
    PromptParam,
    SeedParam,
    NegativePromptParam,
    PixverseTemplateParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
        pixverse_template: Option<PixverseTemplateParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            negative_prompt,
            pixverse_template,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    PixverseTemplateParam: crate::nodes::types::PixverseTemplate,
> crate::nodes::TypedNode
for PixverseTextToVideoNode<
    PromptParam,
    SeedParam,
    NegativePromptParam,
    PixverseTemplateParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pixverse_template {
            output.insert("pixverse_template".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PixverseTextToVideoNode";
    const DISPLAY_NAME: &'static str = "PixVerse Text to Video";
    const DESCRIPTION: &'static str = "Generates videos based on prompt and output_size.";
    const CATEGORY: &'static str = "api node/video/PixVerse";
}
///**PixVerse Transition Video**: Generates videos based on prompt and output_size.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PixverseTransitionVideoNode<
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub first_frame: FirstFrameParam,
    ///No documentation.
    pub last_frame: LastFrameParam,
    /**Prompt for the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed for video generation.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> PixverseTransitionVideoNode<
    FirstFrameParam,
    LastFrameParam,
    PromptParam,
    SeedParam,
    NegativePromptParam,
> {
    /// Create a new node.
    pub fn new(
        first_frame: FirstFrameParam,
        last_frame: LastFrameParam,
        prompt: PromptParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
    ) -> Self {
        Self {
            first_frame,
            last_frame,
            prompt,
            seed,
            negative_prompt,
        }
    }
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for PixverseTransitionVideoNode<
    FirstFrameParam,
    LastFrameParam,
    PromptParam,
    SeedParam,
    NegativePromptParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("last_frame".to_string(), self.last_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PixverseTransitionVideoNode";
    const DISPLAY_NAME: &'static str = "PixVerse Transition Video";
    const DESCRIPTION: &'static str = "Generates videos based on prompt and output_size.";
    const CATEGORY: &'static str = "api node/video/PixVerse";
}
