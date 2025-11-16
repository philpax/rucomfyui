//!`OpenAI` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**OpenAI ChatGPT Advanced Options**: Allows specifying advanced configuration options for the OpenAI Chat Nodes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIChatConfig<
    MaxOutputTokensParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    InstructionsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**An upper bound for the number of tokens that can be generated for a response, including visible output tokens

**Metadata**:
  - Default: 4096
  - Max: 16384
  - Min: 16
*/
    pub max_output_tokens: Option<MaxOutputTokensParam>,
    /**Instructions for the model on how to generate the response

**Metadata**:
  - Multiline: true
*/
    pub instructions: Option<InstructionsParam>,
}
impl<
    MaxOutputTokensParam: crate::nodes::types::Int,
    InstructionsParam: crate::nodes::types::String,
> OpenAIChatConfig<MaxOutputTokensParam, InstructionsParam> {
    /// Create a new node.
    pub fn new(
        max_output_tokens: Option<MaxOutputTokensParam>,
        instructions: Option<InstructionsParam>,
    ) -> Self {
        Self {
            max_output_tokens,
            instructions,
        }
    }
}
impl<
    MaxOutputTokensParam: crate::nodes::types::Int,
    InstructionsParam: crate::nodes::types::String,
> crate::nodes::TypedNode for OpenAIChatConfig<MaxOutputTokensParam, InstructionsParam> {
    type Output = crate::nodes::types::OpenAiChatConfigOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.max_output_tokens {
            output.insert("max_output_tokens".to_string(), v.clone().into());
        }
        if let Some(v) = &self.instructions {
            output.insert("instructions".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIChatConfig";
    const DISPLAY_NAME: &'static str = "OpenAI ChatGPT Advanced Options";
    const DESCRIPTION: &'static str = "Allows specifying advanced configuration options for the OpenAI Chat Nodes.";
    const CATEGORY: &'static str = "api node/text/OpenAI";
}
///**OpenAI ChatGPT**: Generate text responses from an OpenAI model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIChatNode<
    PromptParam: crate::nodes::types::String,
    PersistContextParam: crate::nodes::types::Boolean,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FilesParam: crate::nodes::types::OpenAiInputFiles
        = crate::nodes::types::OpenAiInputFilesOut,
    AdvancedOptionsParam: crate::nodes::types::OpenAiChatConfig
        = crate::nodes::types::OpenAiChatConfigOut,
> {
    /**Text inputs to the model, used to generate a response.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**This parameter is deprecated and has no effect.

**Metadata**:
  - Default: false
*/
    pub persist_context: PersistContextParam,
    ///Optional image(s) to use as context for the model. To include multiple images, you can use the Batch Images node.
    pub images: Option<ImagesParam>,
    ///Optional file(s) to use as context for the model. Accepts inputs from the OpenAI Chat Input Files node.
    pub files: Option<FilesParam>,
    ///Optional configuration for the model. Accepts inputs from the OpenAI Chat Advanced Options node.
    pub advanced_options: Option<AdvancedOptionsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    PersistContextParam: crate::nodes::types::Boolean,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::OpenAiInputFiles,
    AdvancedOptionsParam: crate::nodes::types::OpenAiChatConfig,
> OpenAIChatNode<
    PromptParam,
    PersistContextParam,
    ImagesParam,
    FilesParam,
    AdvancedOptionsParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        persist_context: PersistContextParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
        advanced_options: Option<AdvancedOptionsParam>,
    ) -> Self {
        Self {
            prompt,
            persist_context,
            images,
            files,
            advanced_options,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    PersistContextParam: crate::nodes::types::Boolean,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::OpenAiInputFiles,
    AdvancedOptionsParam: crate::nodes::types::OpenAiChatConfig,
> crate::nodes::TypedNode
for OpenAIChatNode<
    PromptParam,
    PersistContextParam,
    ImagesParam,
    FilesParam,
    AdvancedOptionsParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("persist_context".to_string(), self.persist_context.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.files {
            output.insert("files".to_string(), v.clone().into());
        }
        if let Some(v) = &self.advanced_options {
            output.insert("advanced_options".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIChatNode";
    const DISPLAY_NAME: &'static str = "OpenAI ChatGPT";
    const DESCRIPTION: &'static str = "Generate text responses from an OpenAI model.";
    const CATEGORY: &'static str = "api node/text/OpenAI";
}
///**OpenAI ChatGPT Input Files**: Loads and prepares input files (text, pdf, etc.) to include as inputs for the OpenAI Chat Node. The files will be read by the OpenAI model when generating a response. 🛈 TIP: Can be chained together with other OpenAI Input File nodes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIInputFiles<
    OpenaiInputFilesParam: crate::nodes::types::OpenAiInputFiles
        = crate::nodes::types::OpenAiInputFilesOut,
> {
    ///An optional additional file(s) to batch together with the file loaded from this node. Allows chaining of input files so that a single message can include multiple input files.
    pub openai_input_files: Option<OpenaiInputFilesParam>,
}
impl<
    OpenaiInputFilesParam: crate::nodes::types::OpenAiInputFiles,
> OpenAIInputFiles<OpenaiInputFilesParam> {
    /// Create a new node.
    pub fn new(openai_input_files: Option<OpenaiInputFilesParam>) -> Self {
        Self { openai_input_files }
    }
}
impl<
    OpenaiInputFilesParam: crate::nodes::types::OpenAiInputFiles,
> crate::nodes::TypedNode for OpenAIInputFiles<OpenaiInputFilesParam> {
    type Output = crate::nodes::types::OpenAiInputFilesOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.openai_input_files {
            output.insert("OPENAI_INPUT_FILES".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIInputFiles";
    const DISPLAY_NAME: &'static str = "OpenAI ChatGPT Input Files";
    const DESCRIPTION: &'static str = "Loads and prepares input files (text, pdf, etc.) to include as inputs for the OpenAI Chat Node. The files will be read by the OpenAI model when generating a response. 🛈 TIP: Can be chained together with other OpenAI Input File nodes.";
    const CATEGORY: &'static str = "api node/text/OpenAI";
}
