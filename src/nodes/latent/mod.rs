//!latent
pub mod advanced;
pub mod audio;
pub mod batch;
pub mod inpaint;
pub mod sd_3;
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
///Output for [`EmptyLatentImage`].
pub struct EmptyLatentImageOutput {
    ///The empty latent image batch.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    BatchSize: crate::nodes::Int,
> crate::nodes::TypedNode for EmptyLatentImage<Width, Height, BatchSize> {
    type Output = EmptyLatentImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "Empty Latent Image";
    const DESCRIPTION: &'static str = "Create a new batch of empty latent images to be denoised via sampling.";
    const CATEGORY: &'static str = "latent";
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
///Output for [`LatentComposite`].
pub struct LatentCompositeOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    SamplesTo: crate::nodes::Latent,
    SamplesFrom: crate::nodes::Latent,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Feather: crate::nodes::Int,
> crate::nodes::TypedNode for LatentComposite<SamplesTo, SamplesFrom, X, Y, Feather> {
    type Output = LatentCompositeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentComposite";
    const DISPLAY_NAME: &'static str = "Latent Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**LatentCompositeMasked**
pub struct LatentCompositeMasked<
    Destination: crate::nodes::Latent,
    Source: crate::nodes::Latent,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask = crate::nodes::MaskOut,
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
    pub mask: Option<Mask>,
}
///Output for [`LatentCompositeMasked`].
pub struct LatentCompositeMaskedOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Destination: crate::nodes::Latent,
    Source: crate::nodes::Latent,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    ResizeSource: crate::nodes::Boolean,
    Mask: crate::nodes::Mask,
> crate::nodes::TypedNode
for LatentCompositeMasked<Destination, Source, X, Y, ResizeSource, Mask> {
    type Output = LatentCompositeMaskedOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentCompositeMasked";
    const DISPLAY_NAME: &'static str = "LatentCompositeMasked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent**
pub struct LatentUpscale<
    Samples: crate::nodes::Latent,
    UpscaleMethod: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Crop: crate::nodes::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub crop: Crop,
}
///Output for [`LatentUpscale`].
pub struct LatentUpscaleOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    UpscaleMethod: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Crop: crate::nodes::String,
> crate::nodes::TypedNode
for LatentUpscale<Samples, UpscaleMethod, Width, Height, Crop> {
    type Output = LatentUpscaleOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentUpscale";
    const DISPLAY_NAME: &'static str = "Upscale Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
///**Upscale Latent By**
pub struct LatentUpscaleBy<
    Samples: crate::nodes::Latent,
    UpscaleMethod: crate::nodes::String,
    ScaleBy: crate::nodes::Float,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
    ///No documentation.
    pub scale_by: ScaleBy,
}
///Output for [`LatentUpscaleBy`].
pub struct LatentUpscaleByOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    UpscaleMethod: crate::nodes::String,
    ScaleBy: crate::nodes::Float,
> crate::nodes::TypedNode for LatentUpscaleBy<Samples, UpscaleMethod, ScaleBy> {
    type Output = LatentUpscaleByOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentUpscaleBy";
    const DISPLAY_NAME: &'static str = "Upscale Latent By";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
#[doc = "**VAE Decode**\n\nDecodes latent images back into pixel space images."]
pub struct VaeDecode<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> {
    ///The latent to be decoded.
    pub samples: Samples,
    ///The VAE model used for decoding the latent.
    pub vae: Vae,
}
///Output for [`VaeDecode`].
pub struct VaeDecodeOutput {
    ///The decoded image.
    pub image: crate::nodes::ImageOut,
}
impl<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> crate::nodes::TypedNode
for VaeDecode<Samples, Vae> {
    type Output = VaeDecodeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "VAEDecode";
    const DISPLAY_NAME: &'static str = "VAE Decode";
    const DESCRIPTION: &'static str = "Decodes latent images back into pixel space images.";
    const CATEGORY: &'static str = "latent";
}
///**VAE Encode**
pub struct VaeEncode<Pixels: crate::nodes::Image, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub vae: Vae,
}
///Output for [`VaeEncode`].
pub struct VaeEncodeOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<Pixels: crate::nodes::Image, Vae: crate::nodes::Vae> crate::nodes::TypedNode
for VaeEncode<Pixels, Vae> {
    type Output = VaeEncodeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "VAEEncode";
    const DISPLAY_NAME: &'static str = "VAE Encode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent";
}
