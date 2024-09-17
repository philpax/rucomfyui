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
///**Audio**
pub trait Audio {}
///A node output of type [`Audio`].
pub struct AudioOut(pub usize);
impl Audio for AudioOut {}
///**Boolean**
pub trait Boolean {}
///A node output of type [`Boolean`].
pub struct BooleanOut(pub usize);
impl Boolean for BooleanOut {}
///**ClipVisionOutput**
pub trait ClipVisionOutput {}
///A node output of type [`ClipVisionOutput`].
pub struct ClipVisionOutputOut(pub usize);
impl ClipVisionOutput for ClipVisionOutputOut {}
///**ClipVision**
pub trait ClipVision {}
///A node output of type [`ClipVision`].
pub struct ClipVisionOut(pub usize);
impl ClipVision for ClipVisionOut {}
///**Clip**
pub trait Clip {}
///A node output of type [`Clip`].
pub struct ClipOut(pub usize);
impl Clip for ClipOut {}
///**Conditioning**
pub trait Conditioning {}
///A node output of type [`Conditioning`].
pub struct ConditioningOut(pub usize);
impl Conditioning for ConditioningOut {}
///**ControlNet**
pub trait ControlNet {}
///A node output of type [`ControlNet`].
pub struct ControlNetOut(pub usize);
impl ControlNet for ControlNetOut {}
///**Float**
pub trait Float {}
///A node output of type [`Float`].
pub struct FloatOut(pub usize);
impl Float for FloatOut {}
///**Gligen**
pub trait Gligen {}
///A node output of type [`Gligen`].
pub struct GligenOut(pub usize);
impl Gligen for GligenOut {}
///**Guider**
pub trait Guider {}
///A node output of type [`Guider`].
pub struct GuiderOut(pub usize);
impl Guider for GuiderOut {}
///**Image**
pub trait Image {}
///A node output of type [`Image`].
pub struct ImageOut(pub usize);
impl Image for ImageOut {}
///**InpaintModel**
pub trait InpaintModel {}
///A node output of type [`InpaintModel`].
pub struct InpaintModelOut(pub usize);
impl InpaintModel for InpaintModelOut {}
///**InpaintPatch**
pub trait InpaintPatch {}
///A node output of type [`InpaintPatch`].
pub struct InpaintPatchOut(pub usize);
impl InpaintPatch for InpaintPatchOut {}
///**Int**
pub trait Int {}
///A node output of type [`Int`].
pub struct IntOut(pub usize);
impl Int for IntOut {}
///**Latent**
pub trait Latent {}
///A node output of type [`Latent`].
pub struct LatentOut(pub usize);
impl Latent for LatentOut {}
///**Mask**
pub trait Mask {}
///A node output of type [`Mask`].
pub struct MaskOut(pub usize);
impl Mask for MaskOut {}
///**Model**
pub trait Model {}
///A node output of type [`Model`].
pub struct ModelOut(pub usize);
impl Model for ModelOut {}
///**Noise**
pub trait Noise {}
///A node output of type [`Noise`].
pub struct NoiseOut(pub usize);
impl Noise for NoiseOut {}
///**Photomaker**
pub trait Photomaker {}
///A node output of type [`Photomaker`].
pub struct PhotomakerOut(pub usize);
impl Photomaker for PhotomakerOut {}
///**Sampler**
pub trait Sampler {}
///A node output of type [`Sampler`].
pub struct SamplerOut(pub usize);
impl Sampler for SamplerOut {}
///**String**
pub trait String {}
///A node output of type [`String`].
pub struct StringOut(pub usize);
impl String for StringOut {}
///**Sigmas**
pub trait Sigmas {}
///A node output of type [`Sigmas`].
pub struct SigmasOut(pub usize);
impl Sigmas for SigmasOut {}
///**StyleModel**
pub trait StyleModel {}
///A node output of type [`StyleModel`].
pub struct StyleModelOut(pub usize);
impl StyleModel for StyleModelOut {}
///**UpscaleModel**
pub trait UpscaleModel {}
///A node output of type [`UpscaleModel`].
pub struct UpscaleModelOut(pub usize);
impl UpscaleModel for UpscaleModelOut {}
///**Vae**
pub trait Vae {}
///A node output of type [`Vae`].
pub struct VaeOut(pub usize);
impl Vae for VaeOut {}
///**Webcam**
pub trait Webcam {}
///A node output of type [`Webcam`].
pub struct WebcamOut(pub usize);
impl Webcam for WebcamOut {}
