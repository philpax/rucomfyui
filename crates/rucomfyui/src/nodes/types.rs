//! Definitions for all ComfyUI types.
use crate::workflow::{WorkflowInput, WorkflowNodeId};
/// Implemented for all `*Out` types.
pub trait Out: Sized {
    /// Create an output from a dynamic node. Use carefully.
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self;
    /// Convert the output to a dynamic input.
    fn into_input(self) -> WorkflowInput;
}
impl<T: Out> From<T> for WorkflowInput {
    fn from(value: T) -> Self {
        value.into_input()
    }
}
/// A generic output with no specific type. Can be used when you
/// don't care about the output type.
///
/// Wrapped by the type-safe output types.
#[derive(Clone, Copy)]
pub struct UntypedOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl Out for UntypedOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
    fn into_input(self) -> WorkflowInput {
        self.node_id.to_input_with_slot(self.node_slot)
    }
}
///A value of ComfyUI type `AUDIO`.
pub trait Audio: Clone + Into<WorkflowInput> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut(pub UntypedOut);
impl Out for AudioOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Audio for AudioOut {}
///A value of ComfyUI type `BOOLEAN`.
pub trait Boolean: Clone + Into<WorkflowInput> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut(pub UntypedOut);
impl Out for BooleanOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Boolean for BooleanOut {}
///A value of ComfyUI type `CLIP_VISION_OUTPUT`.
pub trait ClipVisionOutput: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut(pub UntypedOut);
impl Out for ClipVisionOutputOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///A value of ComfyUI type `CLIP_VISION`.
pub trait ClipVision: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut(pub UntypedOut);
impl Out for ClipVisionOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ClipVision for ClipVisionOut {}
///A value of ComfyUI type `CLIP`.
pub trait Clip: Clone + Into<WorkflowInput> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut(pub UntypedOut);
impl Out for ClipOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Clip for ClipOut {}
///A value of ComfyUI type `CONDITIONING`.
pub trait Conditioning: Clone + Into<WorkflowInput> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut(pub UntypedOut);
impl Out for ConditioningOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Conditioning for ConditioningOut {}
///A value of ComfyUI type `CONTROL_NET`.
pub trait ControlNet: Clone + Into<WorkflowInput> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut(pub UntypedOut);
impl Out for ControlNetOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ControlNet for ControlNetOut {}
///A value of ComfyUI type `FLOAT`.
pub trait Float: Clone + Into<WorkflowInput> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut(pub UntypedOut);
impl Out for FloatOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Float for FloatOut {}
///A value of ComfyUI type `GLIGEN`.
pub trait Gligen: Clone + Into<WorkflowInput> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut(pub UntypedOut);
impl Out for GligenOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Gligen for GligenOut {}
///A value of ComfyUI type `GUIDER`.
pub trait Guider: Clone + Into<WorkflowInput> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut(pub UntypedOut);
impl Out for GuiderOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Guider for GuiderOut {}
///A value of ComfyUI type `HOOKS`.
pub trait Hooks: Clone + Into<WorkflowInput> {}
///A node output of type [`Hooks`].
#[derive(Clone, Copy)]
pub struct HooksOut(pub UntypedOut);
impl Out for HooksOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Hooks for HooksOut {}
///A value of ComfyUI type `HOOK_KEYFRAMES`.
pub trait HookKeyframes: Clone + Into<WorkflowInput> {}
///A node output of type [`HookKeyframes`].
#[derive(Clone, Copy)]
pub struct HookKeyframesOut(pub UntypedOut);
impl Out for HookKeyframesOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl HookKeyframes for HookKeyframesOut {}
///A value of ComfyUI type `IMAGE`.
pub trait Image: Clone + Into<WorkflowInput> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut(pub UntypedOut);
impl Out for ImageOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Image for ImageOut {}
///A value of ComfyUI type `INPAINT_MODEL`.
pub trait InpaintModel: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut(pub UntypedOut);
impl Out for InpaintModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl InpaintModel for InpaintModelOut {}
///A value of ComfyUI type `INPAINT_PATCH`.
pub trait InpaintPatch: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut(pub UntypedOut);
impl Out for InpaintPatchOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl InpaintPatch for InpaintPatchOut {}
///A value of ComfyUI type `INT`.
pub trait Int: Clone + Into<WorkflowInput> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut(pub UntypedOut);
impl Out for IntOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Int for IntOut {}
///A value of ComfyUI type `LATENT`.
pub trait Latent: Clone + Into<WorkflowInput> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut(pub UntypedOut);
impl Out for LatentOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Latent for LatentOut {}
///A value of ComfyUI type `LATENT_OPERATION`.
pub trait LatentOperation: Clone + Into<WorkflowInput> {}
///A node output of type [`LatentOperation`].
#[derive(Clone, Copy)]
pub struct LatentOperationOut(pub UntypedOut);
impl Out for LatentOperationOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LatentOperation for LatentOperationOut {}
///A value of ComfyUI type `MASK`.
pub trait Mask: Clone + Into<WorkflowInput> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut(pub UntypedOut);
impl Out for MaskOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Mask for MaskOut {}
///A value of ComfyUI type `MODEL`.
pub trait Model: Clone + Into<WorkflowInput> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut(pub UntypedOut);
impl Out for ModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Model for ModelOut {}
///A value of ComfyUI type `NOISE`.
pub trait Noise: Clone + Into<WorkflowInput> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut(pub UntypedOut);
impl Out for NoiseOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Noise for NoiseOut {}
///A value of ComfyUI type `PHOTOMAKER`.
pub trait Photomaker: Clone + Into<WorkflowInput> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut(pub UntypedOut);
impl Out for PhotomakerOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Photomaker for PhotomakerOut {}
///A value of ComfyUI type `SAMPLER`.
pub trait Sampler: Clone + Into<WorkflowInput> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut(pub UntypedOut);
impl Out for SamplerOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Sampler for SamplerOut {}
///A value of ComfyUI type `STRING`.
pub trait String: Clone + Into<WorkflowInput> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut(pub UntypedOut);
impl Out for StringOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl String for StringOut {}
///A value of ComfyUI type `SIGMAS`.
pub trait Sigmas: Clone + Into<WorkflowInput> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut(pub UntypedOut);
impl Out for SigmasOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Sigmas for SigmasOut {}
///A value of ComfyUI type `STYLE_MODEL`.
pub trait StyleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut(pub UntypedOut);
impl Out for StyleModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl StyleModel for StyleModelOut {}
///A value of ComfyUI type `TIMESTEPS_RANGE`.
pub trait TimestepsRange: Clone + Into<WorkflowInput> {}
///A node output of type [`TimestepsRange`].
#[derive(Clone, Copy)]
pub struct TimestepsRangeOut(pub UntypedOut);
impl Out for TimestepsRangeOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl TimestepsRange for TimestepsRangeOut {}
///A value of ComfyUI type `UPSCALE_MODEL`.
pub trait UpscaleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut(pub UntypedOut);
impl Out for UpscaleModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl UpscaleModel for UpscaleModelOut {}
///A value of ComfyUI type `VAE`.
pub trait Vae: Clone + Into<WorkflowInput> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut(pub UntypedOut);
impl Out for VaeOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Vae for VaeOut {}
///A value of ComfyUI type `WEBCAM`.
pub trait Webcam: Clone + Into<WorkflowInput> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut(pub UntypedOut);
impl Out for WebcamOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Webcam for WebcamOut {}
impl String for std::string::String {}
impl String for &str {}
impl Float for f32 {}
impl Float for f64 {}
impl Int for u32 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for u64 {}
impl Boolean for bool {}
