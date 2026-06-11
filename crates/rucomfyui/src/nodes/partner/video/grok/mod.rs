//!`Grok` definitions/categories.
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
///**Grok Video Edit**: Edit an existing video based on a text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokVideoEditNode<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text description of the desired video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///Maximum supported duration is 8.7 seconds and 50MB file size.
    pub video: VideoParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> GrokVideoEditNode<PromptParam, VideoParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, video: VideoParam, seed: SeedParam) -> Self {
        Self { prompt, video, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for GrokVideoEditNode<PromptParam, VideoParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "GrokVideoEditNode";
    const DISPLAY_NAME: &'static str = "Grok Video Edit";
    const DESCRIPTION: &'static str = "Edit an existing video based on a text prompt.";
    const CATEGORY: &'static str = "partner/video/Grok";
}
///**Grok Video Extend**: Extend an existing video with a seamless continuation based on a text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokVideoExtendNode<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text description of what should happen next in the video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///Source video to extend. MP4 format, 2-15 seconds.
    pub video: VideoParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> GrokVideoExtendNode<PromptParam, VideoParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, video: VideoParam, seed: SeedParam) -> Self {
        Self { prompt, video, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for GrokVideoExtendNode<PromptParam, VideoParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "GrokVideoExtendNode";
    const DISPLAY_NAME: &'static str = "Grok Video Extend";
    const DESCRIPTION: &'static str = "Extend an existing video with a seamless continuation based on a text prompt.";
    const CATEGORY: &'static str = "partner/video/Grok";
}
///**Grok Video**: Generate video from a prompt or an image
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Text description of the desired video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**The duration of the output video in seconds.

**Metadata**:
  - Default: 6
  - Display: slider
  - Max: 15
  - Min: 1
  - Step: 1
*/
    pub duration: DurationParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///Optional starting image for grok-imagine-video. Required for grok-imagine-video-1.5.
    pub image: Option<ImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> GrokVideoNode<PromptParam, DurationParam, SeedParam, ImageParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        seed: SeedParam,
        image: Option<ImageParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            seed,
            image,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for GrokVideoNode<PromptParam, DurationParam, SeedParam, ImageParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GrokVideoNode";
    const DISPLAY_NAME: &'static str = "Grok Video";
    const DESCRIPTION: &'static str = "Generate video from a prompt or an image";
    const CATEGORY: &'static str = "partner/video/Grok";
}
///**Grok Reference-to-Video**: Generate video guided by reference images as style and content references.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrokVideoReferenceNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text description of the desired video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> GrokVideoReferenceNode<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for GrokVideoReferenceNode<PromptParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "GrokVideoReferenceNode";
    const DISPLAY_NAME: &'static str = "Grok Reference-to-Video";
    const DESCRIPTION: &'static str = "Generate video guided by reference images as style and content references.";
    const CATEGORY: &'static str = "partner/video/Grok";
}
