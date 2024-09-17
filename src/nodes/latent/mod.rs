//!latent
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd3;
pub mod stablecascade;
pub mod transform;
#[doc = "**Empty Latent Image**\n\nCreate a new batch of empty latent images to be denoised via sampling."]
pub struct EmptyLatentImage {}
///**Latent Composite**
pub struct LatentComposite {}
///**LatentCompositeMasked**
pub struct LatentCompositeMasked {}
///**Upscale Latent**
pub struct LatentUpscale {}
///**Upscale Latent By**
pub struct LatentUpscaleBy {}
#[doc = "**VAE Decode**\n\nDecodes latent images back into pixel space images."]
pub struct VAEDecode {}
///**VAE Encode**
pub struct VAEEncode {}
