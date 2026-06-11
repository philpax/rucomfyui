//!`Sonilo` definitions/categories.
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
///**Sonilo Text to Music**: Generate music from a text prompt using Sonilo's AI model. Leave duration at 0 to let the model infer it from the prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SoniloTextToMusic<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt describing the music to generate.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Target duration in seconds. Set to 0 to let the model infer the duration from the prompt. Maximum: 6 minutes.

**Metadata**:
  - Default: 0
  - Max: 360
  - Min: 0
*/
    pub duration: DurationParam,
    /**Seed for reproducibility. Currently ignored by the Sonilo service but kept for graph consistency.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> SoniloTextToMusic<PromptParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, duration: DurationParam, seed: SeedParam) -> Self {
        Self { prompt, duration, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SoniloTextToMusic<PromptParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "SoniloTextToMusic";
    const DISPLAY_NAME: &'static str = "Sonilo Text to Music";
    const DESCRIPTION: &'static str = "Generate music from a text prompt using Sonilo's AI model. Leave duration at 0 to let the model infer it from the prompt.";
    const CATEGORY: &'static str = "partner/audio/Sonilo";
}
///**Sonilo Video to Music**: Generate music from video content using Sonilo's AI model. Analyzes the video and creates matching music.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SoniloVideoToMusic<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///Input video to generate music from. Maximum duration: 6 minutes.
    pub video: VideoParam,
    /**Optional text prompt to guide music generation. Leave empty for best quality - the model will fully analyze the video content.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed for reproducibility. Currently ignored by the Sonilo service but kept for graph consistency.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> SoniloVideoToMusic<VideoParam, PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, prompt: PromptParam, seed: SeedParam) -> Self {
        Self { video, prompt, seed }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SoniloVideoToMusic<VideoParam, PromptParam, SeedParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "SoniloVideoToMusic";
    const DISPLAY_NAME: &'static str = "Sonilo Video to Music";
    const DESCRIPTION: &'static str = "Generate music from video content using Sonilo's AI model. Analyzes the video and creates matching music.";
    const CATEGORY: &'static str = "partner/audio/Sonilo";
}
