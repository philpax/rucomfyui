//!`ByteDance` definitions/categories.
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
/// Output types for nodes.
pub mod out {
    ///Output for [`ByteDanceCreateVideoAsset`](super::ByteDanceCreateVideoAsset).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ByteDanceCreateVideoAssetOutput {
        ///No documentation.
        pub asset_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub group_id: crate::nodes::types::StringOut,
    }
}
///**ByteDance Seedance 2.0 First-Last-Frame to Video**: Generate video using Seedance 2.0 from a first frame image and optional last frame image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDance2FirstLastFrameNode<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
    FirstFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    LastFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FirstFrameAssetIdParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    LastFrameAssetIdParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add a watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
    ///First frame image for the video.
    pub first_frame: Option<FirstFrameParam>,
    ///Last frame image for the video.
    pub last_frame: Option<LastFrameParam>,
    /**Seedance asset_id to use as the first frame. Mutually exclusive with the first_frame image input.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub first_frame_asset_id: Option<FirstFrameAssetIdParam>,
    /**Seedance asset_id to use as the last frame. Mutually exclusive with the last_frame image input.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub last_frame_asset_id: Option<LastFrameAssetIdParam>,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    FirstFrameAssetIdParam: crate::nodes::types::String,
    LastFrameAssetIdParam: crate::nodes::types::String,
> ByteDance2FirstLastFrameNode<
    SeedParam,
    WatermarkParam,
    FirstFrameParam,
    LastFrameParam,
    FirstFrameAssetIdParam,
    LastFrameAssetIdParam,
> {
    /// Create a new node.
    pub fn new(
        seed: SeedParam,
        watermark: WatermarkParam,
        first_frame: Option<FirstFrameParam>,
        last_frame: Option<LastFrameParam>,
        first_frame_asset_id: Option<FirstFrameAssetIdParam>,
        last_frame_asset_id: Option<LastFrameAssetIdParam>,
    ) -> Self {
        Self {
            seed,
            watermark,
            first_frame,
            last_frame,
            first_frame_asset_id,
            last_frame_asset_id,
        }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    FirstFrameAssetIdParam: crate::nodes::types::String,
    LastFrameAssetIdParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ByteDance2FirstLastFrameNode<
    SeedParam,
    WatermarkParam,
    FirstFrameParam,
    LastFrameParam,
    FirstFrameAssetIdParam,
    LastFrameAssetIdParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        if let Some(v) = &self.first_frame {
            output.insert("first_frame".to_string(), v.clone().into());
        }
        if let Some(v) = &self.last_frame {
            output.insert("last_frame".to_string(), v.clone().into());
        }
        if let Some(v) = &self.first_frame_asset_id {
            output.insert("first_frame_asset_id".to_string(), v.clone().into());
        }
        if let Some(v) = &self.last_frame_asset_id {
            output.insert("last_frame_asset_id".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDance2FirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "ByteDance Seedance 2.0 First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Generate video using Seedance 2.0 from a first frame image and optional last frame image.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Seedance 2.0 Reference to Video**: Generate, edit, or extend video using Seedance 2.0 with reference images, videos, and audio. Supports multimodal reference, video editing, and video extension.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDance2ReferenceNode<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add a watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDance2ReferenceNode<SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { seed, watermark }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for ByteDance2ReferenceNode<SeedParam, WatermarkParam> {
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
    const NAME: &'static str = "ByteDance2ReferenceNode";
    const DISPLAY_NAME: &'static str = "ByteDance Seedance 2.0 Reference to Video";
    const DESCRIPTION: &'static str = "Generate, edit, or extend video using Seedance 2.0 with reference images, videos, and audio. Supports multimodal reference, video editing, and video extension.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Seedance 2.0 Text to Video**: Generate video using Seedance 2.0 models based on a text prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDance2TextToVideoNode<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add a watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDance2TextToVideoNode<SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { seed, watermark }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for ByteDance2TextToVideoNode<SeedParam, WatermarkParam> {
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
    const NAME: &'static str = "ByteDance2TextToVideoNode";
    const DISPLAY_NAME: &'static str = "ByteDance Seedance 2.0 Text to Video";
    const DESCRIPTION: &'static str = "Generate video using Seedance 2.0 models based on a text prompt.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Create Video Asset**: Create a Seedance 2.0 personal video asset. Uploads the input video and registers it in the given asset group. If group_id is empty, runs a real-person H5 authentication flow to create a new group before adding the asset.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceCreateVideoAsset<
    VideoParam: crate::nodes::types::Video,
    GroupIdParam: crate::nodes::types::String,
> {
    ///Video to register as a personal asset.
    pub video: VideoParam,
    /**Reuse an existing Seedance asset group ID to skip repeated human verification for the same person. Leave empty to run real-person authentication in the browser and create a new group.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub group_id: GroupIdParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    GroupIdParam: crate::nodes::types::String,
> ByteDanceCreateVideoAsset<VideoParam, GroupIdParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, group_id: GroupIdParam) -> Self {
        Self { video, group_id }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    GroupIdParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ByteDanceCreateVideoAsset<VideoParam, GroupIdParam> {
    type Output = out::ByteDanceCreateVideoAssetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            asset_id: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            group_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("group_id".to_string(), self.group_id.clone().into());
        output
    }
    const NAME: &'static str = "ByteDanceCreateVideoAsset";
    const DISPLAY_NAME: &'static str = "ByteDance Create Video Asset";
    const DESCRIPTION: &'static str = "Create a Seedance 2.0 personal video asset. Uploads the input video and registers it in the given asset group. If group_id is empty, runs a real-person H5 authentication flow to create a new group before adding the asset.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance First-Last-Frame to Video**: Generate video using prompt and first and last frames.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceFirstLastFrameNode<
    PromptParam: crate::nodes::types::String,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CameraFixedParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**The text prompt used to generate the video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///First frame to be used for the video.
    pub first_frame: FirstFrameParam,
    ///Last frame to be used for the video.
    pub last_frame: LastFrameParam,
    /**The duration of the output video in seconds.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 12
  - Min: 3
  - Step: 1
*/
    pub duration: DurationParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Specifies whether to fix the camera. The platform appends an instruction to fix the camera to your prompt, but does not guarantee the actual effect.

