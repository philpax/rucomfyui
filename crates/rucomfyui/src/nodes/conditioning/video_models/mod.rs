//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`LtxvConditioning`](super::LtxvConditioning).
    #[derive(Clone)]
    pub struct LtxvConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`LtxvImgToVideo`](super::LtxvImgToVideo).
    #[derive(Clone)]
    pub struct LtxvImgToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`SvdImg2VidConditioning`](super::SvdImg2VidConditioning).
    #[derive(Clone)]
    pub struct SvdImg2VidConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**LTXVConditioning**: No description.
#[derive(Clone)]
pub struct LtxvConditioning<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub frame_rate: FrameRate,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> LtxvConditioning<Positive, Negative, FrameRate> {
    /// Create a new node.
    pub fn new(positive: Positive, negative: Negative, frame_rate: FrameRate) -> Self {
        Self {
            positive,
            negative,
            frame_rate,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> crate::nodes::TypedNode for LtxvConditioning<Positive, Negative, FrameRate> {
    type Output = out::LtxvConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("frame_rate".to_string(), self.frame_rate.clone().into());
        output
    }
    const NAME: &'static str = "LTXVConditioning";
    const DISPLAY_NAME: &'static str = "LTXVConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**LTXVImgToVideo**: No description.
#[derive(Clone)]
pub struct LtxvImgToVideo<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ImageNoiseScale: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 768
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 97
  - Max: 16384
  - Min: 9
  - Step: 8
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    /**Amount of noise to apply on conditioning image latent.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub image_noise_scale: ImageNoiseScale,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ImageNoiseScale: crate::nodes::types::Float,
> LtxvImgToVideo<
    Positive,
    Negative,
    Vae,
    Image,
    Width,
    Height,
    Length,
    BatchSize,
    ImageNoiseScale,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        image: Image,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        image_noise_scale: ImageNoiseScale,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            image,
            width,
            height,
            length,
            batch_size,
            image_noise_scale,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ImageNoiseScale: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LtxvImgToVideo<
    Positive,
    Negative,
    Vae,
    Image,
    Width,
    Height,
    Length,
    BatchSize,
    ImageNoiseScale,
> {
    type Output = out::LtxvImgToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
            .insert(
                "image_noise_scale".to_string(),
                self.image_noise_scale.clone().into(),
            );
        output
    }
    const NAME: &'static str = "LTXVImgToVideo";
    const DISPLAY_NAME: &'static str = "LTXVImgToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**SVD_img2vid_Conditioning**: No description.
#[derive(Clone)]
pub struct SvdImg2VidConditioning<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    VideoFrames: crate::nodes::types::Int,
    MotionBucketId: crate::nodes::types::Int,
    Fps: crate::nodes::types::Int,
    AugmentationLevel: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_vision: ClipVision,
    ///No documentation.
    pub init_image: InitImage,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 576
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 14
  - Max: 4096
  - Min: 1
*/
    pub video_frames: VideoFrames,
    /**No documentation.

**Metadata**:
  - Default: 127
  - Max: 1023
  - Min: 1
*/
    pub motion_bucket_id: MotionBucketId,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1024
  - Min: 1
*/
    pub fps: Fps,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub augmentation_level: AugmentationLevel,
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    VideoFrames: crate::nodes::types::Int,
    MotionBucketId: crate::nodes::types::Int,
    Fps: crate::nodes::types::Int,
    AugmentationLevel: crate::nodes::types::Float,
> SvdImg2VidConditioning<
    ClipVision,
    InitImage,
    Vae,
    Width,
    Height,
    VideoFrames,
    MotionBucketId,
    Fps,
    AugmentationLevel,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVision,
        init_image: InitImage,
        vae: Vae,
        width: Width,
        height: Height,
        video_frames: VideoFrames,
        motion_bucket_id: MotionBucketId,
        fps: Fps,
        augmentation_level: AugmentationLevel,
    ) -> Self {
        Self {
            clip_vision,
            init_image,
            vae,
            width,
            height,
            video_frames,
            motion_bucket_id,
            fps,
            augmentation_level,
        }
    }
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    VideoFrames: crate::nodes::types::Int,
    MotionBucketId: crate::nodes::types::Int,
    Fps: crate::nodes::types::Int,
    AugmentationLevel: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SvdImg2VidConditioning<
    ClipVision,
    InitImage,
    Vae,
    Width,
    Height,
    VideoFrames,
    MotionBucketId,
    Fps,
    AugmentationLevel,
> {
    type Output = out::SvdImg2VidConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("init_image".to_string(), self.init_image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("video_frames".to_string(), self.video_frames.clone().into());
        output
            .insert(
                "motion_bucket_id".to_string(),
                self.motion_bucket_id.clone().into(),
            );
        output.insert("fps".to_string(), self.fps.clone().into());
        output
            .insert(
                "augmentation_level".to_string(),
                self.augmentation_level.clone().into(),
            );
        output
    }
    const NAME: &'static str = "SVD_img2vid_Conditioning";
    const DISPLAY_NAME: &'static str = "SVD_img2vid_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
