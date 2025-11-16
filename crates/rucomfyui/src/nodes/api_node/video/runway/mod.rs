//!`Runway` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Runway First-Last-Frame to Video**: Upload first and last keyframes, draft a prompt, and generate a video. More complex transitions, such as cases where the Last frame is completely different from the First frame, may benefit from the longer 10s duration. This would give the generation more time to smoothly transition between the two inputs. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/34170748696595-Creating-with-Keyframes-on-Gen-3.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RunwayFirstLastFrameNode<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt for the generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Start frame to be used for the video
    pub start_frame: StartFrameParam,
    ///End frame to be used for the video. Supported for gen3a_turbo only.
    pub end_frame: EndFrameParam,
    /**Random seed for generation

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> RunwayFirstLastFrameNode<PromptParam, StartFrameParam, EndFrameParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        start_frame: StartFrameParam,
        end_frame: EndFrameParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            start_frame,
            end_frame,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RunwayFirstLastFrameNode<PromptParam, StartFrameParam, EndFrameParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("end_frame".to_string(), self.end_frame.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "RunwayFirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "Runway First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Upload first and last keyframes, draft a prompt, and generate a video. More complex transitions, such as cases where the Last frame is completely different from the First frame, may benefit from the longer 10s duration. This would give the generation more time to smoothly transition between the two inputs. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/34170748696595-Creating-with-Keyframes-on-Gen-3.";
    const CATEGORY: &'static str = "api node/video/Runway";
}
///**Runway Image to Video (Gen3a Turbo)**: Generate a video from a single starting frame using Gen3a Turbo model. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/33927968552339-Creating-with-Act-One-on-Gen-3-Alpha-and-Turbo.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RunwayImageToVideoNodeGen3a<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt for the generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Start frame to be used for the video
    pub start_frame: StartFrameParam,
    /**Random seed for generation

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> RunwayImageToVideoNodeGen3a<PromptParam, StartFrameParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        start_frame: StartFrameParam,
        seed: SeedParam,
    ) -> Self {
        Self { prompt, start_frame, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RunwayImageToVideoNodeGen3a<PromptParam, StartFrameParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "RunwayImageToVideoNodeGen3a";
    const DISPLAY_NAME: &'static str = "Runway Image to Video (Gen3a Turbo)";
    const DESCRIPTION: &'static str = "Generate a video from a single starting frame using Gen3a Turbo model. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/33927968552339-Creating-with-Act-One-on-Gen-3-Alpha-and-Turbo.";
    const CATEGORY: &'static str = "api node/video/Runway";
}
///**Runway Image to Video (Gen4 Turbo)**: Generate a video from a single starting frame using Gen4 Turbo model. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/37327109429011-Creating-with-Gen-4-Video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RunwayImageToVideoNodeGen4<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    /**Text prompt for the generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Start frame to be used for the video
    pub start_frame: StartFrameParam,
    /**Random seed for generation

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> RunwayImageToVideoNodeGen4<PromptParam, StartFrameParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        start_frame: StartFrameParam,
        seed: SeedParam,
    ) -> Self {
        Self { prompt, start_frame, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    StartFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RunwayImageToVideoNodeGen4<PromptParam, StartFrameParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "RunwayImageToVideoNodeGen4";
    const DISPLAY_NAME: &'static str = "Runway Image to Video (Gen4 Turbo)";
    const DESCRIPTION: &'static str = "Generate a video from a single starting frame using Gen4 Turbo model. Before diving in, review these best practices to ensure that your input selections will set your generation up for success: https://help.runwayml.com/hc/en-us/articles/37327109429011-Creating-with-Gen-4-Video.";
    const CATEGORY: &'static str = "api node/video/Runway";
}
