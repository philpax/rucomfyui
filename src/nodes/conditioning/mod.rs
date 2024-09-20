//!`conditioning` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
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
    ///Output for [`ClipSetLastLayer`](super::ClipSetLastLayer).
    #[derive(Clone)]
    pub struct ClipSetLastLayerOutput {
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`ClipTextEncode`](super::ClipTextEncode).
    #[derive(Clone)]
    pub struct ClipTextEncodeOutput {
        ///A conditioning containing the embedded text used to guide the diffusion model.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ClipVisionEncode`](super::ClipVisionEncode).
    #[derive(Clone)]
    pub struct ClipVisionEncodeOutput {
        ///No documentation.
        pub clip_vision_output: crate::nodes::types::ClipVisionOutputOut,
    }
    ///Output for [`ConditioningAverage`](super::ConditioningAverage).
    #[derive(Clone)]
    pub struct ConditioningAverageOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningCombine`](super::ConditioningCombine).
    #[derive(Clone)]
    pub struct ConditioningCombineOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningConcat`](super::ConditioningConcat).
    #[derive(Clone)]
    pub struct ConditioningConcatOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningSetArea`](super::ConditioningSetArea).
    #[derive(Clone)]
    pub struct ConditioningSetAreaOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningSetAreaPercentage`](super::ConditioningSetAreaPercentage).
    #[derive(Clone)]
    pub struct ConditioningSetAreaPercentageOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningSetAreaStrength`](super::ConditioningSetAreaStrength).
    #[derive(Clone)]
    pub struct ConditioningSetAreaStrengthOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningSetMask`](super::ConditioningSetMask).
    #[derive(Clone)]
    pub struct ConditioningSetMaskOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`UnClipConditioning`](super::UnClipConditioning).
    #[derive(Clone)]
    pub struct UnClipConditioningOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
}
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
> crate::nodes::TypedNode for ClipSetLastLayer<Clip, StopAtClipLayer> {
    type Output = out::ClipSetLastLayerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode for ClipTextEncode<Text, Clip> {
    type Output = out::ClipTextEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode for ClipVisionEncode<ClipVision, Image> {
    type Output = out::ClipVisionEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip_vision_output: crate::nodes::types::ClipVisionOutputOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode
for ConditioningAverage<ConditioningTo, ConditioningFrom, ConditioningToStrength> {
    type Output = out::ConditioningAverageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode for ConditioningCombine<Conditioning1, Conditioning2> {
    type Output = out::ConditioningCombineOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode for ConditioningConcat<ConditioningTo, ConditioningFrom> {
    type Output = out::ConditioningConcatOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode
for ConditioningSetArea<Conditioning, Width, Height, X, Y, Strength> {
    type Output = out::ConditioningSetAreaOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode
for ConditioningSetAreaPercentage<Conditioning, Width, Height, X, Y, Strength> {
    type Output = out::ConditioningSetAreaPercentageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode for ConditioningSetAreaStrength<Conditioning, Strength> {
    type Output = out::ConditioningSetAreaStrengthOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode
for ConditioningSetMask<Conditioning, Mask, Strength, SetCondArea> {
    type Output = out::ConditioningSetMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
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
> crate::nodes::TypedNode
for UnClipConditioning<Conditioning, ClipVisionOutput, Strength, NoiseAugmentation> {
    type Output = out::UnClipConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "unCLIPConditioning";
    const DISPLAY_NAME: &'static str = "unCLIPConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
