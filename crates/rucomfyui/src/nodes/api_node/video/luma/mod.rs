//!`Luma` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Luma Concepts**: Camera Concepts for use with Luma Text to Video and Luma Image to Video nodes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaConceptsNode<
    LumaConceptsParam: crate::nodes::types::LumaConcepts
        = crate::nodes::types::LumaConceptsOut,
> {
    ///Optional Camera Concepts to add to the ones chosen here.
    pub luma_concepts: Option<LumaConceptsParam>,
}
impl<
    LumaConceptsParam: crate::nodes::types::LumaConcepts,
> LumaConceptsNode<LumaConceptsParam> {
    /// Create a new node.
    pub fn new(luma_concepts: Option<LumaConceptsParam>) -> Self {
        Self { luma_concepts }
    }
}
impl<LumaConceptsParam: crate::nodes::types::LumaConcepts> crate::nodes::TypedNode
for LumaConceptsNode<LumaConceptsParam> {
    type Output = crate::nodes::types::LumaConceptsOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.luma_concepts {
            output.insert("luma_concepts".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LumaConceptsNode";
    const DISPLAY_NAME: &'static str = "Luma Concepts";
    const DESCRIPTION: &'static str = "Camera Concepts for use with Luma Text to Video and Luma Image to Video nodes.";
    const CATEGORY: &'static str = "api node/video/Luma";
}
///**Luma Image to Video**: Generates videos synchronously based on prompt, input images, and output_size.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaImageToVideoNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    FirstImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    LastImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    LumaConceptsParam: crate::nodes::types::LumaConcepts
        = crate::nodes::types::LumaConceptsOut,
> {
    /**Prompt for the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///First frame of generated video.
    pub first_image: Option<FirstImageParam>,
    ///Last frame of generated video.
    pub last_image: Option<LastImageParam>,
    ///Optional Camera Concepts to dictate camera motion via the Luma Concepts node.
    pub luma_concepts: Option<LumaConceptsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    FirstImageParam: crate::nodes::types::Image,
    LastImageParam: crate::nodes::types::Image,
    LumaConceptsParam: crate::nodes::types::LumaConcepts,
> LumaImageToVideoNode<
    PromptParam,
    SeedParam,
    FirstImageParam,
    LastImageParam,
    LumaConceptsParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        first_image: Option<FirstImageParam>,
        last_image: Option<LastImageParam>,
        luma_concepts: Option<LumaConceptsParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            first_image,
            last_image,
            luma_concepts,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    FirstImageParam: crate::nodes::types::Image,
    LastImageParam: crate::nodes::types::Image,
    LumaConceptsParam: crate::nodes::types::LumaConcepts,
> crate::nodes::TypedNode
for LumaImageToVideoNode<
    PromptParam,
    SeedParam,
    FirstImageParam,
    LastImageParam,
    LumaConceptsParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.first_image {
            output.insert("first_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.last_image {
            output.insert("last_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.luma_concepts {
            output.insert("luma_concepts".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LumaImageToVideoNode";
    const DISPLAY_NAME: &'static str = "Luma Image to Video";
    const DESCRIPTION: &'static str = "Generates videos synchronously based on prompt, input images, and output_size.";
    const CATEGORY: &'static str = "api node/video/Luma";
}
///**Luma Text to Video**: Generates videos synchronously based on prompt and output_size.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaVideoNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    LumaConceptsParam: crate::nodes::types::LumaConcepts
        = crate::nodes::types::LumaConceptsOut,
> {
    /**Prompt for the video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///Optional Camera Concepts to dictate camera motion via the Luma Concepts node.
    pub luma_concepts: Option<LumaConceptsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    LumaConceptsParam: crate::nodes::types::LumaConcepts,
> LumaVideoNode<PromptParam, SeedParam, LumaConceptsParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        luma_concepts: Option<LumaConceptsParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            luma_concepts,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    LumaConceptsParam: crate::nodes::types::LumaConcepts,
> crate::nodes::TypedNode for LumaVideoNode<PromptParam, SeedParam, LumaConceptsParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.luma_concepts {
            output.insert("luma_concepts".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LumaVideoNode";
    const DISPLAY_NAME: &'static str = "Luma Text to Video";
    const DESCRIPTION: &'static str = "Generates videos synchronously based on prompt and output_size.";
    const CATEGORY: &'static str = "api node/video/Luma";
}
