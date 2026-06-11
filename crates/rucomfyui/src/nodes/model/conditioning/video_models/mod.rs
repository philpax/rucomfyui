//!`video_models` definitions/categories.
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
    ///Output for [`ARVideoI2V`](super::ARVideoI2V).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ARVideoI2VOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`GenerateTracks`](super::GenerateTracks).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GenerateTracksOutput {
        ///No documentation.
        pub tracks: crate::nodes::types::TracksOut,
        ///No documentation.
        pub track_length: crate::nodes::types::IntOut,
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
    ///Output for [`HunyuanRefinerLatent`](super::HunyuanRefinerLatent).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanRefinerLatentOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`HunyuanVideo15ImageToVideo`](super::HunyuanVideo15ImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanVideo15ImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`HunyuanVideo15SuperResolution`](super::HunyuanVideo15SuperResolution).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HunyuanVideo15SuperResolutionOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`Kandinsky5ImageToVideo`](super::Kandinsky5ImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Kandinsky5ImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///Empty video latent
        pub latent: crate::nodes::types::LatentOut,
        ///Clean encoded start images, used to replace the noisy start of the model output latents
        pub cond_latent: crate::nodes::types::LatentOut,
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
    ///Output for [`VOIDInpaintConditioning`](super::VOIDInpaintConditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct VOIDInpaintConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`Wan22FunControlToVideo`](super::Wan22FunControlToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Wan22FunControlToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanAnimateToVideo`](super::WanAnimateToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanAnimateToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
        ///No documentation.
        pub trim_latent: crate::nodes::types::IntOut,
        ///No documentation.
        pub trim_image: crate::nodes::types::IntOut,
        ///No documentation.
        pub video_frame_offset: crate::nodes::types::IntOut,
    }
    ///Output for [`WanCameraEmbedding`](super::WanCameraEmbedding).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanCameraEmbeddingOutput {
        ///No documentation.
        pub camera_embedding: crate::nodes::types::WanCameraEmbeddingOut,
        ///No documentation.
        pub width: crate::nodes::types::IntOut,
        ///No documentation.
        pub height: crate::nodes::types::IntOut,
        ///No documentation.
        pub length: crate::nodes::types::IntOut,
    }
    ///Output for [`WanCameraImageToVideo`](super::WanCameraImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanCameraImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanDancerEncodeAudio`](super::WanDancerEncodeAudio).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanDancerEncodeAudioOutput {
        ///No documentation.
        pub audio_encoder_output: crate::nodes::types::AudioEncoderOutputOut,
        ///The calculated fps based on the audio length and the number of video frames. Used in the prompt.
        pub fps_string: crate::nodes::types::StringOut,
    }
    ///Output for [`WanDancerVideo`](super::WanDancerVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanDancerVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///Empty latent.
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
    ///Output for [`WanHuMoImageToVideo`](super::WanHuMoImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanHuMoImageToVideoOutput {
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
    ///Output for [`WanInfiniteTalkToVideo`](super::WanInfiniteTalkToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanInfiniteTalkToVideoOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
        ///No documentation.
        pub trim_image: crate::nodes::types::IntOut,
    }
    ///Output for [`WanMoveTrackToVideo`](super::WanMoveTrackToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanMoveTrackToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanMoveTracksFromCoords`](super::WanMoveTracksFromCoords).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanMoveTracksFromCoordsOutput {
        ///No documentation.
        pub tracks: crate::nodes::types::TracksOut,
        ///No documentation.
        pub track_length: crate::nodes::types::IntOut,
    }
    ///Output for [`WanPhantomSubjectToVideo`](super::WanPhantomSubjectToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanPhantomSubjectToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative_text: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative_img_text: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanSCAILToVideo`](super::WanSCAILToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanSCAILToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///Empty latent of the generation size.
        pub latent: crate::nodes::types::LatentOut,
        ///Adjusted offset + length. Wire into the next chunk.
        pub video_frame_offset: crate::nodes::types::IntOut,
    }
    ///Output for [`WanSoundImageToVideo`](super::WanSoundImageToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanSoundImageToVideoOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanSoundImageToVideoExtend`](super::WanSoundImageToVideoExtend).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanSoundImageToVideoExtendOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`WanTrackToVideo`](super::WanTrackToVideo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanTrackToVideoOutput {
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
///**ARVideoI2V**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARVideoI2V<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub start_image: StartImageParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 1024
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> ARVideoI2V<
    ModelParam,
    VaeParam,
    StartImageParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        vae: VaeParam,
        start_image: StartImageParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            model,
            vae,
            start_image,
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ARVideoI2V<
    ModelParam,
    VaeParam,
    StartImageParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    type Output = out::ARVideoI2VOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("start_image".to_string(), self.start_image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "ARVideoI2V";
    const DISPLAY_NAME: &'static str = "ARVideoI2V";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**GenerateTracks**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GenerateTracks<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    StartXParam: crate::nodes::types::Float,
    StartYParam: crate::nodes::types::Float,
    EndXParam: crate::nodes::types::Float,
    EndYParam: crate::nodes::types::Float,
    NumFramesParam: crate::nodes::types::Int,
    NumTracksParam: crate::nodes::types::Int,
    TrackSpreadParam: crate::nodes::types::Float,
    BezierParam: crate::nodes::types::Boolean,
    MidXParam: crate::nodes::types::Float,
    MidYParam: crate::nodes::types::Float,
    TrackMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 4096
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 4096
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**Normalized X coordinate (0-1) for start position.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub start_x: StartXParam,
    /**Normalized Y coordinate (0-1) for start position.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub start_y: StartYParam,
    /**Normalized X coordinate (0-1) for end position.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub end_x: EndXParam,
    /**Normalized Y coordinate (0-1) for end position.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub end_y: EndYParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 1024
  - Min: 1
*/
    pub num_frames: NumFramesParam,
    /**No documentation.

**Metadata**:
  - Default: 5
  - Max: 100
  - Min: 1
*/
    pub num_tracks: NumTracksParam,
    /**Normalized distance between tracks. Tracks are spread perpendicular to the motion direction.

**Metadata**:
  - Default: 0.025
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub track_spread: TrackSpreadParam,
    /**Enable Bezier curve path using the mid point as control point.

**Metadata**:
  - Default: false
*/
    pub bezier: BezierParam,
    /**Normalized X control point for Bezier curve. Only used when 'bezier' is enabled.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub mid_x: MidXParam,
    /**Normalized Y control point for Bezier curve. Only used when 'bezier' is enabled.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub mid_y: MidYParam,
    ///Optional mask to indicate visible frames.
    pub track_mask: Option<TrackMaskParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    StartXParam: crate::nodes::types::Float,
    StartYParam: crate::nodes::types::Float,
    EndXParam: crate::nodes::types::Float,
    EndYParam: crate::nodes::types::Float,
    NumFramesParam: crate::nodes::types::Int,
    NumTracksParam: crate::nodes::types::Int,
    TrackSpreadParam: crate::nodes::types::Float,
    BezierParam: crate::nodes::types::Boolean,
    MidXParam: crate::nodes::types::Float,
    MidYParam: crate::nodes::types::Float,
    TrackMaskParam: crate::nodes::types::Mask,
> GenerateTracks<
    WidthParam,
    HeightParam,
    StartXParam,
    StartYParam,
    EndXParam,
    EndYParam,
    NumFramesParam,
    NumTracksParam,
    TrackSpreadParam,
    BezierParam,
    MidXParam,
    MidYParam,
    TrackMaskParam,
> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        start_x: StartXParam,
        start_y: StartYParam,
        end_x: EndXParam,
        end_y: EndYParam,
        num_frames: NumFramesParam,
        num_tracks: NumTracksParam,
        track_spread: TrackSpreadParam,
        bezier: BezierParam,
        mid_x: MidXParam,
        mid_y: MidYParam,
        track_mask: Option<TrackMaskParam>,
    ) -> Self {
        Self {
            width,
            height,
            start_x,
            start_y,
            end_x,
            end_y,
            num_frames,
            num_tracks,
            track_spread,
            bezier,
            mid_x,
            mid_y,
            track_mask,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    StartXParam: crate::nodes::types::Float,
    StartYParam: crate::nodes::types::Float,
    EndXParam: crate::nodes::types::Float,
    EndYParam: crate::nodes::types::Float,
    NumFramesParam: crate::nodes::types::Int,
    NumTracksParam: crate::nodes::types::Int,
    TrackSpreadParam: crate::nodes::types::Float,
    BezierParam: crate::nodes::types::Boolean,
    MidXParam: crate::nodes::types::Float,
    MidYParam: crate::nodes::types::Float,
    TrackMaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for GenerateTracks<
    WidthParam,
    HeightParam,
    StartXParam,
    StartYParam,
    EndXParam,
    EndYParam,
    NumFramesParam,
    NumTracksParam,
    TrackSpreadParam,
    BezierParam,
    MidXParam,
    MidYParam,
    TrackMaskParam,
> {
    type Output = out::GenerateTracksOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            tracks: crate::nodes::types::TracksOut::from_dynamic(node_id, 0u32),
            track_length: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("start_x".to_string(), self.start_x.clone().into());
        output.insert("start_y".to_string(), self.start_y.clone().into());
        output.insert("end_x".to_string(), self.end_x.clone().into());
        output.insert("end_y".to_string(), self.end_y.clone().into());
        output.insert("num_frames".to_string(), self.num_frames.clone().into());
        output.insert("num_tracks".to_string(), self.num_tracks.clone().into());
        output.insert("track_spread".to_string(), self.track_spread.clone().into());
        output.insert("bezier".to_string(), self.bezier.clone().into());
        output.insert("mid_x".to_string(), self.mid_x.clone().into());
        output.insert("mid_y".to_string(), self.mid_y.clone().into());
        if let Some(v) = &self.track_mask {
            output.insert("track_mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "GenerateTracks";
    const DISPLAY_NAME: &'static str = "GenerateTracks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**Get IC-LoRA Parameters**: Extracts IC-LoRA parameters from the safetensors metadata of a LoRA-loaded model and outputs them for LTXVAddGuide (eg. reference_downscale_factor).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GetICLoRAParameters<IcloraModelParam: crate::nodes::types::Model> {
    ///Direct output from a LoRA Loader for the specific IC-LoRA from which to extract the metadata.
    pub iclora_model: IcloraModelParam,
}
impl<
    IcloraModelParam: crate::nodes::types::Model,
> GetICLoRAParameters<IcloraModelParam> {
    /// Create a new node.
    pub fn new(iclora_model: IcloraModelParam) -> Self {
        Self { iclora_model }
    }
}
impl<IcloraModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for GetICLoRAParameters<IcloraModelParam> {
    type Output = crate::nodes::types::IcLoraParametersOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("iclora_model".to_string(), self.iclora_model.clone().into());
        output
    }
    const NAME: &'static str = "GetICLoRAParameters";
    const DISPLAY_NAME: &'static str = "Get IC-LoRA Parameters";
    const DESCRIPTION: &'static str = "Extracts IC-LoRA parameters from the safetensors metadata of a LoRA-loaded model and outputs them for LTXVAddGuide (eg. reference_downscale_factor).";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    pub start_image: Option<StartImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
> HunyuanImageToVideo<
    PositiveParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
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
        start_image: Option<StartImageParam>,
    ) -> Self {
        Self {
            positive,
            vae,
            width,
            height,
            length,
            batch_size,
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
    StartImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for HunyuanImageToVideo<
    PositiveParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
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
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "HunyuanImageToVideo";
    const DISPLAY_NAME: &'static str = "HunyuanImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**Hunyuan Latent Refiner**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanRefinerLatent<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent: LatentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentationParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> HunyuanRefinerLatent<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        latent: LatentParam,
        noise_augmentation: NoiseAugmentationParam,
    ) -> Self {
        Self {
            positive,
            negative,
            latent,
            noise_augmentation,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for HunyuanRefinerLatent<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
> {
    type Output = out::HunyuanRefinerLatentOutput;
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
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        output
    }
    const NAME: &'static str = "HunyuanRefinerLatent";
    const DISPLAY_NAME: &'static str = "Hunyuan Latent Refiner";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**HunyuanVideo15ImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanVideo15ImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
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
  - Default: 33
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
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> HunyuanVideo15ImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    ClipVisionOutputParam,
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
        start_image: Option<StartImageParam>,
        clip_vision_output: Option<ClipVisionOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            start_image,
            clip_vision_output,
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
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for HunyuanVideo15ImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    type Output = out::HunyuanVideo15ImageToVideoOutput;
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
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "HunyuanVideo15ImageToVideo";
    const DISPLAY_NAME: &'static str = "HunyuanVideo15ImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**Hunyuan Video 1.5 Super Resolution**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HunyuanVideo15SuperResolution<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae = crate::nodes::types::VaeOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub latent: LatentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.7
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentationParam,
    ///No documentation.
    pub vae: Option<VaeParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> HunyuanVideo15SuperResolution<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
    VaeParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        latent: LatentParam,
        noise_augmentation: NoiseAugmentationParam,
        vae: Option<VaeParam>,
        start_image: Option<StartImageParam>,
        clip_vision_output: Option<ClipVisionOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            latent,
            noise_augmentation,
            vae,
            start_image,
            clip_vision_output,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    LatentParam: crate::nodes::types::Latent,
    NoiseAugmentationParam: crate::nodes::types::Float,
    VaeParam: crate::nodes::types::Vae,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for HunyuanVideo15SuperResolution<
    PositiveParam,
    NegativeParam,
    LatentParam,
    NoiseAugmentationParam,
    VaeParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    type Output = out::HunyuanVideo15SuperResolutionOutput;
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
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        if let Some(v) = &self.vae {
            output.insert("vae".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "HunyuanVideo15SuperResolution";
    const DISPLAY_NAME: &'static str = "Hunyuan Video 1.5 Super Resolution";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**Kandinsky5ImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Kandinsky5ImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
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
  - Default: 768
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 121
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
    StartImageParam: crate::nodes::types::Image,
> Kandinsky5ImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
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
    StartImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for Kandinsky5ImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
> {
    type Output = out::Kandinsky5ImageToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
            cond_latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 3u32),
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
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Kandinsky5ImageToVideo";
    const DISPLAY_NAME: &'static str = "Kandinsky5ImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    AttentionMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    IcloraParametersParam: crate::nodes::types::IcLoraParameters
        = crate::nodes::types::IcLoraParametersOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub latent: LatentParam,
    ///Image or video to condition the latent video on. Must be 8*n + 1 frames. If the video is not 8*n + 1 frames, it will be cropped to the nearest 8*n + 1 frames.
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
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///Optional pixel-space spatial mask. Controls per-region conditioning influence via self-attention, multiplied by strength.
    pub attention_mask: Option<AttentionMaskParam>,
    ///Optional IC-LoRA parameters from a Get IC-LoRA Parameters node. Used for adjusting guide processing as required by certain IC-LoRAs (eg. those with a reference_downscale_factor > 1). When chained, each LTXVAddGuide uses only the parameters connected to it.
    pub iclora_parameters: Option<IcloraParametersParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LatentParam: crate::nodes::types::Latent,
    ImageParam: crate::nodes::types::Image,
    FrameIdxParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    AttentionMaskParam: crate::nodes::types::Mask,
    IcloraParametersParam: crate::nodes::types::IcLoraParameters,
> LTXVAddGuide<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LatentParam,
    ImageParam,
    FrameIdxParam,
    StrengthParam,
    AttentionMaskParam,
    IcloraParametersParam,
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
        attention_mask: Option<AttentionMaskParam>,
        iclora_parameters: Option<IcloraParametersParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            latent,
            image,
            frame_idx,
            strength,
            attention_mask,
            iclora_parameters,
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
    AttentionMaskParam: crate::nodes::types::Mask,
    IcloraParametersParam: crate::nodes::types::IcLoraParameters,
> crate::nodes::TypedNode
for LTXVAddGuide<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LatentParam,
    ImageParam,
    FrameIdxParam,
    StrengthParam,
    AttentionMaskParam,
    IcloraParametersParam,
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
        if let Some(v) = &self.attention_mask {
            output.insert("attention_mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.iclora_parameters {
            output.insert("iclora_parameters".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LTXVAddGuide";
    const DISPLAY_NAME: &'static str = "LTXVAddGuide";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**LTXVImgToVideoInplace**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVImgToVideoInplace<
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    LatentParam: crate::nodes::types::Latent,
    StrengthParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub latent: LatentParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
*/
    pub strength: StrengthParam,
    /**Bypass the conditioning.

**Metadata**:
  - Default: false
*/
    pub bypass: BypassParam,
}
impl<
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    LatentParam: crate::nodes::types::Latent,
    StrengthParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
