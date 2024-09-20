//!`advanced` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**LatentAdd**: No description.
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
> LatentAdd<Samples1, Samples2> {
    /// Create a new node.
    pub fn new(samples_1: Samples1, samples_2: Samples2) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentAdd<Samples1, Samples2> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.to_workflow_input());
        output.insert("samples2".to_string(), self.samples_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentAdd";
    const DISPLAY_NAME: &'static str = "LatentAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentBatchSeedBehavior**: No description.
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
> LatentBatchSeedBehavior<Samples, SeedBehavior> {
    /// Create a new node.
    pub fn new(samples: Samples, seed_behavior: SeedBehavior) -> Self {
        Self { samples, seed_behavior }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    SeedBehavior: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentBatchSeedBehavior<Samples, SeedBehavior> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output
            .insert("seed_behavior".to_string(), self.seed_behavior.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentBatchSeedBehavior";
    const DISPLAY_NAME: &'static str = "LatentBatchSeedBehavior";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentInterpolate**: No description.
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
> LatentInterpolate<Samples1, Samples2, Ratio> {
    /// Create a new node.
    pub fn new(samples_1: Samples1, samples_2: Samples2, ratio: Ratio) -> Self {
        Self {
            samples_1,
            samples_2,
            ratio,
        }
    }
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentInterpolate<Samples1, Samples2, Ratio> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.to_workflow_input());
        output.insert("samples2".to_string(), self.samples_2.to_workflow_input());
        output.insert("ratio".to_string(), self.ratio.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentInterpolate";
    const DISPLAY_NAME: &'static str = "LatentInterpolate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentMultiply**: No description.
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
> LatentMultiply<Samples, Multiplier> {
    /// Create a new node.
    pub fn new(samples: Samples, multiplier: Multiplier) -> Self {
        Self { samples, multiplier }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for LatentMultiply<Samples, Multiplier> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("multiplier".to_string(), self.multiplier.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentMultiply";
    const DISPLAY_NAME: &'static str = "LatentMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
///**LatentSubtract**: No description.
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
> LatentSubtract<Samples1, Samples2> {
    /// Create a new node.
    pub fn new(samples_1: Samples1, samples_2: Samples2) -> Self {
        Self { samples_1, samples_2 }
    }
}
impl<
    Samples1: crate::nodes::types::Latent,
    Samples2: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LatentSubtract<Samples1, Samples2> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples1".to_string(), self.samples_1.to_workflow_input());
        output.insert("samples2".to_string(), self.samples_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentSubtract";
    const DISPLAY_NAME: &'static str = "LatentSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/advanced";
}
