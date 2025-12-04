//!`loaders` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod video_models;
/// Output types for nodes.
pub mod out {
    ///Output for [`CheckpointLoaderSimple`](super::CheckpointLoaderSimple).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
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
    #[allow(non_camel_case_types)]
    pub struct LoraLoaderOutput {
        ///The modified diffusion model.
        pub model: crate::nodes::types::ModelOut,
        ///The modified CLIP model.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`unCLIPCheckpointLoader`](super::unCLIPCheckpointLoader).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct unCLIPCheckpointLoaderOutput {
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
///**AudioEncoderLoader**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioEncoderLoader {}
impl AudioEncoderLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for AudioEncoderLoader {
    type Output = crate::nodes::types::AudioEncoderOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "AudioEncoderLoader";
    const DISPLAY_NAME: &'static str = "AudioEncoderLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load CLIP Vision**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPVisionLoader<ClipNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub clip_name: ClipNameParam,
}
impl<ClipNameParam: crate::nodes::types::String> CLIPVisionLoader<ClipNameParam> {
    /// Create a new node.
    pub fn new(clip_name: ClipNameParam) -> Self {
        Self { clip_name }
    }
}
impl<ClipNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for CLIPVisionLoader<ClipNameParam> {
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
#[allow(non_camel_case_types)]
pub struct CheckpointLoaderSimple<CkptNameParam: crate::nodes::types::String> {
    ///The name of the checkpoint (model) to load.
    pub ckpt_name: CkptNameParam,
}
impl<CkptNameParam: crate::nodes::types::String> CheckpointLoaderSimple<CkptNameParam> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptNameParam) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for CheckpointLoaderSimple<CkptNameParam> {
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
#[allow(non_camel_case_types)]
pub struct ControlNetLoader<ControlNetNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub control_net_name: ControlNetNameParam,
}
impl<
    ControlNetNameParam: crate::nodes::types::String,
> ControlNetLoader<ControlNetNameParam> {
    /// Create a new node.
    pub fn new(control_net_name: ControlNetNameParam) -> Self {
        Self { control_net_name }
    }
}
impl<ControlNetNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for ControlNetLoader<ControlNetNameParam> {
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
#[allow(non_camel_case_types)]
pub struct DiffControlNetLoader<
    ModelParam: crate::nodes::types::Model,
    ControlNetNameParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub control_net_name: ControlNetNameParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ControlNetNameParam: crate::nodes::types::String,
> DiffControlNetLoader<ModelParam, ControlNetNameParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, control_net_name: ControlNetNameParam) -> Self {
        Self { model, control_net_name }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ControlNetNameParam: crate::nodes::types::String,
> crate::nodes::TypedNode for DiffControlNetLoader<ModelParam, ControlNetNameParam> {
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
#[allow(non_camel_case_types)]
pub struct GLIGENLoader<GligenNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub gligen_name: GligenNameParam,
}
impl<GligenNameParam: crate::nodes::types::String> GLIGENLoader<GligenNameParam> {
    /// Create a new node.
    pub fn new(gligen_name: GligenNameParam) -> Self {
        Self { gligen_name }
    }
}
impl<GligenNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for GLIGENLoader<GligenNameParam> {
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
#[allow(non_camel_case_types)]
pub struct HypernetworkLoader<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
> HypernetworkLoader<ModelParam, StrengthParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, strength: StrengthParam) -> Self {
        Self { model, strength }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for HypernetworkLoader<ModelParam, StrengthParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "HypernetworkLoader";
    const DISPLAY_NAME: &'static str = "HypernetworkLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load Latent Upscale Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LatentUpscaleModelLoader {}
impl LatentUpscaleModelLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LatentUpscaleModelLoader {
    type Output = crate::nodes::types::LatentUpscaleModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LatentUpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Latent Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load LoRA**: LoRAs are used to modify diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraLoader<
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
> {
    ///The diffusion model the LoRA will be applied to.
    pub model: ModelParam,
    ///The CLIP model the LoRA will be applied to.
    pub clip: ClipParam,
    ///The name of the LoRA.
    pub lora_name: LoraNameParam,
    /**How strongly to modify the diffusion model. This value can be negative.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
    /**How strongly to modify the CLIP model. This value can be negative.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_clip: StrengthClipParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
> LoraLoader<
    ModelParam,
    ClipParam,
    LoraNameParam,
    StrengthModelParam,
    StrengthClipParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        clip: ClipParam,
        lora_name: LoraNameParam,
        strength_model: StrengthModelParam,
        strength_clip: StrengthClipParam,
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
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LoraLoader<
    ModelParam,
    ClipParam,
    LoraNameParam,
    StrengthModelParam,
    StrengthClipParam,
> {
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
#[allow(non_camel_case_types)]
pub struct LoraLoaderModelOnly<
    ModelParam: crate::nodes::types::Model,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub lora_name: LoraNameParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
> LoraLoaderModelOnly<ModelParam, LoraNameParam, StrengthModelParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        lora_name: LoraNameParam,
        strength_model: StrengthModelParam,
    ) -> Self {
        Self {
            model,
            lora_name,
            strength_model,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LoraLoaderModelOnly<ModelParam, LoraNameParam, StrengthModelParam> {
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
///**Load LoRA Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraModelLoader<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
> {
    ///The diffusion model the LoRA will be applied to.
    pub model: ModelParam,
    ///The LoRA model to apply to the diffusion model.
    pub lora: LoraParam,
    /**How strongly to modify the diffusion model. This value can be negative.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
*/
    pub strength_model: StrengthModelParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
> LoraModelLoader<ModelParam, LoraParam, StrengthModelParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        lora: LoraParam,
        strength_model: StrengthModelParam,
    ) -> Self {
        Self {
            model,
            lora,
            strength_model,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LoraModelLoader<ModelParam, LoraParam, StrengthModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("lora".to_string(), self.lora.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output
    }
    const NAME: &'static str = "LoraModelLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Save LoRA Weights**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveLoRA<
    LoraParam: crate::nodes::types::LoraModel,
    PrefixParam: crate::nodes::types::String,
    StepsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///The LoRA model to save. Do not use the model with LoRA layers.
    pub lora: LoraParam,
    /**The prefix to use for the saved LoRA file.

**Metadata**:
  - Multiline: false
  - Default: loras/ComfyUI_trained_lora
*/
    pub prefix: PrefixParam,
    ///Optional: The number of steps to LoRA has been trained for, used to name the saved file.
    pub steps: Option<StepsParam>,
}
impl<
    LoraParam: crate::nodes::types::LoraModel,
    PrefixParam: crate::nodes::types::String,
    StepsParam: crate::nodes::types::Int,
> SaveLoRA<LoraParam, PrefixParam, StepsParam> {
    /// Create a new node.
    pub fn new(lora: LoraParam, prefix: PrefixParam, steps: Option<StepsParam>) -> Self {
        Self { lora, prefix, steps }
    }
}
impl<
    LoraParam: crate::nodes::types::LoraModel,
    PrefixParam: crate::nodes::types::String,
    StepsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SaveLoRA<LoraParam, PrefixParam, StepsParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("lora".to_string(), self.lora.clone().into());
        output.insert("prefix".to_string(), self.prefix.clone().into());
        if let Some(v) = &self.steps {
            output.insert("steps".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SaveLoRA";
    const DISPLAY_NAME: &'static str = "Save LoRA Weights";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
impl<
    LoraParam: crate::nodes::types::LoraModel,
    PrefixParam: crate::nodes::types::String,
    StepsParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode for SaveLoRA<LoraParam, PrefixParam, StepsParam> {}
///**Load Style Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StyleModelLoader<StyleModelNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub style_model_name: StyleModelNameParam,
}
impl<
    StyleModelNameParam: crate::nodes::types::String,
> StyleModelLoader<StyleModelNameParam> {
    /// Create a new node.
    pub fn new(style_model_name: StyleModelNameParam) -> Self {
        Self { style_model_name }
    }
}
impl<StyleModelNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for StyleModelLoader<StyleModelNameParam> {
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
#[allow(non_camel_case_types)]
pub struct UpscaleModelLoader {}
impl UpscaleModelLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for UpscaleModelLoader {
    type Output = crate::nodes::types::UpscaleModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "UpscaleModelLoader";
    const DISPLAY_NAME: &'static str = "Load Upscale Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders";
}
///**Load VAE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAELoader<VaeNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub vae_name: VaeNameParam,
}
impl<VaeNameParam: crate::nodes::types::String> VAELoader<VaeNameParam> {
    /// Create a new node.
    pub fn new(vae_name: VaeNameParam) -> Self {
        Self { vae_name }
    }
}
impl<VaeNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for VAELoader<VaeNameParam> {
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
#[allow(non_camel_case_types)]
pub struct unCLIPCheckpointLoader<CkptNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptNameParam,
}
impl<CkptNameParam: crate::nodes::types::String> unCLIPCheckpointLoader<CkptNameParam> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptNameParam) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for unCLIPCheckpointLoader<CkptNameParam> {
    type Output = out::unCLIPCheckpointLoaderOutput;
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
