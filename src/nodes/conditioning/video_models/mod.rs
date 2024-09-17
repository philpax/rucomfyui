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
