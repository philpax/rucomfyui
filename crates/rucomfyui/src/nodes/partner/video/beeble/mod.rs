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
    ///Output for [`BeebleSwitchXVideoEdit`](super::BeebleSwitchXVideoEdit).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct BeebleSwitchXVideoEditOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///The alpha matte Beeble used. Empty for 'fill' mode, which has no separate matte.
        pub alpha: crate::nodes::types::VideoOut,
    }
}
///**Beeble SwitchX Video Edit**: Edit a video with Beeble SwitchX. Switches anything in the scene (background, lighting, costume) while preserving the original subject's pixels and motion. Provide a reference image and/or text prompt to describe the new look. Max 240 frames, max ~2.77MP per frame.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BeebleSwitchXVideoEdit<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub video: VideoParam,
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
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image,
> BeebleSwitchXVideoEdit<VideoParam, PromptParam, SeedParam, ReferenceImageParam> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        prompt: PromptParam,
        seed: SeedParam,
        reference_image: Option<ReferenceImageParam>,
    ) -> Self {
        Self {
            video,
            prompt,
            seed,
            reference_image,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ReferenceImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for BeebleSwitchXVideoEdit<VideoParam, PromptParam, SeedParam, ReferenceImageParam> {
    type Output = out::BeebleSwitchXVideoEditOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            alpha: crate::nodes::types::VideoOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "BeebleSwitchXVideoEdit";
    const DISPLAY_NAME: &'static str = "Beeble SwitchX Video Edit";
    const DESCRIPTION: &'static str = "Edit a video with Beeble SwitchX. Switches anything in the scene (background, lighting, costume) while preserving the original subject's pixels and motion. Provide a reference image and/or text prompt to describe the new look. Max 240 frames, max ~2.77MP per frame.";
    const CATEGORY: &'static str = "partner/video/Beeble";
}
