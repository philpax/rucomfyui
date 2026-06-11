//!`WaveSpeed` definitions/categories.
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
///**FlashVSR Video Upscale**: Fast, high-quality video upscaler that boosts resolution and restores clarity for low-resolution or blurry footage.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WavespeedFlashVSRNode<VideoParam: crate::nodes::types::Video> {
    ///No documentation.
    pub video: VideoParam,
}
impl<VideoParam: crate::nodes::types::Video> WavespeedFlashVSRNode<VideoParam> {
    /// Create a new node.
    pub fn new(video: VideoParam) -> Self {
        Self { video }
    }
}
impl<VideoParam: crate::nodes::types::Video> crate::nodes::TypedNode
for WavespeedFlashVSRNode<VideoParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output
    }
    const NAME: &'static str = "WavespeedFlashVSRNode";
    const DISPLAY_NAME: &'static str = "FlashVSR Video Upscale";
    const DESCRIPTION: &'static str = "Fast, high-quality video upscaler that boosts resolution and restores clarity for low-resolution or blurry footage.";
    const CATEGORY: &'static str = "partner/video/WaveSpeed";
}
