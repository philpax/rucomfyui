//!`api node` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[doc = "**OpenAI DALL·E 2**: Generates images synchronously via OpenAI's DALL·E 2 endpoint.\n\n\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\n\nso download or cache results if you need to keep them."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIDalle2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    /**Text prompt for DALL·E

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**not implemented yet in backend

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**How many images to generate

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub n: Option<NParam>,
    ///Optional reference image for image editing.
    pub image: Option<ImageParam>,
    ///Optional mask for inpainting (white areas will be replaced)
    pub mask: Option<MaskParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> OpenAIDalle2<PromptParam, SeedParam, NParam, ImageParam, MaskParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: Option<SeedParam>,
        n: Option<NParam>,
        image: Option<ImageParam>,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            n,
            image,
            mask,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for OpenAIDalle2<PromptParam, SeedParam, NParam, ImageParam, MaskParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.n {
            output.insert("n".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIDalle2";
    const DISPLAY_NAME: &'static str = "OpenAI DALL·E 2";
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's DALL·E 2 endpoint.\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\nso download or cache results if you need to keep them.";
    const CATEGORY: &'static str = "api node";
}
#[doc = "**OpenAI DALL·E 3**: Generates images synchronously via OpenAI's DALL·E 3 endpoint.\n\n\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\n\nso download or cache results if you need to keep them."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIDalle3<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Text prompt for DALL·E

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**not implemented yet in backend

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
    SeedParam: crate::nodes::types::Int,
> OpenAIDalle3<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: Option<SeedParam>) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for OpenAIDalle3<PromptParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIDalle3";
    const DISPLAY_NAME: &'static str = "OpenAI DALL·E 3";
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's DALL·E 3 endpoint.\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\nso download or cache results if you need to keep them.";
    const CATEGORY: &'static str = "api node";
}
#[doc = "**OpenAI GPT Image 1**: Generates images synchronously via OpenAI's GPT Image 1 endpoint.\n\n\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\n\nso download or cache results if you need to keep them."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIGPTImage1<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    /**Text prompt for GPT Image 1

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**not implemented yet in backend

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**How many images to generate

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub n: Option<NParam>,
    ///Optional reference image for image editing.
    pub image: Option<ImageParam>,
    ///Optional mask for inpainting (white areas will be replaced)
    pub mask: Option<MaskParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> OpenAIGPTImage1<PromptParam, SeedParam, NParam, ImageParam, MaskParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: Option<SeedParam>,
        n: Option<NParam>,
        image: Option<ImageParam>,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            n,
            image,
            mask,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for OpenAIGPTImage1<PromptParam, SeedParam, NParam, ImageParam, MaskParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.n {
            output.insert("n".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIGPTImage1";
    const DISPLAY_NAME: &'static str = "OpenAI GPT Image 1";
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's GPT Image 1 endpoint.\n\nUses the proxy at /proxy/openai/images/generations. Returned URLs are short‑lived,\nso download or cache results if you need to keep them.";
    const CATEGORY: &'static str = "api node";
}
