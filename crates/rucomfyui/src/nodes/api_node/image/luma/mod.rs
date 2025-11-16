//!`Luma` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Luma Image to Image**: Modifies images synchronously based on prompt and aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaImageModifyNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    ImageWeightParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Weight of the image; the closer to 1.0, the less the image will be modified.

**Metadata**:
  - Default: 0.1
  - Max: 0.98
  - Min: 0
  - Step: 0.01
*/
    pub image_weight: ImageWeightParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    ImageWeightParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
> LumaImageModifyNode<ImageParam, PromptParam, ImageWeightParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        image_weight: ImageWeightParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            prompt,
            image_weight,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    ImageWeightParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for LumaImageModifyNode<ImageParam, PromptParam, ImageWeightParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("image_weight".to_string(), self.image_weight.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "LumaImageModifyNode";
    const DISPLAY_NAME: &'static str = "Luma Image to Image";
    const DESCRIPTION: &'static str = "Modifies images synchronously based on prompt and aspect ratio.";
    const CATEGORY: &'static str = "api node/image/Luma";
}
///**Luma Text to Image**: Generates images synchronously based on prompt and aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaImageNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    StyleImageWeightParam: crate::nodes::types::Float,
    ImageLumaRefParam: crate::nodes::types::LumaRef = crate::nodes::types::LumaRefOut,
    StyleImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    CharacterImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Prompt for the image generation

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
    /**Weight of style image. Ignored if no style_image provided.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub style_image_weight: StyleImageWeightParam,
    ///Luma Reference node connection to influence generation with input images; up to 4 images can be considered.
    pub image_luma_ref: Option<ImageLumaRefParam>,
    ///Style reference image; only 1 image will be used.
    pub style_image: Option<StyleImageParam>,
    ///Character reference images; can be a batch of multiple, up to 4 images can be considered.
    pub character_image: Option<CharacterImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    StyleImageWeightParam: crate::nodes::types::Float,
    ImageLumaRefParam: crate::nodes::types::LumaRef,
    StyleImageParam: crate::nodes::types::Image,
    CharacterImageParam: crate::nodes::types::Image,
> LumaImageNode<
    PromptParam,
    SeedParam,
    StyleImageWeightParam,
    ImageLumaRefParam,
    StyleImageParam,
    CharacterImageParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        style_image_weight: StyleImageWeightParam,
        image_luma_ref: Option<ImageLumaRefParam>,
        style_image: Option<StyleImageParam>,
        character_image: Option<CharacterImageParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            style_image_weight,
            image_luma_ref,
            style_image,
            character_image,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    StyleImageWeightParam: crate::nodes::types::Float,
    ImageLumaRefParam: crate::nodes::types::LumaRef,
    StyleImageParam: crate::nodes::types::Image,
    CharacterImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for LumaImageNode<
    PromptParam,
    SeedParam,
    StyleImageWeightParam,
    ImageLumaRefParam,
    StyleImageParam,
    CharacterImageParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "style_image_weight".to_string(),
                self.style_image_weight.clone().into(),
            );
        if let Some(v) = &self.image_luma_ref {
            output.insert("image_luma_ref".to_string(), v.clone().into());
        }
        if let Some(v) = &self.style_image {
            output.insert("style_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.character_image {
            output.insert("character_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LumaImageNode";
    const DISPLAY_NAME: &'static str = "Luma Text to Image";
    const DESCRIPTION: &'static str = "Generates images synchronously based on prompt and aspect ratio.";
    const CATEGORY: &'static str = "api node/image/Luma";
}
///**Luma Reference**: Holds an image and weight for use with Luma Generate Image node.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LumaReferenceNode<
    ImageParam: crate::nodes::types::Image,
    WeightParam: crate::nodes::types::Float,
    LumaRefParam: crate::nodes::types::LumaRef = crate::nodes::types::LumaRefOut,
> {
    ///Image to use as reference.
    pub image: ImageParam,
    /**Weight of image reference.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub weight: WeightParam,
    ///No documentation.
    pub luma_ref: Option<LumaRefParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    WeightParam: crate::nodes::types::Float,
    LumaRefParam: crate::nodes::types::LumaRef,
> LumaReferenceNode<ImageParam, WeightParam, LumaRefParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        weight: WeightParam,
        luma_ref: Option<LumaRefParam>,
    ) -> Self {
        Self { image, weight, luma_ref }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    WeightParam: crate::nodes::types::Float,
    LumaRefParam: crate::nodes::types::LumaRef,
> crate::nodes::TypedNode for LumaReferenceNode<ImageParam, WeightParam, LumaRefParam> {
    type Output = crate::nodes::types::LumaRefOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("weight".to_string(), self.weight.clone().into());
        if let Some(v) = &self.luma_ref {
            output.insert("luma_ref".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LumaReferenceNode";
    const DISPLAY_NAME: &'static str = "Luma Reference";
    const DESCRIPTION: &'static str = "Holds an image and weight for use with Luma Generate Image node.";
    const CATEGORY: &'static str = "api node/image/Luma";
}
