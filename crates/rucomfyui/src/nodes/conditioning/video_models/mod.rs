//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
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
