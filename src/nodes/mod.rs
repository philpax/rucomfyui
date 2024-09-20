//!Root module
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
/// Implemented for all typed nodes. Provides the node's output and metadata.
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
///**Audio**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for AudioOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Audio for AudioOut {}
///**Boolean**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for BooleanOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Boolean for BooleanOut {}
///**ClipVisionOutput**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ClipVisionOutputOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///**ClipVision**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ClipVisionOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl ClipVision for ClipVisionOut {}
///**Clip**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ClipOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Clip for ClipOut {}
///**Conditioning**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ConditioningOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Conditioning for ConditioningOut {}
///**ControlNet**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ControlNetOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl ControlNet for ControlNetOut {}
///**Float**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for FloatOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Float for FloatOut {}
///**Gligen**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for GligenOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Gligen for GligenOut {}
///**Guider**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for GuiderOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Guider for GuiderOut {}
///**Image**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ImageOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Image for ImageOut {}
///**InpaintModel**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for InpaintModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl InpaintModel for InpaintModelOut {}
///**InpaintPatch**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for InpaintPatchOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl InpaintPatch for InpaintPatchOut {}
///**Int**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for IntOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Int for IntOut {}
///**Latent**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for LatentOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Latent for LatentOut {}
///**Mask**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for MaskOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Mask for MaskOut {}
///**Model**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for ModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Model for ModelOut {}
///**Noise**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for NoiseOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Noise for NoiseOut {}
///**Photomaker**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for PhotomakerOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Photomaker for PhotomakerOut {}
///**Sampler**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for SamplerOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Sampler for SamplerOut {}
///**String**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for StringOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl String for StringOut {}
///**Sigmas**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for SigmasOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Sigmas for SigmasOut {}
///**StyleModel**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for StyleModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl StyleModel for StyleModelOut {}
///**UpscaleModel**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for UpscaleModelOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl UpscaleModel for UpscaleModelOut {}
///**Vae**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for VaeOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Vae for VaeOut {}
///**Webcam**
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
    node_id: WorkflowNodeId,
    slot: u32,
}
impl ToWorkflowInput for WebcamOut {
    fn to_workflow_input(&self) -> WorkflowInput {
        WorkflowInput::Slot(self.node_id.to_string(), self.slot)
    }
}
impl Webcam for WebcamOut {}
