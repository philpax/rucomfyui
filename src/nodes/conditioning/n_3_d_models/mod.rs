//!`3d_models` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`Sv3DConditioning`](super::Sv3DConditioning).
    #[derive(Clone)]
    pub struct Sv3DConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableZero123Conditioning`](super::StableZero123Conditioning).
    #[derive(Clone)]
    pub struct StableZero123ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableZero123ConditioningBatched`](super::StableZero123ConditioningBatched).
    #[derive(Clone)]
    pub struct StableZero123ConditioningBatchedOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**SV3D_Conditioning**
pub struct Sv3DConditioning<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    VideoFrames: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
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
impl<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    VideoFrames: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
> crate::nodes::TypedNode
for Sv3DConditioning<ClipVision, InitImage, Vae, Width, Height, VideoFrames, Elevation> {
    type Output = out::Sv3DConditioningOutput;
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
    const NAME: &'static str = "SV3D_Conditioning";
    const DISPLAY_NAME: &'static str = "SV3D_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
///**StableZero123_Conditioning**
pub struct StableZero123Conditioning<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
    Azimuth: crate::nodes::types::Float,
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
impl<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
    Azimuth: crate::nodes::types::Float,
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
    type Output = out::StableZero123ConditioningOutput;
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
    const NAME: &'static str = "StableZero123_Conditioning";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
///**StableZero123_Conditioning_Batched**
pub struct StableZero123ConditioningBatched<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
    Azimuth: crate::nodes::types::Float,
    ElevationBatchIncrement: crate::nodes::types::Float,
    AzimuthBatchIncrement: crate::nodes::types::Float,
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
impl<
    ClipVision: crate::nodes::types::ClipVision,
    InitImage: crate::nodes::types::Image,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    Elevation: crate::nodes::types::Float,
    Azimuth: crate::nodes::types::Float,
    ElevationBatchIncrement: crate::nodes::types::Float,
    AzimuthBatchIncrement: crate::nodes::types::Float,
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
    type Output = out::StableZero123ConditioningBatchedOutput;
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
    const NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
