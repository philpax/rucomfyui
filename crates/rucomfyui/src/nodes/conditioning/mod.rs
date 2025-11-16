//!`conditioning` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod n_3_d_models;
pub mod controlnet;
pub mod gligen;
pub mod inpaint;
pub mod instructpix_2_pix;
pub mod lotus;
pub mod stable_cascade;
pub mod style_model;
pub mod upscale_diffusion;
pub mod video_models;
/// Output types for nodes.
pub mod out {
    ///Output for [`ConditioningStableAudio`](super::ConditioningStableAudio).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ConditioningStableAudioOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**AudioEncoderEncode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioEncoderEncode<
    AudioEncoderParam: crate::nodes::types::AudioEncoder,
    AudioParam: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub audio_encoder: AudioEncoderParam,
    ///No documentation.
    pub audio: AudioParam,
}
impl<
    AudioEncoderParam: crate::nodes::types::AudioEncoder,
    AudioParam: crate::nodes::types::Audio,
> AudioEncoderEncode<AudioEncoderParam, AudioParam> {
    /// Create a new node.
    pub fn new(audio_encoder: AudioEncoderParam, audio: AudioParam) -> Self {
        Self { audio_encoder, audio }
    }
}
impl<
    AudioEncoderParam: crate::nodes::types::AudioEncoder,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode for AudioEncoderEncode<AudioEncoderParam, AudioParam> {
    type Output = crate::nodes::types::AudioEncoderOutputOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio_encoder".to_string(), self.audio_encoder.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "AudioEncoderEncode";
    const DISPLAY_NAME: &'static str = "AudioEncoderEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Set Last Layer**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPSetLastLayer<
    ClipParam: crate::nodes::types::Clip,
    StopAtClipLayerParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: -1
  - Min: -24
  - Step: 1
*/
    pub stop_at_clip_layer: StopAtClipLayerParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    StopAtClipLayerParam: crate::nodes::types::Int,
> CLIPSetLastLayer<ClipParam, StopAtClipLayerParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam, stop_at_clip_layer: StopAtClipLayerParam) -> Self {
        Self { clip, stop_at_clip_layer }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    StopAtClipLayerParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for CLIPSetLastLayer<ClipParam, StopAtClipLayerParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output
            .insert(
                "stop_at_clip_layer".to_string(),
                self.stop_at_clip_layer.clone().into(),
            );
        output
    }
    const NAME: &'static str = "CLIPSetLastLayer";
    const DISPLAY_NAME: &'static str = "CLIP Set Last Layer";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Text Encode (Prompt)**: Encodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPTextEncode<
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> {
    /**The text to be encoded.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: TextParam,
    ///The CLIP model used for encoding the text.
    pub clip: ClipParam,
}
impl<
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> CLIPTextEncode<TextParam, ClipParam> {
    /// Create a new node.
    pub fn new(text: TextParam, clip: ClipParam) -> Self {
        Self { text, clip }
    }
}
impl<
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> crate::nodes::TypedNode for CLIPTextEncode<TextParam, ClipParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncode";
    const DISPLAY_NAME: &'static str = "CLIP Text Encode (Prompt)";
    const DESCRIPTION: &'static str = "Encodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Text Encode for Lumina2**: Encodes a system prompt and a user prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPTextEncodeLumina2<
    UserPromptParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> {
    /**The text to be encoded.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub user_prompt: UserPromptParam,
    ///The CLIP model used for encoding the text.
    pub clip: ClipParam,
}
impl<
    UserPromptParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> CLIPTextEncodeLumina2<UserPromptParam, ClipParam> {
    /// Create a new node.
    pub fn new(user_prompt: UserPromptParam, clip: ClipParam) -> Self {
        Self { user_prompt, clip }
    }
}
impl<
    UserPromptParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> crate::nodes::TypedNode for CLIPTextEncodeLumina2<UserPromptParam, ClipParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("user_prompt".to_string(), self.user_prompt.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeLumina2";
    const DISPLAY_NAME: &'static str = "CLIP Text Encode for Lumina2";
    const DESCRIPTION: &'static str = "Encodes a system prompt and a user prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Vision Encode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPVisionEncode<
    ClipVisionParam: crate::nodes::types::ClipVision,
    ImageParam: crate::nodes::types::Image,
    CropParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub crop: CropParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    ImageParam: crate::nodes::types::Image,
    CropParam: crate::nodes::types::String,
> CLIPVisionEncode<ClipVisionParam, ImageParam, CropParam> {
    /// Create a new node.
    pub fn new(
        clip_vision: ClipVisionParam,
        image: ImageParam,
        crop: CropParam,
    ) -> Self {
        Self { clip_vision, image, crop }
    }
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    ImageParam: crate::nodes::types::Image,
    CropParam: crate::nodes::types::String,
> crate::nodes::TypedNode for CLIPVisionEncode<ClipVisionParam, ImageParam, CropParam> {
    type Output = crate::nodes::types::ClipVisionOutputOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("crop".to_string(), self.crop.clone().into());
        output
    }
    const NAME: &'static str = "CLIPVisionEncode";
    const DISPLAY_NAME: &'static str = "CLIP Vision Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningAverage**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningAverage<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
    ConditioningToStrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning_to: ConditioningToParam,
    ///No documentation.
    pub conditioning_from: ConditioningFromParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub conditioning_to_strength: ConditioningToStrengthParam,
}
impl<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
    ConditioningToStrengthParam: crate::nodes::types::Float,
> ConditioningAverage<
    ConditioningToParam,
    ConditioningFromParam,
    ConditioningToStrengthParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningToParam,
        conditioning_from: ConditioningFromParam,
        conditioning_to_strength: ConditioningToStrengthParam,
    ) -> Self {
        Self {
            conditioning_to,
            conditioning_from,
            conditioning_to_strength,
        }
    }
}
impl<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
    ConditioningToStrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningAverage<
    ConditioningToParam,
    ConditioningFromParam,
    ConditioningToStrengthParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("conditioning_to".to_string(), self.conditioning_to.clone().into());
        output
            .insert(
                "conditioning_from".to_string(),
                self.conditioning_from.clone().into(),
            );
        output
            .insert(
                "conditioning_to_strength".to_string(),
                self.conditioning_to_strength.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ConditioningAverage";
    const DISPLAY_NAME: &'static str = "ConditioningAverage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Combine)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningCombine<
    Conditioning1Param: crate::nodes::types::Conditioning,
    Conditioning2Param: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub conditioning_1: Conditioning1Param,
    ///No documentation.
    pub conditioning_2: Conditioning2Param,
}
impl<
    Conditioning1Param: crate::nodes::types::Conditioning,
    Conditioning2Param: crate::nodes::types::Conditioning,
> ConditioningCombine<Conditioning1Param, Conditioning2Param> {
    /// Create a new node.
    pub fn new(
        conditioning_1: Conditioning1Param,
        conditioning_2: Conditioning2Param,
    ) -> Self {
        Self {
            conditioning_1,
            conditioning_2,
        }
    }
}
impl<
    Conditioning1Param: crate::nodes::types::Conditioning,
    Conditioning2Param: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for ConditioningCombine<Conditioning1Param, Conditioning2Param> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning_1".to_string(), self.conditioning_1.clone().into());
        output.insert("conditioning_2".to_string(), self.conditioning_2.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningCombine";
    const DISPLAY_NAME: &'static str = "Conditioning (Combine)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Concat)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningConcat<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub conditioning_to: ConditioningToParam,
    ///No documentation.
    pub conditioning_from: ConditioningFromParam,
}
impl<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
> ConditioningConcat<ConditioningToParam, ConditioningFromParam> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningToParam,
        conditioning_from: ConditioningFromParam,
    ) -> Self {
        Self {
            conditioning_to,
            conditioning_from,
        }
    }
}
impl<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ConditioningFromParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for ConditioningConcat<ConditioningToParam, ConditioningFromParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("conditioning_to".to_string(), self.conditioning_to.clone().into());
        output
            .insert(
                "conditioning_from".to_string(),
                self.conditioning_from.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ConditioningConcat";
    const DISPLAY_NAME: &'static str = "Conditioning (Concat)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningSetArea<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> ConditioningSetArea<
    ConditioningParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            conditioning,
            width,
            height,
            x,
            y,
            strength,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetArea<
    ConditioningParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
    StrengthParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetArea";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area with Percentage)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningSetAreaPercentage<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> ConditioningSetAreaPercentage<
    ConditioningParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            conditioning,
            width,
            height,
            x,
            y,
            strength,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetAreaPercentage<
    ConditioningParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
    StrengthParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetAreaPercentage";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area with Percentage)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningSetAreaPercentageVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningSetAreaPercentageVideo<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    TemporalParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    ZParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub temporal: TemporalParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub z: ZParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    TemporalParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    ZParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> ConditioningSetAreaPercentageVideo<
    ConditioningParam,
    WidthParam,
    HeightParam,
    TemporalParam,
    XParam,
    YParam,
    ZParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        width: WidthParam,
        height: HeightParam,
        temporal: TemporalParam,
        x: XParam,
        y: YParam,
        z: ZParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            conditioning,
            width,
            height,
            temporal,
            x,
            y,
            z,
            strength,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    WidthParam: crate::nodes::types::Float,
    HeightParam: crate::nodes::types::Float,
    TemporalParam: crate::nodes::types::Float,
    XParam: crate::nodes::types::Float,
    YParam: crate::nodes::types::Float,
    ZParam: crate::nodes::types::Float,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetAreaPercentageVideo<
    ConditioningParam,
    WidthParam,
    HeightParam,
    TemporalParam,
    XParam,
    YParam,
    ZParam,
    StrengthParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("temporal".to_string(), self.temporal.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("z".to_string(), self.z.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetAreaPercentageVideo";
    const DISPLAY_NAME: &'static str = "ConditioningSetAreaPercentageVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningSetAreaStrength**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningSetAreaStrength<
    ConditioningParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
> ConditioningSetAreaStrength<ConditioningParam, StrengthParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam, strength: StrengthParam) -> Self {
        Self { conditioning, strength }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetAreaStrength<ConditioningParam, StrengthParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetAreaStrength";
    const DISPLAY_NAME: &'static str = "ConditioningSetAreaStrength";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Mask)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningSetMask<
    ConditioningParam: crate::nodes::types::Conditioning,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    ///No documentation.
    pub set_cond_area: SetCondAreaParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
> ConditioningSetMask<ConditioningParam, MaskParam, StrengthParam, SetCondAreaParam> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        mask: MaskParam,
        strength: StrengthParam,
        set_cond_area: SetCondAreaParam,
    ) -> Self {
        Self {
            conditioning,
            mask,
            strength,
            set_cond_area,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    MaskParam: crate::nodes::types::Mask,
    StrengthParam: crate::nodes::types::Float,
    SetCondAreaParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ConditioningSetMask<ConditioningParam, MaskParam, StrengthParam, SetCondAreaParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("set_cond_area".to_string(), self.set_cond_area.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetMask";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningStableAudio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ConditioningStableAudio<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SecondsStartParam: crate::nodes::types::Float,
    SecondsTotalParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1000
  - Min: 0
  - Step: 0.1
*/
    pub seconds_start: SecondsStartParam,
    /**No documentation.

**Metadata**:
  - Default: 47
  - Max: 1000
  - Min: 0
  - Step: 0.1
*/
    pub seconds_total: SecondsTotalParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SecondsStartParam: crate::nodes::types::Float,
    SecondsTotalParam: crate::nodes::types::Float,
> ConditioningStableAudio<
    PositiveParam,
    NegativeParam,
    SecondsStartParam,
    SecondsTotalParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        seconds_start: SecondsStartParam,
        seconds_total: SecondsTotalParam,
    ) -> Self {
        Self {
            positive,
            negative,
            seconds_start,
            seconds_total,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    SecondsStartParam: crate::nodes::types::Float,
    SecondsTotalParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningStableAudio<
    PositiveParam,
    NegativeParam,
    SecondsStartParam,
    SecondsTotalParam,
> {
    type Output = out::ConditioningStableAudioOutput;
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
        output.insert("seconds_start".to_string(), self.seconds_start.clone().into());
        output.insert("seconds_total".to_string(), self.seconds_total.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningStableAudio";
    const DISPLAY_NAME: &'static str = "ConditioningStableAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**TextEncodeAceStepAudio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TextEncodeAceStepAudio<
    ClipParam: crate::nodes::types::Clip,
    TagsParam: crate::nodes::types::String,
    LyricsParam: crate::nodes::types::String,
    LyricsStrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub tags: TagsParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub lyrics: LyricsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub lyrics_strength: LyricsStrengthParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    TagsParam: crate::nodes::types::String,
    LyricsParam: crate::nodes::types::String,
    LyricsStrengthParam: crate::nodes::types::Float,
> TextEncodeAceStepAudio<ClipParam, TagsParam, LyricsParam, LyricsStrengthParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        tags: TagsParam,
        lyrics: LyricsParam,
        lyrics_strength: LyricsStrengthParam,
    ) -> Self {
        Self {
            clip,
            tags,
            lyrics,
            lyrics_strength,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    TagsParam: crate::nodes::types::String,
    LyricsParam: crate::nodes::types::String,
    LyricsStrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for TextEncodeAceStepAudio<ClipParam, TagsParam, LyricsParam, LyricsStrengthParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("tags".to_string(), self.tags.clone().into());
        output.insert("lyrics".to_string(), self.lyrics.clone().into());
        output
            .insert("lyrics_strength".to_string(), self.lyrics_strength.clone().into());
        output
    }
    const NAME: &'static str = "TextEncodeAceStepAudio";
    const DISPLAY_NAME: &'static str = "TextEncodeAceStepAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**unCLIPConditioning**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct unCLIPConditioning<
    ConditioningParam: crate::nodes::types::Conditioning,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StrengthParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutputParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentationParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StrengthParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> unCLIPConditioning<
    ConditioningParam,
    ClipVisionOutputParam,
    StrengthParam,
    NoiseAugmentationParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        clip_vision_output: ClipVisionOutputParam,
        strength: StrengthParam,
        noise_augmentation: NoiseAugmentationParam,
    ) -> Self {
        Self {
            conditioning,
            clip_vision_output,
            strength,
            noise_augmentation,
        }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    StrengthParam: crate::nodes::types::Float,
    NoiseAugmentationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for unCLIPConditioning<
    ConditioningParam,
    ClipVisionOutputParam,
    StrengthParam,
    NoiseAugmentationParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.clone().into(),
            );
        output.insert("strength".to_string(), self.strength.clone().into());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.clone().into(),
            );
        output
    }
    const NAME: &'static str = "unCLIPConditioning";
    const DISPLAY_NAME: &'static str = "unCLIPConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
