//!`advanced` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**LatentAdd**
pub struct LatentAdd<Samples1: crate::nodes::Latent, Samples2: crate::nodes::Latent> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
///Output for [`LatentAdd`].
#[derive(Clone)]
pub struct LatentAddOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> crate::nodes::TypedNode for LatentAdd<Samples1, Samples2> {
    type Output = LatentAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentAdd";
    const DISPLAY_NAME: &'static str = "LatentAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentBatchSeedBehavior**
pub struct LatentBatchSeedBehavior<
    Samples: crate::nodes::Latent,
    SeedBehavior: crate::nodes::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub seed_behavior: SeedBehavior,
}
///Output for [`LatentBatchSeedBehavior`].
#[derive(Clone)]
pub struct LatentBatchSeedBehaviorOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    SeedBehavior: crate::nodes::String,
> crate::nodes::TypedNode for LatentBatchSeedBehavior<Samples, SeedBehavior> {
    type Output = LatentBatchSeedBehaviorOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
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
#[derive(Clone)]
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
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
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
#[derive(Clone)]
pub struct LatentMultiplyOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    Multiplier: crate::nodes::Float,
> crate::nodes::TypedNode for LatentMultiply<Samples, Multiplier> {
    type Output = LatentMultiplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
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
#[derive(Clone)]
pub struct LatentSubtractOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples1: crate::nodes::Latent,
    Samples2: crate::nodes::Latent,
> crate::nodes::TypedNode for LatentSubtract<Samples1, Samples2> {
    type Output = LatentSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentSubtract";
    const DISPLAY_NAME: &'static str = "LatentSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
