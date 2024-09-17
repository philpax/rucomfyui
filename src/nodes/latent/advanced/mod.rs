//!advanced
///**LatentAdd**
pub struct LatentAdd<Samples1: crate::nodes::Latent, Samples2: crate::nodes::Latent> {
    ///No documentation.
    pub samples1: Samples1,
    ///No documentation.
    pub samples2: Samples2,
}
///**LatentBatchSeedBehavior**
pub struct LatentBatchSeedBehavior<Samples: crate::nodes::Latent> {
    ///No documentation.
    pub samples: Samples,
}
///**LatentInterpolate**
pub struct LatentInterpolate<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
    Ratio: crate::nodes::Float,
> {
    ///No documentation.
    pub samples1: Samples1,
    ///No documentation.
    pub samples2: Samples2,
    ///No documentation.
    pub ratio: Ratio,
}
///**LatentMultiply**
pub struct LatentMultiply<
    Samples: crate::nodes::Latent,
    Multiplier: crate::nodes::Float,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub multiplier: Multiplier,
}
///**LatentSubtract**
pub struct LatentSubtract<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> {
    ///No documentation.
    pub samples1: Samples1,
    ///No documentation.
    pub samples2: Samples2,
}
