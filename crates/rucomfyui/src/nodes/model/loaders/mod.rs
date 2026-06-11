//!`loaders` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
    ///Output for [`ImageOnlyCheckpointLoader`](super::ImageOnlyCheckpointLoader).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ImageOnlyCheckpointLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub clip_vision: crate::nodes::types::ClipVisionOut,
        ///No documentation.
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
    ///Output for [`LoraLoaderBypass`](super::LoraLoaderBypass).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoraLoaderBypassOutput {
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
///**Load Audio Encoder**: No description.
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
    const DISPLAY_NAME: &'static str = "Load Audio Encoder";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Frame Interpolation Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FrameInterpolationModelLoader {}
impl FrameInterpolationModelLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for FrameInterpolationModelLoader {
    type Output = crate::nodes::types::InterpModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "FrameInterpolationModelLoader";
    const DISPLAY_NAME: &'static str = "Load Frame Interpolation Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load GLIGEN Model**: No description.
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
    const DISPLAY_NAME: &'static str = "Load GLIGEN Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Hypernetwork**: No description.
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
    const DISPLAY_NAME: &'static str = "Load Hypernetwork";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Checkpoint Image Only (img2vid model)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageOnlyCheckpointLoader<CkptNameParam: crate::nodes::types::String> {
    ///No documentation.
    pub ckpt_name: CkptNameParam,
}
impl<
    CkptNameParam: crate::nodes::types::String,
> ImageOnlyCheckpointLoader<CkptNameParam> {
    /// Create a new node.
    pub fn new(ckpt_name: CkptNameParam) -> Self {
        Self { ckpt_name }
    }
}
impl<CkptNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for ImageOnlyCheckpointLoader<CkptNameParam> {
    type Output = out::ImageOnlyCheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip_vision: crate::nodes::types::ClipVisionOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output
    }
    const NAME: &'static str = "ImageOnlyCheckpointLoader";
    const DISPLAY_NAME: &'static str = "Load Checkpoint Image Only (img2vid model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LTXV Audio VAE**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVAudioVAELoader {}
impl LTXVAudioVAELoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LTXVAudioVAELoader {
    type Output = crate::nodes::types::VaeOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LTXVAudioVAELoader";
    const DISPLAY_NAME: &'static str = "Load LTXV Audio VAE";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Background Removal Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadBackgroundRemovalModel {}
impl LoadBackgroundRemovalModel {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadBackgroundRemovalModel {
    type Output = crate::nodes::types::BackgroundRemovalOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadBackgroundRemovalModel";
    const DISPLAY_NAME: &'static str = "Load Background Removal Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Depth Anything 3**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadDA3Model {}
impl LoadDA3Model {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadDA3Model {
    type Output = crate::nodes::types::Da3ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadDA3Model";
    const DISPLAY_NAME: &'static str = "Load Depth Anything 3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Face Detection Model (MediaPipe)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadMediaPipeFaceLandmarker {}
impl LoadMediaPipeFaceLandmarker {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadMediaPipeFaceLandmarker {
    type Output = crate::nodes::types::FaceDetectionModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadMediaPipeFaceLandmarker";
    const DISPLAY_NAME: &'static str = "Load Face Detection Model (MediaPipe)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load MoGe Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadMoGeModel {}
impl LoadMoGeModel {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadMoGeModel {
    type Output = crate::nodes::types::MogeModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadMoGeModel";
    const DISPLAY_NAME: &'static str = "Load MoGe Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LoRA (Model and CLIP)**: This LoRA loader is used to modify both diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
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
    const DISPLAY_NAME: &'static str = "Load LoRA (Model and CLIP)";
    const DESCRIPTION: &'static str = "This LoRA loader is used to modify both diffusion and CLIP models, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LoRA (Bypass) (For debugging)**: Apply LoRA in bypass mode. Unlike regular LoRA, this doesn't modify model weights - instead it injects the LoRA computation during forward pass. Useful for training scenarios.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraLoaderBypass<
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
> LoraLoaderBypass<
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
for LoraLoaderBypass<
    ModelParam,
    ClipParam,
    LoraNameParam,
    StrengthModelParam,
    StrengthClipParam,
> {
    type Output = out::LoraLoaderBypassOutput;
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
    const NAME: &'static str = "LoraLoaderBypass";
    const DISPLAY_NAME: &'static str = "Load LoRA (Bypass) (For debugging)";
    const DESCRIPTION: &'static str = "Apply LoRA in bypass mode. Unlike regular LoRA, this doesn't modify model weights - instead it injects the LoRA computation during forward pass. Useful for training scenarios.";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LoRA (Bypass, Model Only) (for debugging)**: Apply LoRA in bypass mode. Unlike regular LoRA, this doesn't modify model weights - instead it injects the LoRA computation during forward pass. Useful for training scenarios.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraLoaderBypassModelOnly<
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
> LoraLoaderBypassModelOnly<ModelParam, LoraNameParam, StrengthModelParam> {
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
for LoraLoaderBypassModelOnly<ModelParam, LoraNameParam, StrengthModelParam> {
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
    const NAME: &'static str = "LoraLoaderBypassModelOnly";
    const DISPLAY_NAME: &'static str = "Load LoRA (Bypass, Model Only) (for debugging)";
    const DESCRIPTION: &'static str = "Apply LoRA in bypass mode. Unlike regular LoRA, this doesn't modify model weights - instead it injects the LoRA computation during forward pass. Useful for training scenarios.";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LoRA**: This LoRAs loader is used to modify the diffusion model, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.
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
    const DISPLAY_NAME: &'static str = "Load LoRA";
    const DESCRIPTION: &'static str = "This LoRAs loader is used to modify the diffusion model, altering the way in which latents are denoised such as applying styles. Multiple LoRA nodes can be linked together.";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load LoRA Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoraModelLoader<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
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
    /**When enabled, applies LoRA in bypass mode without modifying base model weights. Useful for training and when model weights are offloaded.

**Metadata**:
  - Default: false
*/
    pub bypass: BypassParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
> LoraModelLoader<ModelParam, LoraParam, StrengthModelParam, BypassParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        lora: LoraParam,
        strength_model: StrengthModelParam,
        bypass: BypassParam,
    ) -> Self {
        Self {
            model,
            lora,
            strength_model,
            bypass,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    LoraParam: crate::nodes::types::LoraModel,
    StrengthModelParam: crate::nodes::types::Float,
    BypassParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for LoraModelLoader<ModelParam, LoraParam, StrengthModelParam, BypassParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("lora".to_string(), self.lora.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output.insert("bypass".to_string(), self.bypass.clone().into());
        output
    }
    const NAME: &'static str = "LoraModelLoader";
    const DISPLAY_NAME: &'static str = "Load LoRA Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
///**Load Optical Flow Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OpticalFlowLoader {}
impl OpticalFlowLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for OpticalFlowLoader {
    type Output = crate::nodes::types::OpticalFlowOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "OpticalFlowLoader";
    const DISPLAY_NAME: &'static str = "Load Optical Flow Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
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
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
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
    const CATEGORY: &'static str = "model/loaders";
}
///**Load unCLIP Checkpoint**: No description.
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
    const DISPLAY_NAME: &'static str = "Load unCLIP Checkpoint";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/loaders";
}
