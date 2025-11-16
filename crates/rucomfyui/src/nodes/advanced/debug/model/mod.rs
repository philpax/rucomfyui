//!`model` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**EasyCache**: Native EasyCache implementation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EasyCache<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> {
    ///The model to add EasyCache to.
    pub model: ModelParam,
    /**The threshold for reusing cached steps.

**Metadata**:
  - Default: 0.2
  - Max: 3
  - Min: 0
  - Step: 0.01
*/
    pub reuse_threshold: ReuseThresholdParam,
    /**The relative sampling step to begin use of EasyCache.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub start_percent: StartPercentParam,
    /**The relative sampling step to end use of EasyCache.

**Metadata**:
  - Default: 0.95
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub end_percent: EndPercentParam,
    /**Whether to log verbose information.

**Metadata**:
  - Default: false
*/
    pub verbose: VerboseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> EasyCache<
    ModelParam,
    ReuseThresholdParam,
    StartPercentParam,
    EndPercentParam,
    VerboseParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        reuse_threshold: ReuseThresholdParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        verbose: VerboseParam,
    ) -> Self {
        Self {
            model,
            reuse_threshold,
            start_percent,
            end_percent,
            verbose,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for EasyCache<
    ModelParam,
    ReuseThresholdParam,
    StartPercentParam,
    EndPercentParam,
    VerboseParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert("reuse_threshold".to_string(), self.reuse_threshold.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output.insert("verbose".to_string(), self.verbose.clone().into());
        output
    }
    const NAME: &'static str = "EasyCache";
    const DISPLAY_NAME: &'static str = "EasyCache";
    const DESCRIPTION: &'static str = "Native EasyCache implementation.";
    const CATEGORY: &'static str = "advanced/debug/model";
}
///**LazyCache**: A homebrew version of EasyCache - even 'easier' version of EasyCache to implement. Overall works worse than EasyCache, but better in some rare cases AND universal compatibility with everything in ComfyUI.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LazyCache<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> {
    ///The model to add LazyCache to.
    pub model: ModelParam,
    /**The threshold for reusing cached steps.

**Metadata**:
  - Default: 0.2
  - Max: 3
  - Min: 0
  - Step: 0.01
*/
    pub reuse_threshold: ReuseThresholdParam,
    /**The relative sampling step to begin use of LazyCache.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub start_percent: StartPercentParam,
    /**The relative sampling step to end use of LazyCache.

**Metadata**:
  - Default: 0.95
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub end_percent: EndPercentParam,
    /**Whether to log verbose information.

**Metadata**:
  - Default: false
*/
    pub verbose: VerboseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> LazyCache<
    ModelParam,
    ReuseThresholdParam,
    StartPercentParam,
    EndPercentParam,
    VerboseParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        reuse_threshold: ReuseThresholdParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        verbose: VerboseParam,
    ) -> Self {
        Self {
            model,
            reuse_threshold,
            start_percent,
            end_percent,
            verbose,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ReuseThresholdParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    VerboseParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for LazyCache<
    ModelParam,
    ReuseThresholdParam,
    StartPercentParam,
    EndPercentParam,
    VerboseParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert("reuse_threshold".to_string(), self.reuse_threshold.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output.insert("verbose".to_string(), self.verbose.clone().into());
        output
    }
    const NAME: &'static str = "LazyCache";
    const DISPLAY_NAME: &'static str = "LazyCache";
    const DESCRIPTION: &'static str = "A homebrew version of EasyCache - even 'easier' version of EasyCache to implement. Overall works worse than EasyCache, but better in some rare cases AND universal compatibility with everything in ComfyUI.";
    const CATEGORY: &'static str = "advanced/debug/model";
}
///**ModelComputeDtype**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelComputeDtype<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub dtype: DtypeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> ModelComputeDtype<ModelParam, DtypeParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, dtype: DtypeParam) -> Self {
        Self { model, dtype }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    DtypeParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ModelComputeDtype<ModelParam, DtypeParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("dtype".to_string(), self.dtype.clone().into());
        output
    }
    const NAME: &'static str = "ModelComputeDtype";
    const DISPLAY_NAME: &'static str = "ModelComputeDtype";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/debug/model";
}
