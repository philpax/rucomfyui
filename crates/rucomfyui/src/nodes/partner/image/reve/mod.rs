//!`Reve` definitions/categories.
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
///**Reve Image Create**: Generate images from text descriptions using Reve.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReveImageCreateNode<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text description of the desired image. Maximum 2560 characters.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Remove the background from the generated image. May add additional cost.

**Metadata**:
  - Default: false
*/
    pub remove_background: RemoveBackgroundParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> ReveImageCreateNode<PromptParam, RemoveBackgroundParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        remove_background: RemoveBackgroundParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            remove_background,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ReveImageCreateNode<PromptParam, RemoveBackgroundParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "remove_background".to_string(),
                self.remove_background.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ReveImageCreateNode";
    const DISPLAY_NAME: &'static str = "Reve Image Create";
    const DESCRIPTION: &'static str = "Generate images from text descriptions using Reve.";
    const CATEGORY: &'static str = "partner/image/Reve";
}
///**Reve Image Edit**: Edit images using natural language instructions with Reve.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReveImageEditNode<
    ImageParam: crate::nodes::types::Image,
    EditInstructionParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> {
    ///The image to edit.
    pub image: ImageParam,
    /**Text description of how to edit the image. Maximum 2560 characters.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub edit_instruction: EditInstructionParam,
    /**Remove the background from the generated image. May add additional cost.

**Metadata**:
  - Default: false
*/
    pub remove_background: RemoveBackgroundParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    EditInstructionParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> ReveImageEditNode<ImageParam, EditInstructionParam, RemoveBackgroundParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        edit_instruction: EditInstructionParam,
        remove_background: RemoveBackgroundParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            edit_instruction,
            remove_background,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    EditInstructionParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ReveImageEditNode<
    ImageParam,
    EditInstructionParam,
    RemoveBackgroundParam,
    SeedParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert(
                "edit_instruction".to_string(),
                self.edit_instruction.clone().into(),
            );
        output
            .insert(
                "remove_background".to_string(),
                self.remove_background.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ReveImageEditNode";
    const DISPLAY_NAME: &'static str = "Reve Image Edit";
    const DESCRIPTION: &'static str = "Edit images using natural language instructions with Reve.";
    const CATEGORY: &'static str = "partner/image/Reve";
}
///**Reve Image Remix**: Combine reference images with text prompts to create new images using Reve.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReveImageRemixNode<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text description of the desired image. May include XML img tags to reference specific images by index, e.g. <img>0</img>, <img>1</img>, etc.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Remove the background from the generated image. May add additional cost.

**Metadata**:
  - Default: false
*/
    pub remove_background: RemoveBackgroundParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> ReveImageRemixNode<PromptParam, RemoveBackgroundParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        remove_background: RemoveBackgroundParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            remove_background,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    RemoveBackgroundParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ReveImageRemixNode<PromptParam, RemoveBackgroundParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "remove_background".to_string(),
                self.remove_background.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ReveImageRemixNode";
    const DISPLAY_NAME: &'static str = "Reve Image Remix";
    const DESCRIPTION: &'static str = "Combine reference images with text prompts to create new images using Reve.";
    const CATEGORY: &'static str = "partner/image/Reve";
}
