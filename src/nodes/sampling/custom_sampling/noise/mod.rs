//!noise
///**DisableNoise**
pub struct DisableNoise {}
///**RandomNoise**
pub struct RandomNoise<NoiseSeed: crate::nodes::Int> {
    ///No documentation.
    pub noise_seed: NoiseSeed,
}
