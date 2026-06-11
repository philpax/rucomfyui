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
/// Output types for nodes.
pub mod out {
    ///Output for [`GeminiImage2Node`](super::GeminiImage2Node).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GeminiImage2NodeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub string: crate::nodes::types::StringOut,
    }
    ///Output for [`GeminiImageNode`](super::GeminiImageNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GeminiImageNodeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub string: crate::nodes::types::StringOut,
    }
    ///Output for [`GeminiNanoBanana2`](super::GeminiNanoBanana2).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GeminiNanoBanana2Output {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub string: crate::nodes::types::StringOut,
        ///First image from the model's thinking process. Only available with thinking_level HIGH and IMAGE+TEXT modality.
        pub thought_image: crate::nodes::types::ImageOut,
    }
    ///Output for [`GeminiNanoBanana2V2`](super::GeminiNanoBanana2V2).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GeminiNanoBanana2V2Output {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub string: crate::nodes::types::StringOut,
        ///First image from the model's thinking process. Only available with thinking_level HIGH and IMAGE+TEXT modality.
        pub thought_image: crate::nodes::types::ImageOut,
    }
}
///**Nano Banana Pro (Google Gemini Image)**: Generate or edit images synchronously via Google Vertex API.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiImage2Node<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FilesParam: crate::nodes::types::GeminiInputFiles
        = crate::nodes::types::GeminiInputFilesOut,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text prompt describing the image to generate or the edits to apply. Include any constraints, styles, or details the model should follow.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When the seed is fixed to a specific value, the model makes a best effort to provide the same response for repeated requests. Deterministic output isn't guaranteed. Also, changing the model or parameter settings, such as the temperature, can cause variations in the response even when you use the same seed value. By default, a random seed value is used.

