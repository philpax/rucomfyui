//!`advanced` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`LatentAdd`](super::LatentAdd).
    #[derive(Clone)]
    pub struct LatentAddOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentBatchSeedBehavior`](super::LatentBatchSeedBehavior).
    #[derive(Clone)]
    pub struct LatentBatchSeedBehaviorOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentInterpolate`](super::LatentInterpolate).
    #[derive(Clone)]
    pub struct LatentInterpolateOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentMultiply`](super::LatentMultiply).
    #[derive(Clone)]
    pub struct LatentMultiplyOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentSubtract`](super::LatentSubtract).
    #[derive(Clone)]
    pub struct LatentSubtractOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**LatentAdd**
pub struct LatentAdd<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentAdd<Samples1, Samples2> {
    type Output = out::LatentAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
    Samples: crate::nodes::types::Latent,
    SeedBehavior: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub seed_behavior: SeedBehavior,
}
impl<
    Samples: crate::nodes::types::Latent,
    SeedBehavior: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentBatchSeedBehavior<Samples, SeedBehavior> {
    type Output = out::LatentBatchSeedBehaviorOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
    Ratio: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
    ///No documentation.
    pub ratio: Ratio,
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentInterpolate<Samples1, Samples2, Ratio> {
    type Output = out::LatentInterpolateOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
    Samples: crate::nodes::types::Latent,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub multiplier: Multiplier,
}
impl<
    Samples: crate::nodes::types::Latent,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentMultiply<Samples, Multiplier> {
    type Output = out::LatentMultiplyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentSubtract<Samples1, Samples2> {
    type Output = out::LatentSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
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
