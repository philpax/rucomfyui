//!video_models
///**SVD_img2vid_Conditioning**
pub struct SvdImg2VidConditioning<
    ClipVision: crate::nodes::ClipVision,
    InitImage: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    VideoFrames: crate::nodes::Int,
    MotionBucketId: crate::nodes::Int,
    Fps: crate::nodes::Int,
    AugmentationLevel: crate::nodes::Float,
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
pub struct SvdImg2VidConditioningOutput {
    ///No documentation.
    pub positive: crate::nodes::ConditioningOut,
    ///No documentation.
    pub negative: crate::nodes::ConditioningOut,
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    ClipVision: crate::nodes::ClipVision,
    InitImage: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    VideoFrames: crate::nodes::Int,
    MotionBucketId: crate::nodes::Int,
    Fps: crate::nodes::Int,
    AugmentationLevel: crate::nodes::Float,
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
    fn output(&self) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut(0u32),
            negative: crate::nodes::ConditioningOut(1u32),
            latent: crate::nodes::LatentOut(2u32),
        }
    }
    const NAME: &'static str = "SVD_img2vid_Conditioning";
    const DISPLAY_NAME: &'static str = "SVD_img2vid_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
