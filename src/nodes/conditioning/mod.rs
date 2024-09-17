//!conditioning
pub mod n3d_models;
pub mod controlnet;
pub mod gligen;
pub mod inpaint;
pub mod instructpix2pix;
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
#[doc = "**CLIP Text Encode (Prompt)**\n\nEncodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images."]
pub struct ClipTextEncode<Text: crate::nodes::String, Clip: crate::nodes::Clip> {
    ///The text to be encoded.
    pub text: Text,
    ///The CLIP model used for encoding the text.
    pub clip: Clip,
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
///**Conditioning (Set Mask)**
pub struct ConditioningSetMask<
    Conditioning: crate::nodes::Conditioning,
    Mask: crate::nodes::Mask,
    Strength: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub strength: Strength,
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
