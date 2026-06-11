//!`3d_models` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`Hunyuan3Dv2Conditioning`](super::Hunyuan3Dv2Conditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Hunyuan3Dv2ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`Hunyuan3Dv2ConditioningMultiView`](super::Hunyuan3Dv2ConditioningMultiView).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Hunyuan3Dv2ConditioningMultiViewOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`SV3D_Conditioning`](super::SV3D_Conditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SV3D_ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableZero123_Conditioning`](super::StableZero123_Conditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct StableZero123_ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableZero123_Conditioning_Batched`](super::StableZero123_Conditioning_Batched).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct StableZero123_Conditioning_BatchedOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**Hunyuan3Dv2Conditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Hunyuan3Dv2Conditioning<
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> {
    ///No documentation.
    pub clip_vision_output: ClipVisionOutputParam,
}
impl<
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> Hunyuan3Dv2Conditioning<ClipVisionOutputParam> {
    /// Create a new node.
    pub fn new(clip_vision_output: ClipVisionOutputParam) -> Self {
        Self { clip_vision_output }
    }
}
impl<
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode for Hunyuan3Dv2Conditioning<ClipVisionOutputParam> {
    type Output = out::Hunyuan3Dv2ConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.clone().into(),
            );
        output
    }
    const NAME: &'static str = "Hunyuan3Dv2Conditioning";
    const DISPLAY_NAME: &'static str = "Hunyuan3Dv2Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/3d_models";
}
///**Hunyuan3Dv2ConditioningMultiView**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Hunyuan3Dv2ConditioningMultiView<
    FrontParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    LeftParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    BackParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    RightParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub front: Option<FrontParam>,
    ///No documentation.
    pub left: Option<LeftParam>,
    ///No documentation.
    pub back: Option<BackParam>,
    ///No documentation.
    pub right: Option<RightParam>,
}
impl<
    FrontParam: crate::nodes::types::ClipVisionOutput,
    LeftParam: crate::nodes::types::ClipVisionOutput,
    BackParam: crate::nodes::types::ClipVisionOutput,
    RightParam: crate::nodes::types::ClipVisionOutput,
> Hunyuan3Dv2ConditioningMultiView<FrontParam, LeftParam, BackParam, RightParam> {
    /// Create a new node.
    pub fn new(
        front: Option<FrontParam>,
        left: Option<LeftParam>,
        back: Option<BackParam>,
        right: Option<RightParam>,
    ) -> Self {
        Self { front, left, back, right }
    }
}
impl<
    FrontParam: crate::nodes::types::ClipVisionOutput,
    LeftParam: crate::nodes::types::ClipVisionOutput,
    BackParam: crate::nodes::types::ClipVisionOutput,
    RightParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for Hunyuan3Dv2ConditioningMultiView<FrontParam, LeftParam, BackParam, RightParam> {
    type Output = out::Hunyuan3Dv2ConditioningMultiViewOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.front {
            output.insert("front".to_string(), v.clone().into());
        }
        if let Some(v) = &self.left {
            output.insert("left".to_string(), v.clone().into());
        }
        if let Some(v) = &self.back {
            output.insert("back".to_string(), v.clone().into());
        }
        if let Some(v) = &self.right {
            output.insert("right".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Hunyuan3Dv2ConditioningMultiView";
    const DISPLAY_NAME: &'static str = "Hunyuan3Dv2ConditioningMultiView";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/3d_models";
}
///**SV3D_Conditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SV3D_Conditioning<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub init_image: InitImageParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 576
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 576
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 21
  - Max: 4096
  - Min: 1
*/
    pub video_frames: VideoFramesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 90
  - Min: -90
  - Round: false
  - Step: 0.1
*/
    pub elevation: ElevationParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
> SV3D_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    VideoFramesParam,
    ElevationParam,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVisionParam,
        init_image: InitImageParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        video_frames: VideoFramesParam,
        elevation: ElevationParam,
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
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SV3D_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    VideoFramesParam,
    ElevationParam,
> {
    type Output = out::SV3D_ConditioningOutput;
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
        output.insert("elevation".to_string(), self.elevation.clone().into());
        output
    }
    const NAME: &'static str = "SV3D_Conditioning";
    const DISPLAY_NAME: &'static str = "SV3D_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/3d_models";
}
///**StableZero123_Conditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StableZero123_Conditioning<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub init_image: InitImageParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub elevation: ElevationParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub azimuth: AzimuthParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
> StableZero123_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    BatchSizeParam,
    ElevationParam,
    AzimuthParam,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVisionParam,
        init_image: InitImageParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
        elevation: ElevationParam,
        azimuth: AzimuthParam,
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
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for StableZero123_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    BatchSizeParam,
    ElevationParam,
    AzimuthParam,
> {
    type Output = out::StableZero123_ConditioningOutput;
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
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("elevation".to_string(), self.elevation.clone().into());
        output.insert("azimuth".to_string(), self.azimuth.clone().into());
        output
    }
    const NAME: &'static str = "StableZero123_Conditioning";
    const DISPLAY_NAME: &'static str = "StableZero123_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/3d_models";
}
///**StableZero123_Conditioning_Batched**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StableZero123_Conditioning_Batched<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
    ElevationBatchIncrementParam: crate::nodes::types::Float,
    AzimuthBatchIncrementParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub init_image: InitImageParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub elevation: ElevationParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub azimuth: AzimuthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub elevation_batch_increment: ElevationBatchIncrementParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Round: false
  - Step: 0.1
*/
    pub azimuth_batch_increment: AzimuthBatchIncrementParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
    ElevationBatchIncrementParam: crate::nodes::types::Float,
    AzimuthBatchIncrementParam: crate::nodes::types::Float,
> StableZero123_Conditioning_Batched<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    BatchSizeParam,
    ElevationParam,
    AzimuthParam,
    ElevationBatchIncrementParam,
    AzimuthBatchIncrementParam,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVisionParam,
        init_image: InitImageParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
        elevation: ElevationParam,
        azimuth: AzimuthParam,
        elevation_batch_increment: ElevationBatchIncrementParam,
        azimuth_batch_increment: AzimuthBatchIncrementParam,
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
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ElevationParam: crate::nodes::types::Float,
    AzimuthParam: crate::nodes::types::Float,
    ElevationBatchIncrementParam: crate::nodes::types::Float,
    AzimuthBatchIncrementParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for StableZero123_Conditioning_Batched<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    BatchSizeParam,
    ElevationParam,
    AzimuthParam,
    ElevationBatchIncrementParam,
    AzimuthBatchIncrementParam,
> {
    type Output = out::StableZero123_Conditioning_BatchedOutput;
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
    const CATEGORY: &'static str = "model/conditioning/3d_models";
}
