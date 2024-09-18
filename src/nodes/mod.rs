//!Root module
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
/// Implemented for all typed nodes. Provides the node's output and metadata.
pub trait TypedNode {
    /// The type of the node's output.
    type Output;
    /// Returns the node's output.
    fn output(&self) -> Self::Output;
    /// The name of the node.
    const NAME: &'static str;
    /// The display name of the node.
    const DISPLAY_NAME: &'static str;
    /// The description of the node.
    const DESCRIPTION: &'static str;
    /// The category of the node.
    const CATEGORY: &'static str;
}
/// A typed value. Used to determine the value for a given input.
#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
    /// A string value.
    String(std::string::String),
    /// A f32 value.
    F32(f32),
    /// A i32 value.
    I32(i32),
    /// A boolean value.
    Boolean(bool),
    /// A slot value. The inner value is the index of the output slot.
    Slot(u32),
}
/// Converts a value to a typed value for use in a workflow.
pub trait ToTypedValue {
    /// Converts the value to a typed value.
    fn to_typed_value(&self) -> TypedValue;
}
impl ToTypedValue for std::string::String {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::String(self.clone())
    }
}
impl String for std::string::String {}
impl<'a> ToTypedValue for &'a str {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::String(self.to_string())
    }
}
impl<'a> String for &'a str {}
impl ToTypedValue for f32 {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::F32(*self)
    }
}
impl Float for f32 {}
impl ToTypedValue for i32 {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::I32(*self)
    }
}
impl Int for i32 {}
impl ToTypedValue for bool {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Boolean(*self)
    }
}
impl Boolean for bool {}
///**Audio**
pub trait Audio: ToTypedValue {}
impl ToTypedValue for Box<dyn Audio> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Audio for Box<dyn Audio> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut(pub u32);
impl ToTypedValue for AudioOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Audio for AudioOut {}
///**Boolean**
pub trait Boolean: ToTypedValue {}
impl ToTypedValue for Box<dyn Boolean> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Boolean for Box<dyn Boolean> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut(pub u32);
impl ToTypedValue for BooleanOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Boolean for BooleanOut {}
///**ClipVisionOutput**
pub trait ClipVisionOutput: ToTypedValue {}
impl ToTypedValue for Box<dyn ClipVisionOutput> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl ClipVisionOutput for Box<dyn ClipVisionOutput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut(pub u32);
impl ToTypedValue for ClipVisionOutputOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///**ClipVision**
pub trait ClipVision: ToTypedValue {}
impl ToTypedValue for Box<dyn ClipVision> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl ClipVision for Box<dyn ClipVision> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut(pub u32);
impl ToTypedValue for ClipVisionOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ClipVision for ClipVisionOut {}
///**Clip**
pub trait Clip: ToTypedValue {}
impl ToTypedValue for Box<dyn Clip> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Clip for Box<dyn Clip> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut(pub u32);
impl ToTypedValue for ClipOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Clip for ClipOut {}
///**Conditioning**
pub trait Conditioning: ToTypedValue {}
impl ToTypedValue for Box<dyn Conditioning> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Conditioning for Box<dyn Conditioning> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut(pub u32);
impl ToTypedValue for ConditioningOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Conditioning for ConditioningOut {}
///**ControlNet**
pub trait ControlNet: ToTypedValue {}
impl ToTypedValue for Box<dyn ControlNet> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl ControlNet for Box<dyn ControlNet> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut(pub u32);
impl ToTypedValue for ControlNetOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ControlNet for ControlNetOut {}
///**Float**
pub trait Float: ToTypedValue {}
impl ToTypedValue for Box<dyn Float> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Float for Box<dyn Float> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut(pub u32);
impl ToTypedValue for FloatOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Float for FloatOut {}
///**Gligen**
pub trait Gligen: ToTypedValue {}
impl ToTypedValue for Box<dyn Gligen> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Gligen for Box<dyn Gligen> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut(pub u32);
impl ToTypedValue for GligenOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Gligen for GligenOut {}
///**Guider**
pub trait Guider: ToTypedValue {}
impl ToTypedValue for Box<dyn Guider> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Guider for Box<dyn Guider> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut(pub u32);
impl ToTypedValue for GuiderOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Guider for GuiderOut {}
///**Image**
pub trait Image: ToTypedValue {}
impl ToTypedValue for Box<dyn Image> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Image for Box<dyn Image> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut(pub u32);
impl ToTypedValue for ImageOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Image for ImageOut {}
///**InpaintModel**
pub trait InpaintModel: ToTypedValue {}
impl ToTypedValue for Box<dyn InpaintModel> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl InpaintModel for Box<dyn InpaintModel> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut(pub u32);
impl ToTypedValue for InpaintModelOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl InpaintModel for InpaintModelOut {}
///**InpaintPatch**
pub trait InpaintPatch: ToTypedValue {}
impl ToTypedValue for Box<dyn InpaintPatch> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl InpaintPatch for Box<dyn InpaintPatch> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut(pub u32);
impl ToTypedValue for InpaintPatchOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl InpaintPatch for InpaintPatchOut {}
///**Int**
pub trait Int: ToTypedValue {}
impl ToTypedValue for Box<dyn Int> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Int for Box<dyn Int> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut(pub u32);
impl ToTypedValue for IntOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Int for IntOut {}
///**Latent**
pub trait Latent: ToTypedValue {}
impl ToTypedValue for Box<dyn Latent> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Latent for Box<dyn Latent> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut(pub u32);
impl ToTypedValue for LatentOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Latent for LatentOut {}
///**Mask**
pub trait Mask: ToTypedValue {}
impl ToTypedValue for Box<dyn Mask> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Mask for Box<dyn Mask> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut(pub u32);
impl ToTypedValue for MaskOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Mask for MaskOut {}
///**Model**
pub trait Model: ToTypedValue {}
impl ToTypedValue for Box<dyn Model> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Model for Box<dyn Model> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut(pub u32);
impl ToTypedValue for ModelOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Model for ModelOut {}
///**Noise**
pub trait Noise: ToTypedValue {}
impl ToTypedValue for Box<dyn Noise> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Noise for Box<dyn Noise> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut(pub u32);
impl ToTypedValue for NoiseOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Noise for NoiseOut {}
///**Photomaker**
pub trait Photomaker: ToTypedValue {}
impl ToTypedValue for Box<dyn Photomaker> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Photomaker for Box<dyn Photomaker> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut(pub u32);
impl ToTypedValue for PhotomakerOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Photomaker for PhotomakerOut {}
///**Sampler**
pub trait Sampler: ToTypedValue {}
impl ToTypedValue for Box<dyn Sampler> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Sampler for Box<dyn Sampler> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut(pub u32);
impl ToTypedValue for SamplerOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Sampler for SamplerOut {}
///**String**
pub trait String: ToTypedValue {}
impl ToTypedValue for Box<dyn String> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl String for Box<dyn String> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut(pub u32);
impl ToTypedValue for StringOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl String for StringOut {}
///**Sigmas**
pub trait Sigmas: ToTypedValue {}
impl ToTypedValue for Box<dyn Sigmas> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Sigmas for Box<dyn Sigmas> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut(pub u32);
impl ToTypedValue for SigmasOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Sigmas for SigmasOut {}
///**StyleModel**
pub trait StyleModel: ToTypedValue {}
impl ToTypedValue for Box<dyn StyleModel> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl StyleModel for Box<dyn StyleModel> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut(pub u32);
impl ToTypedValue for StyleModelOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl StyleModel for StyleModelOut {}
///**UpscaleModel**
pub trait UpscaleModel: ToTypedValue {}
impl ToTypedValue for Box<dyn UpscaleModel> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl UpscaleModel for Box<dyn UpscaleModel> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut(pub u32);
impl ToTypedValue for UpscaleModelOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl UpscaleModel for UpscaleModelOut {}
///**Vae**
pub trait Vae: ToTypedValue {}
impl ToTypedValue for Box<dyn Vae> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Vae for Box<dyn Vae> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut(pub u32);
impl ToTypedValue for VaeOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Vae for VaeOut {}
///**Webcam**
pub trait Webcam: ToTypedValue {}
impl ToTypedValue for Box<dyn Webcam> {
    fn to_typed_value(&self) -> TypedValue {
        self.as_ref().to_typed_value()
    }
}
impl Webcam for Box<dyn Webcam> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut(pub u32);
impl ToTypedValue for WebcamOut {
    fn to_typed_value(&self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Webcam for WebcamOut {}
