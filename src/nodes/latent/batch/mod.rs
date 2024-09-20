//!`batch` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**LatentBatch**: No description.
#[derive(Clone)]
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
> LatentBatch<Samples1, Samples2> {
    /// Create a new node.
    pub fn new(samples_1: Samples1, samples_2: Samples2) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentBatch<Samples1, Samples2> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.clone().into());
        output.insert("samples2".to_string(), self.samples_2.clone().into());
        output
    }
    const NAME: &'static str = "LatentBatch";
    const DISPLAY_NAME: &'static str = "LatentBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Latent From Batch**: No description.
#[derive(Clone)]
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
> LatentFromBatch<Samples, BatchIndex, Length> {
    /// Create a new node.
    pub fn new(samples: Samples, batch_index: BatchIndex, length: Length) -> Self {
        Self {
            samples,
            batch_index,
            length,
        }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentFromBatch<Samples, BatchIndex, Length> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("batch_index".to_string(), self.batch_index.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output
    }
    const NAME: &'static str = "LatentFromBatch";
    const DISPLAY_NAME: &'static str = "Latent From Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Rebatch Latents**: No description.
#[derive(Clone)]
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
> RebatchLatents<Latents, BatchSize> {
    /// Create a new node.
    pub fn new(latents: Latents, batch_size: BatchSize) -> Self {
        Self { latents, batch_size }
    }
}
impl<
    Latents: crate::nodes::types::Latent,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchLatents<Latents, BatchSize> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("latents".to_string(), self.latents.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "RebatchLatents";
    const DISPLAY_NAME: &'static str = "Rebatch Latents";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
///**Repeat Latent Batch**: No description.
#[derive(Clone)]
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
> RepeatLatentBatch<Samples, Amount> {
    /// Create a new node.
    pub fn new(samples: Samples, amount: Amount) -> Self {
        Self { samples, amount }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Amount: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatLatentBatch<Samples, Amount> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("amount".to_string(), self.amount.clone().into());
        output
    }
    const NAME: &'static str = "RepeatLatentBatch";
    const DISPLAY_NAME: &'static str = "Repeat Latent Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/batch";
}
