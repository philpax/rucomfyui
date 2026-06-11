//!`OpenAI` definitions/categories.
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
///**OpenAI DALL·E 2**: Generates images synchronously via OpenAI's DALL·E 2 endpoint.
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
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's DALL·E 2 endpoint.";
    const CATEGORY: &'static str = "partner/image/OpenAI";
}
///**OpenAI DALL·E 3**: Generates images synchronously via OpenAI's DALL·E 3 endpoint.
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
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's DALL·E 3 endpoint.";
    const CATEGORY: &'static str = "partner/image/OpenAI";
}
///**OpenAI GPT Image 2**: Generates images synchronously via OpenAI's GPT Image endpoint.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIGPTImage1<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    CustomWidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CustomHeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Text prompt for GPT Image

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
    /**Used only when `size` is 'Custom'. Must be a multiple of 16 (GPT Image 2 only).

**Metadata**:
  - Default: 1024
  - Max: 3840
  - Min: 1024
  - Step: 16
*/
    pub custom_width: Option<CustomWidthParam>,
    /**Used only when `size` is 'Custom'. Must be a multiple of 16 (GPT Image 2 only).

**Metadata**:
  - Default: 1024
  - Max: 3840
  - Min: 1024
  - Step: 16
*/
    pub custom_height: Option<CustomHeightParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    CustomWidthParam: crate::nodes::types::Int,
    CustomHeightParam: crate::nodes::types::Int,
> OpenAIGPTImage1<
    PromptParam,
    SeedParam,
    NParam,
    ImageParam,
    MaskParam,
    CustomWidthParam,
    CustomHeightParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: Option<SeedParam>,
        n: Option<NParam>,
        image: Option<ImageParam>,
        mask: Option<MaskParam>,
        custom_width: Option<CustomWidthParam>,
        custom_height: Option<CustomHeightParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            n,
            image,
            mask,
            custom_width,
            custom_height,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    CustomWidthParam: crate::nodes::types::Int,
    CustomHeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for OpenAIGPTImage1<
    PromptParam,
    SeedParam,
    NParam,
    ImageParam,
    MaskParam,
    CustomWidthParam,
    CustomHeightParam,
> {
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
        if let Some(v) = &self.custom_width {
            output.insert("custom_width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.custom_height {
            output.insert("custom_height".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "OpenAIGPTImage1";
    const DISPLAY_NAME: &'static str = "OpenAI GPT Image 2";
    const DESCRIPTION: &'static str = "Generates images synchronously via OpenAI's GPT Image endpoint.";
    const CATEGORY: &'static str = "partner/image/OpenAI";
}
///**OpenAI GPT Image 2**: Generates images via OpenAI's GPT Image endpoint.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpenAIGPTImageNodeV2<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt for GPT Image

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**How many images to generate

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub n: NParam,
    /**not implemented yet in backend

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> OpenAIGPTImageNodeV2<PromptParam, NParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, n: NParam, seed: SeedParam) -> Self {
        Self { prompt, n, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for OpenAIGPTImageNodeV2<PromptParam, NParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "OpenAIGPTImageNodeV2";
    const DISPLAY_NAME: &'static str = "OpenAI GPT Image 2";
    const DESCRIPTION: &'static str = "Generates images via OpenAI's GPT Image endpoint.";
    const CATEGORY: &'static str = "partner/image/OpenAI";
}
