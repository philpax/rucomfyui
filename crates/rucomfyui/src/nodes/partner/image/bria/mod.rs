//!`Bria` definitions/categories.
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
/// Output types for nodes.
pub mod out {
    ///Output for [`BriaImageEditNode`](super::BriaImageEditNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct BriaImageEditNodeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub structured_prompt: crate::nodes::types::StringOut,
    }
}
///**Bria FIBO Image Edit**: Edit images using Bria latest model
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BriaImageEditNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    StructuredPromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    GuidanceScaleParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Instruction to edit image

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: NegativePromptParam,
    /**A string containing the structured edit prompt in JSON format. Use this instead of usual prompt for precise, programmatic control.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub structured_prompt: StructuredPromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 1
  - Step: 1
*/
    pub seed: SeedParam,
    /**Higher value makes the image follow the prompt more closely.

**Metadata**:
  - Default: 3
  - Display: number
  - Max: 5
  - Min: 3
  - Step: 0.01
*/
    pub guidance_scale: GuidanceScaleParam,
    /**No documentation.

**Metadata**:
  - Default: 50
  - Display: number
  - Max: 50
  - Min: 20
  - Step: 1
*/
    pub steps: StepsParam,
    ///If omitted, the edit applies to the entire image.
    pub mask: Option<MaskParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    StructuredPromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    GuidanceScaleParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    MaskParam: crate::nodes::types::Mask,
> BriaImageEditNode<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    StructuredPromptParam,
    SeedParam,
    GuidanceScaleParam,
    StepsParam,
    MaskParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        structured_prompt: StructuredPromptParam,
        seed: SeedParam,
        guidance_scale: GuidanceScaleParam,
        steps: StepsParam,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            negative_prompt,
            structured_prompt,
            seed,
            guidance_scale,
            steps,
            mask,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    StructuredPromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    GuidanceScaleParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for BriaImageEditNode<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    StructuredPromptParam,
    SeedParam,
    GuidanceScaleParam,
    StepsParam,
    MaskParam,
> {
    type Output = out::BriaImageEditNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            structured_prompt: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                1u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output
            .insert(
                "structured_prompt".to_string(),
                self.structured_prompt.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("guidance_scale".to_string(), self.guidance_scale.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "BriaImageEditNode";
    const DISPLAY_NAME: &'static str = "Bria FIBO Image Edit";
    const DESCRIPTION: &'static str = "Edit images using Bria latest model";
    const CATEGORY: &'static str = "partner/image/Bria";
}
///**Bria Remove Image Background**: Remove the background from an image using Bria RMBG 2.0.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BriaRemoveImageBackground<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> BriaRemoveImageBackground<ImageParam, SeedParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, seed: SeedParam) -> Self {
        Self { image, seed }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for BriaRemoveImageBackground<ImageParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "BriaRemoveImageBackground";
    const DISPLAY_NAME: &'static str = "Bria Remove Image Background";
    const DESCRIPTION: &'static str = "Remove the background from an image using Bria RMBG 2.0.";
    const CATEGORY: &'static str = "partner/image/Bria";
}
