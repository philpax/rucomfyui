//!loaders
pub mod video_models;
///**Load CLIP Vision**
pub struct ClipVisionLoader {}
///Output for [`ClipVisionLoader`].
pub struct ClipVisionLoaderOutput {
    ///No documentation.
    pub clip_vision: crate::nodes::ClipVisionOut,
}
impl crate::nodes::TypedNode for ClipVisionLoader {
    type Output = ClipVisionLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip_vision: crate::nodes::ClipVisionOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPVisionLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP Vision";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
#[doc = "**Load Checkpoint**\n\nLoads a diffusion model checkpoint, diffusion models are used to denoise latents."]
pub struct CheckpointLoaderSimple {}
///Output for [`CheckpointLoaderSimple`].
pub struct CheckpointLoaderSimpleOutput {
    ///The model used for denoising latents.
    pub model: crate::nodes::ModelOut,
    ///The CLIP model used for encoding text prompts.
    pub clip: crate::nodes::ClipOut,
    ///The VAE model used for encoding and decoding images to and from latent space.
    pub vae: crate::nodes::VaeOut,
}
impl crate::nodes::TypedNode for CheckpointLoaderSimple {
    type Output = CheckpointLoaderSimpleOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
            clip: crate::nodes::ClipOut(1u32),
            vae: crate::nodes::VaeOut(2u32),
        }
    }
    const NAME: &'static str = "CheckpointLoaderSimple";
    const DISPLAY_NAME: &'static str = "Load Checkpoint";
    const DESCRIPTION: &'static str = "Loads a diffusion model checkpoint, diffusion models are used to denoise latents.";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model**
pub struct ControlNetLoader {}
///Output for [`ControlNetLoader`].
pub struct ControlNetLoaderOutput {
    ///No documentation.
    pub control_net: crate::nodes::ControlNetOut,
}
impl crate::nodes::TypedNode for ControlNetLoader {
    type Output = ControlNetLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::ControlNetOut(0u32),
        }
    }
    const NAME: &'static str = "ControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model (diff)**
pub struct DiffControlNetLoader<Model: crate::nodes::Model> {
    ///No documentation.
    pub model: Model,
}
///Output for [`DiffControlNetLoader`].
pub struct DiffControlNetLoaderOutput {
    ///No documentation.
    pub control_net: crate::nodes::ControlNetOut,
}
impl<Model: crate::nodes::Model> crate::nodes::TypedNode
for DiffControlNetLoader<Model> {
    type Output = DiffControlNetLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::ControlNetOut(0u32),
        }
    }
    const NAME: &'static str = "DiffControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model (diff)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**GLIGENLoader**
pub struct GligenLoader {}
///Output for [`GligenLoader`].
pub struct GligenLoaderOutput {
    ///No documentation.
    pub gligen: crate::nodes::GligenOut,
}
impl crate::nodes::TypedNode for GligenLoader {
    type Output = GligenLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            gligen: crate::nodes::GligenOut(0u32),
        }
    }
    const NAME: &'static str = "GLIGENLoader";
    const DISPLAY_NAME: &'static str = "GLIGENLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
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
///Output for [`HypernetworkLoader`].
pub struct HypernetworkLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, Strength: crate::nodes::Float> crate::nodes::TypedNode
for HypernetworkLoader<Model, Strength> {
    type Output = HypernetworkLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "HypernetworkLoader";
    const DISPLAY_NAME: &'static str = "HypernetworkLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
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
///Output for [`LoraLoader`].
pub struct LoraLoaderOutput {
    ///The modified diffusion model.
    pub model: crate::nodes::ModelOut,
    ///The modified CLIP model.
    pub clip: crate::nodes::ClipOut,
}
impl<
    Model: crate::nodes::Model,
    Clip: crate::nodes::Clip,
    StrengthModel: crate::nodes::Float,
    StrengthClip: crate::nodes::Float,
> crate::nodes::TypedNode for LoraLoader<Model, Clip, StrengthModel, StrengthClip> {
    type Output = LoraLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
            clip: crate::nodes::ClipOut(1u32),
        }
    }
    const NAME: &'static str = "LoraLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
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
///Output for [`LoraLoaderModelOnly`].
pub struct LoraLoaderModelOnlyOutput {
    ///The modified diffusion model.
    pub model: crate::nodes::ModelOut,
}
impl<
    Model: crate::nodes::Model,
    StrengthModel: crate::nodes::Float,
> crate::nodes::TypedNode for LoraLoaderModelOnly<Model, StrengthModel> {
    type Output = LoraLoaderModelOnlyOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "LoraLoaderModelOnly";
    const DISPLAY_NAME: &'static str = "LoraLoaderModelOnly";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**Load Style Model**
pub struct StyleModelLoader {}
///Output for [`StyleModelLoader`].
pub struct StyleModelLoaderOutput {
    ///No documentation.
    pub style_model: crate::nodes::StyleModelOut,
}
impl crate::nodes::TypedNode for StyleModelLoader {
    type Output = StyleModelLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            style_model: crate::nodes::StyleModelOut(0u32),
        }
    }
    const NAME: &'static str = "StyleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Upscale Model**
pub struct UpscaleModelLoader {}
///Output for [`UpscaleModelLoader`].
pub struct UpscaleModelLoaderOutput {
    ///No documentation.
    pub upscale_model: crate::nodes::UpscaleModelOut,
}
impl crate::nodes::TypedNode for UpscaleModelLoader {
    type Output = UpscaleModelLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            upscale_model: crate::nodes::UpscaleModelOut(0u32),
        }
    }
    const NAME: &'static str = "UpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load VAE**
pub struct VaeLoader {}
///Output for [`VaeLoader`].
pub struct VaeLoaderOutput {
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl crate::nodes::TypedNode for VaeLoader {
    type Output = VaeLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            vae: crate::nodes::VaeOut(0u32),
        }
    }
    const NAME: &'static str = "VAELoader";
    const DISPLAY_NAME: &'static str = "Load VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**unCLIPCheckpointLoader**
pub struct UnClipCheckpointLoader {}
///Output for [`UnClipCheckpointLoader`].
pub struct UnClipCheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
    ///No documentation.
    pub clip_vision: crate::nodes::ClipVisionOut,
}
impl crate::nodes::TypedNode for UnClipCheckpointLoader {
    type Output = UnClipCheckpointLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
            clip: crate::nodes::ClipOut(1u32),
            vae: crate::nodes::VaeOut(2u32),
            clip_vision: crate::nodes::ClipVisionOut(3u32),
        }
    }
    const NAME: &'static str = "unCLIPCheckpointLoader";
    const DISPLAY_NAME: &'static str = "unCLIPCheckpointLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
