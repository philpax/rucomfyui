//!noise
///**DisableNoise**
pub struct DisableNoise {}
///Output for [`DisableNoise`].
pub struct DisableNoiseOutput {
    ///No documentation.
    pub noise: crate::nodes::NoiseOut,
}
impl crate::nodes::TypedNode for DisableNoise {
    type Output = DisableNoiseOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            noise: crate::nodes::NoiseOut(0usize),
        }
    }
    const NAME: &'static str = "DisableNoise";
    const DISPLAY_NAME: &'static str = "DisableNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
///**RandomNoise**
pub struct RandomNoise<NoiseSeed: crate::nodes::Int> {
    ///No documentation.
    pub noise_seed: NoiseSeed,
}
///Output for [`RandomNoise`].
pub struct RandomNoiseOutput {
    ///No documentation.
    pub noise: crate::nodes::NoiseOut,
}
impl<NoiseSeed: crate::nodes::Int> crate::nodes::TypedNode for RandomNoise<NoiseSeed> {
    type Output = RandomNoiseOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            noise: crate::nodes::NoiseOut(0usize),
        }
    }
    const NAME: &'static str = "RandomNoise";
    const DISPLAY_NAME: &'static str = "RandomNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
