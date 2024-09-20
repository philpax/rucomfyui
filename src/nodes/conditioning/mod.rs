//!`conditioning` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod n_3_d_models;
pub mod controlnet;
pub mod gligen;
pub mod inpaint;
pub mod instructpix_2_pix;
pub mod stable_cascade;
pub mod style_model;
pub mod upscale_diffusion;
pub mod video_models;
///**CLIP Set Last Layer**: No description.
pub struct ClipSetLastLayer<
    Clip: crate::nodes::types::Clip,
    StopAtClipLayer: crate::nodes::types::Int,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output
            .insert(
                "stop_at_clip_layer".to_string(),
                self.stop_at_clip_layer.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "CLIPSetLastLayer";
    const DISPLAY_NAME: &'static str = "CLIP Set Last Layer";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Text Encode (Prompt)**: Encodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.
pub struct ClipTextEncode<
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> {
    ///The text to be encoded.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("text".to_string(), self.text.to_workflow_input());
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPTextEncode";
    const DISPLAY_NAME: &'static str = "CLIP Text Encode (Prompt)";
    const DESCRIPTION: &'static str = "Encodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Vision Encode**: No description.
pub struct ClipVisionEncode<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
> {
    ///No documentation.
    pub clip_vision: ClipVision,
    ///No documentation.
    pub image: Image,
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
> ClipVisionEncode<ClipVision, Image> {
    /// Create a new node.
    pub fn new(clip_vision: ClipVision, image: Image) -> Self {
        Self { clip_vision, image }
    }
}
impl<
    ClipVision: crate::nodes::types::ClipVision,
    Image: crate::nodes::types::Image,
> crate::nodes::TypedNode for ClipVisionEncode<ClipVision, Image> {
    type Output = crate::nodes::types::ClipVisionOutputOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.to_workflow_input());
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPVisionEncode";
    const DISPLAY_NAME: &'static str = "CLIP Vision Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningAverage**: No description.
pub struct ConditioningAverage<
    ConditioningTo: crate::nodes::types::Conditioning,
    ConditioningFrom: crate::nodes::types::Conditioning,
    ConditioningToStrength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub conditioning_from: ConditioningFrom,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "conditioning_to".to_string(),
                self.conditioning_to.to_workflow_input(),
            );
        output
            .insert(
                "conditioning_from".to_string(),
                self.conditioning_from.to_workflow_input(),
            );
        output
            .insert(
                "conditioning_to_strength".to_string(),
                self.conditioning_to_strength.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "ConditioningAverage";
    const DISPLAY_NAME: &'static str = "ConditioningAverage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Combine)**: No description.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "conditioning_1".to_string(),
                self.conditioning_1.to_workflow_input(),
            );
        output
            .insert(
                "conditioning_2".to_string(),
                self.conditioning_2.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "ConditioningCombine";
    const DISPLAY_NAME: &'static str = "Conditioning (Combine)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Concat)**: No description.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "conditioning_to".to_string(),
                self.conditioning_to.to_workflow_input(),
            );
        output
            .insert(
                "conditioning_from".to_string(),
                self.conditioning_from.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "ConditioningConcat";
    const DISPLAY_NAME: &'static str = "Conditioning (Concat)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area)**: No description.
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
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
    }
    const NAME: &'static str = "ConditioningSetArea";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area with Percentage)**: No description.
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
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
    }
    const NAME: &'static str = "ConditioningSetAreaPercentage";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area with Percentage)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningSetAreaStrength**: No description.
pub struct ConditioningSetAreaStrength<
    Conditioning: crate::nodes::types::Conditioning,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
    }
    const NAME: &'static str = "ConditioningSetAreaStrength";
    const DISPLAY_NAME: &'static str = "ConditioningSetAreaStrength";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Mask)**: No description.
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
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("mask".to_string(), self.mask.to_workflow_input());
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
            .insert("set_cond_area".to_string(), self.set_cond_area.to_workflow_input());
        output
    }
    const NAME: &'static str = "ConditioningSetMask";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**unCLIPConditioning**: No description.
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
    ///No documentation.
    pub strength: Strength,
    ///No documentation.
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
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.to_workflow_input(),
            );
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
            .insert(
                "noise_augmentation".to_string(),
                self.noise_augmentation.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "unCLIPConditioning";
    const DISPLAY_NAME: &'static str = "unCLIPConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
