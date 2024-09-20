//!`video_models` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**SVD_img2vid_Conditioning**
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
///Output for [`SvdImg2VidConditioning`].
#[derive(Clone)]
pub struct SvdImg2VidConditioningOutput {
    ///No documentation.
    pub positive: crate::nodes::types::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::types::ConditioningOut,
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
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
    type Output = SvdImg2VidConditioningOutput;
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
    const NAME: &'static str = "SVD_img2vid_Conditioning";
    const DISPLAY_NAME: &'static str = "SVD_img2vid_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
