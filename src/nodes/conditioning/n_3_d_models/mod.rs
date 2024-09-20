//!3d_models
#![allow(unused_imports)]
use crate::WorkflowNodeId;
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
///Output for [`Sv3DConditioning`].
#[derive(Clone)]
pub struct Sv3DConditioningOutput {
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
    Elevation: crate::nodes::Float,
> crate::nodes::TypedNode
for Sv3DConditioning<ClipVision, InitImage, Vae, Width, Height, VideoFrames, Elevation> {
    type Output = Sv3DConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
            negative: crate::nodes::ConditioningOut {
                node_id,
                slot: 1u32,
            },
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "SV3D_Conditioning";
    const DISPLAY_NAME: &'static str = "SV3D_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
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
///Output for [`StableZero123Conditioning`].
#[derive(Clone)]
pub struct StableZero123ConditioningOutput {
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
    BatchSize: crate::nodes::Int,
    Elevation: crate::nodes::Float,
    Azimuth: crate::nodes::Float,
> crate::nodes::TypedNode
for StableZero123Conditioning<
    ClipVision,
    InitImage,
    Vae,
    Width,
    Height,
    BatchSize,
    Elevation,
    Azimuth,
> {
    type Output = StableZero123ConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
            negative: crate::nodes::ConditioningOut {
                node_id,
                slot: 1u32,
            },
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "StableZero123_Conditioning";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
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
///Output for [`StableZero123ConditioningBatched`].
#[derive(Clone)]
pub struct StableZero123ConditioningBatchedOutput {
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
    BatchSize: crate::nodes::Int,
    Elevation: crate::nodes::Float,
    Azimuth: crate::nodes::Float,
    ElevationBatchIncrement: crate::nodes::Float,
    AzimuthBatchIncrement: crate::nodes::Float,
> crate::nodes::TypedNode
for StableZero123ConditioningBatched<
    ClipVision,
    InitImage,
    Vae,
    Width,
    Height,
    BatchSize,
    Elevation,
    Azimuth,
    ElevationBatchIncrement,
    AzimuthBatchIncrement,
> {
    type Output = StableZero123ConditioningBatchedOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
            negative: crate::nodes::ConditioningOut {
                node_id,
                slot: 1u32,
            },
            latent: crate::nodes::LatentOut {
                node_id,
                slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
