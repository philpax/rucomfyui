//!`3d_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
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
///**SV3D_Conditioning**: No description.
#[derive(Clone)]
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
> Sv3DConditioning<ClipVision, InitImage, Vae, Width, Height, VideoFrames, Elevation> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVision,
        init_image: InitImage,
        vae: Vae,
        width: Width,
        height: Height,
        video_frames: VideoFrames,
        elevation: Elevation,
    ) -> Self {
        Self {
            clip_vision,
            init_image,
            vae,
            width,
            height,
            video_frames,
            elevation,
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("init_image".to_string(), self.init_image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("video_frames".to_string(), self.video_frames.clone().into());
        output.insert("elevation".to_string(), self.elevation.clone().into());
        output
    }
    const NAME: &'static str = "SV3D_Conditioning";
    const DISPLAY_NAME: &'static str = "SV3D_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
///**StableZero123_Conditioning**: No description.
#[derive(Clone)]
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
> StableZero123Conditioning<
    ClipVision,
    InitImage,
    Vae,
    Width,
    Height,
    BatchSize,
    Elevation,
    Azimuth,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVision,
        init_image: InitImage,
        vae: Vae,
        width: Width,
        height: Height,
        batch_size: BatchSize,
        elevation: Elevation,
        azimuth: Azimuth,
    ) -> Self {
        Self {
            clip_vision,
            init_image,
            vae,
            width,
            height,
            batch_size,
            elevation,
            azimuth,
        }
    }
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("init_image".to_string(), self.init_image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("elevation".to_string(), self.elevation.clone().into());
        output.insert("azimuth".to_string(), self.azimuth.clone().into());
        output
    }
    const NAME: &'static str = "StableZero123_Conditioning";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
///**StableZero123_Conditioning_Batched**: No description.
#[derive(Clone)]
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
> StableZero123ConditioningBatched<
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
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVision,
        init_image: InitImage,
        vae: Vae,
        width: Width,
        height: Height,
        batch_size: BatchSize,
        elevation: Elevation,
        azimuth: Azimuth,
        elevation_batch_increment: ElevationBatchIncrement,
        azimuth_batch_increment: AzimuthBatchIncrement,
    ) -> Self {
        Self {
            clip_vision,
            init_image,
            vae,
            width,
            height,
            batch_size,
            elevation,
            azimuth,
            elevation_batch_increment,
            azimuth_batch_increment,
        }
    }
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("init_image".to_string(), self.init_image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("elevation".to_string(), self.elevation.clone().into());
        output.insert("azimuth".to_string(), self.azimuth.clone().into());
        output
            .insert(
                "elevation_batch_increment".to_string(),
                self.elevation_batch_increment.clone().into(),
            );
        output
            .insert(
                "azimuth_batch_increment".to_string(),
                self.azimuth_batch_increment.clone().into(),
            );
        output
    }
    const NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning_Batched";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/3d_models";
}
