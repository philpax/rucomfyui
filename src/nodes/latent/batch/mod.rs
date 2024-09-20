//!`batch` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`LatentBatch`](super::LatentBatch).
    #[derive(Clone)]
    pub struct LatentBatchOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentFromBatch`](super::LatentFromBatch).
    #[derive(Clone)]
    pub struct LatentFromBatchOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`RebatchLatents`](super::RebatchLatents).
    #[derive(Clone)]
    pub struct RebatchLatentsOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`RepeatLatentBatch`](super::RepeatLatentBatch).
    #[derive(Clone)]
    pub struct RepeatLatentBatchOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**LatentBatch**: No description.
pub struct LatentBatch<
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
> crate::nodes::TypedNode for LatentBatch<Samples1, Samples2> {
    type Output = out::LatentBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentBatch";
    const DISPLAY_NAME: &'static str = "LatentBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Latent From Batch**: No description.
pub struct LatentFromBatch<
    Samples: crate::nodes::types::Latent,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub batch_index: BatchIndex,
    ///No documentation.
    pub length: Length,
}
impl<
    Samples: crate::nodes::types::Latent,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentFromBatch<Samples, BatchIndex, Length> {
    type Output = out::LatentFromBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentFromBatch";
    const DISPLAY_NAME: &'static str = "Latent From Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Rebatch Latents**: No description.
pub struct RebatchLatents<
    Latents: crate::nodes::types::Latent,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub latents: Latents,
    ///No documentation.
    pub batch_size: BatchSize,
}
impl<
    Latents: crate::nodes::types::Latent,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchLatents<Latents, BatchSize> {
    type Output = out::RebatchLatentsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RebatchLatents";
    const DISPLAY_NAME: &'static str = "Rebatch Latents";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Repeat Latent Batch**: No description.
pub struct RepeatLatentBatch<
    Samples: crate::nodes::types::Latent,
    Amount: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub amount: Amount,
}
impl<
    Samples: crate::nodes::types::Latent,
    Amount: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatLatentBatch<Samples, Amount> {
    type Output = out::RepeatLatentBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RepeatLatentBatch";
    const DISPLAY_NAME: &'static str = "Repeat Latent Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
