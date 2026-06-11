//!`Krea` definitions/categories.
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
///**Krea 2 Image**: Generate images via Krea 2 — pick Medium (expressive illustrations) or Large (expressive photorealism). Supports an optional moodboard and up to 10 chained image style references.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Krea2ImageNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt for the image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Random seed for reproducibility.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Krea2ImageNode<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Krea2ImageNode<PromptParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Krea2ImageNode";
    const DISPLAY_NAME: &'static str = "Krea 2 Image";
    const DESCRIPTION: &'static str = "Generate images via Krea 2 — pick Medium (expressive illustrations) or Large (expressive photorealism). Supports an optional moodboard and up to 10 chained image style references.";
    const CATEGORY: &'static str = "partner/image/Krea";
}
///**Krea 2 Style Reference**: Add an image style reference to a Krea 2 generation. Chain multiple Krea 2 Style Reference nodes (max 10) and feed the final `style_reference` output into Krea 2 Image. Each image is uploaded to ComfyAPI storage and passed as URL.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Krea2StyleReferenceNode<
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StyleReferenceParam: crate::nodes::types::KreaStyleRef
        = crate::nodes::types::KreaStyleRefOut,
> {
    ///Reference image whose style influences the generation.
    pub image: ImageParam,
    /**Reference strength; negative values invert the style influence.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: -2
  - Step: 0.05
*/
    pub strength: StrengthParam,
    ///Optional incoming chain of style references; this node appends one more.
    pub style_reference: Option<StyleReferenceParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StyleReferenceParam: crate::nodes::types::KreaStyleRef,
> Krea2StyleReferenceNode<ImageParam, StrengthParam, StyleReferenceParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        strength: StrengthParam,
        style_reference: Option<StyleReferenceParam>,
    ) -> Self {
        Self {
            image,
            strength,
            style_reference,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
    StyleReferenceParam: crate::nodes::types::KreaStyleRef,
> crate::nodes::TypedNode
for Krea2StyleReferenceNode<ImageParam, StrengthParam, StyleReferenceParam> {
    type Output = crate::nodes::types::KreaStyleRefOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        if let Some(v) = &self.style_reference {
            output.insert("style_reference".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Krea2StyleReferenceNode";
    const DISPLAY_NAME: &'static str = "Krea 2 Style Reference";
    const DESCRIPTION: &'static str = "Add an image style reference to a Krea 2 generation. Chain multiple Krea 2 Style Reference nodes (max 10) and feed the final `style_reference` output into Krea 2 Image. Each image is uploaded to ComfyAPI storage and passed as URL.";
    const CATEGORY: &'static str = "partner/image/Krea";
}
