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
pub mod stable_cascade;
pub mod style_model;
pub mod upscale_diffusion;
pub mod video_models;
/// Output types for nodes.
pub mod out {
    ///Output for [`ConditioningStableAudio`](super::ConditioningStableAudio).
    #[derive(Clone)]
    pub struct ConditioningStableAudioOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**CLIP Set Last Layer**: No description.
#[derive(Clone)]
pub struct ClipSetLastLayer<
    Clip: crate::nodes::types::Clip,
    StopAtClipLayer: crate::nodes::types::Int,
> {
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: -1
  - Min: -24
  - Step: 1
*/
    pub stop_at_clip_layer: StopAtClipLayer,
}
impl<
    Clip: crate::nodes::types::Clip,
    StopAtClipLayer: crate::nodes::types::Int,
> ClipSetLastLayer<Clip, StopAtClipLayer> {
    /// Create a new node.
    pub fn new(clip: Clip, stop_at_clip_layer: StopAtClipLayer) -> Self {
        Self { clip, stop_at_clip_layer }
    }
}
impl<
    Clip: crate::nodes::types::Clip,
    StopAtClipLayer: crate::nodes::types::Int,
> crate::nodes::TypedNode for ClipSetLastLayer<Clip, StopAtClipLayer> {
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
pub struct ClipTextEncode<
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> {
    /**The text to be encoded.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: Text,
    ///The CLIP model used for encoding the text.
    pub clip: Clip,
}
impl<
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> ClipTextEncode<Text, Clip> {
    /// Create a new node.
    pub fn new(text: Text, clip: Clip) -> Self {
        Self { text, clip }
    }
}
impl<
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> crate::nodes::TypedNode for ClipTextEncode<Text, Clip> {
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
///**CLIP Vision Encode**: No description.
#[derive(Clone)]
pub struct ClipVisionEncode<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
    Crop: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip_vision: ClipVision,
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub crop: Crop,
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
    Crop: crate::nodes::types::String,
> ClipVisionEncode<ClipVision, Image, Crop> {
    /// Create a new node.
    pub fn new(clip_vision: ClipVision, image: Image, crop: Crop) -> Self {
        Self { clip_vision, image, crop }
    }
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
    Crop: crate::nodes::types::String,
> crate::nodes::TypedNode for ClipVisionEncode<ClipVision, Image, Crop> {
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
pub struct ConditioningAverage<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
    ConditioningToStrength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub conditioning_from: ConditioningFrom,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub conditioning_to_strength: ConditioningToStrength,
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
    ConditioningToStrength: crate::nodes::types::Float,
> ConditioningAverage<ConditioningTo, ConditioningFrom, ConditioningToStrength> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningTo,
        conditioning_from: ConditioningFrom,
        conditioning_to_strength: ConditioningToStrength,
    ) -> Self {
        Self {
            conditioning_to,
            conditioning_from,
            conditioning_to_strength,
        }
    }
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
    ConditioningToStrength: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningAverage<ConditioningTo, ConditioningFrom, ConditioningToStrength> {
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
pub struct ConditioningCombine<
    Conditioning1: crate::nodes::types::Conditioning,
    Conditioning2: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub conditioning_1: Conditioning1,
    ///No documentation.
    pub conditioning_2: Conditioning2,
}
impl<
    Conditioning1: crate::nodes::types::Conditioning,
    Conditioning2: crate::nodes::types::Conditioning,
> ConditioningCombine<Conditioning1, Conditioning2> {
    /// Create a new node.
    pub fn new(conditioning_1: Conditioning1, conditioning_2: Conditioning2) -> Self {
        Self {
            conditioning_1,
            conditioning_2,
        }
    }
}
impl<
    Conditioning1: crate::nodes::types::Conditioning,
    Conditioning2: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for ConditioningCombine<Conditioning1, Conditioning2> {
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
pub struct ConditioningConcat<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub conditioning_from: ConditioningFrom,
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
> ConditioningConcat<ConditioningTo, ConditioningFrom> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningTo,
        conditioning_from: ConditioningFrom,
    ) -> Self {
        Self {
            conditioning_to,
            conditioning_from,
        }
    }
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for ConditioningConcat<ConditioningTo, ConditioningFrom> {
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
pub struct ConditioningSetArea<
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: X,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: Y,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> ConditioningSetArea<Conditioning, Width, Height, X, Y, Strength> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        width: Width,
        height: Height,
        x: X,
        y: Y,
        strength: Strength,
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
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetArea<Conditioning, Width, Height, X, Y, Strength> {
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
pub struct ConditioningSetAreaPercentage<
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Float,
    Height: crate::nodes::types::Float,
    X: crate::nodes::types::Float,
    Y: crate::nodes::types::Float,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x: X,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y: Y,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Float,
    Height: crate::nodes::types::Float,
    X: crate::nodes::types::Float,
    Y: crate::nodes::types::Float,
    Strength: crate::nodes::types::Float,
> ConditioningSetAreaPercentage<Conditioning, Width, Height, X, Y, Strength> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        width: Width,
        height: Height,
        x: X,
        y: Y,
        strength: Strength,
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
    Conditioning: crate::nodes::types::Conditioning,
    Width: crate::nodes::types::Float,
    Height: crate::nodes::types::Float,
    X: crate::nodes::types::Float,
    Y: crate::nodes::types::Float,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetAreaPercentage<Conditioning, Width, Height, X, Y, Strength> {
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
///**ConditioningSetAreaStrength**: No description.
#[derive(Clone)]
pub struct ConditioningSetAreaStrength<
    Conditioning: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
> ConditioningSetAreaStrength<Conditioning, Strength> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning, strength: Strength) -> Self {
        Self { conditioning, strength }
    }
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode for ConditioningSetAreaStrength<Conditioning, Strength> {
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
pub struct ConditioningSetMask<
    Conditioning: crate::nodes::types::Conditioning,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub mask: Mask,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: Strength,
    ///No documentation.
    pub set_cond_area: SetCondArea,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
> ConditioningSetMask<Conditioning, Mask, Strength, SetCondArea> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        mask: Mask,
        strength: Strength,
        set_cond_area: SetCondArea,
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
    Conditioning: crate::nodes::types::Conditioning,
    Mask: crate::nodes::types::Mask,
    Strength: crate::nodes::types::Float,
    SetCondArea: crate::nodes::types::String,
> crate::nodes::TypedNode
for ConditioningSetMask<Conditioning, Mask, Strength, SetCondArea> {
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
pub struct ConditioningStableAudio<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    SecondsStart: crate::nodes::types::Float,
    SecondsTotal: crate::nodes::types::Float,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1000
  - Min: 0
  - Step: 0.1
*/
    pub seconds_start: SecondsStart,
    /**No documentation.

**Metadata**:
  - Default: 47
  - Max: 1000
  - Min: 0
  - Step: 0.1
*/
    pub seconds_total: SecondsTotal,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    SecondsStart: crate::nodes::types::Float,
    SecondsTotal: crate::nodes::types::Float,
> ConditioningStableAudio<Positive, Negative, SecondsStart, SecondsTotal> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        seconds_start: SecondsStart,
        seconds_total: SecondsTotal,
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
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    SecondsStart: crate::nodes::types::Float,
    SecondsTotal: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningStableAudio<Positive, Negative, SecondsStart, SecondsTotal> {
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
///**unCLIPConditioning**: No description.
#[derive(Clone)]
pub struct UnClipConditioning<
    Conditioning: crate::nodes::types::Conditioning,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub strength: Strength,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub noise_augmentation: NoiseAugmentation,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> UnClipConditioning<Conditioning, ClipVisionOutput, Strength, NoiseAugmentation> {
    /// Create a new node.
    pub fn new(
        conditioning: Conditioning,
        clip_vision_output: ClipVisionOutput,
        strength: Strength,
        noise_augmentation: NoiseAugmentation,
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
    Conditioning: crate::nodes::types::Conditioning,
    ClipVisionOutput: crate::nodes::types::ClipVisionOutput,
    Strength: crate::nodes::types::Float,
    NoiseAugmentation: crate::nodes::types::Float,
> crate::nodes::TypedNode
for UnClipConditioning<Conditioning, ClipVisionOutput, Strength, NoiseAugmentation> {
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
