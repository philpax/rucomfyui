//!`Wan` definitions/categories.
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
///**HappyHorse Image to Video**: Generate a video from a first-frame image using the HappyHorse model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HappyHorseImageToVideoApi<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    ///First frame image. The output aspect ratio is derived from this image.
    pub first_frame: FirstFrameParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> HappyHorseImageToVideoApi<FirstFrameParam, SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(
        first_frame: FirstFrameParam,
        seed: SeedParam,
        watermark: WatermarkParam,
    ) -> Self {
        Self {
            first_frame,
            seed,
            watermark,
        }
    }
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for HappyHorseImageToVideoApi<FirstFrameParam, SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "HappyHorseImageToVideoApi";
    const DISPLAY_NAME: &'static str = "HappyHorse Image to Video";
    const DESCRIPTION: &'static str = "Generate a video from a first-frame image using the HappyHorse model.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**HappyHorse Reference to Video**: Generate a video featuring a person or object from reference materials with the HappyHorse model. Supports single-character performances and multi-character interactions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HappyHorseReferenceVideoApi<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> HappyHorseReferenceVideoApi<SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { seed, watermark }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for HappyHorseReferenceVideoApi<SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "HappyHorseReferenceVideoApi";
    const DISPLAY_NAME: &'static str = "HappyHorse Reference to Video";
    const DESCRIPTION: &'static str = "Generate a video featuring a person or object from reference materials with the HappyHorse model. Supports single-character performances and multi-character interactions.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**HappyHorse Text to Video**: Generates a video based on a text prompt using the HappyHorse model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HappyHorseTextToVideoApi<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> HappyHorseTextToVideoApi<SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { seed, watermark }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for HappyHorseTextToVideoApi<SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "HappyHorseTextToVideoApi";
    const DISPLAY_NAME: &'static str = "HappyHorse Text to Video";
    const DESCRIPTION: &'static str = "Generates a video based on a text prompt using the HappyHorse model.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**HappyHorse Video Edit**: Edit a video using text instructions or reference images with the HappyHorse model. Output duration is 3-15s and matches the input video; inputs longer than 15s are truncated.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HappyHorseVideoEditApi<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    ///The video to edit.
    pub video: VideoParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> HappyHorseVideoEditApi<VideoParam, SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { video, seed, watermark }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for HappyHorseVideoEditApi<VideoParam, SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "HappyHorseVideoEditApi";
    const DISPLAY_NAME: &'static str = "HappyHorse Video Edit";
    const DESCRIPTION: &'static str = "Edit a video using text instructions or reference images with the HappyHorse model. Output duration is 3-15s and matches the input video; inputs longer than 15s are truncated.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan 2.7 Image to Video**: Generate a video from a first-frame image, with optional last-frame image and audio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan2ImageToVideoApi<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
