//!advanced
///**LatentAdd**
pub struct LatentAdd<Samples1: crate::nodes::Latent, Samples2: crate::nodes::Latent> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
///Output for [`LatentAdd`].
pub struct LatentAddOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> crate::nodes::TypedNode for LatentAdd<Samples1, Samples2> {
    type Output = LatentAddOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentAdd";
    const DISPLAY_NAME: &'static str = "LatentAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentBatchSeedBehavior**
pub struct LatentBatchSeedBehavior<Samples: crate::nodes::Latent> {
    ///No documentation.
    pub samples: Samples,
}
///Output for [`LatentBatchSeedBehavior`].
pub struct LatentBatchSeedBehaviorOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<Samples: crate::nodes::Latent> crate::nodes::TypedNode
for LatentBatchSeedBehavior<Samples> {
    type Output = LatentBatchSeedBehaviorOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentBatchSeedBehavior";
    const DISPLAY_NAME: &'static str = "LatentBatchSeedBehavior";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentInterpolate**
pub struct LatentInterpolate<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
    Ratio: crate::nodes::Float,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
    ///No documentation.
    pub ratio: Ratio,
}
///Output for [`LatentInterpolate`].
pub struct LatentInterpolateOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
    Ratio: crate::nodes::Float,
> crate::nodes::TypedNode for LatentInterpolate<Samples1, Samples2, Ratio> {
    type Output = LatentInterpolateOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentInterpolate";
    const DISPLAY_NAME: &'static str = "LatentInterpolate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
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
///Output for [`LatentMultiply`].
pub struct LatentMultiplyOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    Multiplier: crate::nodes::Float,
> crate::nodes::TypedNode for LatentMultiply<Samples, Multiplier> {
    type Output = LatentMultiplyOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentMultiply";
    const DISPLAY_NAME: &'static str = "LatentMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentSubtract**
pub struct LatentSubtract<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
///Output for [`LatentSubtract`].
pub struct LatentSubtractOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> crate::nodes::TypedNode for LatentSubtract<Samples1, Samples2> {
    type Output = LatentSubtractOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentSubtract";
    const DISPLAY_NAME: &'static str = "LatentSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
