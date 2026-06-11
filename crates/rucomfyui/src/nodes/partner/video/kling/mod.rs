//!`Kling` definitions/categories.
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
    ///Output for [`KlingCameraControlI2VNode`](super::KlingCameraControlI2VNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingCameraControlI2VNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingCameraControlT2VNode`](super::KlingCameraControlT2VNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingCameraControlT2VNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingDualCharacterVideoEffectNode`](super::KlingDualCharacterVideoEffectNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingDualCharacterVideoEffectNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingImage2VideoNode`](super::KlingImage2VideoNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingImage2VideoNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingLipSyncAudioToVideoNode`](super::KlingLipSyncAudioToVideoNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingLipSyncAudioToVideoNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingLipSyncTextToVideoNode`](super::KlingLipSyncTextToVideoNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingLipSyncTextToVideoNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingSingleImageVideoEffectNode`](super::KlingSingleImageVideoEffectNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingSingleImageVideoEffectNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingStartEndFrameNode`](super::KlingStartEndFrameNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingStartEndFrameNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingTextToVideoNode`](super::KlingTextToVideoNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingTextToVideoNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
    ///Output for [`KlingVideoExtendNode`](super::KlingVideoExtendNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct KlingVideoExtendNodeOutput {
        ///No documentation.
        pub video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub video_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub duration: crate::nodes::types::StringOut,
    }
}
///**Kling Avatar 2.0**: Generate broadcast-style digital human videos from a single photo and an audio file.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingAvatarNode<
    ImageParam: crate::nodes::types::Image,
    SoundFileParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    PromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///Avatar reference image. Width and height must be at least 300px. Aspect ratio must be between 1:2.5 and 2.5:1.
    pub image: ImageParam,
    ///Audio input. Must be between 2 and 300 seconds in duration.
    pub sound_file: SoundFileParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**Optional prompt to define avatar actions, emotions, and camera movements.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: Option<PromptParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SoundFileParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    PromptParam: crate::nodes::types::String,
