//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub video_frames: VideoFrames,
    ///No documentation.
    pub motion_bucket_id: MotionBucketId,
    ///No documentation.
    pub fps: Fps,
    ///No documentation.
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
            positive: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
            negative: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 1u32,
            },
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.to_workflow_input());
        output.insert("init_image".to_string(), self.init_image.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("video_frames".to_string(), self.video_frames.to_workflow_input());
        output
            .insert(
                "motion_bucket_id".to_string(),
                self.motion_bucket_id.to_workflow_input(),
            );
        output.insert("fps".to_string(), self.fps.to_workflow_input());
        output
            .insert(
                "augmentation_level".to_string(),
                self.augmentation_level.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "SVD_img2vid_Conditioning";
    const DISPLAY_NAME: &'static str = "SVD_img2vid_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
