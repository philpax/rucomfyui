//!`Wan` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Wan Image to Video**: Generates video based on the first frame and text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanImageToVideoApi<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PromptExtendParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt used to describe the elements and visual features, supports English/Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Available durations: 5 and 10 seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 10
  - Min: 5
  - Step: 5
*/
    pub duration: Option<DurationParam>,
    ///Audio must contain a clear, loud voice, without extraneous noise, background music.
    pub audio: Option<AudioParam>,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**If there is no audio input, generate audio automatically.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: Option<PromptExtendParam>,
    /**Whether to add an "AI generated" watermark to the result.

**Metadata**:
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> WanImageToVideoApi<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    DurationParam,
    AudioParam,
    SeedParam,
    GenerateAudioParam,
    PromptExtendParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        duration: Option<DurationParam>,
        audio: Option<AudioParam>,
        seed: Option<SeedParam>,
        generate_audio: Option<GenerateAudioParam>,
        prompt_extend: Option<PromptExtendParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            negative_prompt,
            duration,
            audio,
            seed,
            generate_audio,
            prompt_extend,
            watermark,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanImageToVideoApi<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    DurationParam,
    AudioParam,
    SeedParam,
    GenerateAudioParam,
    PromptExtendParam,
    WatermarkParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.prompt_extend {
            output.insert("prompt_extend".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanImageToVideoApi";
    const DISPLAY_NAME: &'static str = "Wan Image to Video";
    const DESCRIPTION: &'static str = "Generates video based on the first frame and text prompt.";
    const CATEGORY: &'static str = "api node/video/Wan";
}
///**Wan Text to Video**: Generates video based on text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanTextToVideoApi<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PromptExtendParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Prompt used to describe the elements and visual features, supports English/Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative text prompt to guide what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Available durations: 5 and 10 seconds

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 10
  - Min: 5
  - Step: 5
*/
    pub duration: Option<DurationParam>,
    ///Audio must contain a clear, loud voice, without extraneous noise, background music.
    pub audio: Option<AudioParam>,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**If there is no audio input, generate audio automatically.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: Option<PromptExtendParam>,
    /**Whether to add an "AI generated" watermark to the result.

**Metadata**:
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> WanTextToVideoApi<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    AudioParam,
    SeedParam,
    GenerateAudioParam,
    PromptExtendParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        duration: Option<DurationParam>,
        audio: Option<AudioParam>,
        seed: Option<SeedParam>,
        generate_audio: Option<GenerateAudioParam>,
        prompt_extend: Option<PromptExtendParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            duration,
            audio,
            seed,
            generate_audio,
            prompt_extend,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanTextToVideoApi<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    AudioParam,
    SeedParam,
    GenerateAudioParam,
    PromptExtendParam,
    WatermarkParam,
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
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.prompt_extend {
            output.insert("prompt_extend".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanTextToVideoApi";
    const DISPLAY_NAME: &'static str = "Wan Text to Video";
    const DESCRIPTION: &'static str = "Generates video based on text prompt.";
    const CATEGORY: &'static str = "api node/video/Wan";
}
