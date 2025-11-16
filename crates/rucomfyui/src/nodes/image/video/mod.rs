//!`video` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
    const CATEGORY: &'static str = "image/video";
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
    const CATEGORY: &'static str = "image/video";
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
    const CATEGORY: &'static str = "image/video";
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
    const CATEGORY: &'static str = "image/video";
}
impl<
    VideoParam: crate::nodes::types::Video,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveVideo<VideoParam, FilenamePrefixParam> {}
///**SaveWEBM**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveWEBM<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> {
    ///No documentation.
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
    const DISPLAY_NAME: &'static str = "SaveWEBM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/video";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> crate::nodes::TypedOutputNode
for SaveWEBM<ImagesParam, FilenamePrefixParam, FpsParam, CrfParam> {}
