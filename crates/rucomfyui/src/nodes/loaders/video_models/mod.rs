//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageOnlyCheckpointLoader`](super::ImageOnlyCheckpointLoader).
    #[derive(Clone)]
    pub struct ImageOnlyCheckpointLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub clip_vision: crate::nodes::types::ClipVisionOut,
        ///No documentation.
        pub vae: crate::nodes::types::VaeOut,
    }
}
///**Image Only Checkpoint Loader (img2vid model)**: No description.
#[derive(Clone)]
pub struct ImageOnlyCheckpointLoader<CkptName: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptName,
}
impl<CkptName: crate::nodes::types::String> ImageOnlyCheckpointLoader<CkptName> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptName) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for ImageOnlyCheckpointLoader<CkptName> {
    type Output = out::ImageOnlyCheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip_vision: crate::nodes::types::ClipVisionOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output
    }
    const NAME: &'static str = "ImageOnlyCheckpointLoader";
    const DISPLAY_NAME: &'static str = "Image Only Checkpoint Loader (img2vid model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders/video_models";
}
