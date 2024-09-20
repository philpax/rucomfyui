//!`batch` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**LatentBatch**
pub struct LatentBatch<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub samples_1: Samples1,
    ///No documentation.
    pub samples_2: Samples2,
}
///Output for [`LatentBatch`].
#[derive(Clone)]
pub struct LatentBatchOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentBatch<Samples1, Samples2> {
    type Output = LatentBatchOutput;
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
///**Latent From Batch**
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
///Output for [`LatentFromBatch`].
#[derive(Clone)]
pub struct LatentFromBatchOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentFromBatch<Samples, BatchIndex, Length> {
    type Output = LatentFromBatchOutput;
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
///**Rebatch Latents**
pub struct RebatchLatents<
    Latents: crate::nodes::types::Latent,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub latents: Latents,
    ///No documentation.
    pub batch_size: BatchSize,
}
///Output for [`RebatchLatents`].
#[derive(Clone)]
pub struct RebatchLatentsOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Latents: crate::nodes::types::Latent,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchLatents<Latents, BatchSize> {
    type Output = RebatchLatentsOutput;
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
///**Repeat Latent Batch**
pub struct RepeatLatentBatch<
    Samples: crate::nodes::types::Latent,
    Amount: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub amount: Amount,
}
///Output for [`RepeatLatentBatch`].
#[derive(Clone)]
pub struct RepeatLatentBatchOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    Amount: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatLatentBatch<Samples, Amount> {
    type Output = RepeatLatentBatchOutput;
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
