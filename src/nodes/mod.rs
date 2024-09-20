//!Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod advanced;
pub mod audio;
pub mod conditioning;
pub mod image;
pub mod latent;
pub mod loaders;
pub mod mask;
pub mod model_patches;
pub mod sampling;
pub mod all;
use crate::WorkflowInput;
/// Implemented for all typed nodes; provides the node's output and metadata.
pub trait TypedNode {
    /// The type of the node's output.
    type Output;
    /// Returns the node's output.
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output;
    /// The name of the node.
    const NAME: &'static str;
    /// The display name of the node.
    const DISPLAY_NAME: &'static str;
    /// The description of the node.
    const DESCRIPTION: &'static str;
    /// The category of the node.
    const CATEGORY: &'static str;
}
/// Implemented for all output nodes (i.e. nodes at which a workflow terminates).
pub trait TypedOutputNode {}
/// Converts a value to a workflow input.
pub trait ToWorkflowInput {
    /// Converts the value to a workflow input.
    fn to_workflow_input(&self) -> WorkflowInput;
}
impl ToWorkflowInput for std::string::String {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::String(self.clone())
    }
}
impl String for std::string::String {}
impl<'a> ToWorkflowInput for &'a str {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::String(self.to_string())
    }
}
impl<'a> String for &'a str {}
impl ToWorkflowInput for f32 {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::F32(*self)
    }
}
impl Float for f32 {}
impl ToWorkflowInput for i32 {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::I32(*self)
    }
}
impl Int for i32 {}
impl ToWorkflowInput for bool {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Boolean(*self)
    }
}
impl Boolean for bool {}
///A value of ComfyUI type `AUDIO`.
pub trait Audio: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Audio> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Audio for Box<dyn Audio> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for AudioOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Audio for AudioOut {}
///A value of ComfyUI type `BOOLEAN`.
pub trait Boolean: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Boolean> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Boolean for Box<dyn Boolean> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for BooleanOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Boolean for BooleanOut {}
///A value of ComfyUI type `CLIP_VISION_OUTPUT`.
pub trait ClipVisionOutput: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn ClipVisionOutput> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl ClipVisionOutput for Box<dyn ClipVisionOutput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ClipVisionOutputOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///A value of ComfyUI type `CLIP_VISION`.
pub trait ClipVision: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn ClipVision> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl ClipVision for Box<dyn ClipVision> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ClipVisionOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl ClipVision for ClipVisionOut {}
///A value of ComfyUI type `CLIP`.
pub trait Clip: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Clip> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Clip for Box<dyn Clip> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ClipOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Clip for ClipOut {}
///A value of ComfyUI type `CONDITIONING`.
pub trait Conditioning: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Conditioning> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Conditioning for Box<dyn Conditioning> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ConditioningOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Conditioning for ConditioningOut {}
///A value of ComfyUI type `CONTROL_NET`.
pub trait ControlNet: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn ControlNet> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl ControlNet for Box<dyn ControlNet> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ControlNetOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl ControlNet for ControlNetOut {}
///A value of ComfyUI type `FLOAT`.
pub trait Float: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Float> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Float for Box<dyn Float> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for FloatOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Float for FloatOut {}
///A value of ComfyUI type `GLIGEN`.
pub trait Gligen: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Gligen> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Gligen for Box<dyn Gligen> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for GligenOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Gligen for GligenOut {}
///A value of ComfyUI type `GUIDER`.
pub trait Guider: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Guider> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Guider for Box<dyn Guider> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for GuiderOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Guider for GuiderOut {}
///A value of ComfyUI type `IMAGE`.
pub trait Image: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Image> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Image for Box<dyn Image> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ImageOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Image for ImageOut {}
///A value of ComfyUI type `INPAINT_MODEL`.
pub trait InpaintModel: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn InpaintModel> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl InpaintModel for Box<dyn InpaintModel> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for InpaintModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl InpaintModel for InpaintModelOut {}
///A value of ComfyUI type `INPAINT_PATCH`.
pub trait InpaintPatch: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn InpaintPatch> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl InpaintPatch for Box<dyn InpaintPatch> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for InpaintPatchOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl InpaintPatch for InpaintPatchOut {}
///A value of ComfyUI type `INT`.
pub trait Int: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Int> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Int for Box<dyn Int> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for IntOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Int for IntOut {}
///A value of ComfyUI type `LATENT`.
pub trait Latent: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Latent> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Latent for Box<dyn Latent> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for LatentOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Latent for LatentOut {}
///A value of ComfyUI type `MASK`.
pub trait Mask: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Mask> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Mask for Box<dyn Mask> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for MaskOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Mask for MaskOut {}
///A value of ComfyUI type `MODEL`.
pub trait Model: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Model> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Model for Box<dyn Model> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for ModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Model for ModelOut {}
///A value of ComfyUI type `NOISE`.
pub trait Noise: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Noise> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Noise for Box<dyn Noise> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for NoiseOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Noise for NoiseOut {}
///A value of ComfyUI type `PHOTOMAKER`.
pub trait Photomaker: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Photomaker> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Photomaker for Box<dyn Photomaker> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for PhotomakerOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Photomaker for PhotomakerOut {}
///A value of ComfyUI type `SAMPLER`.
pub trait Sampler: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Sampler> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Sampler for Box<dyn Sampler> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for SamplerOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Sampler for SamplerOut {}
///A value of ComfyUI type `STRING`.
pub trait String: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn String> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl String for Box<dyn String> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for StringOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl String for StringOut {}
///A value of ComfyUI type `SIGMAS`.
pub trait Sigmas: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Sigmas> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Sigmas for Box<dyn Sigmas> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for SigmasOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Sigmas for SigmasOut {}
///A value of ComfyUI type `STYLE_MODEL`.
pub trait StyleModel: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn StyleModel> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl StyleModel for Box<dyn StyleModel> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for StyleModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl StyleModel for StyleModelOut {}
///A value of ComfyUI type `UPSCALE_MODEL`.
pub trait UpscaleModel: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn UpscaleModel> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl UpscaleModel for Box<dyn UpscaleModel> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for UpscaleModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl UpscaleModel for UpscaleModelOut {}
///A value of ComfyUI type `VAE`.
pub trait Vae: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Vae> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Vae for Box<dyn Vae> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for VaeOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Vae for VaeOut {}
///A value of ComfyUI type `WEBCAM`.
pub trait Webcam: ToWorkflowInput {}
impl ToWorkflowInput for Box<dyn Webcam> {
    fn to_workflow_input(&self) -> WorkflowInput {
        self.as_ref().to_workflow_input()
    }
}
impl Webcam for Box<dyn Webcam> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ToWorkflowInput for WebcamOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
    }
}
impl Webcam for WebcamOut {}
