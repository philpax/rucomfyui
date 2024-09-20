//! Definitions for all ComfyUI types.
use crate::workflow::{WorkflowInput, WorkflowNodeId};
///A value of ComfyUI type `AUDIO`.
pub trait Audio: Clone + Into<WorkflowInput> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl AudioOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<AudioOut> for WorkflowInput {
    fn from(value: AudioOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Audio for AudioOut {}
///A value of ComfyUI type `BOOLEAN`.
pub trait Boolean: Clone + Into<WorkflowInput> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl BooleanOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<BooleanOut> for WorkflowInput {
    fn from(value: BooleanOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Boolean for BooleanOut {}
///A value of ComfyUI type `CLIP_VISION_OUTPUT`.
pub trait ClipVisionOutput: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ClipVisionOutputOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ClipVisionOutputOut> for WorkflowInput {
    fn from(value: ClipVisionOutputOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///A value of ComfyUI type `CLIP_VISION`.
pub trait ClipVision: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ClipVisionOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ClipVisionOut> for WorkflowInput {
    fn from(value: ClipVisionOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl ClipVision for ClipVisionOut {}
///A value of ComfyUI type `CLIP`.
pub trait Clip: Clone + Into<WorkflowInput> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ClipOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ClipOut> for WorkflowInput {
    fn from(value: ClipOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Clip for ClipOut {}
///A value of ComfyUI type `CONDITIONING`.
pub trait Conditioning: Clone + Into<WorkflowInput> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ConditioningOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ConditioningOut> for WorkflowInput {
    fn from(value: ConditioningOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Conditioning for ConditioningOut {}
///A value of ComfyUI type `CONTROL_NET`.
pub trait ControlNet: Clone + Into<WorkflowInput> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ControlNetOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ControlNetOut> for WorkflowInput {
    fn from(value: ControlNetOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl ControlNet for ControlNetOut {}
///A value of ComfyUI type `FLOAT`.
pub trait Float: Clone + Into<WorkflowInput> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl FloatOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<FloatOut> for WorkflowInput {
    fn from(value: FloatOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Float for FloatOut {}
///A value of ComfyUI type `GLIGEN`.
pub trait Gligen: Clone + Into<WorkflowInput> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl GligenOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<GligenOut> for WorkflowInput {
    fn from(value: GligenOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Gligen for GligenOut {}
///A value of ComfyUI type `GUIDER`.
pub trait Guider: Clone + Into<WorkflowInput> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl GuiderOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<GuiderOut> for WorkflowInput {
    fn from(value: GuiderOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Guider for GuiderOut {}
///A value of ComfyUI type `IMAGE`.
pub trait Image: Clone + Into<WorkflowInput> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ImageOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ImageOut> for WorkflowInput {
    fn from(value: ImageOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Image for ImageOut {}
///A value of ComfyUI type `INPAINT_MODEL`.
pub trait InpaintModel: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl InpaintModelOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<InpaintModelOut> for WorkflowInput {
    fn from(value: InpaintModelOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl InpaintModel for InpaintModelOut {}
///A value of ComfyUI type `INPAINT_PATCH`.
pub trait InpaintPatch: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl InpaintPatchOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<InpaintPatchOut> for WorkflowInput {
    fn from(value: InpaintPatchOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl InpaintPatch for InpaintPatchOut {}
///A value of ComfyUI type `INT`.
pub trait Int: Clone + Into<WorkflowInput> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl IntOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<IntOut> for WorkflowInput {
    fn from(value: IntOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Int for IntOut {}
///A value of ComfyUI type `LATENT`.
pub trait Latent: Clone + Into<WorkflowInput> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl LatentOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<LatentOut> for WorkflowInput {
    fn from(value: LatentOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Latent for LatentOut {}
///A value of ComfyUI type `MASK`.
pub trait Mask: Clone + Into<WorkflowInput> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl MaskOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<MaskOut> for WorkflowInput {
    fn from(value: MaskOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Mask for MaskOut {}
///A value of ComfyUI type `MODEL`.
pub trait Model: Clone + Into<WorkflowInput> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl ModelOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<ModelOut> for WorkflowInput {
    fn from(value: ModelOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Model for ModelOut {}
///A value of ComfyUI type `NOISE`.
pub trait Noise: Clone + Into<WorkflowInput> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl NoiseOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<NoiseOut> for WorkflowInput {
    fn from(value: NoiseOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Noise for NoiseOut {}
///A value of ComfyUI type `PHOTOMAKER`.
pub trait Photomaker: Clone + Into<WorkflowInput> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl PhotomakerOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<PhotomakerOut> for WorkflowInput {
    fn from(value: PhotomakerOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Photomaker for PhotomakerOut {}
///A value of ComfyUI type `SAMPLER`.
pub trait Sampler: Clone + Into<WorkflowInput> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl SamplerOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<SamplerOut> for WorkflowInput {
    fn from(value: SamplerOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Sampler for SamplerOut {}
///A value of ComfyUI type `STRING`.
pub trait String: Clone + Into<WorkflowInput> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl StringOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<StringOut> for WorkflowInput {
    fn from(value: StringOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl String for StringOut {}
///A value of ComfyUI type `SIGMAS`.
pub trait Sigmas: Clone + Into<WorkflowInput> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl SigmasOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<SigmasOut> for WorkflowInput {
    fn from(value: SigmasOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Sigmas for SigmasOut {}
///A value of ComfyUI type `STYLE_MODEL`.
pub trait StyleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl StyleModelOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<StyleModelOut> for WorkflowInput {
    fn from(value: StyleModelOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl StyleModel for StyleModelOut {}
///A value of ComfyUI type `UPSCALE_MODEL`.
pub trait UpscaleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl UpscaleModelOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<UpscaleModelOut> for WorkflowInput {
    fn from(value: UpscaleModelOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl UpscaleModel for UpscaleModelOut {}
///A value of ComfyUI type `VAE`.
pub trait Vae: Clone + Into<WorkflowInput> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl VaeOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<VaeOut> for WorkflowInput {
    fn from(value: VaeOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Vae for VaeOut {}
///A value of ComfyUI type `WEBCAM`.
pub trait Webcam: Clone + Into<WorkflowInput> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl WebcamOut {
    /// Create an output from a dynamic node. Use carefully.
    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
}
impl From<WebcamOut> for WorkflowInput {
    fn from(value: WebcamOut) -> Self {
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
impl Webcam for WebcamOut {}
impl String for std::string::String {}
impl<'a> String for &'a str {}
impl Float for f32 {}
impl Int for i32 {}
impl Boolean for bool {}
