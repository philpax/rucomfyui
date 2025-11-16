//!`LTXV` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**LTXV Image To Video**: Professional-quality videos with customizable duration and resolution based on start image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LtxvApiImageToVideo<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///First frame to be used for the video.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When true, the generated video will include AI-generated audio matching the scene.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> LtxvApiImageToVideo<ImageParam, PromptParam, GenerateAudioParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        generate_audio: Option<GenerateAudioParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            generate_audio,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for LtxvApiImageToVideo<ImageParam, PromptParam, GenerateAudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LtxvApiImageToVideo";
    const DISPLAY_NAME: &'static str = "LTXV Image To Video";
    const DESCRIPTION: &'static str = "Professional-quality videos with customizable duration and resolution based on start image.";
    const CATEGORY: &'static str = "api node/video/LTXV";
}
///**LTXV Text To Video**: Professional-quality videos with customizable duration and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LtxvApiTextToVideo<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When true, the generated video will include AI-generated audio matching the scene.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> LtxvApiTextToVideo<PromptParam, GenerateAudioParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, generate_audio: Option<GenerateAudioParam>) -> Self {
        Self { prompt, generate_audio }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for LtxvApiTextToVideo<PromptParam, GenerateAudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LtxvApiTextToVideo";
    const DISPLAY_NAME: &'static str = "LTXV Text To Video";
    const DESCRIPTION: &'static str = "Professional-quality videos with customizable duration and resolution.";
    const CATEGORY: &'static str = "api node/video/LTXV";
}