> LTXVImgToVideoInplace<VaeParam, ImageParam, LatentParam, StrengthParam, BypassParam> {
    /// Create a new node.
    pub fn new(
        vae: VaeParam,
        image: ImageParam,
        latent: LatentParam,
        strength: StrengthParam,
        bypass: BypassParam,
    ) -> Self {
        Self {
            vae,
            image,
            latent,
            strength,
            bypass,
        }
    }
}
impl<
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    LatentParam: crate::nodes::types::Latent,
    StrengthParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for LTXVImgToVideoInplace<
    VaeParam,
    ImageParam,
    LatentParam,
    StrengthParam,
    BypassParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("latent".to_string(), self.latent.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("bypass".to_string(), self.bypass.clone().into());
        output
    }
    const NAME: &'static str = "LTXVImgToVideoInplace";
    const DISPLAY_NAME: &'static str = "LTXVImgToVideoInplace";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**NormalizeVideoLatentStart**: Normalizes the initial frames of a video latent to match the mean and standard deviation of subsequent reference frames. Helps reduce differences between the starting frames and the rest of the video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NormalizeVideoLatentStart<
    LatentParam: crate::nodes::types::Latent,
    StartFrameCountParam: crate::nodes::types::Int,
    ReferenceFrameCountParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub latent: LatentParam,
    /**Number of latent frames to normalize, counted from the start

**Metadata**:
  - Default: 4
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub start_frame_count: StartFrameCountParam,
    /**Number of latent frames after the start frames to use as reference

**Metadata**:
  - Default: 5
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub reference_frame_count: ReferenceFrameCountParam,
}
impl<
    LatentParam: crate::nodes::types::Latent,
    StartFrameCountParam: crate::nodes::types::Int,
    ReferenceFrameCountParam: crate::nodes::types::Int,
> NormalizeVideoLatentStart<
    LatentParam,
    StartFrameCountParam,
    ReferenceFrameCountParam,
> {
    /// Create a new node.
    pub fn new(
        latent: LatentParam,
        start_frame_count: StartFrameCountParam,
        reference_frame_count: ReferenceFrameCountParam,
    ) -> Self {
        Self {
            latent,
            start_frame_count,
            reference_frame_count,
        }
    }
}
impl<
    LatentParam: crate::nodes::types::Latent,
    StartFrameCountParam: crate::nodes::types::Int,
    ReferenceFrameCountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for NormalizeVideoLatentStart<
    LatentParam,
    StartFrameCountParam,
    ReferenceFrameCountParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("latent".to_string(), self.latent.clone().into());
        output
            .insert(
                "start_frame_count".to_string(),
                self.start_frame_count.clone().into(),
            );
        output
            .insert(
                "reference_frame_count".to_string(),
                self.reference_frame_count.clone().into(),
            );
        output
    }
    const NAME: &'static str = "NormalizeVideoLatentStart";
    const DISPLAY_NAME: &'static str = "NormalizeVideoLatentStart";
    const DESCRIPTION: &'static str = "Normalizes the initial frames of a video latent to match the mean and standard deviation of subsequent reference frames. Helps reduce differences between the starting frames and the rest of the video.";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**VOIDInpaintConditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VOIDInpaintConditioning<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    VideoParam: crate::nodes::types::Image,
    QuadmaskParam: crate::nodes::types::Mask,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///Source video frames \[T, H, W, 3\]
    pub video: VideoParam,
    ///Preprocessed quadmask from VOIDQuadmaskPreprocess \[T, H, W\]
    pub quadmask: QuadmaskParam,
    /**No documentation.

**Metadata**:
  - Default: 672
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 384
  - Max: 16384
  - Min: 16
  - Step: 8
*/
    pub height: HeightParam,
    /**Number of pixel frames to process. For CogVideoX-Fun-V1.5 (patch_size_t=2), latent_t must be even — lengths that produce odd latent_t are rounded down (e.g. 49 → 45).

**Metadata**:
  - Default: 45
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    VideoParam: crate::nodes::types::Image,
    QuadmaskParam: crate::nodes::types::Mask,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> VOIDInpaintConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    VideoParam,
    QuadmaskParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        video: VideoParam,
        quadmask: QuadmaskParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            video,
            quadmask,
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    VideoParam: crate::nodes::types::Image,
    QuadmaskParam: crate::nodes::types::Mask,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VOIDInpaintConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    VideoParam,
    QuadmaskParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
> {
    type Output = out::VOIDInpaintConditioningOutput;
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
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("quadmask".to_string(), self.quadmask.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "VOIDInpaintConditioning";
    const DISPLAY_NAME: &'static str = "VOIDInpaintConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**Wan22FunControlToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Wan22FunControlToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    RefImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
    pub ref_image: Option<RefImageParam>,
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
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> Wan22FunControlToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    RefImageParam,
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
        ref_image: Option<RefImageParam>,
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
            ref_image,
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
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for Wan22FunControlToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    RefImageParam,
    ControlVideoParam,
> {
    type Output = out::Wan22FunControlToVideoOutput;
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
        if let Some(v) = &self.ref_image {
            output.insert("ref_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.control_video {
            output.insert("control_video".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Wan22FunControlToVideo";
    const DISPLAY_NAME: &'static str = "Wan22FunControlToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanAnimateToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanAnimateToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ContinueMotionMaxFramesParam: crate::nodes::types::Int,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FaceVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    PoseVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    BackgroundVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    CharacterMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    ContinueMotionParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
  - Default: 77
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
  - Default: 5
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub continue_motion_max_frames: ContinueMotionMaxFramesParam,
    /**The amount of frames to seek in all the input videos. Used for generating longer videos by chunk. Connect to the video_frame_offset output of the previous node for extending a video.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub video_frame_offset: VideoFrameOffsetParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///No documentation.
    pub reference_image: Option<ReferenceImageParam>,
    ///No documentation.
    pub face_video: Option<FaceVideoParam>,
    ///No documentation.
    pub pose_video: Option<PoseVideoParam>,
    ///No documentation.
    pub background_video: Option<BackgroundVideoParam>,
    ///No documentation.
    pub character_mask: Option<CharacterMaskParam>,
    ///No documentation.
    pub continue_motion: Option<ContinueMotionParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ContinueMotionMaxFramesParam: crate::nodes::types::Int,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    ReferenceImageParam: crate::nodes::types::Image,
    FaceVideoParam: crate::nodes::types::Image,
    PoseVideoParam: crate::nodes::types::Image,
    BackgroundVideoParam: crate::nodes::types::Image,
    CharacterMaskParam: crate::nodes::types::Mask,
    ContinueMotionParam: crate::nodes::types::Image,
> WanAnimateToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ContinueMotionMaxFramesParam,
    VideoFrameOffsetParam,
    ClipVisionOutputParam,
    ReferenceImageParam,
    FaceVideoParam,
    PoseVideoParam,
    BackgroundVideoParam,
    CharacterMaskParam,
    ContinueMotionParam,
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
        continue_motion_max_frames: ContinueMotionMaxFramesParam,
        video_frame_offset: VideoFrameOffsetParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        reference_image: Option<ReferenceImageParam>,
        face_video: Option<FaceVideoParam>,
        pose_video: Option<PoseVideoParam>,
        background_video: Option<BackgroundVideoParam>,
        character_mask: Option<CharacterMaskParam>,
        continue_motion: Option<ContinueMotionParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            continue_motion_max_frames,
            video_frame_offset,
            clip_vision_output,
            reference_image,
            face_video,
            pose_video,
            background_video,
            character_mask,
            continue_motion,
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
    ContinueMotionMaxFramesParam: crate::nodes::types::Int,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    ReferenceImageParam: crate::nodes::types::Image,
    FaceVideoParam: crate::nodes::types::Image,
    PoseVideoParam: crate::nodes::types::Image,
    BackgroundVideoParam: crate::nodes::types::Image,
    CharacterMaskParam: crate::nodes::types::Mask,
    ContinueMotionParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanAnimateToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ContinueMotionMaxFramesParam,
    VideoFrameOffsetParam,
    ClipVisionOutputParam,
    ReferenceImageParam,
    FaceVideoParam,
    PoseVideoParam,
    BackgroundVideoParam,
    CharacterMaskParam,
    ContinueMotionParam,
> {
    type Output = out::WanAnimateToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
            trim_latent: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
            trim_image: crate::nodes::types::IntOut::from_dynamic(node_id, 4u32),
            video_frame_offset: crate::nodes::types::IntOut::from_dynamic(node_id, 5u32),
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
        output
            .insert(
                "continue_motion_max_frames".to_string(),
                self.continue_motion_max_frames.clone().into(),
            );
        output
            .insert(
                "video_frame_offset".to_string(),
                self.video_frame_offset.clone().into(),
            );
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_video {
            output.insert("face_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pose_video {
            output.insert("pose_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.background_video {
            output.insert("background_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.character_mask {
            output.insert("character_mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.continue_motion {
            output.insert("continue_motion".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanAnimateToVideo";
    const DISPLAY_NAME: &'static str = "WanAnimateToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanCameraEmbedding**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanCameraEmbedding<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    FxParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    FyParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    CxParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    CyParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
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
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub speed: Option<SpeedParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.000000001
*/
    pub fx: Option<FxParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.000000001
