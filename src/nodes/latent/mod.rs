//!latent
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd3;
pub mod stable_cascade;
pub mod transform;
#[doc = "**Empty Latent Image**\n\nCreate a new batch of empty latent images to be denoised via sampling."]
pub struct EmptyLatentImage<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> {
    ///The width of the latent images in pixels.
    pub width: Width,
    ///The height of the latent images in pixels.
    pub height: Height,
    ///The number of latent images in the batch.
    pub batch_size: BatchSize,
}
///**Latent Composite**
pub struct LatentComposite<
    SamplesTo: crate::nodes::Latent,
    SamplesFrom: crate::nodes::Latent,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Feather: crate::nodes::Int,
> {
    ///No documentation.
    pub samples_to: SamplesTo,
    ///No documentation.
    pub samples_from: SamplesFrom,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub feather: Feather,
}
///**LatentCompositeMasked**
pub struct LatentCompositeMasked<
    Destination: crate::nodes::Latent,
    Source: crate::nodes::Latent,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub resize_source: ResizeSource,
    ///No documentation.
    pub mask: Mask,
}
///**Upscale Latent**
pub struct LatentUpscale<
    Samples: crate::nodes::Latent,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///**Upscale Latent By**
pub struct LatentUpscaleBy<Samples: crate::nodes::Latent, ScaleBy: crate::nodes::Float> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub scale_by: ScaleBy,
}
#[doc = "**VAE Decode**\n\nDecodes latent images back into pixel space images."]
pub struct VaeDecode<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> {
    ///The latent to be decoded.
    pub samples: Samples,
    ///The VAE model used for decoding the latent.
    pub vae: Vae,
}
///**VAE Encode**
pub struct VaeEncode<Pixels: crate::nodes::Image, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
}
