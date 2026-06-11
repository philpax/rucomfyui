//!`Vidu` definitions/categories.
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
///**Vidu2 Image-to-Video Generation**: Generate a video from an image and an optional prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu2ImageToVideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///An image to be used as the start frame of the generated video.
    pub image: ImageParam,
    /**An optional text prompt for video generation (max 2000 characters).

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> Vidu2ImageToVideoNode<ImageParam, PromptParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        duration: DurationParam,
        seed: SeedParam,
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
for Vidu2ImageToVideoNode<ImageParam, PromptParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Vidu2ImageToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu2 Image-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from an image and an optional prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu2 Reference-to-Video Generation**: Generate a video from multiple reference images and a prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu2ReferenceVideoNode<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Boolean,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    /**When enabled, the video will include generated speech and background music based on the prompt.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**When enabled video will contain generated speech and background music based on the prompt.

**Metadata**:
  - Default: false
*/
    pub audio: AudioParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Boolean,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> Vidu2ReferenceVideoNode<PromptParam, AudioParam, DurationParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        audio: AudioParam,
        duration: DurationParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            audio,
            duration,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Boolean,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Vidu2ReferenceVideoNode<PromptParam, AudioParam, DurationParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Vidu2ReferenceVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu2 Reference-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from multiple reference images and a prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu2 Start/End Frame-to-Video Generation**: Generate a video from a start frame, an end frame, and a prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu2StartEndToVideoNode<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub first_frame: FirstFrameParam,
    ///No documentation.
    pub end_frame: EndFrameParam,
    /**Prompt description (max 2000 characters).

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 8
  - Min: 2
  - Step: 1
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> Vidu2StartEndToVideoNode<
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
        prompt: PromptParam,
        duration: DurationParam,
        seed: SeedParam,
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
for Vidu2StartEndToVideoNode<
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
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Vidu2StartEndToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu2 Start/End Frame-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from a start frame, an end frame, and a prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu2 Text-to-Video Generation**: Generate video from a text prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu2TextToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    BackgroundMusicParam: crate::nodes::types::Boolean,
> {
    /**A textual description for video generation, with a maximum length of 2000 characters.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add background music to the generated video.

**Metadata**:
  - Default: false
*/
    pub background_music: BackgroundMusicParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    BackgroundMusicParam: crate::nodes::types::Boolean,
> Vidu2TextToVideoNode<PromptParam, DurationParam, SeedParam, BackgroundMusicParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        seed: SeedParam,
        background_music: BackgroundMusicParam,
    ) -> Self {
        Self {
            prompt,
            duration,
            seed,
            background_music,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    BackgroundMusicParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for Vidu2TextToVideoNode<PromptParam, DurationParam, SeedParam, BackgroundMusicParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "background_music".to_string(),
                self.background_music.clone().into(),
            );
        output
    }
    const NAME: &'static str = "Vidu2TextToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu2 Text-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate video from a text prompt";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Q3 Image-to-Video Generation**: Generate a video from an image and an optional prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu3ImageToVideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///An image to be used as the start frame of the generated video.
    pub image: ImageParam,
    /**An optional text prompt for video generation (max 2000 characters).

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Vidu3ImageToVideoNode<ImageParam, PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, prompt: PromptParam, seed: SeedParam) -> Self {
        Self { image, prompt, seed }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Vidu3ImageToVideoNode<ImageParam, PromptParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Vidu3ImageToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Q3 Image-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from an image and an optional prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Q3 Start/End Frame-to-Video Generation**: Generate a video from a start frame, an end frame, and a prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu3StartEndToVideoNode<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub first_frame: FirstFrameParam,
    ///No documentation.
    pub end_frame: EndFrameParam,
    /**Prompt description (max 2000 characters).

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Vidu3StartEndToVideoNode<FirstFrameParam, EndFrameParam, PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        first_frame: FirstFrameParam,
        end_frame: EndFrameParam,
        prompt: PromptParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            first_frame,
            end_frame,
            prompt,
            seed,
        }
    }
}
impl<
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Vidu3StartEndToVideoNode<FirstFrameParam, EndFrameParam, PromptParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("end_frame".to_string(), self.end_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Vidu3StartEndToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Q3 Start/End Frame-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate a video from a start frame, an end frame, and a prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Q3 Text-to-Video Generation**: Generate video from a text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Vidu3TextToVideoNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**A textual description for video generation, with a maximum length of 2000 characters.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
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
> Vidu3TextToVideoNode<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Vidu3TextToVideoNode<PromptParam, SeedParam> {
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
    const NAME: &'static str = "Vidu3TextToVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Q3 Text-to-Video Generation";
    const DESCRIPTION: &'static str = "Generate video from a text prompt.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Video Extension**: Extend an existing video by generating additional frames.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduExtendVideoNode<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    EndFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///The source video to extend.
    pub video: VideoParam,
    /**An optional text prompt for the extended video (max 2000 characters).

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///No documentation.
    pub end_frame: Option<EndFrameParam>,
}
impl<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    EndFrameParam: crate::nodes::types::Image,
> ViduExtendVideoNode<VideoParam, PromptParam, SeedParam, EndFrameParam> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        prompt: PromptParam,
        seed: SeedParam,
        end_frame: Option<EndFrameParam>,
    ) -> Self {
        Self {
            video,
            prompt,
            seed,
            end_frame,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    EndFrameParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for ViduExtendVideoNode<VideoParam, PromptParam, SeedParam, EndFrameParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.end_frame {
            output.insert("end_frame".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ViduExtendVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Video Extension";
    const DESCRIPTION: &'static str = "Extend an existing video by generating additional frames.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
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
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Multi-Frame Video Generation**: Generate a video with multiple keyframe transitions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ViduMultiFrameVideoNode<
    StartImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    ///The starting frame image. Aspect ratio must be between 1:4 and 4:1.
    pub start_image: StartImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
}
impl<
    StartImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> ViduMultiFrameVideoNode<StartImageParam, SeedParam> {
    /// Create a new node.
    pub fn new(start_image: StartImageParam, seed: SeedParam) -> Self {
        Self { start_image, seed }
    }
}
impl<
    StartImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ViduMultiFrameVideoNode<StartImageParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_image".to_string(), self.start_image.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ViduMultiFrameVideoNode";
    const DISPLAY_NAME: &'static str = "Vidu Multi-Frame Video Generation";
    const DESCRIPTION: &'static str = "Generate a video with multiple keyframe transitions.";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Reference To Video Generation**: Generate video from multiple images and a prompt
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
    const DESCRIPTION: &'static str = "Generate video from multiple images and a prompt";
    const CATEGORY: &'static str = "partner/video/Vidu";
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
    const CATEGORY: &'static str = "partner/video/Vidu";
}
///**Vidu Text To Video Generation**: Generate video from a text prompt
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
    const DESCRIPTION: &'static str = "Generate video from a text prompt";
    const CATEGORY: &'static str = "partner/video/Vidu";
}
