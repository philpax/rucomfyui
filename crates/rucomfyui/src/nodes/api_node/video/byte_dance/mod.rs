//!`ByteDance` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    FirstFrameParam: crate::nodes::types::Image,
    LastFrameParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceFirstLastFrameNode<
    PromptParam,
    FirstFrameParam,
    LastFrameParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
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
    ) -> Self {
        Self {
            prompt,
            first_frame,
            last_frame,
            duration,
            seed,
            camera_fixed,
            watermark,
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
> crate::nodes::TypedNode
for ByteDanceFirstLastFrameNode<
    PromptParam,
    FirstFrameParam,
    LastFrameParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
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
        output
    }
    const NAME: &'static str = "ByteDanceFirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "ByteDance First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Generate video using prompt and first and last frames.";
    const CATEGORY: &'static str = "api node/video/ByteDance";
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
  - Default: true
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
    const CATEGORY: &'static str = "api node/video/ByteDance";
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
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceImageToVideoNode<
    PromptParam,
    ImageParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        image: ImageParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        camera_fixed: Option<CameraFixedParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            image,
            duration,
            seed,
            camera_fixed,
            watermark,
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
> crate::nodes::TypedNode
for ByteDanceImageToVideoNode<
    PromptParam,
    ImageParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
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
        output
    }
    const NAME: &'static str = "ByteDanceImageToVideoNode";
    const DISPLAY_NAME: &'static str = "ByteDance Image to Video";
    const DESCRIPTION: &'static str = "Generate video using ByteDance models via api based on image and prompt";
    const CATEGORY: &'static str = "api node/video/ByteDance";
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
  - Default: true
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceTextToVideoNode<
    PromptParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        seed: Option<SeedParam>,
        camera_fixed: Option<CameraFixedParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            seed,
            camera_fixed,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    CameraFixedParam: crate::nodes::types::Boolean,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceTextToVideoNode<
    PromptParam,
    DurationParam,
    SeedParam,
    CameraFixedParam,
    WatermarkParam,
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
        output
    }
    const NAME: &'static str = "ByteDanceTextToVideoNode";
    const DISPLAY_NAME: &'static str = "ByteDance Text to Video";
    const DESCRIPTION: &'static str = "Generate video using ByteDance models via api based on prompt";
    const CATEGORY: &'static str = "api node/video/ByteDance";
}
