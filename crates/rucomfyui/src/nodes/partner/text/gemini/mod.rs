//!`Gemini` definitions/categories.
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
///**Gemini Input Files**: Loads and prepares input files to include as inputs for Gemini LLM nodes. The files will be read by the Gemini model when generating a response. The contents of the text file count toward the token limit. 🛈 TIP: Can be chained together with other Gemini Input File nodes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiInputFiles<
    GeminiInputFilesParam: crate::nodes::types::GeminiInputFiles
        = crate::nodes::types::GeminiInputFilesOut,
> {
    ///An optional additional file(s) to batch together with the file loaded from this node. Allows chaining of input files so that a single message can include multiple input files.
    pub gemini_input_files: Option<GeminiInputFilesParam>,
}
impl<
    GeminiInputFilesParam: crate::nodes::types::GeminiInputFiles,
> GeminiInputFiles<GeminiInputFilesParam> {
    /// Create a new node.
    pub fn new(gemini_input_files: Option<GeminiInputFilesParam>) -> Self {
        Self { gemini_input_files }
    }
}
impl<
    GeminiInputFilesParam: crate::nodes::types::GeminiInputFiles,
> crate::nodes::TypedNode for GeminiInputFiles<GeminiInputFilesParam> {
    type Output = crate::nodes::types::GeminiInputFilesOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.gemini_input_files {
            output.insert("GEMINI_INPUT_FILES".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiInputFiles";
    const DISPLAY_NAME: &'static str = "Gemini Input Files";
    const DESCRIPTION: &'static str = "Loads and prepares input files to include as inputs for Gemini LLM nodes. The files will be read by the Gemini model when generating a response. The contents of the text file count toward the token limit. 🛈 TIP: Can be chained together with other Gemini Input File nodes.";
    const CATEGORY: &'static str = "partner/text/Gemini";
}
///**Google Gemini**: Generate text responses with Google's Gemini AI model. You can provide multiple types of inputs (text, images, audio, video) as context for generating more relevant and meaningful responses.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
    VideoParam: crate::nodes::types::Video = crate::nodes::types::VideoOut,
    FilesParam: crate::nodes::types::GeminiInputFiles
        = crate::nodes::types::GeminiInputFilesOut,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text inputs to the model, used to generate a response. You can include detailed instructions, questions, or context for the model.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When seed is fixed to a specific value, the model makes a best effort to provide the same response for repeated requests. Deterministic output isn't guaranteed. Also, changing the model or parameter settings, such as the temperature, can cause variations in the response even when you use the same seed value. By default, a random seed value is used.

**Metadata**:
  - Default: 42
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///Optional image(s) to use as context for the model. To include multiple images, you can use the Batch Images node.
    pub images: Option<ImagesParam>,
    ///Optional audio to use as context for the model.
    pub audio: Option<AudioParam>,
    ///Optional video to use as context for the model.
    pub video: Option<VideoParam>,
    ///Optional file(s) to use as context for the model. Accepts inputs from the Gemini Generate Content Input Files node.
    pub files: Option<FilesParam>,
    /**Foundational instructions that dictate an AI's behavior.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    VideoParam: crate::nodes::types::Video,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> GeminiNode<
    PromptParam,
    SeedParam,
    ImagesParam,
    AudioParam,
    VideoParam,
    FilesParam,
    SystemPromptParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        audio: Option<AudioParam>,
        video: Option<VideoParam>,
        files: Option<FilesParam>,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            audio,
            video,
            files,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    VideoParam: crate::nodes::types::Video,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for GeminiNode<
    PromptParam,
    SeedParam,
    ImagesParam,
    AudioParam,
    VideoParam,
    FilesParam,
    SystemPromptParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.video {
            output.insert("video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.files {
            output.insert("files".to_string(), v.clone().into());
        }
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiNode";
    const DISPLAY_NAME: &'static str = "Google Gemini";
    const DESCRIPTION: &'static str = "Generate text responses with Google's Gemini AI model. You can provide multiple types of inputs (text, images, audio, video) as context for generating more relevant and meaningful responses.";
    const CATEGORY: &'static str = "partner/text/Gemini";
}
///**Google Gemini**: Generate text responses with Google's Gemini models. Provide a text prompt and, optionally, one or more images, audio clips, videos, or files as multimodal context.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiNodeV2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text input to the model. Include detailed instructions, questions, or context.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed for sampling. Set to 0 for a random seed. Deterministic output isn't guaranteed.

**Metadata**:
  - Default: 42
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**Foundational instructions that dictate the model's behavior.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
> GeminiNodeV2<PromptParam, SeedParam, SystemPromptParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode for GeminiNodeV2<PromptParam, SeedParam, SystemPromptParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiNodeV2";
    const DISPLAY_NAME: &'static str = "Google Gemini";
    const DESCRIPTION: &'static str = "Generate text responses with Google's Gemini models. Provide a text prompt and, optionally, one or more images, audio clips, videos, or files as multimodal context.";
    const CATEGORY: &'static str = "partner/text/Gemini";
}
