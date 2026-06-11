//!`HitPaw` definitions/categories.
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
///**HitPaw Video Enhance**: Upscale low-resolution videos to high resolution, eliminate artifacts and noise. Prices shown are per second of video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HitPawVideoEnhance<VideoParam: crate::nodes::types::Video> {
    ///No documentation.
    pub video: VideoParam,
}
impl<VideoParam: crate::nodes::types::Video> HitPawVideoEnhance<VideoParam> {
    /// Create a new node.
    pub fn new(video: VideoParam) -> Self {
        Self { video }
    }
}
impl<VideoParam: crate::nodes::types::Video> crate::nodes::TypedNode
for HitPawVideoEnhance<VideoParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output
    }
    const NAME: &'static str = "HitPawVideoEnhance";
    const DISPLAY_NAME: &'static str = "HitPaw Video Enhance";
    const DESCRIPTION: &'static str = "Upscale low-resolution videos to high resolution, eliminate artifacts and noise. Prices shown are per second of video.";
    const CATEGORY: &'static str = "partner/video/HitPaw";
}
