//!`video` definitions/categories.
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
#[rustfmt::skip]
pub mod preprocessors;
/// Output types for nodes.
pub mod out {
    ///Output for [`GetVideoComponents`](super::GetVideoComponents).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GetVideoComponentsOutput {
        ///No documentation.
        pub images: crate::nodes::types::ImageOut,
        ///No documentation.
        pub audio: crate::nodes::types::AudioOut,
        ///No documentation.
        pub fps: crate::nodes::types::FloatOut,
    }
}
///**Create Video**: Create a video from images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CreateVideo<
    ImagesParam: crate::nodes::types::Image,
    FpsParam: crate::nodes::types::Float,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
> {
    ///The images to create a video from.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 30
  - Max: 120
  - Min: 1
  - Step: 1
*/
    pub fps: FpsParam,
    ///The audio to add to the video.
    pub audio: Option<AudioParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FpsParam: crate::nodes::types::Float,
    AudioParam: crate::nodes::types::Audio,
> CreateVideo<ImagesParam, FpsParam, AudioParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, fps: FpsParam, audio: Option<AudioParam>) -> Self {
        Self { images, fps, audio }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FpsParam: crate::nodes::types::Float,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode for CreateVideo<ImagesParam, FpsParam, AudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateVideo";
    const DISPLAY_NAME: &'static str = "Create Video";
    const DESCRIPTION: &'static str = "Create a video from images.";
    const CATEGORY: &'static str = "video";
}
///**Frame Interpolate**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FrameInterpolate<
    InterpModelParam: crate::nodes::types::InterpModel,
    ImagesParam: crate::nodes::types::Image,
    MultiplierParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub interp_model: InterpModelParam,
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 16
  - Min: 2
*/
    pub multiplier: MultiplierParam,
}
impl<
    InterpModelParam: crate::nodes::types::InterpModel,
    ImagesParam: crate::nodes::types::Image,
    MultiplierParam: crate::nodes::types::Int,
> FrameInterpolate<InterpModelParam, ImagesParam, MultiplierParam> {
    /// Create a new node.
    pub fn new(
        interp_model: InterpModelParam,
        images: ImagesParam,
        multiplier: MultiplierParam,
    ) -> Self {
        Self {
            interp_model,
            images,
            multiplier,
        }
    }
}
impl<
    InterpModelParam: crate::nodes::types::InterpModel,
    ImagesParam: crate::nodes::types::Image,
    MultiplierParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for FrameInterpolate<InterpModelParam, ImagesParam, MultiplierParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("interp_model".to_string(), self.interp_model.clone().into());
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "FrameInterpolate";
    const DISPLAY_NAME: &'static str = "Frame Interpolate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "video";
}
///**Get Video Components**: Extracts all components from a video: frames, audio, and framerate.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GetVideoComponents<VideoParam: crate::nodes::types::Video> {
    ///The video to extract components from.
    pub video: VideoParam,
}
impl<VideoParam: crate::nodes::types::Video> GetVideoComponents<VideoParam> {
    /// Create a new node.
    pub fn new(video: VideoParam) -> Self {
        Self { video }
    }
}
impl<VideoParam: crate::nodes::types::Video> crate::nodes::TypedNode
for GetVideoComponents<VideoParam> {
    type Output = out::GetVideoComponentsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            images: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            audio: crate::nodes::types::AudioOut::from_dynamic(node_id, 1u32),
            fps: crate::nodes::types::FloatOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output
    }
    const NAME: &'static str = "GetVideoComponents";
    const DISPLAY_NAME: &'static str = "Get Video Components";
    const DESCRIPTION: &'static str = "Extracts all components from a video: frames, audio, and framerate.";
    const CATEGORY: &'static str = "video";
}
///**Load Video**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadVideo {}
impl LoadVideo {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadVideo {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadVideo";
    const DISPLAY_NAME: &'static str = "Load Video";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "video";
}
///**Save Video**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveVideo<
    VideoParam: crate::nodes::types::Video,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///The video to save.
    pub video: VideoParam,
    /**The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.

**Metadata**:
  - Multiline: false
  - Default: video/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveVideo<VideoParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { video, filename_prefix }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveVideo<VideoParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveVideo";
    const DISPLAY_NAME: &'static str = "Save Video";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "video";
}
impl<
    VideoParam: crate::nodes::types::Video,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveVideo<VideoParam, FilenamePrefixParam> {}
///**Save WEBM**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveWEBM<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> {
    ///RGBA images are saved with their alpha channel as transparency (vp9 codec only).
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 24
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**Higher crf means lower quality with a smaller file size, lower crf means higher quality higher filesize.

**Metadata**:
  - Default: 32
  - Max: 63
  - Min: 0
  - Step: 1
*/
    pub crf: CrfParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> SaveWEBM<ImagesParam, FilenamePrefixParam, FpsParam, CrfParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        fps: FpsParam,
        crf: CrfParam,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            fps,
            crf,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SaveWEBM<ImagesParam, FilenamePrefixParam, FpsParam, CrfParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("crf".to_string(), self.crf.clone().into());
        output
    }
    const NAME: &'static str = "SaveWEBM";
    const DISPLAY_NAME: &'static str = "Save WEBM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "video";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> crate::nodes::TypedOutputNode
for SaveWEBM<ImagesParam, FilenamePrefixParam, FpsParam, CrfParam> {}
///**Video Slice**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Video_Slice<
    VideoParam: crate::nodes::types::Video,
    StartTimeParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
    StrictDurationParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub video: VideoParam,
    /**Start time in seconds

**Metadata**:
  - Default: 0
  - Max: 100000
  - Min: -100000
  - Step: 0.001
*/
    pub start_time: StartTimeParam,
    ///Duration in seconds, or 0 for unlimited duration
    pub duration: DurationParam,
    /**If True, when the specified duration is not possible, an error will be raised.

**Metadata**:
  - Default: false
*/
    pub strict_duration: StrictDurationParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    StartTimeParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
    StrictDurationParam: crate::nodes::types::Boolean,
> Video_Slice<VideoParam, StartTimeParam, DurationParam, StrictDurationParam> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        start_time: StartTimeParam,
        duration: DurationParam,
        strict_duration: StrictDurationParam,
    ) -> Self {
        Self {
            video,
            start_time,
            duration,
            strict_duration,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    StartTimeParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
    StrictDurationParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for Video_Slice<VideoParam, StartTimeParam, DurationParam, StrictDurationParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("start_time".to_string(), self.start_time.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output
            .insert("strict_duration".to_string(), self.strict_duration.clone().into());
        output
    }
    const NAME: &'static str = "Video Slice";
    const DISPLAY_NAME: &'static str = "Video Slice";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "video";
}
