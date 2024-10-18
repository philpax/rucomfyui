//!`operations` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**LatentApplyOperation**: No description.
#[derive(Clone)]
pub struct LatentApplyOperation<
    Samples: crate::nodes::types::Latent,
    Operation: crate::nodes::types::LatentOperation,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub operation: Operation,
}
impl<
    Samples: crate::nodes::types::Latent,
    Operation: crate::nodes::types::LatentOperation,
> LatentApplyOperation<Samples, Operation> {
    /// Create a new node.
    pub fn new(samples: Samples, operation: Operation) -> Self {
        Self { samples, operation }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Operation: crate::nodes::types::LatentOperation,
> crate::nodes::TypedNode for LatentApplyOperation<Samples, Operation> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("operation".to_string(), self.operation.clone().into());
        output
    }
    const NAME: &'static str = "LatentApplyOperation";
    const DISPLAY_NAME: &'static str = "LatentApplyOperation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced/operations";
}
///**LatentApplyOperationCFG**: No description.
#[derive(Clone)]
pub struct LatentApplyOperationCfg<
    Model: crate::nodes::types::Model,
    Operation: crate::nodes::types::LatentOperation,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub operation: Operation,
}
impl<
    Model: crate::nodes::types::Model,
    Operation: crate::nodes::types::LatentOperation,
> LatentApplyOperationCfg<Model, Operation> {
    /// Create a new node.
    pub fn new(model: Model, operation: Operation) -> Self {
        Self { model, operation }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Operation: crate::nodes::types::LatentOperation,
> crate::nodes::TypedNode for LatentApplyOperationCfg<Model, Operation> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("operation".to_string(), self.operation.clone().into());
        output
    }
    const NAME: &'static str = "LatentApplyOperationCFG";
    const DISPLAY_NAME: &'static str = "LatentApplyOperationCFG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced/operations";
}
///**LatentOperationSharpen**: No description.
#[derive(Clone)]
pub struct LatentOperationSharpen<
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 9
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub sharpen_radius: SharpenRadius,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.1
*/
    pub sigma: Sigma,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 5
  - Min: 0
  - Step: 0.01
*/
    pub alpha: Alpha,
}
impl<
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> LatentOperationSharpen<SharpenRadius, Sigma, Alpha> {
    /// Create a new node.
    pub fn new(sharpen_radius: SharpenRadius, sigma: Sigma, alpha: Alpha) -> Self {
        Self {
            sharpen_radius,
            sigma,
            alpha,
        }
    }
}
impl<
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentOperationSharpen<SharpenRadius, Sigma, Alpha> {
    type Output = crate::nodes::types::LatentOperationOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sharpen_radius".to_string(), self.sharpen_radius.clone().into());
        output.insert("sigma".to_string(), self.sigma.clone().into());
        output.insert("alpha".to_string(), self.alpha.clone().into());
        output
    }
    const NAME: &'static str = "LatentOperationSharpen";
    const DISPLAY_NAME: &'static str = "LatentOperationSharpen";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced/operations";
}
///**LatentOperationTonemapReinhard**: No description.
#[derive(Clone)]
pub struct LatentOperationTonemapReinhard<Multiplier: crate::nodes::types::Float> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub multiplier: Multiplier,
}
impl<Multiplier: crate::nodes::types::Float> LatentOperationTonemapReinhard<Multiplier> {
    /// Create a new node.
    pub fn new(multiplier: Multiplier) -> Self {
        Self { multiplier }
    }
}
impl<Multiplier: crate::nodes::types::Float> crate::nodes::TypedNode
for LatentOperationTonemapReinhard<Multiplier> {
    type Output = crate::nodes::types::LatentOperationOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "LatentOperationTonemapReinhard";
    const DISPLAY_NAME: &'static str = "LatentOperationTonemapReinhard";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced/operations";
}
