//!`Kling` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Kling Image Generation**: Kling Image Generation Node. Generate an image from a text prompt with an optional reference image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingImageGenerationNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageFidelityParam: crate::nodes::types::Float,
    HumanFidelityParam: crate::nodes::types::Float,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageFidelityParam: crate::nodes::types::Float,
    HumanFidelityParam: crate::nodes::types::Float,
    NParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> KlingImageGenerationNode<
    PromptParam,
    NegativePromptParam,
    ImageFidelityParam,
    HumanFidelityParam,
    NParam,
    ImageParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        image_fidelity: ImageFidelityParam,
        human_fidelity: HumanFidelityParam,
        n: NParam,
        image: Option<ImageParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            image_fidelity,
            human_fidelity,
            n,
            image,
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
> crate::nodes::TypedNode
for KlingImageGenerationNode<
    PromptParam,
    NegativePromptParam,
    ImageFidelityParam,
    HumanFidelityParam,
    NParam,
    ImageParam,
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
        output
    }
    const NAME: &'static str = "KlingImageGenerationNode";
    const DISPLAY_NAME: &'static str = "Kling Image Generation";
    const DESCRIPTION: &'static str = "Kling Image Generation Node. Generate an image from a text prompt with an optional reference image.";
    const CATEGORY: &'static str = "api node/image/Kling";
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
    const CATEGORY: &'static str = "api node/image/Kling";
}
