//!`Wan` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Wan Image to Image**: Generates an image from one or two input images and a text prompt. The output image is currently fixed at 1.6 MP; its aspect ratio matches the input image(s).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanImageToImageApi<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///Single-image editing or multi-image fusion, maximum 2 images.
    pub image: ImageParam,
    /**Prompt used to describe the elements and visual features, supports English/Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Whether to add an "AI generated" watermark to the result.

**Metadata**:
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> WanImageToImageApi<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    SeedParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        seed: Option<SeedParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            negative_prompt,
            seed,
            watermark,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanImageToImageApi<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    SeedParam,
    WatermarkParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanImageToImageApi";
    const DISPLAY_NAME: &'static str = "Wan Image to Image";
    const DESCRIPTION: &'static str = "Generates an image from one or two input images and a text prompt. The output image is currently fixed at 1.6 MP; its aspect ratio matches the input image(s).";
    const CATEGORY: &'static str = "api node/image/Wan";
}
///**Wan Text to Image**: Generates image based on text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanTextToImageApi<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    WidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    HeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    PromptExtendParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Prompt used to describe the elements and visual features, supports English/Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 1440
  - Min: 768
  - Step: 32
*/
    pub width: Option<WidthParam>,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 1440
  - Min: 768
  - Step: 32
*/
    pub height: Option<HeightParam>,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: Option<PromptExtendParam>,
    /**Whether to add an "AI generated" watermark to the result.

**Metadata**:
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> WanTextToImageApi<
    PromptParam,
    NegativePromptParam,
    WidthParam,
    HeightParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        width: Option<WidthParam>,
        height: Option<HeightParam>,
        seed: Option<SeedParam>,
        prompt_extend: Option<PromptExtendParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            width,
            height,
            seed,
            prompt_extend,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanTextToImageApi<
    PromptParam,
    NegativePromptParam,
    WidthParam,
    HeightParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.width {
            output.insert("width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.height {
            output.insert("height".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.prompt_extend {
            output.insert("prompt_extend".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanTextToImageApi";
    const DISPLAY_NAME: &'static str = "Wan Text to Image";
    const DESCRIPTION: &'static str = "Generates image based on text prompt.";
    const CATEGORY: &'static str = "api node/image/Wan";
}
