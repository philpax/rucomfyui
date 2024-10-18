//! Definitions for all ComfyUI types.
use crate::workflow::{WorkflowInput, WorkflowNodeId};
/// Implemented for all `*Out` types.
pub trait Out: Sized {
    /// Create an output from a dynamic node. Use carefully.
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self;
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
impl<T: From<UntypedOut>> Out for T {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        T::from(UntypedOut { node_id, node_slot })
    }
}
impl<T: Into<UntypedOut>> From<T> for WorkflowInput {
    fn from(value: T) -> Self {
        let value: UntypedOut = value.into();
        value.node_id.to_input_with_slot(value.node_slot)
    }
}
///A value of ComfyUI type `AUDIO`.
pub trait Audio: Clone + Into<WorkflowInput> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut(pub UntypedOut);
impl From<UntypedOut> for AudioOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<AudioOut> for UntypedOut {
    fn from(value: AudioOut) -> Self {
        value.0
    }
}
impl Audio for AudioOut {}
///A value of ComfyUI type `BOOLEAN`.
pub trait Boolean: Clone + Into<WorkflowInput> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut(pub UntypedOut);
impl From<UntypedOut> for BooleanOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<BooleanOut> for UntypedOut {
    fn from(value: BooleanOut) -> Self {
        value.0
    }
}
impl Boolean for BooleanOut {}
///A value of ComfyUI type `CLIP_VISION_OUTPUT`.
pub trait ClipVisionOutput: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut(pub UntypedOut);
impl From<UntypedOut> for ClipVisionOutputOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ClipVisionOutputOut> for UntypedOut {
    fn from(value: ClipVisionOutputOut) -> Self {
        value.0
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///A value of ComfyUI type `CLIP_VISION`.
pub trait ClipVision: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut(pub UntypedOut);
impl From<UntypedOut> for ClipVisionOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ClipVisionOut> for UntypedOut {
    fn from(value: ClipVisionOut) -> Self {
        value.0
    }
}
impl ClipVision for ClipVisionOut {}
///A value of ComfyUI type `CLIP`.
pub trait Clip: Clone + Into<WorkflowInput> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut(pub UntypedOut);
impl From<UntypedOut> for ClipOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ClipOut> for UntypedOut {
    fn from(value: ClipOut) -> Self {
        value.0
    }
}
impl Clip for ClipOut {}
///A value of ComfyUI type `CONDITIONING`.
pub trait Conditioning: Clone + Into<WorkflowInput> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut(pub UntypedOut);
impl From<UntypedOut> for ConditioningOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ConditioningOut> for UntypedOut {
    fn from(value: ConditioningOut) -> Self {
        value.0
    }
}
impl Conditioning for ConditioningOut {}
///A value of ComfyUI type `CONTROL_NET`.
pub trait ControlNet: Clone + Into<WorkflowInput> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut(pub UntypedOut);
impl From<UntypedOut> for ControlNetOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ControlNetOut> for UntypedOut {
    fn from(value: ControlNetOut) -> Self {
        value.0
    }
}
impl ControlNet for ControlNetOut {}
///A value of ComfyUI type `FLOAT`.
pub trait Float: Clone + Into<WorkflowInput> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut(pub UntypedOut);
impl From<UntypedOut> for FloatOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<FloatOut> for UntypedOut {
    fn from(value: FloatOut) -> Self {
        value.0
    }
}
impl Float for FloatOut {}
///A value of ComfyUI type `GLIGEN`.
pub trait Gligen: Clone + Into<WorkflowInput> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut(pub UntypedOut);
impl From<UntypedOut> for GligenOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<GligenOut> for UntypedOut {
    fn from(value: GligenOut) -> Self {
        value.0
    }
}
impl Gligen for GligenOut {}
///A value of ComfyUI type `GUIDER`.
pub trait Guider: Clone + Into<WorkflowInput> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut(pub UntypedOut);
impl From<UntypedOut> for GuiderOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<GuiderOut> for UntypedOut {
    fn from(value: GuiderOut) -> Self {
        value.0
    }
}
impl Guider for GuiderOut {}
///A value of ComfyUI type `IMAGE`.
pub trait Image: Clone + Into<WorkflowInput> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut(pub UntypedOut);
impl From<UntypedOut> for ImageOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ImageOut> for UntypedOut {
    fn from(value: ImageOut) -> Self {
        value.0
    }
}
impl Image for ImageOut {}
///A value of ComfyUI type `INPAINT_MODEL`.
pub trait InpaintModel: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut(pub UntypedOut);
impl From<UntypedOut> for InpaintModelOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<InpaintModelOut> for UntypedOut {
    fn from(value: InpaintModelOut) -> Self {
        value.0
    }
}
impl InpaintModel for InpaintModelOut {}
///A value of ComfyUI type `INPAINT_PATCH`.
pub trait InpaintPatch: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut(pub UntypedOut);
impl From<UntypedOut> for InpaintPatchOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<InpaintPatchOut> for UntypedOut {
    fn from(value: InpaintPatchOut) -> Self {
        value.0
    }
}
impl InpaintPatch for InpaintPatchOut {}
///A value of ComfyUI type `INT`.
pub trait Int: Clone + Into<WorkflowInput> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut(pub UntypedOut);
impl From<UntypedOut> for IntOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<IntOut> for UntypedOut {
    fn from(value: IntOut) -> Self {
        value.0
    }
}
impl Int for IntOut {}
///A value of ComfyUI type `LATENT`.
pub trait Latent: Clone + Into<WorkflowInput> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut(pub UntypedOut);
impl From<UntypedOut> for LatentOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<LatentOut> for UntypedOut {
    fn from(value: LatentOut) -> Self {
        value.0
    }
}
impl Latent for LatentOut {}
///A value of ComfyUI type `LATENT_OPERATION`.
pub trait LatentOperation: Clone + Into<WorkflowInput> {}
///A node output of type [`LatentOperation`].
#[derive(Clone, Copy)]
pub struct LatentOperationOut(pub UntypedOut);
impl From<UntypedOut> for LatentOperationOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<LatentOperationOut> for UntypedOut {
    fn from(value: LatentOperationOut) -> Self {
        value.0
    }
}
impl LatentOperation for LatentOperationOut {}
///A value of ComfyUI type `MASK`.
pub trait Mask: Clone + Into<WorkflowInput> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut(pub UntypedOut);
impl From<UntypedOut> for MaskOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<MaskOut> for UntypedOut {
    fn from(value: MaskOut) -> Self {
        value.0
    }
}
impl Mask for MaskOut {}
///A value of ComfyUI type `MODEL`.
pub trait Model: Clone + Into<WorkflowInput> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut(pub UntypedOut);
impl From<UntypedOut> for ModelOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<ModelOut> for UntypedOut {
    fn from(value: ModelOut) -> Self {
        value.0
    }
}
impl Model for ModelOut {}
///A value of ComfyUI type `NOISE`.
pub trait Noise: Clone + Into<WorkflowInput> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut(pub UntypedOut);
impl From<UntypedOut> for NoiseOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<NoiseOut> for UntypedOut {
    fn from(value: NoiseOut) -> Self {
        value.0
    }
}
impl Noise for NoiseOut {}
///A value of ComfyUI type `PHOTOMAKER`.
pub trait Photomaker: Clone + Into<WorkflowInput> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut(pub UntypedOut);
impl From<UntypedOut> for PhotomakerOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<PhotomakerOut> for UntypedOut {
    fn from(value: PhotomakerOut) -> Self {
        value.0
    }
}
impl Photomaker for PhotomakerOut {}
///A value of ComfyUI type `SAMPLER`.
pub trait Sampler: Clone + Into<WorkflowInput> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut(pub UntypedOut);
impl From<UntypedOut> for SamplerOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<SamplerOut> for UntypedOut {
    fn from(value: SamplerOut) -> Self {
        value.0
    }
}
impl Sampler for SamplerOut {}
///A value of ComfyUI type `STRING`.
pub trait String: Clone + Into<WorkflowInput> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut(pub UntypedOut);
impl From<UntypedOut> for StringOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<StringOut> for UntypedOut {
    fn from(value: StringOut) -> Self {
        value.0
    }
}
impl String for StringOut {}
///A value of ComfyUI type `SIGMAS`.
pub trait Sigmas: Clone + Into<WorkflowInput> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut(pub UntypedOut);
impl From<UntypedOut> for SigmasOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<SigmasOut> for UntypedOut {
    fn from(value: SigmasOut) -> Self {
        value.0
    }
}
impl Sigmas for SigmasOut {}
///A value of ComfyUI type `STYLE_MODEL`.
pub trait StyleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut(pub UntypedOut);
impl From<UntypedOut> for StyleModelOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<StyleModelOut> for UntypedOut {
    fn from(value: StyleModelOut) -> Self {
        value.0
    }
}
impl StyleModel for StyleModelOut {}
///A value of ComfyUI type `UPSCALE_MODEL`.
pub trait UpscaleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut(pub UntypedOut);
impl From<UntypedOut> for UpscaleModelOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<UpscaleModelOut> for UntypedOut {
    fn from(value: UpscaleModelOut) -> Self {
        value.0
    }
}
impl UpscaleModel for UpscaleModelOut {}
///A value of ComfyUI type `VAE`.
pub trait Vae: Clone + Into<WorkflowInput> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut(pub UntypedOut);
impl From<UntypedOut> for VaeOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<VaeOut> for UntypedOut {
    fn from(value: VaeOut) -> Self {
        value.0
    }
}
impl Vae for VaeOut {}
///A value of ComfyUI type `WEBCAM`.
pub trait Webcam: Clone + Into<WorkflowInput> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut(pub UntypedOut);
impl From<UntypedOut> for WebcamOut {
    fn from(value: UntypedOut) -> Self {
        Self(value)
    }
}
impl From<WebcamOut> for UntypedOut {
    fn from(value: WebcamOut) -> Self {
        value.0
    }
}
impl Webcam for WebcamOut {}
impl String for std::string::String {}
impl<'a> String for &'a str {}
impl Float for f32 {}
impl Float for f64 {}
impl Int for u32 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for u64 {}
impl Boolean for bool {}
