//!`video_models` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`Hunyuan3Dv2Conditioning`](super::Hunyuan3Dv2Conditioning).
    #[derive(Clone)]
    pub struct Hunyuan3Dv2ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`Hunyuan3Dv2ConditioningMultiView`](super::Hunyuan3Dv2ConditioningMultiView).
    #[derive(Clone)]
    pub struct Hunyuan3Dv2ConditioningMultiViewOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`HunyuanImageToVideo`](super::HunyuanImageToVideo).
    #[derive(Clone)]
    pub struct HunyuanImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LtxvAddGuide`](super::LtxvAddGuide).
    #[derive(Clone)]
    pub struct LtxvAddGuideOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LtxvConditioning`](super::LtxvConditioning).
    #[derive(Clone)]
    pub struct LtxvConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`LtxvCropGuides`](super::LtxvCropGuides).
    #[derive(Clone)]
    pub struct LtxvCropGuidesOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LtxvImgToVideo`](super::LtxvImgToVideo).
    #[derive(Clone)]
    pub struct LtxvImgToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
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
    ///Output for [`WanFunControlToVideo`](super::WanFunControlToVideo).
    #[derive(Clone)]
    pub struct WanFunControlToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanFunInpaintToVideo`](super::WanFunInpaintToVideo).
    #[derive(Clone)]
    pub struct WanFunInpaintToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanImageToVideo`](super::WanImageToVideo).
    #[derive(Clone)]
    pub struct WanImageToVideoOutput {
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
pub struct Hunyuan3Dv2Conditioning<
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
> {
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
}
impl<
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
> Hunyuan3Dv2Conditioning<ClipVisionOutput> {
    /// Create a new node.
    pub fn new(clip_vision_output: ClipVisionOutput) -> Self {
        Self { clip_vision_output }
    }
}
impl<ClipVisionOutput: crate::nodes::types::ClipVisionOutput> crate::nodes::TypedNode
for Hunyuan3Dv2Conditioning<ClipVisionOutput> {
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
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**Hunyuan3Dv2ConditioningMultiView**: No description.
#[derive(Clone)]
pub struct Hunyuan3Dv2ConditioningMultiView<
    Front: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    Left: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    Back: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    Right: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub front: Option<Front>,
    ///No documentation.
    pub left: Option<Left>,
    ///No documentation.
    pub back: Option<Back>,
    ///No documentation.
    pub right: Option<Right>,
}
impl<
    Front: crate::nodes::types::ClipVisionOutput,
    Left: crate::nodes::types::ClipVisionOutput,
    Back: crate::nodes::types::ClipVisionOutput,
    Right: crate::nodes::types::ClipVisionOutput,
> Hunyuan3Dv2ConditioningMultiView<Front, Left, Back, Right> {
    /// Create a new node.
    pub fn new(
        front: Option<Front>,
        left: Option<Left>,
        back: Option<Back>,
        right: Option<Right>,
    ) -> Self {
        Self { front, left, back, right }
    }
}
impl<
    Front: crate::nodes::types::ClipVisionOutput,
    Left: crate::nodes::types::ClipVisionOutput,
    Back: crate::nodes::types::ClipVisionOutput,
    Right: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for Hunyuan3Dv2ConditioningMultiView<Front, Left, Back, Right> {
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
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**HunyuanImageToVideo**: No description.
#[derive(Clone)]
pub struct HunyuanImageToVideo<
    Positive: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    GuidanceType: crate::nodes::types::String,
    StartImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 53
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    ///No documentation.
    pub guidance_type: GuidanceType,
    ///No documentation.
    pub start_image: Option<StartImage>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    GuidanceType: crate::nodes::types::String,
    StartImage: crate::nodes::types::Image,
> HunyuanImageToVideo<
    Positive,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    GuidanceType,
    StartImage,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        vae: Vae,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        guidance_type: GuidanceType,
        start_image: Option<StartImage>,
    ) -> Self {
        Self {
            positive,
            vae,
            width,
            height,
            length,
            batch_size,
            guidance_type,
            start_image,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    GuidanceType: crate::nodes::types::String,
    StartImage: crate::nodes::types::Image,
> crate::nodes::TypedNode
for HunyuanImageToVideo<
    Positive,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    GuidanceType,
    StartImage,
> {
    type Output = out::HunyuanImageToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("guidance_type".to_string(), self.guidance_type.clone().into());
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "HunyuanImageToVideo";
    const DISPLAY_NAME: &'static str = "HunyuanImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**LTXVAddGuide**: No description.
#[derive(Clone)]
pub struct LtxvAddGuide<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Latent: crate::nodes::types::Latent,
    Image: crate::nodes::types::Image,
    FrameIdx: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub latent: Latent,
    ///Image or video to condition the latent video on. Must be 8*n + 1 frames.If the video is not 8*n + 1 frames, it will be cropped to the nearest 8*n + 1 frames.
    pub image: Image,
    /**Frame index to start the conditioning at. For single-frame images or videos with 1-8 frames, any frame_idx value is acceptable. For videos with 9+ frames, frame_idx must be divisible by 8, otherwise it will be rounded down to the nearest multiple of 8. Negative values are counted from the end of the video.

**Metadata**:
  - Default: 0
  - Max: 9999
  - Min: -9999
*/
    pub frame_idx: FrameIdx,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Latent: crate::nodes::types::Latent,
    Image: crate::nodes::types::Image,
    FrameIdx: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> LtxvAddGuide<Positive, Negative, Vae, Latent, Image, FrameIdx, Strength> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        latent: Latent,
        image: Image,
        frame_idx: FrameIdx,
        strength: Strength,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            latent,
            image,
            frame_idx,
            strength,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Latent: crate::nodes::types::Latent,
    Image: crate::nodes::types::Image,
    FrameIdx: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LtxvAddGuide<Positive, Negative, Vae, Latent, Image, FrameIdx, Strength> {
    type Output = out::LtxvAddGuideOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("latent".to_string(), self.latent.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("frame_idx".to_string(), self.frame_idx.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "LTXVAddGuide";
    const DISPLAY_NAME: &'static str = "LTXVAddGuide";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**LTXVConditioning**: No description.
#[derive(Clone)]
pub struct LtxvConditioning<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub frame_rate: FrameRate,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> LtxvConditioning<Positive, Negative, FrameRate> {
    /// Create a new node.
    pub fn new(positive: Positive, negative: Negative, frame_rate: FrameRate) -> Self {
        Self {
            positive,
            negative,
            frame_rate,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    FrameRate: crate::nodes::types::Float,
> crate::nodes::TypedNode for LtxvConditioning<Positive, Negative, FrameRate> {
    type Output = out::LtxvConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("frame_rate".to_string(), self.frame_rate.clone().into());
        output
    }
    const NAME: &'static str = "LTXVConditioning";
    const DISPLAY_NAME: &'static str = "LTXVConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**LTXVCropGuides**: No description.
#[derive(Clone)]
pub struct LtxvCropGuides<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Latent: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub latent: Latent,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Latent: crate::nodes::types::Latent,
> LtxvCropGuides<Positive, Negative, Latent> {
    /// Create a new node.
    pub fn new(positive: Positive, negative: Negative, latent: Latent) -> Self {
        Self { positive, negative, latent }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Latent: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LtxvCropGuides<Positive, Negative, Latent> {
    type Output = out::LtxvCropGuidesOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("latent".to_string(), self.latent.clone().into());
        output
    }
    const NAME: &'static str = "LTXVCropGuides";
    const DISPLAY_NAME: &'static str = "LTXVCropGuides";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**LTXVImgToVideo**: No description.
#[derive(Clone)]
pub struct LtxvImgToVideo<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 768
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 97
  - Max: 16384
  - Min: 9
  - Step: 8
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> LtxvImgToVideo<Positive, Negative, Vae, Image, Width, Height, Length, BatchSize> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        image: Image,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            image,
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode
for LtxvImgToVideo<Positive, Negative, Vae, Image, Width, Height, Length, BatchSize> {
    type Output = out::LtxvImgToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "LTXVImgToVideo";
    const DISPLAY_NAME: &'static str = "LTXVImgToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
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
///**WanFunControlToVideo**: No description.
#[derive(Clone)]
pub struct WanFunControlToVideo<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ControlVideo: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutput>,
    ///No documentation.
    pub start_image: Option<StartImage>,
    ///No documentation.
    pub control_video: Option<ControlVideo>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
    ControlVideo: crate::nodes::types::Image,
> WanFunControlToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
    ControlVideo,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        clip_vision_output: Option<ClipVisionOutput>,
        start_image: Option<StartImage>,
        control_video: Option<ControlVideo>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            clip_vision_output,
            start_image,
            control_video,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
    ControlVideo: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanFunControlToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
    ControlVideo,
> {
    type Output = out::WanFunControlToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.control_video {
            output.insert("control_video".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanFunControlToVideo";
    const DISPLAY_NAME: &'static str = "WanFunControlToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**WanFunInpaintToVideo**: No description.
#[derive(Clone)]
pub struct WanFunInpaintToVideo<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    EndImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutput>,
    ///No documentation.
    pub start_image: Option<StartImage>,
    ///No documentation.
    pub end_image: Option<EndImage>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
    EndImage: crate::nodes::types::Image,
> WanFunInpaintToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
    EndImage,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        clip_vision_output: Option<ClipVisionOutput>,
        start_image: Option<StartImage>,
        end_image: Option<EndImage>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            clip_vision_output,
            start_image,
            end_image,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
    EndImage: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanFunInpaintToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
    EndImage,
> {
    type Output = out::WanFunInpaintToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.end_image {
            output.insert("end_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanFunInpaintToVideo";
    const DISPLAY_NAME: &'static str = "WanFunInpaintToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**WanImageToVideo**: No description.
#[derive(Clone)]
pub struct WanImageToVideo<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutput>,
    ///No documentation.
    pub start_image: Option<StartImage>,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
> WanImageToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        clip_vision_output: Option<ClipVisionOutput>,
        start_image: Option<StartImage>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            clip_vision_output,
            start_image,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    StartImage: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanImageToVideo<
    Positive,
    Negative,
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    ClipVisionOutput,
    StartImage,
> {
    type Output = out::WanImageToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanImageToVideo";
    const DISPLAY_NAME: &'static str = "WanImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
