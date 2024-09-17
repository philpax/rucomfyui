//!sampling
pub mod custom_sampling;
pub mod video_models;
#[doc = "**KSampler**\n\nUses the provided model, positive and negative conditioning to denoise the latent image."]
pub struct KSampler<
    Model: crate::nodes::Model,
    Seed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    Denoise: crate::nodes::Float,
> {
    ///The model used for denoising the input latent.
    pub model: Model,
    ///The random seed used for creating the noise.
    pub seed: Seed,
    ///The number of steps used in the denoising process.
    pub steps: Steps,
    ///The Classifier-Free Guidance scale balances creativity and adherence to the prompt. Higher values result in images more closely matching the prompt however too high values will negatively impact quality.
    pub cfg: Cfg,
    ///The conditioning describing the attributes you want to include in the image.
    pub positive: Positive,
    ///The conditioning describing the attributes you want to exclude from the image.
    pub negative: Negative,
    ///The latent image to denoise.
    pub latent_image: LatentImage,
    ///The amount of denoising applied, lower values will maintain the structure of the initial image allowing for image to image sampling.
    pub denoise: Denoise,
}
///**KSampler (Advanced)**
pub struct KSamplerAdvanced<
    Model: crate::nodes::Model,
    NoiseSeed: crate::nodes::Int,
    Steps: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    LatentImage: crate::nodes::Latent,
    StartAtStep: crate::nodes::Int,
    EndAtStep: crate::nodes::Int,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub noise_seed: NoiseSeed,
    ///No documentation.
    pub steps: Steps,
    ///No documentation.
    pub cfg: Cfg,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub latent_image: LatentImage,
    ///No documentation.
    pub start_at_step: StartAtStep,
    ///No documentation.
    pub end_at_step: EndAtStep,
}
