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
    ///Output for [`HunyuanImageToVideo`](super::HunyuanImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LTXVAddGuide`](super::LTXVAddGuide).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVAddGuideOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LTXVConditioning`](super::LTXVConditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`LTXVCropGuides`](super::LTXVCropGuides).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVCropGuidesOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LTXVImgToVideo`](super::LTXVImgToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVImgToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`SVD_img2vid_Conditioning`](super::SVD_img2vid_Conditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SVD_img2vid_ConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanFirstLastFrameToVideo`](super::WanFirstLastFrameToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanFirstLastFrameToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanFunControlToVideo`](super::WanFunControlToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
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
    #[allow(non_camel_case_types)]
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
    #[allow(non_camel_case_types)]
    pub struct WanImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanVaceToVideo`](super::WanVaceToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanVaceToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
        ///No documentation.
        pub trim_latent: crate::nodes::types::IntOut,
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
    const CATEGORY: &'static str = "conditioning/video_models";
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
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**HunyuanImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    GuidanceTypeParam: crate::nodes::types::String,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 53
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub guidance_type: GuidanceTypeParam,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    GuidanceTypeParam: crate::nodes::types::String,
    StartImageParam: crate::nodes::types::Image,
> HunyuanImageToVideo<
    PositiveParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    GuidanceTypeParam,
    StartImageParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        guidance_type: GuidanceTypeParam,
        start_image: Option<StartImageParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    GuidanceTypeParam: crate::nodes::types::String,
    StartImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for HunyuanImageToVideo<
    PositiveParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    GuidanceTypeParam,
    StartImageParam,
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
#[allow(non_camel_case_types)]
pub struct LTXVAddGuide<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LatentParam: crate::nodes::types::Latent,
    ImageParam: crate::nodes::types::Image,
    FrameIdxParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub latent: LatentParam,
    ///Image or video to condition the latent video on. Must be 8*n + 1 frames.If the video is not 8*n + 1 frames, it will be cropped to the nearest 8*n + 1 frames.
    pub image: ImageParam,
    /**Frame index to start the conditioning at. For single-frame images or videos with 1-8 frames, any frame_idx value is acceptable. For videos with 9+ frames, frame_idx must be divisible by 8, otherwise it will be rounded down to the nearest multiple of 8. Negative values are counted from the end of the video.

**Metadata**:
  - Default: 0
  - Max: 9999
  - Min: -9999
*/
    pub frame_idx: FrameIdxParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LatentParam: crate::nodes::types::Latent,
    ImageParam: crate::nodes::types::Image,
    FrameIdxParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> LTXVAddGuide<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LatentParam,
    ImageParam,
    FrameIdxParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        latent: LatentParam,
        image: ImageParam,
        frame_idx: FrameIdxParam,
        strength: StrengthParam,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LatentParam: crate::nodes::types::Latent,
    ImageParam: crate::nodes::types::Image,
    FrameIdxParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LTXVAddGuide<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LatentParam,
    ImageParam,
    FrameIdxParam,
    StrengthParam,
> {
    type Output = out::LTXVAddGuideOutput;
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
#[allow(non_camel_case_types)]
pub struct LTXVConditioning<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    FrameRateParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub frame_rate: FrameRateParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    FrameRateParam: crate::nodes::types::Float,
> LTXVConditioning<PositiveParam, NegativeParam, FrameRateParam> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        frame_rate: FrameRateParam,
    ) -> Self {
        Self {
            positive,
            negative,
            frame_rate,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    FrameRateParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LTXVConditioning<PositiveParam, NegativeParam, FrameRateParam> {
    type Output = out::LTXVConditioningOutput;
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
#[allow(non_camel_case_types)]
pub struct LTXVCropGuides<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent: LatentParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> LTXVCropGuides<PositiveParam, NegativeParam, LatentParam> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        latent: LatentParam,
    ) -> Self {
        Self { positive, negative, latent }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LTXVCropGuides<PositiveParam, NegativeParam, LatentParam> {
    type Output = out::LTXVCropGuidesOutput;
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
#[allow(non_camel_case_types)]
pub struct LTXVImgToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 768
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 97
  - Max: 16384
  - Min: 9
  - Step: 8
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
*/
    pub strength: StrengthParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> LTXVImgToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    ImageParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        image: ImageParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        strength: StrengthParam,
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
            strength,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LTXVImgToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    ImageParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StrengthParam,
> {
    type Output = out::LTXVImgToVideoOutput;
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
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "LTXVImgToVideo";
    const DISPLAY_NAME: &'static str = "LTXVImgToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**SVD_img2vid_Conditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SVD_img2vid_Conditioning<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    MotionBucketIdParam: crate::nodes::types::Int,
    FpsParam: crate::nodes::types::Int,
    AugmentationLevelParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub init_image: InitImageParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
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
  - Default: 14
  - Max: 4096
  - Min: 1
*/
    pub video_frames: VideoFramesParam,
    /**No documentation.

**Metadata**:
  - Default: 127
  - Max: 1023
  - Min: 1
*/
    pub motion_bucket_id: MotionBucketIdParam,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1024
  - Min: 1
*/
    pub fps: FpsParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub augmentation_level: AugmentationLevelParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    MotionBucketIdParam: crate::nodes::types::Int,
    FpsParam: crate::nodes::types::Int,
    AugmentationLevelParam: crate::nodes::types::Float,
> SVD_img2vid_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    VideoFramesParam,
    MotionBucketIdParam,
    FpsParam,
    AugmentationLevelParam,
> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVisionParam,
        init_image: InitImageParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        video_frames: VideoFramesParam,
        motion_bucket_id: MotionBucketIdParam,
        fps: FpsParam,
        augmentation_level: AugmentationLevelParam,
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
    ClipVisionParam: crate::nodes::types::ClipVision,
    InitImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    VideoFramesParam: crate::nodes::types::Int,
    MotionBucketIdParam: crate::nodes::types::Int,
    FpsParam: crate::nodes::types::Int,
    AugmentationLevelParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SVD_img2vid_Conditioning<
    ClipVisionParam,
    InitImageParam,
    VaeParam,
    WidthParam,
    HeightParam,
    VideoFramesParam,
    MotionBucketIdParam,
    FpsParam,
    AugmentationLevelParam,
> {
    type Output = out::SVD_img2vid_ConditioningOutput;
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
///**WanFirstLastFrameToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanFirstLastFrameToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionStartImageParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    ClipVisionEndImageParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    EndImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub clip_vision_start_image: Option<ClipVisionStartImageParam>,
    ///No documentation.
    pub clip_vision_end_image: Option<ClipVisionEndImageParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub end_image: Option<EndImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionStartImageParam: crate::nodes::types::ClipVisionOutput,
    ClipVisionEndImageParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> WanFirstLastFrameToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionStartImageParam,
    ClipVisionEndImageParam,
    StartImageParam,
    EndImageParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        clip_vision_start_image: Option<ClipVisionStartImageParam>,
        clip_vision_end_image: Option<ClipVisionEndImageParam>,
        start_image: Option<StartImageParam>,
        end_image: Option<EndImageParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            clip_vision_start_image,
            clip_vision_end_image,
            start_image,
            end_image,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionStartImageParam: crate::nodes::types::ClipVisionOutput,
    ClipVisionEndImageParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanFirstLastFrameToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionStartImageParam,
    ClipVisionEndImageParam,
    StartImageParam,
    EndImageParam,
> {
    type Output = out::WanFirstLastFrameToVideoOutput;
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
        if let Some(v) = &self.clip_vision_start_image {
            output.insert("clip_vision_start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_end_image {
            output.insert("clip_vision_end_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.end_image {
            output.insert("end_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanFirstLastFrameToVideo";
    const DISPLAY_NAME: &'static str = "WanFirstLastFrameToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
///**WanFunControlToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanFunControlToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ControlVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub control_video: Option<ControlVideoParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> WanFunControlToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    ControlVideoParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        start_image: Option<StartImageParam>,
        control_video: Option<ControlVideoParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanFunControlToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    ControlVideoParam,
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
#[allow(non_camel_case_types)]
pub struct WanFunInpaintToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    EndImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub end_image: Option<EndImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> WanFunInpaintToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    EndImageParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        start_image: Option<StartImageParam>,
        end_image: Option<EndImageParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanFunInpaintToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    EndImageParam,
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
#[allow(non_camel_case_types)]
pub struct WanImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
> WanImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        start_image: Option<StartImageParam>,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
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
///**WanVaceToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanVaceToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    ControlVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ControlMasksParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///No documentation.
    pub control_video: Option<ControlVideoParam>,
    ///No documentation.
    pub control_masks: Option<ControlMasksParam>,
    ///No documentation.
    pub reference_image: Option<ReferenceImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    ControlVideoParam: crate::nodes::types::Image,
    ControlMasksParam: crate::nodes::types::Mask,
    ReferenceImageParam: crate::nodes::types::Image,
> WanVaceToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StrengthParam,
    ControlVideoParam,
    ControlMasksParam,
    ReferenceImageParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        strength: StrengthParam,
        control_video: Option<ControlVideoParam>,
        control_masks: Option<ControlMasksParam>,
        reference_image: Option<ReferenceImageParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            strength,
            control_video,
            control_masks,
            reference_image,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    ControlVideoParam: crate::nodes::types::Image,
    ControlMasksParam: crate::nodes::types::Mask,
    ReferenceImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanVaceToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StrengthParam,
    ControlVideoParam,
    ControlMasksParam,
    ReferenceImageParam,
> {
    type Output = out::WanVaceToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
            trim_latent: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
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
        output.insert("strength".to_string(), self.strength.clone().into());
        if let Some(v) = &self.control_video {
            output.insert("control_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.control_masks {
            output.insert("control_masks".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanVaceToVideo";
    const DISPLAY_NAME: &'static str = "WanVaceToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models";
}
