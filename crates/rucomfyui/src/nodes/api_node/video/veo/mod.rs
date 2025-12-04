//!`Veo` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Google Veo 3 First-Last-Frame to Video**: Generate video using prompt and first and last frames.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Veo3FirstLastFrameNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
> {
    /**Text description of the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid in the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: NegativePromptParam,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 8
  - Display: slider
  - Max: 8
  - Min: 4
  - Step: 2
*/
    pub duration: DurationParam,
    /**Seed for video generation

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///Start frame
    pub first_frame: FirstFrameParam,
    ///End frame
    pub last_frame: LastFrameParam,
    /**Generate audio for the video.

**Metadata**:
  - Default: true
*/
    pub generate_audio: GenerateAudioParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
> Veo3FirstLastFrameNode<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    SeedParam,
    FirstFrameParam,
    LastFrameParam,
    GenerateAudioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        duration: DurationParam,
        seed: SeedParam,
        first_frame: FirstFrameParam,
        last_frame: LastFrameParam,
        generate_audio: GenerateAudioParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            duration,
            seed,
            first_frame,
            last_frame,
            generate_audio,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for Veo3FirstLastFrameNode<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    SeedParam,
    FirstFrameParam,
    LastFrameParam,
    GenerateAudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("last_frame".to_string(), self.last_frame.clone().into());
        output.insert("generate_audio".to_string(), self.generate_audio.clone().into());
        output
    }
    const NAME: &'static str = "Veo3FirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "Google Veo 3 First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Generate video using prompt and first and last frames.";
    const CATEGORY: &'static str = "api node/video/Veo";
}
///**Google Veo 3 Video Generation**: Generates videos from text prompts using Google's Veo 3 API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Veo3VideoGenerationNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationSecondsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    EnhancePromptParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Text description of the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid in the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Duration of the output video in seconds (Veo 3 only supports 8 seconds)

**Metadata**:
  - Default: 8
  - Display: number
  - Max: 8
  - Min: 8
  - Step: 1
*/
    pub duration_seconds: Option<DurationSecondsParam>,
    /**Whether to enhance the prompt with AI assistance

**Metadata**:
  - Default: true
*/
    pub enhance_prompt: Option<EnhancePromptParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    ///Optional reference image to guide video generation
    pub image: Option<ImageParam>,
    /**Generate audio for the video. Supported by all Veo 3 models.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationSecondsParam: crate::nodes::types::Int,
    EnhancePromptParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
> Veo3VideoGenerationNode<
    PromptParam,
    NegativePromptParam,
    DurationSecondsParam,
    EnhancePromptParam,
    SeedParam,
    ImageParam,
    GenerateAudioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        duration_seconds: Option<DurationSecondsParam>,
        enhance_prompt: Option<EnhancePromptParam>,
        seed: Option<SeedParam>,
        image: Option<ImageParam>,
        generate_audio: Option<GenerateAudioParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            duration_seconds,
            enhance_prompt,
            seed,
            image,
            generate_audio,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationSecondsParam: crate::nodes::types::Int,
    EnhancePromptParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for Veo3VideoGenerationNode<
    PromptParam,
    NegativePromptParam,
    DurationSecondsParam,
    EnhancePromptParam,
    SeedParam,
    ImageParam,
    GenerateAudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.duration_seconds {
            output.insert("duration_seconds".to_string(), v.clone().into());
        }
        if let Some(v) = &self.enhance_prompt {
            output.insert("enhance_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Veo3VideoGenerationNode";
    const DISPLAY_NAME: &'static str = "Google Veo 3 Video Generation";
    const DESCRIPTION: &'static str = "Generates videos from text prompts using Google's Veo 3 API";
    const CATEGORY: &'static str = "api node/video/Veo";
}
///**Google Veo 2 Video Generation**: Generates videos from text prompts using Google's Veo 2 API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VeoVideoGenerationNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationSecondsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    EnhancePromptParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Text description of the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid in the video

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Duration of the output video in seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 8
  - Min: 5
  - Step: 1
*/
    pub duration_seconds: Option<DurationSecondsParam>,
    /**Whether to enhance the prompt with AI assistance

**Metadata**:
  - Default: true
*/
    pub enhance_prompt: Option<EnhancePromptParam>,
    /**Seed for video generation (0 for random)

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    ///Optional reference image to guide video generation
    pub image: Option<ImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationSecondsParam: crate::nodes::types::Int,
    EnhancePromptParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> VeoVideoGenerationNode<
    PromptParam,
    NegativePromptParam,
    DurationSecondsParam,
    EnhancePromptParam,
    SeedParam,
    ImageParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        duration_seconds: Option<DurationSecondsParam>,
        enhance_prompt: Option<EnhancePromptParam>,
        seed: Option<SeedParam>,
        image: Option<ImageParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            duration_seconds,
            enhance_prompt,
            seed,
            image,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationSecondsParam: crate::nodes::types::Int,
    EnhancePromptParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for VeoVideoGenerationNode<
    PromptParam,
    NegativePromptParam,
    DurationSecondsParam,
    EnhancePromptParam,
    SeedParam,
    ImageParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.duration_seconds {
            output.insert("duration_seconds".to_string(), v.clone().into());
        }
        if let Some(v) = &self.enhance_prompt {
            output.insert("enhance_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "VeoVideoGenerationNode";
    const DISPLAY_NAME: &'static str = "Google Veo 2 Video Generation";
    const DESCRIPTION: &'static str = "Generates videos from text prompts using Google's Veo 2 API";
    const CATEGORY: &'static str = "api node/video/Veo";
}
