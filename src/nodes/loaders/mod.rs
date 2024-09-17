//!loaders
pub mod videomodels;
///**Load CLIP Vision**
pub struct CLIPVisionLoader {}
#[doc = "**Load Checkpoint**\n\nLoads a diffusion model checkpoint, diffusion models are used to denoise latents."]
pub struct CheckpointLoaderSimple {}
///**Load ControlNet Model**
pub struct ControlNetLoader {}
///**Load ControlNet Model (diff)**
pub struct DiffControlNetLoader {}
///**GLIGENLoader**
pub struct GLIGENLoader {}
///**HypernetworkLoader**
pub struct HypernetworkLoader {}
#[doc = "**Load LoRA**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoader {}
#[doc = "**LoraLoaderModelOnly**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoaderModelOnly {}
///**Load Style Model**
pub struct StyleModelLoader {}
///**Load Upscale Model**
pub struct UpscaleModelLoader {}
///**Load VAE**
pub struct VAELoader {}
///**unCLIPCheckpointLoader**
pub struct UnCLIPCheckpointLoader {}
