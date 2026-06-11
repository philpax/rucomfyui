//!`Kling` definitions/categories.
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
///**Kling 3.0 Image**: Kling Image Generation Node. Generate an image from a text prompt with an optional reference image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingImageGenerationNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageFidelityParam: crate::nodes::types::Float,
    HumanFidelityParam: crate::nodes::types::Float,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**Reference intensity for user-uploaded images

**Metadata**:
  - Default: 0.5
  - Display: slider
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub image_fidelity: ImageFidelityParam,
    /**Subject reference similarity

**Metadata**:
  - Default: 0.45
  - Display: slider
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub human_fidelity: HumanFidelityParam,
    /**Number of generated images

**Metadata**:
  - Default: 1
  - Max: 9
  - Min: 1
*/
    pub n: NParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageFidelityParam: crate::nodes::types::Float,
    HumanFidelityParam: crate::nodes::types::Float,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> KlingImageGenerationNode<
    PromptParam,
    NegativePromptParam,
    ImageFidelityParam,
    HumanFidelityParam,
    NParam,
    ImageParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        image_fidelity: ImageFidelityParam,
        human_fidelity: HumanFidelityParam,
        n: NParam,
        image: Option<ImageParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            image_fidelity,
            human_fidelity,
            n,
            image,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageFidelityParam: crate::nodes::types::Float,
    HumanFidelityParam: crate::nodes::types::Float,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingImageGenerationNode<
    PromptParam,
    NegativePromptParam,
    ImageFidelityParam,
    HumanFidelityParam,
    NParam,
    ImageParam,
    SeedParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("image_fidelity".to_string(), self.image_fidelity.clone().into());
        output.insert("human_fidelity".to_string(), self.human_fidelity.clone().into());
        output.insert("n".to_string(), self.n.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingImageGenerationNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Image";
    const DESCRIPTION: &'static str = "Kling Image Generation Node. Generate an image from a text prompt with an optional reference image.";
    const CATEGORY: &'static str = "partner/image/Kling";
}
///**Kling 3.0 Omni Image**: Create or edit images with the latest model from Kling.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProImageNode<
    PromptParam: crate::nodes::types::String,
    ReferenceImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the image content. This can include both positive and negative descriptions.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///Up to 10 additional reference images.
    pub reference_images: Option<ReferenceImagesParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProImageNode<PromptParam, ReferenceImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        reference_images: Option<ReferenceImagesParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            reference_images,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProImageNode<PromptParam, ReferenceImagesParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.reference_images {
            output.insert("reference_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProImageNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni Image";
    const DESCRIPTION: &'static str = "Create or edit images with the latest model from Kling.";
    const CATEGORY: &'static str = "partner/image/Kling";
}
///**Kling Virtual Try On**: Kling Virtual Try On Node. Input a human image and a cloth image to try on the cloth on the human. You can merge multiple clothing item pictures into one image with a white background.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingVirtualTryOnNode<
    HumanImageParam: crate::nodes::types::Image,
    ClothImageParam: crate::nodes::types::Image,
> {
    ///No documentation.
    pub human_image: HumanImageParam,
    ///No documentation.
    pub cloth_image: ClothImageParam,
}
impl<
    HumanImageParam: crate::nodes::types::Image,
    ClothImageParam: crate::nodes::types::Image,
> KlingVirtualTryOnNode<HumanImageParam, ClothImageParam> {
    /// Create a new node.
    pub fn new(human_image: HumanImageParam, cloth_image: ClothImageParam) -> Self {
        Self { human_image, cloth_image }
    }
}
impl<
    HumanImageParam: crate::nodes::types::Image,
    ClothImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for KlingVirtualTryOnNode<HumanImageParam, ClothImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("human_image".to_string(), self.human_image.clone().into());
        output.insert("cloth_image".to_string(), self.cloth_image.clone().into());
        output
    }
    const NAME: &'static str = "KlingVirtualTryOnNode";
    const DISPLAY_NAME: &'static str = "Kling Virtual Try On";
    const DESCRIPTION: &'static str = "Kling Virtual Try On Node. Input a human image and a cloth image to try on the cloth on the human. You can merge multiple clothing item pictures into one image with a white background.";
    const CATEGORY: &'static str = "partner/image/Kling";
}