**Metadata**:
  - Default: false
*/
    pub camera_fixed: Option<CameraFixedParam>,
    /**Whether to add an "AI generated" watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
    /**This parameter is ignored for any model except seedance-1-5-pro.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> ByteDanceFirstLastFrameNode<
    PromptParam,
    FirstFrameParam,
    LastFrameParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        first_frame: FirstFrameParam,
        last_frame: LastFrameParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        camera_fixed: Option<CameraFixedParam>,
        watermark: Option<WatermarkParam>,
        generate_audio: Option<GenerateAudioParam>,
    ) -> Self {
        Self {
            prompt,
            first_frame,
            last_frame,
            duration,
            seed,
            camera_fixed,
            watermark,
            generate_audio,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceFirstLastFrameNode<
    PromptParam,
    FirstFrameParam,
    LastFrameParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("last_frame".to_string(), self.last_frame.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_fixed {
            output.insert("camera_fixed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceFirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "ByteDance First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Generate video using prompt and first and last frames.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Reference Images to Video**: Generate video using prompt and reference images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceImageReferenceNode<
    PromptParam: crate::nodes::types::String,
    ImagesParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**The text prompt used to generate the video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///One to four images.
    pub images: ImagesParam,
    /**The duration of the output video in seconds.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 12
  - Min: 3
  - Step: 1
*/
    pub duration: DurationParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Whether to add an "AI generated" watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImagesParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceImageReferenceNode<
    PromptParam,
    ImagesParam,
    DurationParam,
    SeedParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        images: ImagesParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            images,
            duration,
            seed,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ImagesParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceImageReferenceNode<
    PromptParam,
    ImagesParam,
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
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceImageReferenceNode";
    const DISPLAY_NAME: &'static str = "ByteDance Reference Images to Video";
    const DESCRIPTION: &'static str = "Generate video using prompt and reference images.";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Image to Video**: Generate video using ByteDance models via api based on image and prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceImageToVideoNode<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CameraFixedParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**The text prompt used to generate the video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///First frame to be used for the video.
    pub image: ImageParam,
    /**The duration of the output video in seconds.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 12
  - Min: 3
  - Step: 1
*/
    pub duration: DurationParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Specifies whether to fix the camera. The platform appends an instruction to fix the camera to your prompt, but does not guarantee the actual effect.

**Metadata**:
  - Default: false
*/
    pub camera_fixed: Option<CameraFixedParam>,
    /**Whether to add an "AI generated" watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
    /**This parameter is ignored for any model except seedance-1-5-pro.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> ByteDanceImageToVideoNode<
    PromptParam,
    ImageParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        image: ImageParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        camera_fixed: Option<CameraFixedParam>,
        watermark: Option<WatermarkParam>,
        generate_audio: Option<GenerateAudioParam>,
    ) -> Self {
        Self {
            prompt,
            image,
            duration,
            seed,
            camera_fixed,
            watermark,
            generate_audio,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceImageToVideoNode<
    PromptParam,
    ImageParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_fixed {
            output.insert("camera_fixed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceImageToVideoNode";
    const DISPLAY_NAME: &'static str = "ByteDance Image to Video";
    const DESCRIPTION: &'static str = "Generate video using ByteDance models via api based on image and prompt";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
///**ByteDance Text to Video**: Generate video using ByteDance models via api based on prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceTextToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CameraFixedParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**The text prompt used to generate the video.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**The duration of the output video in seconds.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 12
  - Min: 3
  - Step: 1
*/
    pub duration: DurationParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Specifies whether to fix the camera. The platform appends an instruction to fix the camera to your prompt, but does not guarantee the actual effect.

**Metadata**:
  - Default: false
*/
    pub camera_fixed: Option<CameraFixedParam>,
    /**Whether to add an "AI generated" watermark to the video.

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
    /**This parameter is ignored for any model except seedance-1-5-pro.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> ByteDanceTextToVideoNode<
    PromptParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        camera_fixed: Option<CameraFixedParam>,
        watermark: Option<WatermarkParam>,
        generate_audio: Option<GenerateAudioParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            seed,
            camera_fixed,
            watermark,
            generate_audio,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceTextToVideoNode<
    PromptParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
    GenerateAudioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_fixed {
            output.insert("camera_fixed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceTextToVideoNode";
    const DISPLAY_NAME: &'static str = "ByteDance Text to Video";
    const DESCRIPTION: &'static str = "Generate video using ByteDance models via api based on prompt";
    const CATEGORY: &'static str = "partner/video/ByteDance";
}
