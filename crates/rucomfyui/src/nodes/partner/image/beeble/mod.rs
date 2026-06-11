//!`Beeble` definitions/categories.
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
    ///Output for [`BeebleSwitchXImageEdit`](super::BeebleSwitchXImageEdit).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct BeebleSwitchXImageEditOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///The alpha matte Beeble used. Empty for 'fill' mode, which has no separate matte.
        pub alpha: crate::nodes::types::MaskOut,
    }
}
///**Beeble SwitchX Image Edit**: Edit a single image with Beeble SwitchX. Switches anything in the scene (background, lighting, costume) while preserving the original subject's pixels. Provide a reference image and/or text prompt to describe the new look. Max ~2.77MP.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BeebleSwitchXImageEdit<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Text description of the desired output (max 2000 chars). At least one of 'prompt' or 'reference_image' is required.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    ///Reference image whose look (background, lighting, costume) the result should adopt. At least one of 'reference_image' or 'prompt' is required.
    pub reference_image: Option<ReferenceImageParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image,
> BeebleSwitchXImageEdit<ImageParam, PromptParam, SeedParam, ReferenceImageParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        seed: SeedParam,
        reference_image: Option<ReferenceImageParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            seed,
            reference_image,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for BeebleSwitchXImageEdit<ImageParam, PromptParam, SeedParam, ReferenceImageParam> {
    type Output = out::BeebleSwitchXImageEditOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            alpha: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "BeebleSwitchXImageEdit";
    const DISPLAY_NAME: &'static str = "Beeble SwitchX Image Edit";
    const DESCRIPTION: &'static str = "Edit a single image with Beeble SwitchX. Switches anything in the scene (background, lighting, costume) while preserving the original subject's pixels. Provide a reference image and/or text prompt to describe the new look. Max ~2.77MP.";
    const CATEGORY: &'static str = "partner/image/Beeble";
}
