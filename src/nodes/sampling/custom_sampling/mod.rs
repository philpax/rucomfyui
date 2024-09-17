//!custom_sampling
pub mod guiders;
pub mod noise;
pub mod samplers;
pub mod schedulers;
pub mod sigmas;
///**SamplerCustom**
pub struct SamplerCustom<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::Boolean,
    NoiseSeed: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub add_noise: AddNoise,
    ///No documentation.
    pub noise_seed: NoiseSeed,
    ///No documentation.
    pub cfg: Cfg,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub sampler: Sampler,
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub latent_image: LatentImage,
}
///**SamplerCustomAdvanced**
pub struct SamplerCustomAdvanced<
    Noise: crate::nodes::Noise,
    Guider: crate::nodes::Guider,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> {
    ///No documentation.
    pub noise: Noise,
    ///No documentation.
    pub guider: Guider,
    ///No documentation.
    pub sampler: Sampler,
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub latent_image: LatentImage,
}
