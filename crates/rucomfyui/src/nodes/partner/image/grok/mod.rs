//!`Grok` definitions/categories.
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
///**Grok Image Edit**: Modify an existing image based on a text prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokImageEditNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**The text prompt used to generate the image

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Number of edited images to generate

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub number_of_images: NumberOfImagesParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

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
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> GrokImageEditNode<ImageParam, PromptParam, NumberOfImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        number_of_images: NumberOfImagesParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            prompt,
            number_of_images,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for GrokImageEditNode<ImageParam, PromptParam, NumberOfImagesParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "number_of_images".to_string(),
                self.number_of_images.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "GrokImageEditNode";
    const DISPLAY_NAME: &'static str = "Grok Image Edit";
    const DESCRIPTION: &'static str = "Modify an existing image based on a text prompt";
    const CATEGORY: &'static str = "partner/image/Grok";
}
///**Grok Image Edit**: Modify an existing image based on a text prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokImageEditNodeV2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**The text prompt used to generate the image

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

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
    SeedParam: crate::nodes::types::Int,
> GrokImageEditNodeV2<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for GrokImageEditNodeV2<PromptParam, SeedParam> {
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
    const NAME: &'static str = "GrokImageEditNodeV2";
    const DISPLAY_NAME: &'static str = "Grok Image Edit";
    const DESCRIPTION: &'static str = "Modify an existing image based on a text prompt";
    const CATEGORY: &'static str = "partner/image/Grok";
}
///**Grok Image**: Generate images using Grok based on a text prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokImageNode<
    PromptParam: crate::nodes::types::String,
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    /**The text prompt used to generate the image

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Number of images to generate

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub number_of_images: NumberOfImagesParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

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
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> GrokImageNode<PromptParam, NumberOfImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        number_of_images: NumberOfImagesParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            number_of_images,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NumberOfImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for GrokImageNode<PromptParam, NumberOfImagesParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "number_of_images".to_string(),
                self.number_of_images.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "GrokImageNode";
    const DISPLAY_NAME: &'static str = "Grok Image";
    const DESCRIPTION: &'static str = "Generate images using Grok based on a text prompt";
    const CATEGORY: &'static str = "partner/image/Grok";
}
