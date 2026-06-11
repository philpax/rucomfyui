//!`Quiver` definitions/categories.
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
///**Quiver Image to SVG**: Vectorize a raster image into SVG using Quiver AI.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct QuiverImageToSVGNode<
    ImageParam: crate::nodes::types::Image,
    AutoCropParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> {
    ///Input image to vectorize.
    pub image: ImageParam,
    /**Automatically crop to the dominant subject.

**Metadata**:
  - Default: false
*/
    pub auto_crop: AutoCropParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    AutoCropParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> QuiverImageToSVGNode<ImageParam, AutoCropParam, SeedParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, auto_crop: AutoCropParam, seed: SeedParam) -> Self {
        Self { image, auto_crop, seed }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    AutoCropParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for QuiverImageToSVGNode<ImageParam, AutoCropParam, SeedParam> {
    type Output = crate::nodes::types::SvgOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("auto_crop".to_string(), self.auto_crop.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "QuiverImageToSVGNode";
    const DISPLAY_NAME: &'static str = "Quiver Image to SVG";
    const DESCRIPTION: &'static str = "Vectorize a raster image into SVG using Quiver AI.";
    const CATEGORY: &'static str = "partner/image/Quiver";
}
///**Quiver Text to SVG**: Generate an SVG from a text prompt using Quiver AI.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct QuiverTextToSVGNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    InstructionsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text description of the desired SVG output.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**Additional style or formatting guidance.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub instructions: Option<InstructionsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    InstructionsParam: crate::nodes::types::String,
> QuiverTextToSVGNode<PromptParam, SeedParam, InstructionsParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        instructions: Option<InstructionsParam>,
    ) -> Self {
        Self { prompt, seed, instructions }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    InstructionsParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for QuiverTextToSVGNode<PromptParam, SeedParam, InstructionsParam> {
    type Output = crate::nodes::types::SvgOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.instructions {
            output.insert("instructions".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "QuiverTextToSVGNode";
    const DISPLAY_NAME: &'static str = "Quiver Text to SVG";
    const DESCRIPTION: &'static str = "Generate an SVG from a text prompt using Quiver AI.";
    const CATEGORY: &'static str = "partner/image/Quiver";
}
