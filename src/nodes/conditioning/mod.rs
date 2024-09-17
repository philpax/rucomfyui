//!conditioning
pub mod n_3_d_models;
pub mod controlnet;
pub mod gligen;
pub mod inpaint;
pub mod instructpix_2_pix;
pub mod stable_cascade;
pub mod style_model;
pub mod upscale_diffusion;
pub mod video_models;
///**CLIP Set Last Layer**
pub struct ClipSetLastLayer<
    Clip: crate::nodes::Clip,
    StopAtClipLayer: crate::nodes::Int,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub stop_at_clip_layer: StopAtClipLayer,
}
///Output for [`ClipSetLastLayer`].
#[derive(Clone)]
pub struct ClipSetLastLayerOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl<
    Clip: crate::nodes::Clip,
    StopAtClipLayer: crate::nodes::Int,
> crate::nodes::TypedNode for ClipSetLastLayer<Clip, StopAtClipLayer> {
    type Output = ClipSetLastLayerOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPSetLastLayer";
    const DISPLAY_NAME: &'static str = "CLIP Set Last Layer";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
#[doc = "**CLIP Text Encode (Prompt)**\n\nEncodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images."]
pub struct ClipTextEncode<Text: crate::nodes::String, Clip: crate::nodes::Clip> {
    ///The text to be encoded.
    pub text: Text,
    ///The CLIP model used for encoding the text.
    pub clip: Clip,
}
///Output for [`ClipTextEncode`].
#[derive(Clone)]
pub struct ClipTextEncodeOutput {
    ///A conditioning containing the embedded text used to guide the diffusion model.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<Text: crate::nodes::String, Clip: crate::nodes::Clip> crate::nodes::TypedNode
for ClipTextEncode<Text, Clip> {
    type Output = ClipTextEncodeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPTextEncode";
    const DISPLAY_NAME: &'static str = "CLIP Text Encode (Prompt)";
    const DESCRIPTION: &'static str = "Encodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images.";
    const CATEGORY: &'static str = "conditioning";
}
///**CLIP Vision Encode**
pub struct ClipVisionEncode<
    ClipVision: crate::nodes::ClipVision,
    Image: crate::nodes::Image,
> {
    ///No documentation.
    pub clip_vision: ClipVision,
    ///No documentation.
    pub image: Image,
}
///Output for [`ClipVisionEncode`].
#[derive(Clone)]
pub struct ClipVisionEncodeOutput {
    ///No documentation.
    pub clip_vision_output: crate::nodes::ClipVisionOutputOut,
}
impl<
    ClipVision: crate::nodes::ClipVision,
    Image: crate::nodes::Image,
> crate::nodes::TypedNode for ClipVisionEncode<ClipVision, Image> {
    type Output = ClipVisionEncodeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip_vision_output: crate::nodes::ClipVisionOutputOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPVisionEncode";
    const DISPLAY_NAME: &'static str = "CLIP Vision Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningAverage**
pub struct ConditioningAverage<
    ConditioningTo: crate::nodes::Conditioning,
    ConditioningFrom: crate::nodes::Conditioning,
    ConditioningToStrength: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub conditioning_from: ConditioningFrom,
    ///No documentation.
    pub conditioning_to_strength: ConditioningToStrength,
}
///Output for [`ConditioningAverage`].
#[derive(Clone)]
pub struct ConditioningAverageOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    ConditioningTo: crate::nodes::Conditioning,
    ConditioningFrom: crate::nodes::Conditioning,
    ConditioningToStrength: crate::nodes::Float,
> crate::nodes::TypedNode
for ConditioningAverage<ConditioningTo, ConditioningFrom, ConditioningToStrength> {
    type Output = ConditioningAverageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningAverage";
    const DISPLAY_NAME: &'static str = "ConditioningAverage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Combine)**
pub struct ConditioningCombine<
    Conditioning1: crate::nodes::Conditioning,
    Conditioning2: crate::nodes::Conditioning,
> {
    ///No documentation.
    pub conditioning_1: Conditioning1,
    ///No documentation.
    pub conditioning_2: Conditioning2,
}
///Output for [`ConditioningCombine`].
#[derive(Clone)]
pub struct ConditioningCombineOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning1: crate::nodes::Conditioning,
    Conditioning2: crate::nodes::Conditioning,
> crate::nodes::TypedNode for ConditioningCombine<Conditioning1, Conditioning2> {
    type Output = ConditioningCombineOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningCombine";
    const DISPLAY_NAME: &'static str = "Conditioning (Combine)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Concat)**
pub struct ConditioningConcat<
    ConditioningTo: crate::nodes::Conditioning,
    ConditioningFrom: crate::nodes::Conditioning,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub conditioning_from: ConditioningFrom,
}
///Output for [`ConditioningConcat`].
#[derive(Clone)]
pub struct ConditioningConcatOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    ConditioningTo: crate::nodes::Conditioning,
    ConditioningFrom: crate::nodes::Conditioning,
> crate::nodes::TypedNode for ConditioningConcat<ConditioningTo, ConditioningFrom> {
    type Output = ConditioningConcatOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningConcat";
    const DISPLAY_NAME: &'static str = "Conditioning (Concat)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area)**
pub struct ConditioningSetArea<
    Conditioning: crate::nodes::Conditioning,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Strength: crate::nodes::Float,
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
///Output for [`ConditioningSetArea`].
#[derive(Clone)]
pub struct ConditioningSetAreaOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Strength: crate::nodes::Float,
> crate::nodes::TypedNode
for ConditioningSetArea<Conditioning, Width, Height, X, Y, Strength> {
    type Output = ConditioningSetAreaOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningSetArea";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Area with Percentage)**
pub struct ConditioningSetAreaPercentage<
    Conditioning: crate::nodes::Conditioning,
    Width: crate::nodes::Float,
    Height: crate::nodes::Float,
    X: crate::nodes::Float,
    Y: crate::nodes::Float,
    Strength: crate::nodes::Float,
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
///Output for [`ConditioningSetAreaPercentage`].
#[derive(Clone)]
pub struct ConditioningSetAreaPercentageOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Width: crate::nodes::Float,
    Height: crate::nodes::Float,
    X: crate::nodes::Float,
    Y: crate::nodes::Float,
    Strength: crate::nodes::Float,
> crate::nodes::TypedNode
for ConditioningSetAreaPercentage<Conditioning, Width, Height, X, Y, Strength> {
    type Output = ConditioningSetAreaPercentageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningSetAreaPercentage";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Area with Percentage)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**ConditioningSetAreaStrength**
pub struct ConditioningSetAreaStrength<
    Conditioning: crate::nodes::Conditioning,
    Strength: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub strength: Strength,
}
///Output for [`ConditioningSetAreaStrength`].
#[derive(Clone)]
pub struct ConditioningSetAreaStrengthOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Strength: crate::nodes::Float,
> crate::nodes::TypedNode for ConditioningSetAreaStrength<Conditioning, Strength> {
    type Output = ConditioningSetAreaStrengthOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningSetAreaStrength";
    const DISPLAY_NAME: &'static str = "ConditioningSetAreaStrength";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**Conditioning (Set Mask)**
pub struct ConditioningSetMask<
    Conditioning: crate::nodes::Conditioning,
    Mask: crate::nodes::Mask,
    Strength: crate::nodes::Float,
    SetCondArea: crate::nodes::String,
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
///Output for [`ConditioningSetMask`].
#[derive(Clone)]
pub struct ConditioningSetMaskOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Mask: crate::nodes::Mask,
    Strength: crate::nodes::Float,
    SetCondArea: crate::nodes::String,
> crate::nodes::TypedNode
for ConditioningSetMask<Conditioning, Mask, Strength, SetCondArea> {
    type Output = ConditioningSetMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningSetMask";
    const DISPLAY_NAME: &'static str = "Conditioning (Set Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
///**unCLIPConditioning**
pub struct UnClipConditioning<
    Conditioning: crate::nodes::Conditioning,
    ClipVisionOutput: crate::nodes::ClipVisionOutput,
    Strength: crate::nodes::Float,
    NoiseAugmentation: crate::nodes::Float,
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
///Output for [`UnClipConditioning`].
#[derive(Clone)]
pub struct UnClipConditioningOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    ClipVisionOutput: crate::nodes::ClipVisionOutput,
    Strength: crate::nodes::Float,
    NoiseAugmentation: crate::nodes::Float,
> crate::nodes::TypedNode
for UnClipConditioning<Conditioning, ClipVisionOutput, Strength, NoiseAugmentation> {
    type Output = UnClipConditioningOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "unCLIPConditioning";
    const DISPLAY_NAME: &'static str = "unCLIPConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning";
}
