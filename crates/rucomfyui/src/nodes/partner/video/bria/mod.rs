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
    ///Output for [`BriaTransparentVideoBackground`](super::BriaTransparentVideoBackground).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct BriaTransparentVideoBackgroundOutput {
        ///No documentation.
        pub images: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Bria Remove Video Background**: Remove the background from a video using Bria.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BriaRemoveVideoBackground<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub video: VideoParam,
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
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> BriaRemoveVideoBackground<VideoParam, SeedParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, seed: SeedParam) -> Self {
        Self { video, seed }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for BriaRemoveVideoBackground<VideoParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "BriaRemoveVideoBackground";
    const DISPLAY_NAME: &'static str = "Bria Remove Video Background";
    const DESCRIPTION: &'static str = "Remove the background from a video using Bria. ";
    const CATEGORY: &'static str = "partner/video/Bria";
}
///**Bria Remove Video Background (Transparent)**: Remove the background from a video using Bria and return the cut-out frames plus an alpha mask. Connect both to a compositing node, or feed them to Save WEBM to write a transparent video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BriaTransparentVideoBackground<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub video: VideoParam,
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
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> BriaTransparentVideoBackground<VideoParam, SeedParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, seed: SeedParam) -> Self {
        Self { video, seed }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for BriaTransparentVideoBackground<VideoParam, SeedParam> {
    type Output = out::BriaTransparentVideoBackgroundOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            images: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "BriaTransparentVideoBackground";
    const DISPLAY_NAME: &'static str = "Bria Remove Video Background (Transparent)";
    const DESCRIPTION: &'static str = "Remove the background from a video using Bria and return the cut-out frames plus an alpha mask. Connect both to a compositing node, or feed them to Save WEBM to write a transparent video.";
    const CATEGORY: &'static str = "partner/video/Bria";
}
///**Bria Video Green Screen**: Replace a video's background with a solid chroma-key screen using Bria.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BriaVideoGreenScreen<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub video: VideoParam,
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
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> BriaVideoGreenScreen<VideoParam, SeedParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, seed: SeedParam) -> Self {
        Self { video, seed }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for BriaVideoGreenScreen<VideoParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "BriaVideoGreenScreen";
    const DISPLAY_NAME: &'static str = "Bria Video Green Screen";
    const DESCRIPTION: &'static str = "Replace a video's background with a solid chroma-key screen using Bria.";
    const CATEGORY: &'static str = "partner/video/Bria";
}
