//!`loaders` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod video_models;
///**Load CLIP Vision**
pub struct ClipVisionLoader<ClipName: crate::nodes::types::String> {
    ///No documentation.
    pub clip_name: ClipName,
}
///Output for [`ClipVisionLoader`].
#[derive(Clone)]
pub struct ClipVisionLoaderOutput {
    ///No documentation.
    pub clip_vision: crate::nodes::types::ClipVisionOut,
}
impl<ClipName: crate::nodes::types::String> crate::nodes::TypedNode
for ClipVisionLoader<ClipName> {
    type Output = ClipVisionLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip_vision: crate::nodes::types::ClipVisionOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPVisionLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP Vision";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
#[doc = "**Load Checkpoint**\n\nLoads a diffusion model checkpoint, diffusion models are used to denoise latents."]
pub struct CheckpointLoaderSimple<CkptName: crate::nodes::types::String> {
    ///The name of the checkpoint (model) to load.
    pub ckpt_name: CkptName,
}
///Output for [`CheckpointLoaderSimple`].
#[derive(Clone)]
pub struct CheckpointLoaderSimpleOutput {
    ///The model used for denoising latents.
    pub model: crate::nodes::types::ModelOut,
    ///The CLIP model used for encoding text prompts.
    pub clip: crate::nodes::types::ClipOut,
    ///The VAE model used for encoding and decoding images to and from latent space.
    pub vae: crate::nodes::types::VaeOut,
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for CheckpointLoaderSimple<CkptName> {
    type Output = CheckpointLoaderSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 1u32,
            },
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "CheckpointLoaderSimple";
    const DISPLAY_NAME: &'static str = "Load Checkpoint";
    const DESCRIPTION: &'static str = "Loads a diffusion model checkpoint, diffusion models are used to denoise latents.";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model**
pub struct ControlNetLoader<ControlNetName: crate::nodes::types::String> {
    ///No documentation.
    pub control_net_name: ControlNetName,
}
///Output for [`ControlNetLoader`].
#[derive(Clone)]
pub struct ControlNetLoaderOutput {
    ///No documentation.
    pub control_net: crate::nodes::types::ControlNetOut,
}
impl<ControlNetName: crate::nodes::types::String> crate::nodes::TypedNode
for ControlNetLoader<ControlNetName> {
    type Output = ControlNetLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::types::ControlNetOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model (diff)**
pub struct DiffControlNetLoader<
    Model: crate::nodes::types::Model,
    ControlNetName: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub control_net_name: ControlNetName,
}
///Output for [`DiffControlNetLoader`].
#[derive(Clone)]
pub struct DiffControlNetLoaderOutput {
    ///No documentation.
    pub control_net: crate::nodes::types::ControlNetOut,
}
impl<
    Model: crate::nodes::types::Model,
    ControlNetName: crate::nodes::types::String,
> crate::nodes::TypedNode for DiffControlNetLoader<Model, ControlNetName> {
    type Output = DiffControlNetLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::types::ControlNetOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "DiffControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model (diff)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**GLIGENLoader**
pub struct GligenLoader<GligenName: crate::nodes::types::String> {
    ///No documentation.
    pub gligen_name: GligenName,
}
///Output for [`GligenLoader`].
#[derive(Clone)]
pub struct GligenLoaderOutput {
    ///No documentation.
    pub gligen: crate::nodes::types::GligenOut,
}
impl<GligenName: crate::nodes::types::String> crate::nodes::TypedNode
for GligenLoader<GligenName> {
    type Output = GligenLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            gligen: crate::nodes::types::GligenOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "GLIGENLoader";
    const DISPLAY_NAME: &'static str = "GLIGENLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**HypernetworkLoader**
pub struct HypernetworkLoader<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub hypernetwork_name: HypernetworkName,
    ///No documentation.
    pub strength: Strength,
}
///Output for [`HypernetworkLoader`].
#[derive(Clone)]
pub struct HypernetworkLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode for HypernetworkLoader<Model, HypernetworkName, Strength> {
    type Output = HypernetworkLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "HypernetworkLoader";
    const DISPLAY_NAME: &'static str = "HypernetworkLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
#[doc = "**Load LoRA**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoader<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
> {
    ///The diffusion model the LoRA will be applied to.
    pub model: Model,
    ///The CLIP model the LoRA will be applied to.
    pub clip: Clip,
    ///The name of the LoRA.
    pub lora_name: LoraName,
    ///How strongly to modify the diffusion model. This value can be negative.
    pub strength_model: StrengthModel,
    ///How strongly to modify the CLIP model. This value can be negative.
    pub strength_clip: StrengthClip,
}
///Output for [`LoraLoader`].
#[derive(Clone)]
pub struct LoraLoaderOutput {
    ///The modified diffusion model.
    pub model: crate::nodes::types::ModelOut,
    ///The modified CLIP model.
    pub clip: crate::nodes::types::ClipOut,
}
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LoraLoader<Model, Clip, LoraName, StrengthModel, StrengthClip> {
    type Output = LoraLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "LoraLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
#[doc = "**LoraLoaderModelOnly**\n\nLoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together."]
pub struct LoraLoaderModelOnly<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub lora_name: LoraName,
    ///No documentation.
    pub strength_model: StrengthModel,
}
///Output for [`LoraLoaderModelOnly`].
#[derive(Clone)]
pub struct LoraLoaderModelOnlyOutput {
    ///The modified diffusion model.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> crate::nodes::TypedNode for LoraLoaderModelOnly<Model, LoraName, StrengthModel> {
    type Output = LoraLoaderModelOnlyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LoraLoaderModelOnly";
    const DISPLAY_NAME: &'static str = "LoraLoaderModelOnly";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**Load Style Model**
pub struct StyleModelLoader<StyleModelName: crate::nodes::types::String> {
    ///No documentation.
    pub style_model_name: StyleModelName,
}
///Output for [`StyleModelLoader`].
#[derive(Clone)]
pub struct StyleModelLoaderOutput {
    ///No documentation.
    pub style_model: crate::nodes::types::StyleModelOut,
}
impl<StyleModelName: crate::nodes::types::String> crate::nodes::TypedNode
for StyleModelLoader<StyleModelName> {
    type Output = StyleModelLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            style_model: crate::nodes::types::StyleModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "StyleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Upscale Model**
pub struct UpscaleModelLoader<ModelName: crate::nodes::types::String> {
    ///No documentation.
    pub model_name: ModelName,
}
///Output for [`UpscaleModelLoader`].
#[derive(Clone)]
pub struct UpscaleModelLoaderOutput {
    ///No documentation.
    pub upscale_model: crate::nodes::types::UpscaleModelOut,
}
impl<ModelName: crate::nodes::types::String> crate::nodes::TypedNode
for UpscaleModelLoader<ModelName> {
    type Output = UpscaleModelLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            upscale_model: crate::nodes::types::UpscaleModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "UpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load VAE**
pub struct VaeLoader<VaeName: crate::nodes::types::String> {
    ///No documentation.
    pub vae_name: VaeName,
}
///Output for [`VaeLoader`].
#[derive(Clone)]
pub struct VaeLoaderOutput {
    ///No documentation.
    pub vae: crate::nodes::types::VaeOut,
}
impl<VaeName: crate::nodes::types::String> crate::nodes::TypedNode
for VaeLoader<VaeName> {
    type Output = VaeLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "VAELoader";
    const DISPLAY_NAME: &'static str = "Load VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**unCLIPCheckpointLoader**
pub struct UnClipCheckpointLoader<CkptName: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptName,
}
///Output for [`UnClipCheckpointLoader`].
#[derive(Clone)]
pub struct UnClipCheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::types::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::types::VaeOut,
    ///No documentation.
    pub clip_vision: crate::nodes::types::ClipVisionOut,
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for UnClipCheckpointLoader<CkptName> {
    type Output = UnClipCheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 1u32,
            },
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 2u32,
            },
            clip_vision: crate::nodes::types::ClipVisionOut {
                node_id,
                node_slot: 3u32,
            },
        }
    }
    const NAME: &'static str = "unCLIPCheckpointLoader";
    const DISPLAY_NAME: &'static str = "unCLIPCheckpointLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