> {
    ///First frame image. The output aspect ratio is derived from this image.
    pub first_frame: FirstFrameParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: PromptExtendParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
    ///Last frame image. The model generates a video transitioning from first to last frame.
    pub last_frame: Option<LastFrameParam>,
    ///Audio for driving video generation (e.g., lip sync, beat-matched motion). Duration: 2s-30s. If not provided, the model automatically generates matching background music or sound effects.
    pub audio: Option<AudioParam>,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
> Wan2ImageToVideoApi<
    FirstFrameParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
    LastFrameParam,
    AudioParam,
> {
    /// Create a new node.
    pub fn new(
        first_frame: FirstFrameParam,
        seed: SeedParam,
        prompt_extend: PromptExtendParam,
        watermark: WatermarkParam,
        last_frame: Option<LastFrameParam>,
        audio: Option<AudioParam>,
    ) -> Self {
        Self {
            first_frame,
            seed,
            prompt_extend,
            watermark,
            last_frame,
            audio,
        }
    }
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode
for Wan2ImageToVideoApi<
    FirstFrameParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
    LastFrameParam,
    AudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("prompt_extend".to_string(), self.prompt_extend.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        if let Some(v) = &self.last_frame {
            output.insert("last_frame".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Wan2ImageToVideoApi";
    const DISPLAY_NAME: &'static str = "Wan 2.7 Image to Video";
    const DESCRIPTION: &'static str = "Generate a video from a first-frame image, with optional last-frame image and audio.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan 2.7 Reference to Video**: Generate a video featuring a person or object from reference materials. Supports single-character performances and multi-character interactions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan2ReferenceVideoApi<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> Wan2ReferenceVideoApi<SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { seed, watermark }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for Wan2ReferenceVideoApi<SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "Wan2ReferenceVideoApi";
    const DISPLAY_NAME: &'static str = "Wan 2.7 Reference to Video";
    const DESCRIPTION: &'static str = "Generate a video featuring a person or object from reference materials. Supports single-character performances and multi-character interactions.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan 2.7 Text to Video**: Generates a video based on a text prompt using the Wan 2.7 model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan2TextToVideoApi<
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
> {
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: PromptExtendParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
    ///Audio for driving video generation (e.g., lip sync, beat-matched motion). Duration: 3s-30s. If not provided, the model automatically generates matching background music or sound effects.
    pub audio: Option<AudioParam>,
}
impl<
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    AudioParam: crate::nodes::types::Audio,
> Wan2TextToVideoApi<SeedParam, PromptExtendParam, WatermarkParam, AudioParam> {
    /// Create a new node.
    pub fn new(
        seed: SeedParam,
        prompt_extend: PromptExtendParam,
        watermark: WatermarkParam,
        audio: Option<AudioParam>,
    ) -> Self {
        Self {
            seed,
            prompt_extend,
            watermark,
            audio,
        }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode
for Wan2TextToVideoApi<SeedParam, PromptExtendParam, WatermarkParam, AudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("prompt_extend".to_string(), self.prompt_extend.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Wan2TextToVideoApi";
    const DISPLAY_NAME: &'static str = "Wan 2.7 Text to Video";
    const DESCRIPTION: &'static str = "Generates a video based on a text prompt using the Wan 2.7 model.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan 2.7 Video Continuation**: Continue a video from where it left off, with optional last-frame control.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan2VideoContinuationApi<
    FirstClipParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///Input video to continue from. Duration: 2s-10s. The output aspect ratio is derived from this video.
    pub first_clip: FirstClipParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: PromptExtendParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
    ///Last frame image. The continuation will transition towards this frame.
    pub last_frame: Option<LastFrameParam>,
}
impl<
    FirstClipParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image,
> Wan2VideoContinuationApi<
    FirstClipParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
    LastFrameParam,
> {
    /// Create a new node.
    pub fn new(
        first_clip: FirstClipParam,
        seed: SeedParam,
        prompt_extend: PromptExtendParam,
        watermark: WatermarkParam,
        last_frame: Option<LastFrameParam>,
    ) -> Self {
        Self {
            first_clip,
            seed,
            prompt_extend,
            watermark,
            last_frame,
        }
    }
}
impl<
    FirstClipParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    PromptExtendParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    LastFrameParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for Wan2VideoContinuationApi<
    FirstClipParam,
    SeedParam,
    PromptExtendParam,
    WatermarkParam,
    LastFrameParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_clip".to_string(), self.first_clip.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("prompt_extend".to_string(), self.prompt_extend.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        if let Some(v) = &self.last_frame {
            output.insert("last_frame".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Wan2VideoContinuationApi";
    const DISPLAY_NAME: &'static str = "Wan 2.7 Video Continuation";
    const DESCRIPTION: &'static str = "Continue a video from where it left off, with optional last-frame control.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan 2.7 Video Edit**: Edit a video using text instructions, reference images, or style transfer.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan2VideoEditApi<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    ///The video to edit.
    pub video: VideoParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> Wan2VideoEditApi<VideoParam, SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { video, seed, watermark }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for Wan2VideoEditApi<VideoParam, SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "Wan2VideoEditApi";
    const DISPLAY_NAME: &'static str = "Wan 2.7 Video Edit";
    const DESCRIPTION: &'static str = "Edit a video using text instructions, reference images, or style transfer.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan Image to Video**: Generates a video from the first frame and a text prompt.
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
    /**Prompt describing the elements and visual features. Supports English and Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative prompt describing what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Duration 15 available only for WAN2.6 model.

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 15
  - Min: 5
  - Step: 5
*/
    pub duration: Option<DurationParam>,
    ///Audio must contain a clear, loud voice, without extraneous noise or background music.
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
    /**If no audio input is provided, generate audio automatically.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: Option<PromptExtendParam>,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
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
    const DESCRIPTION: &'static str = "Generates a video from the first frame and a text prompt.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan Reference to Video**: Use the character and voice from input videos, combined with a prompt, to generate a new video that maintains character consistency.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanReferenceVideoApi<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Prompt describing the elements and visual features. Supports English and Chinese. Use identifiers such as `character1` and `character2` to refer to the reference characters.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative prompt describing what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 10
  - Min: 5
  - Step: 5
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> WanReferenceVideoApi<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    SeedParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        duration: DurationParam,
        seed: SeedParam,
        watermark: WatermarkParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            duration,
            seed,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for WanReferenceVideoApi<
    PromptParam,
    NegativePromptParam,
    DurationParam,
    SeedParam,
    WatermarkParam,
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
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "WanReferenceVideoApi";
    const DISPLAY_NAME: &'static str = "Wan Reference to Video";
    const DESCRIPTION: &'static str = "Use the character and voice from input videos, combined with a prompt, to generate a new video that maintains character consistency.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
///**Wan Text to Video**: Generates a video based on a text prompt.
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
    /**Prompt describing the elements and visual features. Supports English and Chinese.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Negative prompt describing what to avoid.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**A 15-second duration is available only for the Wan 2.6 model.

**Metadata**:
  - Default: 5
  - Display: number
  - Max: 15
  - Min: 5
  - Step: 5
*/
    pub duration: Option<DurationParam>,
    ///Audio must contain a clear, loud voice, without extraneous noise or background music.
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
    /**If no audio input is provided, generate audio automatically.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Whether to enhance the prompt with AI assistance.

**Metadata**:
  - Default: true
*/
    pub prompt_extend: Option<PromptExtendParam>,
    /**Whether to add an AI-generated watermark to the result.

**Metadata**:
  - Default: false
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
    const DESCRIPTION: &'static str = "Generates a video based on a text prompt.";
    const CATEGORY: &'static str = "partner/video/Wan";
}
