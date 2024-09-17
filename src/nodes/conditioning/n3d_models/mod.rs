//!3d_models
///**SV3D_Conditioning**
pub struct Sv3DConditioning<
    ClipVision: crate::nodes::ClipVision,
    InitImage: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    VideoFrames: crate::nodes::Int,
    Elevation: crate::nodes::Float,
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
    pub elevation: Elevation,
}
///**StableZero123_Conditioning**
pub struct StableZero123Conditioning<
    ClipVision: crate::nodes::ClipVision,
    InitImage: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
    Elevation: crate::nodes::Float,
    Azimuth: crate::nodes::Float,
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
    pub batch_size: BatchSize,
    ///No documentation.
    pub elevation: Elevation,
    ///No documentation.
    pub azimuth: Azimuth,
}
///**StableZero123_Conditioning_Batched**
pub struct StableZero123ConditioningBatched<
    ClipVision: crate::nodes::ClipVision,
    InitImage: crate::nodes::Image,
    Vae: crate::nodes::Vae,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
    Elevation: crate::nodes::Float,
    Azimuth: crate::nodes::Float,
    ElevationBatchIncrement: crate::nodes::Float,
    AzimuthBatchIncrement: crate::nodes::Float,
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
    pub batch_size: BatchSize,
    ///No documentation.
    pub elevation: Elevation,
    ///No documentation.
    pub azimuth: Azimuth,
    ///No documentation.
    pub elevation_batch_increment: ElevationBatchIncrement,
    ///No documentation.
    pub azimuth_batch_increment: AzimuthBatchIncrement,
}