*/
    pub fy: Option<FyParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cx: Option<CxParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cy: Option<CyParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float,
    FxParam: crate::nodes::types::Float,
    FyParam: crate::nodes::types::Float,
    CxParam: crate::nodes::types::Float,
    CyParam: crate::nodes::types::Float,
> WanCameraEmbedding<
    WidthParam,
    HeightParam,
    LengthParam,
    SpeedParam,
    FxParam,
    FyParam,
    CxParam,
    CyParam,
> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        speed: Option<SpeedParam>,
        fx: Option<FxParam>,
        fy: Option<FyParam>,
        cx: Option<CxParam>,
        cy: Option<CyParam>,
    ) -> Self {
        Self {
            width,
            height,
            length,
            speed,
            fx,
            fy,
            cx,
            cy,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float,
    FxParam: crate::nodes::types::Float,
    FyParam: crate::nodes::types::Float,
    CxParam: crate::nodes::types::Float,
    CyParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for WanCameraEmbedding<
    WidthParam,
    HeightParam,
    LengthParam,
    SpeedParam,
    FxParam,
    FyParam,
    CxParam,
    CyParam,
> {
    type Output = out::WanCameraEmbeddingOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            camera_embedding: crate::nodes::types::WanCameraEmbeddingOut::from_dynamic(
                node_id,
                0u32,
            ),
            width: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
            height: crate::nodes::types::IntOut::from_dynamic(node_id, 2u32),
            length: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        if let Some(v) = &self.speed {
            output.insert("speed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.fx {
            output.insert("fx".to_string(), v.clone().into());
        }
        if let Some(v) = &self.fy {
            output.insert("fy".to_string(), v.clone().into());
        }
        if let Some(v) = &self.cx {
            output.insert("cx".to_string(), v.clone().into());
        }
        if let Some(v) = &self.cy {
            output.insert("cy".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanCameraEmbedding";
    const DISPLAY_NAME: &'static str = "WanCameraEmbedding";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanCameraImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanCameraImageToVideo<
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
    CameraConditionsParam: crate::nodes::types::WanCameraEmbedding
        = crate::nodes::types::WanCameraEmbeddingOut,
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
    pub camera_conditions: Option<CameraConditionsParam>,
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
    CameraConditionsParam: crate::nodes::types::WanCameraEmbedding,
> WanCameraImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    CameraConditionsParam,
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
        camera_conditions: Option<CameraConditionsParam>,
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
            camera_conditions,
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
    CameraConditionsParam: crate::nodes::types::WanCameraEmbedding,
> crate::nodes::TypedNode
for WanCameraImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ClipVisionOutputParam,
    StartImageParam,
    CameraConditionsParam,
> {
    type Output = out::WanCameraImageToVideoOutput;
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
        if let Some(v) = &self.camera_conditions {
            output.insert("camera_conditions".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanCameraImageToVideo";
    const DISPLAY_NAME: &'static str = "WanCameraImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanDancerEncodeAudio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanDancerEncodeAudio<
    AudioParam: crate::nodes::types::Audio,
    VideoFramesParam: crate::nodes::types::Int,
    AudioInjectScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**No documentation.

**Metadata**:
  - Default: 149
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub video_frames: VideoFramesParam,
    /**The scale for the audio features when injected into the video model.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub audio_inject_scale: AudioInjectScaleParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VideoFramesParam: crate::nodes::types::Int,
    AudioInjectScaleParam: crate::nodes::types::Float,
> WanDancerEncodeAudio<AudioParam, VideoFramesParam, AudioInjectScaleParam> {
    /// Create a new node.
    pub fn new(
        audio: AudioParam,
        video_frames: VideoFramesParam,
        audio_inject_scale: AudioInjectScaleParam,
    ) -> Self {
        Self {
            audio,
            video_frames,
            audio_inject_scale,
        }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VideoFramesParam: crate::nodes::types::Int,
    AudioInjectScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for WanDancerEncodeAudio<AudioParam, VideoFramesParam, AudioInjectScaleParam> {
    type Output = out::WanDancerEncodeAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            audio_encoder_output: crate::nodes::types::AudioEncoderOutputOut::from_dynamic(
                node_id,
                0u32,
            ),
            fps_string: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("video_frames".to_string(), self.video_frames.clone().into());
        output
            .insert(
                "audio_inject_scale".to_string(),
                self.audio_inject_scale.clone().into(),
            );
        output
    }
    const NAME: &'static str = "WanDancerEncodeAudio";
    const DISPLAY_NAME: &'static str = "WanDancerEncodeAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanDancerVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanDancerVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    ClipVisionOutputRefParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput
        = crate::nodes::types::AudioEncoderOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**The number of frames in the generated video. Should stay 149 for WanDancer.

**Metadata**:
  - Default: 149
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    ///The CLIP vision embeds for the first frame.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///The CLIP vision embeds for the reference image.
    pub clip_vision_output_ref: Option<ClipVisionOutputRefParam>,
    ///The initial image(s) to be encoded, can be any number of frames.
    pub start_image: Option<StartImageParam>,
    ///Image conditioning mask for the start image(s). White is kept, black is generated. Used for the local generations.
    pub mask: Option<MaskParam>,
    ///No documentation.
    pub audio_encoder_output: Option<AudioEncoderOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    ClipVisionOutputRefParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
> WanDancerVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    ClipVisionOutputParam,
    ClipVisionOutputRefParam,
    StartImageParam,
    MaskParam,
    AudioEncoderOutputParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        clip_vision_output_ref: Option<ClipVisionOutputRefParam>,
        start_image: Option<StartImageParam>,
        mask: Option<MaskParam>,
        audio_encoder_output: Option<AudioEncoderOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            clip_vision_output,
            clip_vision_output_ref,
            start_image,
            mask,
            audio_encoder_output,
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
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    ClipVisionOutputRefParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
> crate::nodes::TypedNode
for WanDancerVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    ClipVisionOutputParam,
    ClipVisionOutputRefParam,
    StartImageParam,
    MaskParam,
    AudioEncoderOutputParam,
> {
    type Output = out::WanDancerVideoOutput;
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
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output_ref {
            output.insert("clip_vision_output_ref".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio_encoder_output {
            output.insert("audio_encoder_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanDancerVideo";
    const DISPLAY_NAME: &'static str = "WanDancerVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanHuMoImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanHuMoImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput
        = crate::nodes::types::AudioEncoderOutputOut,
    RefImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
  - Default: 97
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
    pub audio_encoder_output: Option<AudioEncoderOutputParam>,
    ///No documentation.
    pub ref_image: Option<RefImageParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
> WanHuMoImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    AudioEncoderOutputParam,
    RefImageParam,
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
        audio_encoder_output: Option<AudioEncoderOutputParam>,
        ref_image: Option<RefImageParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            audio_encoder_output,
            ref_image,
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
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanHuMoImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    AudioEncoderOutputParam,
    RefImageParam,
> {
    type Output = out::WanHuMoImageToVideoOutput;
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
        if let Some(v) = &self.audio_encoder_output {
            output.insert("audio_encoder_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ref_image {
            output.insert("ref_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanHuMoImageToVideo";
    const DISPLAY_NAME: &'static str = "WanHuMoImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanInfiniteTalkToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanInfiniteTalkToVideo<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    AudioEncoderOutput1Param: crate::nodes::types::AudioEncoderOutput,
    MotionFrameCountParam: crate::nodes::types::Int,
    AudioScaleParam: crate::nodes::types::Float,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    PreviousFramesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub model_patch: ModelPatchParam,
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
    ///No documentation.
    pub audio_encoder_output_1: AudioEncoderOutput1Param,
    /**Number of previous frames to use as motion context.

**Metadata**:
  - Default: 9
  - Max: 33
  - Min: 1
  - Step: 1
*/
    pub motion_frame_count: MotionFrameCountParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub audio_scale: AudioScaleParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub previous_frames: Option<PreviousFramesParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    AudioEncoderOutput1Param: crate::nodes::types::AudioEncoderOutput,
    MotionFrameCountParam: crate::nodes::types::Int,
    AudioScaleParam: crate::nodes::types::Float,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    PreviousFramesParam: crate::nodes::types::Image,
> WanInfiniteTalkToVideo<
    ModelParam,
    ModelPatchParam,
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    AudioEncoderOutput1Param,
    MotionFrameCountParam,
    AudioScaleParam,
    ClipVisionOutputParam,
    StartImageParam,
    PreviousFramesParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        model_patch: ModelPatchParam,
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        audio_encoder_output_1: AudioEncoderOutput1Param,
        motion_frame_count: MotionFrameCountParam,
        audio_scale: AudioScaleParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
        start_image: Option<StartImageParam>,
        previous_frames: Option<PreviousFramesParam>,
    ) -> Self {
        Self {
            model,
            model_patch,
            positive,
            negative,
            vae,
            width,
            height,
            length,
            audio_encoder_output_1,
            motion_frame_count,
            audio_scale,
            clip_vision_output,
            start_image,
            previous_frames,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ModelPatchParam: crate::nodes::types::ModelPatch,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    AudioEncoderOutput1Param: crate::nodes::types::AudioEncoderOutput,
    MotionFrameCountParam: crate::nodes::types::Int,
    AudioScaleParam: crate::nodes::types::Float,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StartImageParam: crate::nodes::types::Image,
    PreviousFramesParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanInfiniteTalkToVideo<
    ModelParam,
    ModelPatchParam,
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    AudioEncoderOutput1Param,
    MotionFrameCountParam,
    AudioScaleParam,
    ClipVisionOutputParam,
    StartImageParam,
    PreviousFramesParam,
> {
    type Output = out::WanInfiniteTalkToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 2u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 3u32),
            trim_image: crate::nodes::types::IntOut::from_dynamic(node_id, 4u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("model_patch".to_string(), self.model_patch.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output
            .insert(
                "audio_encoder_output_1".to_string(),
                self.audio_encoder_output_1.clone().into(),
            );
        output
            .insert(
                "motion_frame_count".to_string(),
                self.motion_frame_count.clone().into(),
            );
        output.insert("audio_scale".to_string(), self.audio_scale.clone().into());
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.previous_frames {
            output.insert("previous_frames".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanInfiniteTalkToVideo";
    const DISPLAY_NAME: &'static str = "WanInfiniteTalkToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanMoveConcatTrack**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanMoveConcatTrack<
    Tracks1Param: crate::nodes::types::Tracks,
    Tracks2Param: crate::nodes::types::Tracks = crate::nodes::types::TracksOut,
> {
    ///No documentation.
    pub tracks_1: Tracks1Param,
    ///No documentation.
    pub tracks_2: Option<Tracks2Param>,
}
impl<
    Tracks1Param: crate::nodes::types::Tracks,
    Tracks2Param: crate::nodes::types::Tracks,
> WanMoveConcatTrack<Tracks1Param, Tracks2Param> {
    /// Create a new node.
    pub fn new(tracks_1: Tracks1Param, tracks_2: Option<Tracks2Param>) -> Self {
        Self { tracks_1, tracks_2 }
    }
}
impl<
    Tracks1Param: crate::nodes::types::Tracks,
    Tracks2Param: crate::nodes::types::Tracks,
> crate::nodes::TypedNode for WanMoveConcatTrack<Tracks1Param, Tracks2Param> {
    type Output = crate::nodes::types::TracksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("tracks_1".to_string(), self.tracks_1.clone().into());
        if let Some(v) = &self.tracks_2 {
            output.insert("tracks_2".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanMoveConcatTrack";
    const DISPLAY_NAME: &'static str = "WanMoveConcatTrack";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanMoveTrackToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanMoveTrackToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    TracksParam: crate::nodes::types::Tracks = crate::nodes::types::TracksOut,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**Strength of the track conditioning.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
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
    pub start_image: StartImageParam,
    ///No documentation.
    pub tracks: Option<TracksParam>,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    TracksParam: crate::nodes::types::Tracks,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> WanMoveTrackToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    StrengthParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    TracksParam,
    ClipVisionOutputParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        strength: StrengthParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        start_image: StartImageParam,
        tracks: Option<TracksParam>,
        clip_vision_output: Option<ClipVisionOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            strength,
            width,
            height,
            length,
            batch_size,
            start_image,
            tracks,
            clip_vision_output,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    StrengthParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    TracksParam: crate::nodes::types::Tracks,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for WanMoveTrackToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    StrengthParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    TracksParam,
    ClipVisionOutputParam,
> {
    type Output = out::WanMoveTrackToVideoOutput;
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
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("start_image".to_string(), self.start_image.clone().into());
        if let Some(v) = &self.tracks {
            output.insert("tracks".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanMoveTrackToVideo";
    const DISPLAY_NAME: &'static str = "WanMoveTrackToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanMoveTracksFromCoords**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanMoveTracksFromCoords<
    TrackCoordsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    TrackMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: []
*/
    pub track_coords: Option<TrackCoordsParam>,
    ///No documentation.
    pub track_mask: Option<TrackMaskParam>,
}
impl<
    TrackCoordsParam: crate::nodes::types::String,
    TrackMaskParam: crate::nodes::types::Mask,
> WanMoveTracksFromCoords<TrackCoordsParam, TrackMaskParam> {
    /// Create a new node.
    pub fn new(
        track_coords: Option<TrackCoordsParam>,
        track_mask: Option<TrackMaskParam>,
    ) -> Self {
        Self { track_coords, track_mask }
    }
}
impl<
    TrackCoordsParam: crate::nodes::types::String,
    TrackMaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode for WanMoveTracksFromCoords<TrackCoordsParam, TrackMaskParam> {
    type Output = out::WanMoveTracksFromCoordsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            tracks: crate::nodes::types::TracksOut::from_dynamic(node_id, 0u32),
            track_length: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.track_coords {
            output.insert("track_coords".to_string(), v.clone().into());
        }
        if let Some(v) = &self.track_mask {
            output.insert("track_mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanMoveTracksFromCoords";
    const DISPLAY_NAME: &'static str = "WanMoveTracksFromCoords";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanMoveVisualizeTracks**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanMoveVisualizeTracks<
    ImagesParam: crate::nodes::types::Image,
    LineResolutionParam: crate::nodes::types::Int,
    CircleSizeParam: crate::nodes::types::Int,
    OpacityParam: crate::nodes::types::Float,
    LineWidthParam: crate::nodes::types::Int,
    TracksParam: crate::nodes::types::Tracks = crate::nodes::types::TracksOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 24
  - Max: 1024
  - Min: 1
*/
    pub line_resolution: LineResolutionParam,
    /**No documentation.

**Metadata**:
  - Default: 12
  - Max: 128
  - Min: 1
*/
    pub circle_size: CircleSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0.75
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub opacity: OpacityParam,
    /**No documentation.

**Metadata**:
  - Default: 16
  - Max: 128
  - Min: 1
*/
    pub line_width: LineWidthParam,
    ///No documentation.
    pub tracks: Option<TracksParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LineResolutionParam: crate::nodes::types::Int,
    CircleSizeParam: crate::nodes::types::Int,
    OpacityParam: crate::nodes::types::Float,
    LineWidthParam: crate::nodes::types::Int,
    TracksParam: crate::nodes::types::Tracks,
> WanMoveVisualizeTracks<
    ImagesParam,
    LineResolutionParam,
    CircleSizeParam,
    OpacityParam,
    LineWidthParam,
    TracksParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        line_resolution: LineResolutionParam,
        circle_size: CircleSizeParam,
        opacity: OpacityParam,
        line_width: LineWidthParam,
        tracks: Option<TracksParam>,
    ) -> Self {
        Self {
            images,
            line_resolution,
            circle_size,
            opacity,
            line_width,
            tracks,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LineResolutionParam: crate::nodes::types::Int,
    CircleSizeParam: crate::nodes::types::Int,
    OpacityParam: crate::nodes::types::Float,
    LineWidthParam: crate::nodes::types::Int,
    TracksParam: crate::nodes::types::Tracks,
> crate::nodes::TypedNode
for WanMoveVisualizeTracks<
    ImagesParam,
    LineResolutionParam,
    CircleSizeParam,
    OpacityParam,
    LineWidthParam,
    TracksParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("line_resolution".to_string(), self.line_resolution.clone().into());
        output.insert("circle_size".to_string(), self.circle_size.clone().into());
        output.insert("opacity".to_string(), self.opacity.clone().into());
        output.insert("line_width".to_string(), self.line_width.clone().into());
        if let Some(v) = &self.tracks {
            output.insert("tracks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanMoveVisualizeTracks";
    const DISPLAY_NAME: &'static str = "WanMoveVisualizeTracks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanPhantomSubjectToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanPhantomSubjectToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
    pub images: Option<ImagesParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    ImagesParam: crate::nodes::types::Image,
> WanPhantomSubjectToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ImagesParam,
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
        images: Option<ImagesParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            images,
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
    ImagesParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanPhantomSubjectToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    ImagesParam,
> {
    type Output = out::WanPhantomSubjectToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative_text: crate::nodes::types::ConditioningOut::from_dynamic(
                node_id,
                1u32,
            ),
            negative_img_text: crate::nodes::types::ConditioningOut::from_dynamic(
                node_id,
                2u32,
            ),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 3u32),
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
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanPhantomSubjectToVideo";
    const DISPLAY_NAME: &'static str = "WanPhantomSubjectToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanSCAILToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanSCAILToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    PoseStrengthParam: crate::nodes::types::Float,
    PoseStartParam: crate::nodes::types::Float,
    PoseEndParam: crate::nodes::types::Float,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    PreviousFrameCountParam: crate::nodes::types::Int,
    PoseVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    PoseVideoMaskParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ReplacementModeParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ReferenceImageMaskParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
    PreviousFramesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 32
  - Step: 32
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 896
  - Max: 16384
  - Min: 32
  - Step: 32
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
    /**Strength of the pose latent.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub pose_strength: PoseStrengthParam,
    /**Start step of the pose conditioning.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pose_start: PoseStartParam,
    /**End step of the pose conditioning.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pose_end: PoseEndParam,
    /**Cumulative output frame this chunk begins at. Wire from the previous chunk's video_frame_offset output.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub video_frame_offset: VideoFrameOffsetParam,
    /**Tail frames of previous_frames to anchor. SCAIL-2 trained at 5 (81-frame chunks, 76-frame step).

**Metadata**:
  - Default: 5
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub previous_frame_count: PreviousFrameCountParam,
    ///Video used for pose conditioning. Will be downscaled to half the resolution of the main video.
    pub pose_video: Option<PoseVideoParam>,
    ///SCAIL-2 only. Colored per-identity SAM3 mask video at the same resolution as pose_video.
    pub pose_video_mask: Option<PoseVideoMaskParam>,
    /**SCAIL-2 only. False = Animation Mode (pose_video_mask should have black background). True = Replacement Mode (pose_video_mask should have white background).

**Metadata**:
  - Default: false
*/
    pub replacement_mode: Option<ReplacementModeParam>,
    ///Reference image, for multiple references composite all on single image.
    pub reference_image: Option<ReferenceImageParam>,
    ///SCAIL-2 only. Colored reference mask at the same resolution as reference_image.
    pub reference_image_mask: Option<ReferenceImageMaskParam>,
    ///CLIP vision features for conditioning. Model is trained with stretch resize to aspect ratio.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
    ///SCAIL-2 only. Full decoded output of the previous chunk. Only the last previous_frame_count are used as the extension anchor.
    pub previous_frames: Option<PreviousFramesParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    PoseStrengthParam: crate::nodes::types::Float,
    PoseStartParam: crate::nodes::types::Float,
    PoseEndParam: crate::nodes::types::Float,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    PreviousFrameCountParam: crate::nodes::types::Int,
    PoseVideoParam: crate::nodes::types::Image,
    PoseVideoMaskParam: crate::nodes::types::Image,
    ReplacementModeParam: crate::nodes::types::Boolean,
    ReferenceImageParam: crate::nodes::types::Image,
    ReferenceImageMaskParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    PreviousFramesParam: crate::nodes::types::Image,
> WanSCAILToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    PoseStrengthParam,
    PoseStartParam,
    PoseEndParam,
    VideoFrameOffsetParam,
    PreviousFrameCountParam,
    PoseVideoParam,
    PoseVideoMaskParam,
    ReplacementModeParam,
    ReferenceImageParam,
    ReferenceImageMaskParam,
    ClipVisionOutputParam,
    PreviousFramesParam,
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
        pose_strength: PoseStrengthParam,
        pose_start: PoseStartParam,
        pose_end: PoseEndParam,
        video_frame_offset: VideoFrameOffsetParam,
        previous_frame_count: PreviousFrameCountParam,
        pose_video: Option<PoseVideoParam>,
        pose_video_mask: Option<PoseVideoMaskParam>,
        replacement_mode: Option<ReplacementModeParam>,
        reference_image: Option<ReferenceImageParam>,
        reference_image_mask: Option<ReferenceImageMaskParam>,
        clip_vision_output: Option<ClipVisionOutputParam>,
        previous_frames: Option<PreviousFramesParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            pose_strength,
            pose_start,
            pose_end,
            video_frame_offset,
            previous_frame_count,
            pose_video,
            pose_video_mask,
            replacement_mode,
            reference_image,
            reference_image_mask,
            clip_vision_output,
            previous_frames,
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
    PoseStrengthParam: crate::nodes::types::Float,
    PoseStartParam: crate::nodes::types::Float,
    PoseEndParam: crate::nodes::types::Float,
    VideoFrameOffsetParam: crate::nodes::types::Int,
    PreviousFrameCountParam: crate::nodes::types::Int,
    PoseVideoParam: crate::nodes::types::Image,
    PoseVideoMaskParam: crate::nodes::types::Image,
    ReplacementModeParam: crate::nodes::types::Boolean,
    ReferenceImageParam: crate::nodes::types::Image,
    ReferenceImageMaskParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    PreviousFramesParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanSCAILToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    PoseStrengthParam,
    PoseStartParam,
    PoseEndParam,
    VideoFrameOffsetParam,
    PreviousFrameCountParam,
    PoseVideoParam,
    PoseVideoMaskParam,
    ReplacementModeParam,
    ReferenceImageParam,
    ReferenceImageMaskParam,
    ClipVisionOutputParam,
    PreviousFramesParam,
> {
    type Output = out::WanSCAILToVideoOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
            video_frame_offset: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
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
        output.insert("pose_strength".to_string(), self.pose_strength.clone().into());
        output.insert("pose_start".to_string(), self.pose_start.clone().into());
        output.insert("pose_end".to_string(), self.pose_end.clone().into());
        output
            .insert(
                "video_frame_offset".to_string(),
                self.video_frame_offset.clone().into(),
            );
        output
            .insert(
                "previous_frame_count".to_string(),
                self.previous_frame_count.clone().into(),
            );
        if let Some(v) = &self.pose_video {
            output.insert("pose_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pose_video_mask {
            output.insert("pose_video_mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.replacement_mode {
            output.insert("replacement_mode".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_image_mask {
            output.insert("reference_image_mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.previous_frames {
            output.insert("previous_frames".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanSCAILToVideo";
    const DISPLAY_NAME: &'static str = "WanSCAILToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanSoundImageToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanSoundImageToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput
        = crate::nodes::types::AudioEncoderOutputOut,
    RefImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ControlVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    RefMotionParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
  - Default: 77
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
    pub audio_encoder_output: Option<AudioEncoderOutputParam>,
    ///No documentation.
    pub ref_image: Option<RefImageParam>,
    ///No documentation.
    pub control_video: Option<ControlVideoParam>,
    ///No documentation.
    pub ref_motion: Option<RefMotionParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
    RefMotionParam: crate::nodes::types::Image,
> WanSoundImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    AudioEncoderOutputParam,
    RefImageParam,
    ControlVideoParam,
    RefMotionParam,
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
        audio_encoder_output: Option<AudioEncoderOutputParam>,
        ref_image: Option<RefImageParam>,
        control_video: Option<ControlVideoParam>,
        ref_motion: Option<RefMotionParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            audio_encoder_output,
            ref_image,
            control_video,
            ref_motion,
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
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
    RefMotionParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanSoundImageToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    AudioEncoderOutputParam,
    RefImageParam,
    ControlVideoParam,
    RefMotionParam,
> {
    type Output = out::WanSoundImageToVideoOutput;
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
        if let Some(v) = &self.audio_encoder_output {
            output.insert("audio_encoder_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ref_image {
            output.insert("ref_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.control_video {
            output.insert("control_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ref_motion {
            output.insert("ref_motion".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanSoundImageToVideo";
    const DISPLAY_NAME: &'static str = "WanSoundImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanSoundImageToVideoExtend**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanSoundImageToVideoExtend<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LengthParam: crate::nodes::types::Int,
    VideoLatentParam: crate::nodes::types::Latent,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput
        = crate::nodes::types::AudioEncoderOutputOut,
    RefImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
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
  - Default: 77
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    ///No documentation.
    pub video_latent: VideoLatentParam,
    ///No documentation.
    pub audio_encoder_output: Option<AudioEncoderOutputParam>,
    ///No documentation.
    pub ref_image: Option<RefImageParam>,
    ///No documentation.
    pub control_video: Option<ControlVideoParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LengthParam: crate::nodes::types::Int,
    VideoLatentParam: crate::nodes::types::Latent,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> WanSoundImageToVideoExtend<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LengthParam,
    VideoLatentParam,
    AudioEncoderOutputParam,
    RefImageParam,
    ControlVideoParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        length: LengthParam,
        video_latent: VideoLatentParam,
        audio_encoder_output: Option<AudioEncoderOutputParam>,
        ref_image: Option<RefImageParam>,
        control_video: Option<ControlVideoParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            length,
            video_latent,
            audio_encoder_output,
            ref_image,
            control_video,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    LengthParam: crate::nodes::types::Int,
    VideoLatentParam: crate::nodes::types::Latent,
    AudioEncoderOutputParam: crate::nodes::types::AudioEncoderOutput,
    RefImageParam: crate::nodes::types::Image,
    ControlVideoParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for WanSoundImageToVideoExtend<
    PositiveParam,
    NegativeParam,
    VaeParam,
    LengthParam,
    VideoLatentParam,
    AudioEncoderOutputParam,
    RefImageParam,
    ControlVideoParam,
> {
    type Output = out::WanSoundImageToVideoExtendOutput;
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
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("video_latent".to_string(), self.video_latent.clone().into());
        if let Some(v) = &self.audio_encoder_output {
            output.insert("audio_encoder_output".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ref_image {
            output.insert("ref_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.control_video {
            output.insert("control_video".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanSoundImageToVideoExtend";
    const DISPLAY_NAME: &'static str = "WanSoundImageToVideoExtend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
///**WanTrackToVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanTrackToVideo<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    TracksParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    TemperatureParam: crate::nodes::types::Float,
    TopkParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput
        = crate::nodes::types::ClipVisionOutputOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default: []
*/
    pub tracks: TracksParam,
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
  - Default: 220
  - Max: 1000
  - Min: 1
  - Step: 0.1
*/
    pub temperature: TemperatureParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 10
  - Min: 1
*/
    pub topk: TopkParam,
    ///No documentation.
    pub start_image: StartImageParam,
    ///No documentation.
    pub clip_vision_output: Option<ClipVisionOutputParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    TracksParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    TemperatureParam: crate::nodes::types::Float,
    TopkParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> WanTrackToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    TracksParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    TemperatureParam,
    TopkParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        tracks: TracksParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        temperature: TemperatureParam,
        topk: TopkParam,
        start_image: StartImageParam,
        clip_vision_output: Option<ClipVisionOutputParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            tracks,
            width,
            height,
            length,
            batch_size,
            temperature,
            topk,
            start_image,
            clip_vision_output,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    TracksParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    TemperatureParam: crate::nodes::types::Float,
    TopkParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
> crate::nodes::TypedNode
for WanTrackToVideo<
    PositiveParam,
    NegativeParam,
    VaeParam,
    TracksParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    TemperatureParam,
    TopkParam,
    StartImageParam,
    ClipVisionOutputParam,
> {
    type Output = out::WanTrackToVideoOutput;
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
        output.insert("tracks".to_string(), self.tracks.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output.insert("temperature".to_string(), self.temperature.clone().into());
        output.insert("topk".to_string(), self.topk.clone().into());
        output.insert("start_image".to_string(), self.start_image.clone().into());
        if let Some(v) = &self.clip_vision_output {
            output.insert("clip_vision_output".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanTrackToVideo";
    const DISPLAY_NAME: &'static str = "WanTrackToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/conditioning/video_models";
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
    const CATEGORY: &'static str = "model/conditioning/video_models";
}
