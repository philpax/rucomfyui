//!conditioning
pub mod n3dmodels;
pub mod controlnet;
pub mod gligen;
pub mod inpaint;
pub mod instructpix2pix;
pub mod stablecascade;
pub mod stylemodel;
pub mod upscalediffusion;
pub mod videomodels;
///**CLIP Set Last Layer**
pub struct CLIPSetLastLayer {}
#[doc = "**CLIP Text Encode (Prompt)**\n\nEncodes a text prompt using a CLIP model into an embedding that can be used to guide the diffusion model towards generating specific images."]
pub struct CLIPTextEncode {}
///**CLIP Vision Encode**
pub struct CLIPVisionEncode {}
///**ConditioningAverage**
pub struct ConditioningAverage {}
///**Conditioning (Combine)**
pub struct ConditioningCombine {}
///**Conditioning (Concat)**
pub struct ConditioningConcat {}
///**Conditioning (Set Area)**
pub struct ConditioningSetArea {}
///**Conditioning (Set Area with Percentage)**
pub struct ConditioningSetAreaPercentage {}
///**ConditioningSetAreaStrength**
pub struct ConditioningSetAreaStrength {}
///**Conditioning (Set Mask)**
pub struct ConditioningSetMask {}
///**unCLIPConditioning**
pub struct UnCLIPConditioning {}
