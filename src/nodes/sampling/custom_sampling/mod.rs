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
///Output for [`SamplerCustom`].
pub struct SamplerCustomOutput {
    ///No documentation.
    pub output: crate::nodes::LatentOut,
    ///No documentation.
    pub denoised_output: crate::nodes::LatentOut,
}
impl<
    Model: crate::nodes::Model,
    AddNoise: crate::nodes::Boolean,
    NoiseSeed: crate::nodes::Int,
    Cfg: crate::nodes::Float,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> crate::nodes::TypedNode
for SamplerCustom<
    Model,
    AddNoise,
    NoiseSeed,
    Cfg,
    Positive,
    Negative,
    Sampler,
    Sigmas,
    LatentImage,
> {
    type Output = SamplerCustomOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            output: crate::nodes::LatentOut(0usize),
            denoised_output: crate::nodes::LatentOut(1usize),
        }
    }
    const NAME: &'static str = "SamplerCustom";
    const DISPLAY_NAME: &'static str = "SamplerCustom";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
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
///Output for [`SamplerCustomAdvanced`].
pub struct SamplerCustomAdvancedOutput {
    ///No documentation.
    pub output: crate::nodes::LatentOut,
    ///No documentation.
    pub denoised_output: crate::nodes::LatentOut,
}
impl<
    Noise: crate::nodes::Noise,
    Guider: crate::nodes::Guider,
    Sampler: crate::nodes::Sampler,
    Sigmas: crate::nodes::Sigmas,
    LatentImage: crate::nodes::Latent,
> crate::nodes::TypedNode
for SamplerCustomAdvanced<Noise, Guider, Sampler, Sigmas, LatentImage> {
    type Output = SamplerCustomAdvancedOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            output: crate::nodes::LatentOut(0usize),
            denoised_output: crate::nodes::LatentOut(1usize),
        }
    }
    const NAME: &'static str = "SamplerCustomAdvanced";
    const DISPLAY_NAME: &'static str = "SamplerCustomAdvanced";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling";
}
