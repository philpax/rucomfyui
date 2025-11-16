//!`Ideogram` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Ideogram V1**: Generates images using the Ideogram V1 model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IdeogramV1<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    NumImagesParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Whether to use turbo mode (faster generation, potentially lower quality)

**Metadata**:
  - Default: false
*/
    pub turbo: TurboParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Description of what to exclude from the image

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub num_images: Option<NumImagesParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    NumImagesParam: crate::nodes::types::Int,
> IdeogramV1<PromptParam, TurboParam, SeedParam, NegativePromptParam, NumImagesParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        turbo: TurboParam,
        seed: Option<SeedParam>,
        negative_prompt: Option<NegativePromptParam>,
        num_images: Option<NumImagesParam>,
    ) -> Self {
        Self {
            prompt,
            turbo,
            seed,
            negative_prompt,
            num_images,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    NumImagesParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for IdeogramV1<PromptParam, TurboParam, SeedParam, NegativePromptParam, NumImagesParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("turbo".to_string(), self.turbo.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.num_images {
            output.insert("num_images".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "IdeogramV1";
    const DISPLAY_NAME: &'static str = "Ideogram V1";
    const DESCRIPTION: &'static str = "Generates images using the Ideogram V1 model.";
    const CATEGORY: &'static str = "api node/image/Ideogram";
}
///**Ideogram V2**: Generates images using the Ideogram V2 model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IdeogramV2<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    NumImagesParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Whether to use turbo mode (faster generation, potentially lower quality)

**Metadata**:
  - Default: false
*/
    pub turbo: TurboParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Description of what to exclude from the image

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub num_images: Option<NumImagesParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    NumImagesParam: crate::nodes::types::Int,
> IdeogramV2<PromptParam, TurboParam, SeedParam, NegativePromptParam, NumImagesParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        turbo: TurboParam,
        seed: Option<SeedParam>,
        negative_prompt: Option<NegativePromptParam>,
        num_images: Option<NumImagesParam>,
    ) -> Self {
        Self {
            prompt,
            turbo,
            seed,
            negative_prompt,
            num_images,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    TurboParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    NumImagesParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for IdeogramV2<PromptParam, TurboParam, SeedParam, NegativePromptParam, NumImagesParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("turbo".to_string(), self.turbo.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.num_images {
            output.insert("num_images".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "IdeogramV2";
    const DISPLAY_NAME: &'static str = "Ideogram V2";
    const DESCRIPTION: &'static str = "Generates images using the Ideogram V2 model.";
    const CATEGORY: &'static str = "api node/image/Ideogram";
}
///**Ideogram V3**: Generates images using the Ideogram V3 model. Supports both regular image generation from text prompts and image editing with mask.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IdeogramV3<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    NumImagesParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CharacterImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    CharacterMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    /**Prompt for the image generation or editing

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Optional reference image for image editing.
    pub image: Option<ImageParam>,
    ///Optional mask for inpainting (white areas will be replaced)
    pub mask: Option<MaskParam>,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 8
  - Min: 1
  - Step: 1
*/
    pub num_images: Option<NumImagesParam>,
    ///Image to use as character reference.
    pub character_image: Option<CharacterImageParam>,
    ///Optional mask for character reference image.
    pub character_mask: Option<CharacterMaskParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    SeedParam: crate::nodes::types::Int,
    NumImagesParam: crate::nodes::types::Int,
    CharacterImageParam: crate::nodes::types::Image,
    CharacterMaskParam: crate::nodes::types::Mask,
> IdeogramV3<
    PromptParam,
    ImageParam,
    MaskParam,
    SeedParam,
    NumImagesParam,
    CharacterImageParam,
    CharacterMaskParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        image: Option<ImageParam>,
        mask: Option<MaskParam>,
        seed: Option<SeedParam>,
        num_images: Option<NumImagesParam>,
        character_image: Option<CharacterImageParam>,
        character_mask: Option<CharacterMaskParam>,
    ) -> Self {
        Self {
            prompt,
            image,
            mask,
            seed,
            num_images,
            character_image,
            character_mask,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    SeedParam: crate::nodes::types::Int,
    NumImagesParam: crate::nodes::types::Int,
    CharacterImageParam: crate::nodes::types::Image,
    CharacterMaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for IdeogramV3<
    PromptParam,
    ImageParam,
    MaskParam,
    SeedParam,
    NumImagesParam,
    CharacterImageParam,
    CharacterMaskParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.num_images {
            output.insert("num_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.character_image {
            output.insert("character_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.character_mask {
            output.insert("character_mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "IdeogramV3";
    const DISPLAY_NAME: &'static str = "Ideogram V3";
    const DESCRIPTION: &'static str = "Generates images using the Ideogram V3 model. Supports both regular image generation from text prompts and image editing with mask.";
    const CATEGORY: &'static str = "api node/image/Ideogram";
}
