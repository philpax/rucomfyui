//!loaders
pub mod video_models;
///**Load CLIP Vision**
pub struct ClipVisionLoader {}
#[doc = "**Load Checkpoint**\n\nLoads a diffusion model checkpoint, diffusion models are used to denoise latents."]
pub struct CheckpointLoaderSimple {}
///**Load ControlNet Model**
pub struct ControlNetLoader {}
///**Load ControlNet Model (diff)**
pub struct DiffControlNetLoader<Model: crate::nodes::Model> {
    ///No documentation.
    pub model: Model,
}
///**GLIGENLoader**
pub struct GligenLoader {}
///**HypernetworkLoader**
pub struct HypernetworkLoader<
    Model: crate::nodes::Model,
    Strength: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub strength: Strength,
}
#[doc = "**Load LoRA**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoader<
    Model: crate::nodes::Model,
    Clip: crate::nodes::Clip,
    StrengthModel: crate::nodes::Float,
    StrengthClip: crate::nodes::Float,
> {
    ///The diffusion model the LoRA will be applied to.
    pub model: Model,
    ///The CLIP model the LoRA will be applied to.
    pub clip: Clip,
    ///How strongly to modify the diffusion model. This value can be negative.
    pub strength_model: StrengthModel,
    ///How strongly to modify the CLIP model. This value can be negative.
    pub strength_clip: StrengthClip,
}
#[doc = "**LoraLoaderModelOnly**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoaderModelOnly<
    Model: crate::nodes::Model,
    StrengthModel: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub strength_model: StrengthModel,
}
///**Load Style Model**
pub struct StyleModelLoader {}
///**Load Upscale Model**
pub struct UpscaleModelLoader {}
///**Load VAE**
pub struct VaeLoader {}
///**unCLIPCheckpointLoader**
pub struct UnClipCheckpointLoader {}