> KlingAvatarNode<ImageParam, SoundFileParam, SeedParam, PromptParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        sound_file: SoundFileParam,
        seed: SeedParam,
        prompt: Option<PromptParam>,
    ) -> Self {
        Self {
            image,
            sound_file,
            seed,
            prompt,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SoundFileParam: crate::nodes::types::Audio,
    SeedParam: crate::nodes::types::Int,
    PromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for KlingAvatarNode<ImageParam, SoundFileParam, SeedParam, PromptParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("sound_file".to_string(), self.sound_file.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.prompt {
            output.insert("prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingAvatarNode";
    const DISPLAY_NAME: &'static str = "Kling Avatar 2.0";
    const DESCRIPTION: &'static str = "Generate broadcast-style digital human videos from a single photo and an audio file.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Image to Video (Camera Control)**: Transform still images into cinematic videos with professional camera movements that simulate real-world cinematography. Control virtual camera actions including zoom, rotation, pan, tilt, and first-person view, while maintaining focus on your original image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingCameraControlI2VNode<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> {
    ///Reference Image - URL or Base64 encoded string, cannot exceed 10MB, resolution not less than 300*300px, aspect ratio between 1:2.5 ~ 2.5:1. Base64 should not include data:image prefix.
    pub start_frame: StartFrameParam,
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0.75
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
    ///Can be created using the Kling Camera Controls node. Controls the camera movement and motion during the video generation.
    pub camera_control: CameraControlParam,
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> KlingCameraControlI2VNode<
    StartFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
    CameraControlParam,
> {
    /// Create a new node.
    pub fn new(
        start_frame: StartFrameParam,
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
        camera_control: CameraControlParam,
    ) -> Self {
        Self {
            start_frame,
            prompt,
            negative_prompt,
            cfg_scale,
            camera_control,
        }
    }
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> crate::nodes::TypedNode
for KlingCameraControlI2VNode<
    StartFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
    CameraControlParam,
> {
    type Output = out::KlingCameraControlI2VNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output.insert("camera_control".to_string(), self.camera_control.clone().into());
        output
    }
    const NAME: &'static str = "KlingCameraControlI2VNode";
    const DISPLAY_NAME: &'static str = "Kling Image to Video (Camera Control)";
    const DESCRIPTION: &'static str = "Transform still images into cinematic videos with professional camera movements that simulate real-world cinematography. Control virtual camera actions including zoom, rotation, pan, tilt, and first-person view, while maintaining focus on your original image.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Text to Video (Camera Control)**: Transform text into cinematic videos with professional camera movements that simulate real-world cinematography. Control virtual camera actions including zoom, rotation, pan, tilt, and first-person view, while maintaining focus on your original text.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingCameraControlT2VNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> {
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0.75
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
    ///Can be created using the Kling Camera Controls node. Controls the camera movement and motion during the video generation.
    pub camera_control: CameraControlParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> KlingCameraControlT2VNode<
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
    CameraControlParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
        camera_control: CameraControlParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            cfg_scale,
            camera_control,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    CameraControlParam: crate::nodes::types::CameraControl,
> crate::nodes::TypedNode
for KlingCameraControlT2VNode<
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
    CameraControlParam,
> {
    type Output = out::KlingCameraControlT2VNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output.insert("camera_control".to_string(), self.camera_control.clone().into());
        output
    }
    const NAME: &'static str = "KlingCameraControlT2VNode";
    const DISPLAY_NAME: &'static str = "Kling Text to Video (Camera Control)";
    const DESCRIPTION: &'static str = "Transform text into cinematic videos with professional camera movements that simulate real-world cinematography. Control virtual camera actions including zoom, rotation, pan, tilt, and first-person view, while maintaining focus on your original text.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Camera Controls**: Allows specifying configuration options for Kling Camera Controls and motion control effects.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingCameraControls<
    HorizontalMovementParam: crate::nodes::types::Float,
    VerticalMovementParam: crate::nodes::types::Float,
    PanParam: crate::nodes::types::Float,
    TiltParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> {
    /**Controls camera's movement along horizontal axis (x-axis). Negative indicates left, positive indicates right

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub horizontal_movement: HorizontalMovementParam,
    /**Controls camera's movement along vertical axis (y-axis). Negative indicates downward, positive indicates upward.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub vertical_movement: VerticalMovementParam,
    /**Controls camera's rotation in vertical plane (x-axis). Negative indicates downward rotation, positive indicates upward rotation.

**Metadata**:
  - Default: 0.5
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub pan: PanParam,
    /**Controls camera's rotation in horizontal plane (y-axis). Negative indicates left rotation, positive indicates right rotation.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub tilt: TiltParam,
    /**Controls camera's rolling amount (z-axis). Negative indicates counterclockwise, positive indicates clockwise.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub roll: RollParam,
    /**Controls change in camera's focal length. Negative indicates narrower field of view, positive indicates wider field of view.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
  - Step: 0.25
*/
    pub zoom: ZoomParam,
}
impl<
    HorizontalMovementParam: crate::nodes::types::Float,
    VerticalMovementParam: crate::nodes::types::Float,
    PanParam: crate::nodes::types::Float,
    TiltParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> KlingCameraControls<
    HorizontalMovementParam,
    VerticalMovementParam,
    PanParam,
    TiltParam,
    RollParam,
    ZoomParam,
> {
    /// Create a new node.
    pub fn new(
        horizontal_movement: HorizontalMovementParam,
        vertical_movement: VerticalMovementParam,
        pan: PanParam,
        tilt: TiltParam,
        roll: RollParam,
        zoom: ZoomParam,
    ) -> Self {
        Self {
            horizontal_movement,
            vertical_movement,
            pan,
            tilt,
            roll,
            zoom,
        }
    }
}
impl<
    HorizontalMovementParam: crate::nodes::types::Float,
    VerticalMovementParam: crate::nodes::types::Float,
    PanParam: crate::nodes::types::Float,
    TiltParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KlingCameraControls<
    HorizontalMovementParam,
    VerticalMovementParam,
    PanParam,
    TiltParam,
    RollParam,
    ZoomParam,
> {
    type Output = crate::nodes::types::CameraControlOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "horizontal_movement".to_string(),
                self.horizontal_movement.clone().into(),
            );
        output
            .insert(
                "vertical_movement".to_string(),
                self.vertical_movement.clone().into(),
            );
        output.insert("pan".to_string(), self.pan.clone().into());
        output.insert("tilt".to_string(), self.tilt.clone().into());
        output.insert("roll".to_string(), self.roll.clone().into());
        output.insert("zoom".to_string(), self.zoom.clone().into());
        output
    }
    const NAME: &'static str = "KlingCameraControls";
    const DISPLAY_NAME: &'static str = "Kling Camera Controls";
    const DESCRIPTION: &'static str = "Allows specifying configuration options for Kling Camera Controls and motion control effects.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Dual Character Video Effects**: Achieve different special effects when generating a video based on the effect_scene. First image will be positioned on left side, second on right side of the composite.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingDualCharacterVideoEffectNode<
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
> {
    ///Left side image
    pub image_left: ImageLeftParam,
    ///Right side image
    pub image_right: ImageRightParam,
}
impl<
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
> KlingDualCharacterVideoEffectNode<ImageLeftParam, ImageRightParam> {
    /// Create a new node.
    pub fn new(image_left: ImageLeftParam, image_right: ImageRightParam) -> Self {
        Self { image_left, image_right }
    }
}
impl<
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for KlingDualCharacterVideoEffectNode<ImageLeftParam, ImageRightParam> {
    type Output = out::KlingDualCharacterVideoEffectNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_left".to_string(), self.image_left.clone().into());
        output.insert("image_right".to_string(), self.image_right.clone().into());
        output
    }
    const NAME: &'static str = "KlingDualCharacterVideoEffectNode";
    const DISPLAY_NAME: &'static str = "Kling Dual Character Video Effects";
    const DESCRIPTION: &'static str = "Achieve different special effects when generating a video based on the effect_scene. First image will be positioned on left side, second on right side of the composite.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 First-Last-Frame to Video**: Generate videos with Kling V3 using first and last frames.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingFirstLastFrameNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 15
  - Min: 3
*/
    pub duration: DurationParam,
    ///No documentation.
    pub first_frame: FirstFrameParam,
    ///No documentation.
    pub end_frame: EndFrameParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub generate_audio: GenerateAudioParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> KlingFirstLastFrameNode<
    PromptParam,
    DurationParam,
    FirstFrameParam,
    EndFrameParam,
    GenerateAudioParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        first_frame: FirstFrameParam,
        end_frame: EndFrameParam,
        generate_audio: GenerateAudioParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            prompt,
            duration,
            first_frame,
            end_frame,
            generate_audio,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingFirstLastFrameNode<
    PromptParam,
    DurationParam,
    FirstFrameParam,
    EndFrameParam,
    GenerateAudioParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        output.insert("end_frame".to_string(), self.end_frame.clone().into());
        output.insert("generate_audio".to_string(), self.generate_audio.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "KlingFirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Generate videos with Kling V3 using first and last frames.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Image(First Frame) to Video**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingImage2VideoNode<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> {
    ///The reference image used to generate the video.
    pub start_frame: StartFrameParam,
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0.8
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> KlingImage2VideoNode<
    StartFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
> {
    /// Create a new node.
    pub fn new(
        start_frame: StartFrameParam,
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
    ) -> Self {
        Self {
            start_frame,
            prompt,
            negative_prompt,
            cfg_scale,
        }
    }
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KlingImage2VideoNode<
    StartFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
> {
    type Output = out::KlingImage2VideoNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output
    }
    const NAME: &'static str = "KlingImage2VideoNode";
    const DISPLAY_NAME: &'static str = "Kling Image(First Frame) to Video";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 2.6 Image(First Frame) to Video with Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingImageToVideoWithAudio<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub start_frame: StartFrameParam,
    /**Positive text prompt.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub generate_audio: GenerateAudioParam,
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> KlingImageToVideoWithAudio<StartFrameParam, PromptParam, GenerateAudioParam> {
    /// Create a new node.
    pub fn new(
        start_frame: StartFrameParam,
        prompt: PromptParam,
        generate_audio: GenerateAudioParam,
    ) -> Self {
        Self {
            start_frame,
            prompt,
            generate_audio,
        }
    }
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for KlingImageToVideoWithAudio<StartFrameParam, PromptParam, GenerateAudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("generate_audio".to_string(), self.generate_audio.clone().into());
        output
    }
    const NAME: &'static str = "KlingImageToVideoWithAudio";
    const DISPLAY_NAME: &'static str = "Kling 2.6 Image(First Frame) to Video with Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Lip Sync Video with Audio**: Kling Lip Sync Audio to Video Node. Syncs mouth movements in a video file to the audio content of an audio file. When using, ensure that the audio contains clearly distinguishable vocals and that the video contains a distinct face. The audio file should not be larger than 5MB. The video file should not be larger than 100MB, should have height/width between 720px and 1920px, and should be between 2s and 10s in length.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingLipSyncAudioToVideoNode<
    VideoParam: crate::nodes::types::Video,
    AudioParam: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub video: VideoParam,
    ///No documentation.
    pub audio: AudioParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    AudioParam: crate::nodes::types::Audio,
> KlingLipSyncAudioToVideoNode<VideoParam, AudioParam> {
    /// Create a new node.
    pub fn new(video: VideoParam, audio: AudioParam) -> Self {
        Self { video, audio }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode for KlingLipSyncAudioToVideoNode<VideoParam, AudioParam> {
    type Output = out::KlingLipSyncAudioToVideoNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "KlingLipSyncAudioToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling Lip Sync Video with Audio";
    const DESCRIPTION: &'static str = "Kling Lip Sync Audio to Video Node. Syncs mouth movements in a video file to the audio content of an audio file. When using, ensure that the audio contains clearly distinguishable vocals and that the video contains a distinct face. The audio file should not be larger than 5MB. The video file should not be larger than 100MB, should have height/width between 720px and 1920px, and should be between 2s and 10s in length.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Lip Sync Video with Text**: Kling Lip Sync Text to Video Node. Syncs mouth movements in a video file to a text prompt. The video file should not be larger than 100MB, should have height/width between 720px and 1920px, and should be between 2s and 10s in length.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingLipSyncTextToVideoNode<
    VideoParam: crate::nodes::types::Video,
    TextParam: crate::nodes::types::String,
    VoiceSpeedParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub video: VideoParam,
    /**Text Content for Lip-Sync Video Generation. Required when mode is text2video. Maximum length is 120 characters.

**Metadata**:
  - Multiline: true
*/
    pub text: TextParam,
    /**Speech Rate. Valid range: 0.8~2.0, accurate to one decimal place.

**Metadata**:
  - Default: 1
  - Display: slider
  - Max: 2
  - Min: 0.8
*/
    pub voice_speed: VoiceSpeedParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    TextParam: crate::nodes::types::String,
    VoiceSpeedParam: crate::nodes::types::Float,
> KlingLipSyncTextToVideoNode<VideoParam, TextParam, VoiceSpeedParam> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        text: TextParam,
        voice_speed: VoiceSpeedParam,
    ) -> Self {
        Self { video, text, voice_speed }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    TextParam: crate::nodes::types::String,
    VoiceSpeedParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KlingLipSyncTextToVideoNode<VideoParam, TextParam, VoiceSpeedParam> {
    type Output = out::KlingLipSyncTextToVideoNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("voice_speed".to_string(), self.voice_speed.clone().into());
        output
    }
    const NAME: &'static str = "KlingLipSyncTextToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling Lip Sync Video with Text";
    const DESCRIPTION: &'static str = "Kling Lip Sync Text to Video Node. Syncs mouth movements in a video file to a text prompt. The video file should not be larger than 100MB, should have height/width between 720px and 1920px, and should be between 2s and 10s in length.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Motion Control**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingMotionControl<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///No documentation.
    pub reference_image: ReferenceImageParam,
    /**Motion reference video used to drive movement/expression.

Duration limits depend on character_orientation:

 - image: 3–10s (max 10s)

 - video: 3–30s (max 30s)*/
    pub reference_video: ReferenceVideoParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub keep_original_sound: KeepOriginalSoundParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
> KlingMotionControl<
    PromptParam,
    ReferenceImageParam,
    ReferenceVideoParam,
    KeepOriginalSoundParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        reference_image: ReferenceImageParam,
        reference_video: ReferenceVideoParam,
        keep_original_sound: KeepOriginalSoundParam,
    ) -> Self {
        Self {
            prompt,
            reference_image,
            reference_video,
            keep_original_sound,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ReferenceImageParam: crate::nodes::types::Image,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for KlingMotionControl<
    PromptParam,
    ReferenceImageParam,
    ReferenceVideoParam,
    KeepOriginalSoundParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("reference_image".to_string(), self.reference_image.clone().into());
        output
            .insert("reference_video".to_string(), self.reference_video.clone().into());
        output
            .insert(
                "keep_original_sound".to_string(),
                self.keep_original_sound.clone().into(),
            );
        output
    }
    const NAME: &'static str = "KlingMotionControl";
    const DISPLAY_NAME: &'static str = "Kling Motion Control";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Omni Edit Video**: Edit an existing video with the latest model from Kling.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProEditVideoNode<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the video content. This can include both positive and negative descriptions.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    ///Video for editing. The output video length will be the same.
    pub video: VideoParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub keep_original_sound: KeepOriginalSoundParam,
    ///Up to 4 additional reference images.
    pub reference_images: Option<ReferenceImagesParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProEditVideoNode<
    PromptParam,
    VideoParam,
    KeepOriginalSoundParam,
    ReferenceImagesParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        video: VideoParam,
        keep_original_sound: KeepOriginalSoundParam,
        reference_images: Option<ReferenceImagesParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            video,
            keep_original_sound,
            reference_images,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    VideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProEditVideoNode<
    PromptParam,
    VideoParam,
    KeepOriginalSoundParam,
    ReferenceImagesParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("video".to_string(), self.video.clone().into());
        output
            .insert(
                "keep_original_sound".to_string(),
                self.keep_original_sound.clone().into(),
            );
        if let Some(v) = &self.reference_images {
            output.insert("reference_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProEditVideoNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni Edit Video";
    const DESCRIPTION: &'static str = "Edit an existing video with the latest model from Kling.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Omni First-Last-Frame to Video**: Use a start frame, an optional end frame, or reference images with the latest Kling model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProFirstLastFrameNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ReferenceImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the video content. This can include both positive and negative descriptions. Ignored when storyboards are enabled.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 15
  - Min: 3
*/
    pub duration: DurationParam,
    ///No documentation.
    pub first_frame: FirstFrameParam,
    ///An optional end frame for the video. This cannot be used simultaneously with 'reference_images'. Does not work with storyboards.
    pub end_frame: Option<EndFrameParam>,
    ///Up to 6 additional reference images.
    pub reference_images: Option<ReferenceImagesParam>,
    /**Generate audio for the video. Only supported for kling-v3-omni.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    ReferenceImagesParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProFirstLastFrameNode<
    PromptParam,
    DurationParam,
    FirstFrameParam,
    EndFrameParam,
    ReferenceImagesParam,
    GenerateAudioParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        first_frame: FirstFrameParam,
        end_frame: Option<EndFrameParam>,
        reference_images: Option<ReferenceImagesParam>,
        generate_audio: Option<GenerateAudioParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            first_frame,
            end_frame,
            reference_images,
            generate_audio,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    FirstFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    ReferenceImagesParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProFirstLastFrameNode<
    PromptParam,
    DurationParam,
    FirstFrameParam,
    EndFrameParam,
    ReferenceImagesParam,
    GenerateAudioParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("first_frame".to_string(), self.first_frame.clone().into());
        if let Some(v) = &self.end_frame {
            output.insert("end_frame".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_images {
            output.insert("reference_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProFirstLastFrameNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni First-Last-Frame to Video";
    const DESCRIPTION: &'static str = "Use a start frame, an optional end frame, or reference images with the latest Kling model.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Omni Image to Video**: Use up to 7 reference images to generate a video with the latest Kling model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProImageToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceImagesParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the video content. This can include both positive and negative descriptions. Ignored when storyboards are enabled.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 15
  - Min: 3
*/
    pub duration: DurationParam,
    ///Up to 7 reference images.
    pub reference_images: ReferenceImagesParam,
    /**Generate audio for the video. Only supported for kling-v3-omni.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceImagesParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProImageToVideoNode<
    PromptParam,
    DurationParam,
    ReferenceImagesParam,
    GenerateAudioParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        reference_images: ReferenceImagesParam,
        generate_audio: Option<GenerateAudioParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            reference_images,
            generate_audio,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceImagesParam: crate::nodes::types::Image,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProImageToVideoNode<
    PromptParam,
    DurationParam,
    ReferenceImagesParam,
    GenerateAudioParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output
            .insert(
                "reference_images".to_string(),
                self.reference_images.clone().into(),
            );
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProImageToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni Image to Video";
    const DESCRIPTION: &'static str = "Use up to 7 reference images to generate a video with the latest Kling model.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Omni Text to Video**: Use text prompts to generate videos with the latest Kling model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProTextToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the video content. This can include both positive and negative descriptions. Ignored when storyboards are enabled.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Display: slider
  - Max: 15
  - Min: 3
*/
    pub duration: DurationParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub generate_audio: Option<GenerateAudioParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProTextToVideoNode<
    PromptParam,
    DurationParam,
    GenerateAudioParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        generate_audio: Option<GenerateAudioParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            generate_audio,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProTextToVideoNode<
    PromptParam,
    DurationParam,
    GenerateAudioParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        if let Some(v) = &self.generate_audio {
            output.insert("generate_audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProTextToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni Text to Video";
    const DESCRIPTION: &'static str = "Use text prompts to generate videos with the latest Kling model.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Omni Video to Video**: Use a video and up to 4 reference images to generate a video with the latest Kling model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingOmniProVideoToVideoNode<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**A text prompt describing the video content. This can include both positive and negative descriptions.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Display: slider
  - Max: 10
  - Min: 3
*/
    pub duration: DurationParam,
    ///Video to use as a reference.
    pub reference_video: ReferenceVideoParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub keep_original_sound: KeepOriginalSoundParam,
    ///Up to 4 additional reference images.
    pub reference_images: Option<ReferenceImagesParam>,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> KlingOmniProVideoToVideoNode<
    PromptParam,
    DurationParam,
    ReferenceVideoParam,
    KeepOriginalSoundParam,
    ReferenceImagesParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: DurationParam,
        reference_video: ReferenceVideoParam,
        keep_original_sound: KeepOriginalSoundParam,
        reference_images: Option<ReferenceImagesParam>,
        seed: Option<SeedParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            reference_video,
            keep_original_sound,
            reference_images,
            seed,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    ReferenceVideoParam: crate::nodes::types::Video,
    KeepOriginalSoundParam: crate::nodes::types::Boolean,
    ReferenceImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for KlingOmniProVideoToVideoNode<
    PromptParam,
    DurationParam,
    ReferenceVideoParam,
    KeepOriginalSoundParam,
    ReferenceImagesParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output
            .insert("reference_video".to_string(), self.reference_video.clone().into());
        output
            .insert(
                "keep_original_sound".to_string(),
                self.keep_original_sound.clone().into(),
            );
        if let Some(v) = &self.reference_images {
            output.insert("reference_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingOmniProVideoToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Omni Video to Video";
    const DESCRIPTION: &'static str = "Use a video and up to 4 reference images to generate a video with the latest Kling model.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Video Effects**: Achieve different special effects when generating a video based on the effect_scene.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingSingleImageVideoEffectNode<ImageParam: crate::nodes::types::Image> {
    /// Reference Image. URL or Base64 encoded string (without data:image prefix). File size cannot exceed 10MB, resolution not less than 300*300px, aspect ratio between 1:2.5 ~ 2.5:1
    pub image: ImageParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
> KlingSingleImageVideoEffectNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for KlingSingleImageVideoEffectNode<ImageParam> {
    type Output = out::KlingSingleImageVideoEffectNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "KlingSingleImageVideoEffectNode";
    const DISPLAY_NAME: &'static str = "Kling Video Effects";
    const DESCRIPTION: &'static str = "Achieve different special effects when generating a video based on the effect_scene.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Start-End Frame to Video**: Generate a video sequence that transitions between your provided start and end images. The node creates all frames in between, producing a smooth transformation from the first frame to the last.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingStartEndFrameNode<
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> {
    ///Reference Image - URL or Base64 encoded string, cannot exceed 10MB, resolution not less than 300*300px, aspect ratio between 1:2.5 ~ 2.5:1. Base64 should not include data:image prefix.
    pub start_frame: StartFrameParam,
    ///Reference Image - End frame control. URL or Base64 encoded string, cannot exceed 10MB, resolution not less than 300*300px. Base64 should not include data:image prefix.
    pub end_frame: EndFrameParam,
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> KlingStartEndFrameNode<
    StartFrameParam,
    EndFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
> {
    /// Create a new node.
    pub fn new(
        start_frame: StartFrameParam,
        end_frame: EndFrameParam,
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
    ) -> Self {
        Self {
            start_frame,
            end_frame,
            prompt,
            negative_prompt,
            cfg_scale,
        }
    }
}
impl<
    StartFrameParam: crate::nodes::types::Image,
    EndFrameParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KlingStartEndFrameNode<
    StartFrameParam,
    EndFrameParam,
    PromptParam,
    NegativePromptParam,
    CfgScaleParam,
> {
    type Output = out::KlingStartEndFrameNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("start_frame".to_string(), self.start_frame.clone().into());
        output.insert("end_frame".to_string(), self.end_frame.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output
    }
    const NAME: &'static str = "KlingStartEndFrameNode";
    const DISPLAY_NAME: &'static str = "Kling Start-End Frame to Video";
    const DESCRIPTION: &'static str = "Generate a video sequence that transitions between your provided start and end images. The node creates all frames in between, producing a smooth transformation from the first frame to the last.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Text to Video**: Kling Text to Video Node
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingTextToVideoNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> {
    /**Positive text prompt

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> KlingTextToVideoNode<PromptParam, NegativePromptParam, CfgScaleParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            cfg_scale,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KlingTextToVideoNode<PromptParam, NegativePromptParam, CfgScaleParam> {
    type Output = out::KlingTextToVideoNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output
    }
    const NAME: &'static str = "KlingTextToVideoNode";
    const DISPLAY_NAME: &'static str = "Kling Text to Video";
    const DESCRIPTION: &'static str = "Kling Text to Video Node";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 2.6 Text to Video with Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingTextToVideoWithAudio<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> {
    /**Positive text prompt.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub generate_audio: GenerateAudioParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> KlingTextToVideoWithAudio<PromptParam, GenerateAudioParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, generate_audio: GenerateAudioParam) -> Self {
        Self { prompt, generate_audio }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    GenerateAudioParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for KlingTextToVideoWithAudio<PromptParam, GenerateAudioParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("generate_audio".to_string(), self.generate_audio.clone().into());
        output
    }
    const NAME: &'static str = "KlingTextToVideoWithAudio";
    const DISPLAY_NAME: &'static str = "Kling 2.6 Text to Video with Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling Video Extend**: Kling Video Extend Node. Extend videos made by other Kling nodes. The video_id is created by using other Kling Nodes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingVideoExtendNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    VideoIdParam: crate::nodes::types::String,
> {
    /**Positive text prompt for guiding the video extension

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative text prompt for elements to avoid in the extended video

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
*/
    pub cfg_scale: CfgScaleParam,
    /**The ID of the video to be extended. Supports videos generated by text-to-video, image-to-video, and previous video extension operations. Cannot exceed 3 minutes total duration after extension.

**Metadata**:
  - Multiline: false
*/
    pub video_id: VideoIdParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    VideoIdParam: crate::nodes::types::String,
> KlingVideoExtendNode<PromptParam, NegativePromptParam, CfgScaleParam, VideoIdParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        cfg_scale: CfgScaleParam,
        video_id: VideoIdParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            cfg_scale,
            video_id,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    VideoIdParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for KlingVideoExtendNode<PromptParam, NegativePromptParam, CfgScaleParam, VideoIdParam> {
    type Output = out::KlingVideoExtendNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video: crate::nodes::types::VideoOut::from_dynamic(node_id, 0u32),
            video_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            duration: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output.insert("video_id".to_string(), self.video_id.clone().into());
        output
    }
    const NAME: &'static str = "KlingVideoExtendNode";
    const DISPLAY_NAME: &'static str = "Kling Video Extend";
    const DESCRIPTION: &'static str = "Kling Video Extend Node. Extend videos made by other Kling nodes. The video_id is created by using other Kling Nodes.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
///**Kling 3.0 Video**: Generate videos with Kling V3. Supports text-to-video and image-to-video with optional storyboard multi-prompt and audio generation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KlingVideoNode<
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    StartFrameParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub generate_audio: GenerateAudioParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    ///Optional start frame image. When connected, switches to image-to-video mode.
    pub start_frame: Option<StartFrameParam>,
}
impl<
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    StartFrameParam: crate::nodes::types::Image,
> KlingVideoNode<GenerateAudioParam, SeedParam, StartFrameParam> {
    /// Create a new node.
    pub fn new(
        generate_audio: GenerateAudioParam,
        seed: SeedParam,
        start_frame: Option<StartFrameParam>,
    ) -> Self {
        Self {
            generate_audio,
            seed,
            start_frame,
        }
    }
}
impl<
    GenerateAudioParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    StartFrameParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for KlingVideoNode<GenerateAudioParam, SeedParam, StartFrameParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("generate_audio".to_string(), self.generate_audio.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.start_frame {
            output.insert("start_frame".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "KlingVideoNode";
    const DISPLAY_NAME: &'static str = "Kling 3.0 Video";
    const DESCRIPTION: &'static str = "Generate videos with Kling V3. Supports text-to-video and image-to-video with optional storyboard multi-prompt and audio generation.";
    const CATEGORY: &'static str = "partner/video/Kling";
}