**Metadata**:
  - Default: 42
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///Optional reference image(s). To include multiple images, use the Batch Images node (up to 14).
    pub images: Option<ImagesParam>,
    ///Optional file(s) to use as context for the model. Accepts inputs from the Gemini Generate Content Input Files node.
    pub files: Option<FilesParam>,
    /**Foundational instructions that dictate an AI's behavior.

**Metadata**:
  - Multiline: true
  - Default: You are an expert image-generation engine. You must ALWAYS produce an image.
Interpret all user input—regardless of format, intent, or abstraction—as literal visual directives for image composition.
If a prompt is conversational or lacks specific visual details, you must creatively invent a concrete visual scenario that depicts the concept.
Prioritize generating the visual representation above any text, formatting, or conversational requests.
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> GeminiImage2Node<PromptParam, SeedParam, ImagesParam, FilesParam, SystemPromptParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            files,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for GeminiImage2Node<
    PromptParam,
    SeedParam,
    ImagesParam,
    FilesParam,
    SystemPromptParam,
> {
    type Output = out::GeminiImage2NodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            string: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.files {
            output.insert("files".to_string(), v.clone().into());
        }
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiImage2Node";
    const DISPLAY_NAME: &'static str = "Nano Banana Pro (Google Gemini Image)";
    const DESCRIPTION: &'static str = "Generate or edit images synchronously via Google Vertex API.";
    const CATEGORY: &'static str = "partner/image/Gemini";
}
///**Nano Banana (Google Gemini Image)**: Edit images synchronously via Google API.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiImageNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FilesParam: crate::nodes::types::GeminiInputFiles
        = crate::nodes::types::GeminiInputFilesOut,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text prompt for generation

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
    ///Optional file(s) to use as context for the model. Accepts inputs from the Gemini Generate Content Input Files node.
    pub files: Option<FilesParam>,
    /**Foundational instructions that dictate an AI's behavior.

**Metadata**:
  - Multiline: true
  - Default: You are an expert image-generation engine. You must ALWAYS produce an image.
Interpret all user input—regardless of format, intent, or abstraction—as literal visual directives for image composition.
If a prompt is conversational or lacks specific visual details, you must creatively invent a concrete visual scenario that depicts the concept.
Prioritize generating the visual representation above any text, formatting, or conversational requests.
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> GeminiImageNode<PromptParam, SeedParam, ImagesParam, FilesParam, SystemPromptParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            files,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for GeminiImageNode<PromptParam, SeedParam, ImagesParam, FilesParam, SystemPromptParam> {
    type Output = out::GeminiImageNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            string: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.files {
            output.insert("files".to_string(), v.clone().into());
        }
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiImageNode";
    const DISPLAY_NAME: &'static str = "Nano Banana (Google Gemini Image)";
    const DESCRIPTION: &'static str = "Edit images synchronously via Google API.";
    const CATEGORY: &'static str = "partner/image/Gemini";
}
///**Nano Banana 2**: Generate or edit images synchronously via Google Vertex API.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiNanoBanana2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FilesParam: crate::nodes::types::GeminiInputFiles
        = crate::nodes::types::GeminiInputFilesOut,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text prompt describing the image to generate or the edits to apply. Include any constraints, styles, or details the model should follow.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When the seed is fixed to a specific value, the model makes a best effort to provide the same response for repeated requests. Deterministic output isn't guaranteed. Also, changing the model or parameter settings, such as the temperature, can cause variations in the response even when you use the same seed value. By default, a random seed value is used.

**Metadata**:
  - Default: 42
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///Optional reference image(s). To include multiple images, use the Batch Images node (up to 14).
    pub images: Option<ImagesParam>,
    ///Optional file(s) to use as context for the model. Accepts inputs from the Gemini Generate Content Input Files node.
    pub files: Option<FilesParam>,
    /**Foundational instructions that dictate an AI's behavior.

**Metadata**:
  - Multiline: true
  - Default: You are an expert image-generation engine. You must ALWAYS produce an image.
Interpret all user input—regardless of format, intent, or abstraction—as literal visual directives for image composition.
If a prompt is conversational or lacks specific visual details, you must creatively invent a concrete visual scenario that depicts the concept.
Prioritize generating the visual representation above any text, formatting, or conversational requests.
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> GeminiNanoBanana2<PromptParam, SeedParam, ImagesParam, FilesParam, SystemPromptParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            files,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for GeminiNanoBanana2<
    PromptParam,
    SeedParam,
    ImagesParam,
    FilesParam,
    SystemPromptParam,
> {
    type Output = out::GeminiNanoBanana2Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            string: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            thought_image: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.files {
            output.insert("files".to_string(), v.clone().into());
        }
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiNanoBanana2";
    const DISPLAY_NAME: &'static str = "Nano Banana 2";
    const DESCRIPTION: &'static str = "Generate or edit images synchronously via Google Vertex API.";
    const CATEGORY: &'static str = "partner/image/Gemini";
}
///**Nano Banana 2**: Generate or edit images synchronously via Google Vertex API.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GeminiNanoBanana2V2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    TemperatureParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    TopPParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**Text prompt describing the image to generate or the edits to apply. Include any constraints, styles, or details the model should follow.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**When the seed is fixed to a specific value, the model makes a best effort to provide the same response for repeated requests. Deterministic output isn't guaranteed. Also, changing the model or parameter settings, such as the temperature, can cause variations in the response even when you use the same seed value. By default, a random seed value is used.

**Metadata**:
  - Default: 42
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**Foundational instructions that dictate an AI's behavior.

**Metadata**:
  - Multiline: true
  - Default: You are an expert image-generation engine. You must ALWAYS produce an image.
Interpret all user input—regardless of format, intent, or abstraction—as literal visual directives for image composition.
If a prompt is conversational or lacks specific visual details, you must creatively invent a concrete visual scenario that depicts the concept.
Prioritize generating the visual representation above any text, formatting, or conversational requests.
*/
    pub system_prompt: Option<SystemPromptParam>,
    /**Controls randomness in generation. Lower is more focused/deterministic.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: 0
  - Step: 0.01
*/
    pub temperature: Option<TemperatureParam>,
    /**Nucleus sampling threshold. Lower is more focused, higher more diverse.

**Metadata**:
  - Default: 0.95
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub top_p: Option<TopPParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
    TemperatureParam: crate::nodes::types::Float,
    TopPParam: crate::nodes::types::Float,
> GeminiNanoBanana2V2<
    PromptParam,
    SeedParam,
    SystemPromptParam,
    TemperatureParam,
    TopPParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        system_prompt: Option<SystemPromptParam>,
        temperature: Option<TemperatureParam>,
        top_p: Option<TopPParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            system_prompt,
            temperature,
            top_p,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
    TemperatureParam: crate::nodes::types::Float,
    TopPParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for GeminiNanoBanana2V2<
    PromptParam,
    SeedParam,
    SystemPromptParam,
    TemperatureParam,
    TopPParam,
> {
    type Output = out::GeminiNanoBanana2V2Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            string: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            thought_image: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.temperature {
            output.insert("temperature".to_string(), v.clone().into());
        }
        if let Some(v) = &self.top_p {
            output.insert("top_p".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GeminiNanoBanana2V2";
    const DISPLAY_NAME: &'static str = "Nano Banana 2";
    const DESCRIPTION: &'static str = "Generate or edit images synchronously via Google Vertex API.";
    const CATEGORY: &'static str = "partner/image/Gemini";
}
