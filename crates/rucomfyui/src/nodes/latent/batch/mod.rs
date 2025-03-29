//!`batch` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**LatentBatch**: No description.
#[derive(Clone)]
pub struct LatentBatch<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub samples_1: Samples1Param,
    ///No documentation.
    pub samples_2: Samples2Param,
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
> LatentBatch<Samples1Param, Samples2Param> {
    /// Create a new node.
    pub fn new(samples_1: Samples1Param, samples_2: Samples2Param) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentBatch<Samples1Param, Samples2Param> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    SamplesParam: crate::nodes::types::Latent,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 63
  - Min: 0
*/
    pub batch_index: BatchIndexParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub length: LengthParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> LatentFromBatch<SamplesParam, BatchIndexParam, LengthParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        batch_index: BatchIndexParam,
        length: LengthParam,
    ) -> Self {
        Self {
            samples,
            batch_index,
            length,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for LatentFromBatch<SamplesParam, BatchIndexParam, LengthParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    LatentsParam: crate::nodes::types::Latent,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub latents: LatentsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    BatchSizeParam: crate::nodes::types::Int,
> RebatchLatents<LatentsParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(latents: LatentsParam, batch_size: BatchSizeParam) -> Self {
        Self { latents, batch_size }
    }
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchLatents<LatentsParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    SamplesParam: crate::nodes::types::Latent,
    AmountParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub amount: AmountParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    AmountParam: crate::nodes::types::Int,
> RepeatLatentBatch<SamplesParam, AmountParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, amount: AmountParam) -> Self {
        Self { samples, amount }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    AmountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatLatentBatch<SamplesParam, AmountParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
