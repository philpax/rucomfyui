//!`Vidu` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Vidu Image To Video Generation**: Generate video from image and optional prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduImageToVideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///An image to be used as the start frame of the generated video
    pub image: ImageParam,
    /**A textual description for video generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: Option<PromptParam>,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 5
  - Min: 5
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> ViduImageToVideoNode<ImageParam, PromptParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: Option<PromptParam>,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            duration,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ViduImageToVideoNode<ImageParam, PromptParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.prompt {
            output.insert("prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ViduImageToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Image To Video Generation";
    const DESCRIPTION: &'static str = "Generate video from image and optional prompt";
    const CATEGORY: &'static str = "api node/video/Vidu";
}
///**Vidu Reference To Video Generation**: Generate video from multiple images and prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduReferenceVideoNode<
    ImagesParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///Images to use as references to generate a video with consistent subjects (max 7 images).
    pub images: ImagesParam,
    /**A textual description for video generation

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 5
  - Min: 5
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> ViduReferenceVideoNode<ImagesParam, PromptParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        prompt: PromptParam,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            images,
            prompt,
            duration,
            seed,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ViduReferenceVideoNode<ImagesParam, PromptParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ViduReferenceVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Reference To Video Generation";
    const DESCRIPTION: &'static str = "Generate video from multiple images and prompt";
    const CATEGORY: &'static str = "api node/video/Vidu";
}
///**Vidu Start End To Video Generation**: Generate a video from start and end frames and a prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduStartEndToVideoNode<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///Start frame
    pub first_frame: FirstFrameParam,
    ///End frame
    pub end_frame: EndFrameParam,
    /**A textual description for video generation

**Metadata**:
  - Multiline: true
*/
    pub prompt: Option<PromptParam>,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 5
  - Min: 5
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> ViduStartEndToVideoNode<
    FirstFrameParam,
    EndFrameParam,
    PromptParam,
    DurationParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        first_frame: FirstFrameParam,
        end_frame: EndFrameParam,
        prompt: Option<PromptParam>,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            first_frame,
            end_frame,
            prompt,
            duration,
            seed,
        }
    }
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ViduStartEndToVideoNode<
    FirstFrameParam,
    EndFrameParam,
    PromptParam,
    DurationParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("end_frame".to_string(), self.end_frame.clone().into());
        if let Some(v) = &self.prompt {
            output.insert("prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ViduStartEndToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Start End To Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from start and end frames and a prompt";
    const CATEGORY: &'static str = "api node/video/Vidu";
}
///**Vidu Text To Video Generation**: Generate video from text prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduTextToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A textual description for video generation

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 5
  - Min: 5
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> ViduTextToVideoNode<PromptParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self { prompt, duration, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ViduTextToVideoNode<PromptParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ViduTextToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Text To Video Generation";
    const DESCRIPTION: &'static str = "Generate video from text prompt";
    const CATEGORY: &'static str = "api node/video/Vidu";
}
