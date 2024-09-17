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
///**Audio**
pub trait Audio {}
///**Boolean**
pub trait Boolean {}
///**ClipVisionOutput**
pub trait ClipVisionOutput {}
///**ClipVision**
pub trait ClipVision {}
///**Clip**
pub trait Clip {}
///**Conditioning**
pub trait Conditioning {}
///**ControlNet**
pub trait ControlNet {}
///**Float**
pub trait Float {}
///**Gligen**
pub trait Gligen {}
///**Guider**
pub trait Guider {}
///**Image**
pub trait Image {}
///**InpaintModel**
pub trait InpaintModel {}
///**InpaintPatch**
pub trait InpaintPatch {}
///**Int**
pub trait Int {}
///**Latent**
pub trait Latent {}
///**Mask**
pub trait Mask {}
///**Model**
pub trait Model {}
///**Noise**
pub trait Noise {}
///**Photomaker**
pub trait Photomaker {}
///**Sampler**
pub trait Sampler {}
///**String**
pub trait String {}
///**Sigmas**
pub trait Sigmas {}
///**StyleModel**
pub trait StyleModel {}
///**UpscaleModel**
pub trait UpscaleModel {}
///**Vae**
pub trait Vae {}
///**Webcam**
pub trait Webcam {}
