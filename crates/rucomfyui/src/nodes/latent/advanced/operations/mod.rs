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
    SamplesParam: crate::nodes::types::Latent,
    OperationParam: crate::nodes::types::LatentOperation,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub operation: OperationParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    OperationParam: crate::nodes::types::LatentOperation,
> LatentApplyOperation<SamplesParam, OperationParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, operation: OperationParam) -> Self {
        Self { samples, operation }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    OperationParam: crate::nodes::types::LatentOperation,
> crate::nodes::TypedNode for LatentApplyOperation<SamplesParam, OperationParam> {
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
    ModelParam: crate::nodes::types::Model,
    OperationParam: crate::nodes::types::LatentOperation,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub operation: OperationParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    OperationParam: crate::nodes::types::LatentOperation,
> LatentApplyOperationCfg<ModelParam, OperationParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, operation: OperationParam) -> Self {
        Self { model, operation }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    OperationParam: crate::nodes::types::LatentOperation,
> crate::nodes::TypedNode for LatentApplyOperationCfg<ModelParam, OperationParam> {
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
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 9
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub sharpen_radius: SharpenRadiusParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.1
*/
    pub sigma: SigmaParam,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 5
  - Min: 0
  - Step: 0.01
*/
    pub alpha: AlphaParam,
}
impl<
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> LatentOperationSharpen<SharpenRadiusParam, SigmaParam, AlphaParam> {
    /// Create a new node.
    pub fn new(
        sharpen_radius: SharpenRadiusParam,
        sigma: SigmaParam,
        alpha: AlphaParam,
    ) -> Self {
        Self {
            sharpen_radius,
            sigma,
            alpha,
        }
    }
}
impl<
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LatentOperationSharpen<SharpenRadiusParam, SigmaParam, AlphaParam> {
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
pub struct LatentOperationTonemapReinhard<MultiplierParam: crate::nodes::types::Float> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub multiplier: MultiplierParam,
}
impl<
    MultiplierParam: crate::nodes::types::Float,
> LatentOperationTonemapReinhard<MultiplierParam> {
    /// Create a new node.
    pub fn new(multiplier: MultiplierParam) -> Self {
        Self { multiplier }
    }
}
impl<MultiplierParam: crate::nodes::types::Float> crate::nodes::TypedNode
for LatentOperationTonemapReinhard<MultiplierParam> {
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
