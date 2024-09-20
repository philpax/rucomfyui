//!`video_models` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**Image Only Checkpoint Loader (img2vid model)**
pub struct ImageOnlyCheckpointLoader<CkptName: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptName,
}
///Output for [`ImageOnlyCheckpointLoader`].
#[derive(Clone)]
pub struct ImageOnlyCheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
    ///No documentation.
    pub clip_vision: crate::nodes::types::ClipVisionOut,
    ///No documentation.
    pub vae: crate::nodes::types::VaeOut,
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for ImageOnlyCheckpointLoader<CkptName> {
    type Output = ImageOnlyCheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
            clip_vision: crate::nodes::types::ClipVisionOut {
                node_id,
                node_slot: 1u32,
            },
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "ImageOnlyCheckpointLoader";
    const DISPLAY_NAME: &'static str = "Image Only Checkpoint Loader (img2vid model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders/video_models";
}
