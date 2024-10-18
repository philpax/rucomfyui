//!`loaders` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod video_models;
/// Output types for nodes.
pub mod out {
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
    ///Output for [`LoraLoader`](super::LoraLoader).
    #[derive(Clone)]
    pub struct LoraLoaderOutput {
        ///The modified diffusion model.
        pub model: crate::nodes::types::ModelOut,
        ///The modified CLIP model.
        pub clip: crate::nodes::types::ClipOut,
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
#[derive(Clone)]
pub struct ClipVisionLoader<ClipName: crate::nodes::types::String> {
    ///No documentation.
    pub clip_name: ClipName,
}
impl<ClipName: crate::nodes::types::String> ClipVisionLoader<ClipName> {
    /// Create a new node.
    pub fn new(clip_name: ClipName) -> Self {
        Self { clip_name }
    }
}
impl<ClipName: crate::nodes::types::String> crate::nodes::TypedNode
for ClipVisionLoader<ClipName> {
    type Output = crate::nodes::types::ClipVisionOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name".to_string(), self.clip_name.clone().into());
        output
    }
    const NAME: &'static str = "CLIPVisionLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP Vision";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Checkpoint**: Loads a diffusion model checkpoint, diffusion models are used to denoise latents.
#[derive(Clone)]
pub struct CheckpointLoaderSimple<CkptName: crate::nodes::types::String> {
    ///The name of the checkpoint (model) to load.
    pub ckpt_name: CkptName,
}
impl<CkptName: crate::nodes::types::String> CheckpointLoaderSimple<CkptName> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptName) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for CheckpointLoaderSimple<CkptName> {
    type Output = out::CheckpointLoaderSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip: crate::nodes::types::ClipOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output
    }
    const NAME: &'static str = "CheckpointLoaderSimple";
    const DISPLAY_NAME: &'static str = "Load Checkpoint";
    const DESCRIPTION: &'static str = "Loads a diffusion model checkpoint, diffusion models are used to denoise latents.";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model**: No description.
#[derive(Clone)]
pub struct ControlNetLoader<ControlNetName: crate::nodes::types::String> {
    ///No documentation.
    pub control_net_name: ControlNetName,
}
impl<ControlNetName: crate::nodes::types::String> ControlNetLoader<ControlNetName> {
    /// Create a new node.
    pub fn new(control_net_name: ControlNetName) -> Self {
        Self { control_net_name }
    }
}
impl<ControlNetName: crate::nodes::types::String> crate::nodes::TypedNode
for ControlNetLoader<ControlNetName> {
    type Output = crate::nodes::types::ControlNetOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "control_net_name".to_string(),
                self.control_net_name.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load ControlNet Model (diff)**: No description.
#[derive(Clone)]
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
> DiffControlNetLoader<Model, ControlNetName> {
    /// Create a new node.
    pub fn new(model: Model, control_net_name: ControlNetName) -> Self {
        Self { model, control_net_name }
    }
}
impl<
    Model: crate::nodes::types::Model,
    ControlNetName: crate::nodes::types::String,
> crate::nodes::TypedNode for DiffControlNetLoader<Model, ControlNetName> {
    type Output = crate::nodes::types::ControlNetOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert(
                "control_net_name".to_string(),
                self.control_net_name.clone().into(),
            );
        output
    }
    const NAME: &'static str = "DiffControlNetLoader";
    const DISPLAY_NAME: &'static str = "Load ControlNet Model (diff)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**GLIGENLoader**: No description.
#[derive(Clone)]
pub struct GligenLoader<GligenName: crate::nodes::types::String> {
    ///No documentation.
    pub gligen_name: GligenName,
}
impl<GligenName: crate::nodes::types::String> GligenLoader<GligenName> {
    /// Create a new node.
    pub fn new(gligen_name: GligenName) -> Self {
        Self { gligen_name }
    }
}
impl<GligenName: crate::nodes::types::String> crate::nodes::TypedNode
for GligenLoader<GligenName> {
    type Output = crate::nodes::types::GligenOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("gligen_name".to_string(), self.gligen_name.clone().into());
        output
    }
    const NAME: &'static str = "GLIGENLoader";
    const DISPLAY_NAME: &'static str = "GLIGENLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**HypernetworkLoader**: No description.
#[derive(Clone)]
pub struct HypernetworkLoader<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub hypernetwork_name: HypernetworkName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub strength: Strength,
}
impl<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> HypernetworkLoader<Model, HypernetworkName, Strength> {
    /// Create a new node.
    pub fn new(
        model: Model,
        hypernetwork_name: HypernetworkName,
        strength: Strength,
    ) -> Self {
        Self {
            model,
            hypernetwork_name,
            strength,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    HypernetworkName: crate::nodes::types::String,
    Strength: crate::nodes::types::Float,
> crate::nodes::TypedNode for HypernetworkLoader<Model, HypernetworkName, Strength> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert(
                "hypernetwork_name".to_string(),
                self.hypernetwork_name.clone().into(),
            );
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "HypernetworkLoader";
    const DISPLAY_NAME: &'static str = "HypernetworkLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load LoRA**: LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
#[derive(Clone)]
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
    /**How strongly to modify the diffusion model. This value can be negative.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
    /**How strongly to modify the CLIP model. This value can be negative.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_clip: StrengthClip,
}
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
> LoraLoader<Model, Clip, LoraName, StrengthModel, StrengthClip> {
    /// Create a new node.
    pub fn new(
        model: Model,
        clip: Clip,
        lora_name: LoraName,
        strength_model: StrengthModel,
        strength_clip: StrengthClip,
    ) -> Self {
        Self {
            model,
            clip,
            lora_name,
            strength_model,
            strength_clip,
        }
    }
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
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip: crate::nodes::types::ClipOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("lora_name".to_string(), self.lora_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output.insert("strength_clip".to_string(), self.strength_clip.clone().into());
        output
    }
    const NAME: &'static str = "LoraLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**LoraLoaderModelOnly**: LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
#[derive(Clone)]
pub struct LoraLoaderModelOnly<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub lora_name: LoraName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
}
impl<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> LoraLoaderModelOnly<Model, LoraName, StrengthModel> {
    /// Create a new node.
    pub fn new(
        model: Model,
        lora_name: LoraName,
        strength_model: StrengthModel,
    ) -> Self {
        Self {
            model,
            lora_name,
            strength_model,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
> crate::nodes::TypedNode for LoraLoaderModelOnly<Model, LoraName, StrengthModel> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("lora_name".to_string(), self.lora_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output
    }
    const NAME: &'static str = "LoraLoaderModelOnly";
    const DISPLAY_NAME: &'static str = "LoraLoaderModelOnly";
    const DESCRIPTION: &'static str = "LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "loaders";
}
///**Load Style Model**: No description.
#[derive(Clone)]
pub struct StyleModelLoader<StyleModelName: crate::nodes::types::String> {
    ///No documentation.
    pub style_model_name: StyleModelName,
}
impl<StyleModelName: crate::nodes::types::String> StyleModelLoader<StyleModelName> {
    /// Create a new node.
    pub fn new(style_model_name: StyleModelName) -> Self {
        Self { style_model_name }
    }
}
impl<StyleModelName: crate::nodes::types::String> crate::nodes::TypedNode
for StyleModelLoader<StyleModelName> {
    type Output = crate::nodes::types::StyleModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "style_model_name".to_string(),
                self.style_model_name.clone().into(),
            );
        output
    }
    const NAME: &'static str = "StyleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Style Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Upscale Model**: No description.
#[derive(Clone)]
pub struct UpscaleModelLoader<ModelName: crate::nodes::types::String> {
    ///No documentation.
    pub model_name: ModelName,
}
impl<ModelName: crate::nodes::types::String> UpscaleModelLoader<ModelName> {
    /// Create a new node.
    pub fn new(model_name: ModelName) -> Self {
        Self { model_name }
    }
}
impl<ModelName: crate::nodes::types::String> crate::nodes::TypedNode
for UpscaleModelLoader<ModelName> {
    type Output = crate::nodes::types::UpscaleModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_name".to_string(), self.model_name.clone().into());
        output
    }
    const NAME: &'static str = "UpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load VAE**: No description.
#[derive(Clone)]
pub struct VaeLoader<VaeName: crate::nodes::types::String> {
    ///No documentation.
    pub vae_name: VaeName,
}
impl<VaeName: crate::nodes::types::String> VaeLoader<VaeName> {
    /// Create a new node.
    pub fn new(vae_name: VaeName) -> Self {
        Self { vae_name }
    }
}
impl<VaeName: crate::nodes::types::String> crate::nodes::TypedNode
for VaeLoader<VaeName> {
    type Output = crate::nodes::types::VaeOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae_name".to_string(), self.vae_name.clone().into());
        output
    }
    const NAME: &'static str = "VAELoader";
    const DISPLAY_NAME: &'static str = "Load VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**unCLIPCheckpointLoader**: No description.
#[derive(Clone)]
pub struct UnClipCheckpointLoader<CkptName: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptName,
}
impl<CkptName: crate::nodes::types::String> UnClipCheckpointLoader<CkptName> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptName) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptName: crate::nodes::types::String> crate::nodes::TypedNode
for UnClipCheckpointLoader<CkptName> {
    type Output = out::UnClipCheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip: crate::nodes::types::ClipOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
            clip_vision: crate::nodes::types::ClipVisionOut::from_dynamic(node_id, 3u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output
    }
    const NAME: &'static str = "unCLIPCheckpointLoader";
    const DISPLAY_NAME: &'static str = "unCLIPCheckpointLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
