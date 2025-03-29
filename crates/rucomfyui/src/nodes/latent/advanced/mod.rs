//!`advanced` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod operations;
///**LatentAdd**: No description.
#[derive(Clone)]
pub struct LatentAdd<
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
> LatentAdd<Samples1Param, Samples2Param> {
    /// Create a new node.
    pub fn new(samples_1: Samples1Param, samples_2: Samples2Param) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentAdd<Samples1Param, Samples2Param> {
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
    const NAME: &'static str = "LatentAdd";
    const DISPLAY_NAME: &'static str = "LatentAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentBatchSeedBehavior**: No description.
#[derive(Clone)]
pub struct LatentBatchSeedBehavior<
    SamplesParam: crate::nodes::types::Latent,
    SeedBehaviorParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: fixed
*/
    pub seed_behavior: SeedBehaviorParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    SeedBehaviorParam: crate::nodes::types::String,
> LatentBatchSeedBehavior<SamplesParam, SeedBehaviorParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, seed_behavior: SeedBehaviorParam) -> Self {
        Self { samples, seed_behavior }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    SeedBehaviorParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentBatchSeedBehavior<SamplesParam, SeedBehaviorParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("seed_behavior".to_string(), self.seed_behavior.clone().into());
        output
    }
    const NAME: &'static str = "LatentBatchSeedBehavior";
    const DISPLAY_NAME: &'static str = "LatentBatchSeedBehavior";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentInterpolate**: No description.
#[derive(Clone)]
pub struct LatentInterpolate<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    RatioParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples_1: Samples1Param,
    ///No documentation.
    pub samples_2: Samples2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub ratio: RatioParam,
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    RatioParam: crate::nodes::types::Float,
> LatentInterpolate<Samples1Param, Samples2Param, RatioParam> {
    /// Create a new node.
    pub fn new(
        samples_1: Samples1Param,
        samples_2: Samples2Param,
        ratio: RatioParam,
    ) -> Self {
        Self {
            samples_1,
            samples_2,
            ratio,
        }
    }
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
    RatioParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LatentInterpolate<Samples1Param, Samples2Param, RatioParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.clone().into());
        output.insert("samples2".to_string(), self.samples_2.clone().into());
        output.insert("ratio".to_string(), self.ratio.clone().into());
        output
    }
    const NAME: &'static str = "LatentInterpolate";
    const DISPLAY_NAME: &'static str = "LatentInterpolate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentMultiply**: No description.
#[derive(Clone)]
pub struct LatentMultiply<
    SamplesParam: crate::nodes::types::Latent,
    MultiplierParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub multiplier: MultiplierParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    MultiplierParam: crate::nodes::types::Float,
> LatentMultiply<SamplesParam, MultiplierParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, multiplier: MultiplierParam) -> Self {
        Self { samples, multiplier }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    MultiplierParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentMultiply<SamplesParam, MultiplierParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "LatentMultiply";
    const DISPLAY_NAME: &'static str = "LatentMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentSubtract**: No description.
#[derive(Clone)]
pub struct LatentSubtract<
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
> LatentSubtract<Samples1Param, Samples2Param> {
    /// Create a new node.
    pub fn new(samples_1: Samples1Param, samples_2: Samples2Param) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1Param: crate::nodes::types::Latent,
    Samples2Param: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentSubtract<Samples1Param, Samples2Param> {
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
    const NAME: &'static str = "LatentSubtract";
    const DISPLAY_NAME: &'static str = "LatentSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
