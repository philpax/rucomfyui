//!`Gemini` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
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
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
> GeminiImage2Node<PromptParam, SeedParam, ImagesParam, FilesParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            files,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
> crate::nodes::TypedNode
for GeminiImage2Node<PromptParam, SeedParam, ImagesParam, FilesParam> {
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
        output
    }
    const NAME: &'static str = "GeminiImage2Node";
    const DISPLAY_NAME: &'static str = "Nano Banana Pro (Google Gemini Image)";
    const DESCRIPTION: &'static str = "Generate or edit images synchronously via Google Vertex API.";
    const CATEGORY: &'static str = "api node/image/Gemini";
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
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
> GeminiImageNode<PromptParam, SeedParam, ImagesParam, FilesParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        images: Option<ImagesParam>,
        files: Option<FilesParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            images,
            files,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
    FilesParam: crate::nodes::types::GeminiInputFiles,
> crate::nodes::TypedNode
for GeminiImageNode<PromptParam, SeedParam, ImagesParam, FilesParam> {
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
        output
    }
    const NAME: &'static str = "GeminiImageNode";
    const DISPLAY_NAME: &'static str = "Nano Banana (Google Gemini Image)";
    const DESCRIPTION: &'static str = "Edit images synchronously via Google API.";
    const CATEGORY: &'static str = "api node/image/Gemini";
}
