//!`loaders` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod video_models;
/// Output types for nodes.
pub mod out {
    ///Output for [`ClipVisionLoader`](super::ClipVisionLoader).
    #[derive(Clone)]
    pub struct ClipVisionLoaderOutput {
        ///No documentation.
        pub clip_vision: crate::nodes::types::ClipVisionOut,
    }
    ///Output for [`CheckpointLoaderSimple`](super::CheckpointLoaderSimple).
    #[derive(Clone)]
    pub struct CheckpointLoaderSimpleOutput {
        ///The model used for denoising latents.
        pub model: crate::nodes::types::ModelOut,
        ///The CLIP model used for encoding text prompts.
        pub clip: crate::nodes::types::ClipOut,
        ///The VAE model used for encoding and decoding images to and from latent space.
        pub vae: crate::nodes::types::VaeOut,
    }
    ///Output for [`ControlNetLoader`](super::ControlNetLoader).
    #[derive(Clone)]
    pub struct ControlNetLoaderOutput {
        ///No documentation.
        pub control_net: crate::nodes::types::ControlNetOut,
    }
    ///Output for [`DiffControlNetLoader`](super::DiffControlNetLoader).
    #[derive(Clone)]
    pub struct DiffControlNetLoaderOutput {
        ///No documentation.
        pub control_net: crate::nodes::types::ControlNetOut,
    }
    ///Output for [`GligenLoader`](super::GligenLoader).
    #[derive(Clone)]
    pub struct GligenLoaderOutput {
        ///No documentation.
        pub gligen: crate::nodes::types::GligenOut,
    }
    ///Output for [`HypernetworkLoader`](super::HypernetworkLoader).
    #[derive(Clone)]
    pub struct HypernetworkLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`LoraLoader`](super::LoraLoader).
    #[derive(Clone)]
    pub struct LoraLoaderOutput {
        ///The modified diffusion model.
        pub model: crate::nodes::types::ModelOut,
        ///The modified CLIP model.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`LoraLoaderModelOnly`](super::LoraLoaderModelOnly).
    #[derive(Clone)]
    pub struct LoraLoaderModelOnlyOutput {
        ///The modified diffusion model.
        pub model: crate::nodes::types::ModelOut,
    }
    ///Output for [`StyleModelLoader`](super::StyleModelLoader).
    #[derive(Clone)]
    pub struct StyleModelLoaderOutput {
        ///No documentation.
        pub style_model: crate::nodes::types::StyleModelOut,
    }
    ///Output for [`UpscaleModelLoader`](super::UpscaleModelLoader).
    #[derive(Clone)]
    pub struct UpscaleModelLoaderOutput {
        ///No documentation.
        pub upscale_model: crate::nodes::types::UpscaleModelOut,
    }
    ///Output for [`VaeLoader`](super::VaeLoader).
    #[derive(Clone)]
    pub struct VaeLoaderOutput {
        ///No documentation.
        pub vae: crate::nodes::types::VaeOut,
    }
    ///Output for [`UnClipCheckpointLoader`](super::UnClipCheckpointLoader).
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
}
///**Load CLIP Vision**: No description.
pub struct ClipVisionLoader<ClipName: crate::nodes::types::String> {
    ///No documentation.
    pub clip_name: ClipName,
}
impl<ClipName: crate::nodes::types::String> crate::nodes::TypedNode
for ClipVisionLoader<ClipName> {
    type Output = out::ClipVisionLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip_vision: crate::nodes::types::ClipVisionOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name".to_string(), self.clip_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPVisionLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP Vision";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Checkpoint**: Loads a diffusion model checkpoint, diffusion models are used to denoise latents.
pub struct CheckpointLoaderSimple<CkptName: crate::nodes::types::String> {
    ///The name of the checkpoint (model) to load.
    pub ckpt_name: CkptName,
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for CheckpointLoaderSimple<CkptName> {
    type Output = out::CheckpointLoaderSimpleOutput;
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "CheckpointLoaderSimple";
    const DISPLAY_NAME: &'static str = "Load Checkpoint";
    const DESCRIPTION: &'static str = "Loads a diffusion model checkpoint, diffusion models are used to denoise latents.";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model**: No description.
pub struct ControlNetLoader<ControlNetName: crate::nodes::types::String> {
    ///No documentation.
    pub control_net_name: ControlNetName,
}
impl<ControlNetName: crate::nodes::types::String> crate::nodes::TypedNode
for ControlNetLoader<ControlNetName> {
    type Output = out::ControlNetLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::types::ControlNetOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "control_net_name".to_string(),
                self.control_net_name.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "ControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model (diff)**: No description.
pub struct DiffControlNetLoader<
    Model: crate::nodes::types::Model,
    ControlNetName: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub control_net_name: ControlNetName,
}
impl<
    Model: crate::nodes::types::Model,
    ControlNetName: crate::nodes::types::String,
> crate::nodes::TypedNode for DiffControlNetLoader<Model, ControlNetName> {
    type Output = out::DiffControlNetLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            control_net: crate::nodes::types::ControlNetOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output
            .insert(
                "control_net_name".to_string(),
                self.control_net_name.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "DiffControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model (diff)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**GLIGENLoader**: No description.
pub struct GligenLoader<GligenName: crate::nodes::types::String> {
    ///No documentation.
    pub gligen_name: GligenName,
}
impl<GligenName: crate::nodes::types::String> crate::nodes::TypedNode
for GligenLoader<GligenName> {
    type Output = out::GligenLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            gligen: crate::nodes::types::GligenOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("gligen_name".to_string(), self.gligen_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "GLIGENLoader";
    const DISPLAY_NAME: &'static str = "GLIGENLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**HypernetworkLoader**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode for HypernetworkLoader<Model, HypernetworkName, Strength> {
    type Output = out::HypernetworkLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output
            .insert(
                "hypernetwork_name".to_string(),
                self.hypernetwork_name.to_workflow_input(),
            );
        output.insert("strength".to_string(), self.strength.to_workflow_input());
        output
    }
    const NAME: &'static str = "HypernetworkLoader";
    const DISPLAY_NAME: &'static str = "HypernetworkLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load LoRA**: LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
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
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LoraLoader<Model, Clip, LoraName, StrengthModel, StrengthClip> {
    type Output = out::LoraLoaderOutput;
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("lora_name".to_string(), self.lora_name.to_workflow_input());
        output
            .insert(
                "strength_model".to_string(),
                self.strength_model.to_workflow_input(),
            );
        output
            .insert("strength_clip".to_string(), self.strength_clip.to_workflow_input());
        output
    }
    const NAME: &'static str = "LoraLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**LoraLoaderModelOnly**: LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
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
impl<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> crate::nodes::TypedNode for LoraLoaderModelOnly<Model, LoraName, StrengthModel> {
    type Output = out::LoraLoaderModelOnlyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("lora_name".to_string(), self.lora_name.to_workflow_input());
        output
            .insert(
                "strength_model".to_string(),
                self.strength_model.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "LoraLoaderModelOnly";
    const DISPLAY_NAME: &'static str = "LoraLoaderModelOnly";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**Load Style Model**: No description.
pub struct StyleModelLoader<StyleModelName: crate::nodes::types::String> {
    ///No documentation.
    pub style_model_name: StyleModelName,
}
impl<StyleModelName: crate::nodes::types::String> crate::nodes::TypedNode
for StyleModelLoader<StyleModelName> {
    type Output = out::StyleModelLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            style_model: crate::nodes::types::StyleModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "style_model_name".to_string(),
                self.style_model_name.to_workflow_input(),
            );
        output
    }
    const NAME: &'static str = "StyleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Upscale Model**: No description.
pub struct UpscaleModelLoader<ModelName: crate::nodes::types::String> {
    ///No documentation.
    pub model_name: ModelName,
}
impl<ModelName: crate::nodes::types::String> crate::nodes::TypedNode
for UpscaleModelLoader<ModelName> {
    type Output = out::UpscaleModelLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            upscale_model: crate::nodes::types::UpscaleModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_name".to_string(), self.model_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "UpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load VAE**: No description.
pub struct VaeLoader<VaeName: crate::nodes::types::String> {
    ///No documentation.
    pub vae_name: VaeName,
}
impl<VaeName: crate::nodes::types::String> crate::nodes::TypedNode
for VaeLoader<VaeName> {
    type Output = out::VaeLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae_name".to_string(), self.vae_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "VAELoader";
    const DISPLAY_NAME: &'static str = "Load VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**unCLIPCheckpointLoader**: No description.
pub struct UnClipCheckpointLoader<CkptName: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptName,
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for UnClipCheckpointLoader<CkptName> {
    type Output = out::UnClipCheckpointLoaderOutput;
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "unCLIPCheckpointLoader";
    const DISPLAY_NAME: &'static str = "unCLIPCheckpointLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
