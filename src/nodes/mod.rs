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
    fn to_typed_value(self) -> TypedValue;
}
impl ToTypedValue for std::string::String {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::String(self)
    }
}
impl String for std::string::String {}
impl ToTypedValue for f32 {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::F32(self)
    }
}
impl Float for f32 {}
impl ToTypedValue for i32 {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::I32(self)
    }
}
impl Int for i32 {}
impl ToTypedValue for bool {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Boolean(self)
    }
}
impl Boolean for bool {}
///**Audio**
pub trait Audio: ToTypedValue {}
///A node output of type [`Audio`].
pub struct AudioOut(pub u32);
impl ToTypedValue for AudioOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Audio for AudioOut {}
///**Boolean**
pub trait Boolean: ToTypedValue {}
///A node output of type [`Boolean`].
pub struct BooleanOut(pub u32);
impl ToTypedValue for BooleanOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Boolean for BooleanOut {}
///**ClipVisionOutput**
pub trait ClipVisionOutput: ToTypedValue {}
///A node output of type [`ClipVisionOutput`].
pub struct ClipVisionOutputOut(pub u32);
impl ToTypedValue for ClipVisionOutputOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///**ClipVision**
pub trait ClipVision: ToTypedValue {}
///A node output of type [`ClipVision`].
pub struct ClipVisionOut(pub u32);
impl ToTypedValue for ClipVisionOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ClipVision for ClipVisionOut {}
///**Clip**
pub trait Clip: ToTypedValue {}
///A node output of type [`Clip`].
pub struct ClipOut(pub u32);
impl ToTypedValue for ClipOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Clip for ClipOut {}
///**Conditioning**
pub trait Conditioning: ToTypedValue {}
///A node output of type [`Conditioning`].
pub struct ConditioningOut(pub u32);
impl ToTypedValue for ConditioningOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Conditioning for ConditioningOut {}
///**ControlNet**
pub trait ControlNet: ToTypedValue {}
///A node output of type [`ControlNet`].
pub struct ControlNetOut(pub u32);
impl ToTypedValue for ControlNetOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl ControlNet for ControlNetOut {}
///**Float**
pub trait Float: ToTypedValue {}
///A node output of type [`Float`].
pub struct FloatOut(pub u32);
impl ToTypedValue for FloatOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Float for FloatOut {}
///**Gligen**
pub trait Gligen: ToTypedValue {}
///A node output of type [`Gligen`].
pub struct GligenOut(pub u32);
impl ToTypedValue for GligenOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Gligen for GligenOut {}
///**Guider**
pub trait Guider: ToTypedValue {}
///A node output of type [`Guider`].
pub struct GuiderOut(pub u32);
impl ToTypedValue for GuiderOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Guider for GuiderOut {}
///**Image**
pub trait Image: ToTypedValue {}
///A node output of type [`Image`].
pub struct ImageOut(pub u32);
impl ToTypedValue for ImageOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Image for ImageOut {}
///**InpaintModel**
pub trait InpaintModel: ToTypedValue {}
///A node output of type [`InpaintModel`].
pub struct InpaintModelOut(pub u32);
impl ToTypedValue for InpaintModelOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl InpaintModel for InpaintModelOut {}
///**InpaintPatch**
pub trait InpaintPatch: ToTypedValue {}
///A node output of type [`InpaintPatch`].
pub struct InpaintPatchOut(pub u32);
impl ToTypedValue for InpaintPatchOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl InpaintPatch for InpaintPatchOut {}
///**Int**
pub trait Int: ToTypedValue {}
///A node output of type [`Int`].
pub struct IntOut(pub u32);
impl ToTypedValue for IntOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Int for IntOut {}
///**Latent**
pub trait Latent: ToTypedValue {}
///A node output of type [`Latent`].
pub struct LatentOut(pub u32);
impl ToTypedValue for LatentOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Latent for LatentOut {}
///**Mask**
pub trait Mask: ToTypedValue {}
///A node output of type [`Mask`].
pub struct MaskOut(pub u32);
impl ToTypedValue for MaskOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Mask for MaskOut {}
///**Model**
pub trait Model: ToTypedValue {}
///A node output of type [`Model`].
pub struct ModelOut(pub u32);
impl ToTypedValue for ModelOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Model for ModelOut {}
///**Noise**
pub trait Noise: ToTypedValue {}
///A node output of type [`Noise`].
pub struct NoiseOut(pub u32);
impl ToTypedValue for NoiseOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Noise for NoiseOut {}
///**Photomaker**
pub trait Photomaker: ToTypedValue {}
///A node output of type [`Photomaker`].
pub struct PhotomakerOut(pub u32);
impl ToTypedValue for PhotomakerOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Photomaker for PhotomakerOut {}
///**Sampler**
pub trait Sampler: ToTypedValue {}
///A node output of type [`Sampler`].
pub struct SamplerOut(pub u32);
impl ToTypedValue for SamplerOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Sampler for SamplerOut {}
///**String**
pub trait String: ToTypedValue {}
///A node output of type [`String`].
pub struct StringOut(pub u32);
impl ToTypedValue for StringOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl String for StringOut {}
///**Sigmas**
pub trait Sigmas: ToTypedValue {}
///A node output of type [`Sigmas`].
pub struct SigmasOut(pub u32);
impl ToTypedValue for SigmasOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Sigmas for SigmasOut {}
///**StyleModel**
pub trait StyleModel: ToTypedValue {}
///A node output of type [`StyleModel`].
pub struct StyleModelOut(pub u32);
impl ToTypedValue for StyleModelOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl StyleModel for StyleModelOut {}
///**UpscaleModel**
pub trait UpscaleModel: ToTypedValue {}
///A node output of type [`UpscaleModel`].
pub struct UpscaleModelOut(pub u32);
impl ToTypedValue for UpscaleModelOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl UpscaleModel for UpscaleModelOut {}
///**Vae**
pub trait Vae: ToTypedValue {}
///A node output of type [`Vae`].
pub struct VaeOut(pub u32);
impl ToTypedValue for VaeOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Vae for VaeOut {}
///**Webcam**
pub trait Webcam: ToTypedValue {}
///A node output of type [`Webcam`].
pub struct WebcamOut(pub u32);
impl ToTypedValue for WebcamOut {
    fn to_typed_value(self) -> TypedValue {
        TypedValue::Slot(self.0)
    }
}
impl Webcam for WebcamOut {}
