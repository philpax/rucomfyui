//!`noise` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**DisableNoise**: No description.
#[derive(Clone)]
pub struct DisableNoise {}
impl DisableNoise {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for DisableNoise {
    type Output = crate::nodes::types::NoiseOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "DisableNoise";
    const DISPLAY_NAME: &'static str = "DisableNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
///**RandomNoise**: No description.
#[derive(Clone)]
pub struct RandomNoise<NoiseSeed: crate::nodes::types::Int> {
    ///No documentation.
    pub noise_seed: NoiseSeed,
}
impl<NoiseSeed: crate::nodes::types::Int> RandomNoise<NoiseSeed> {
    /// Create a new node.
    pub fn new(noise_seed: NoiseSeed) -> Self {
        Self { noise_seed }
    }
}
impl<NoiseSeed: crate::nodes::types::Int> crate::nodes::TypedNode
for RandomNoise<NoiseSeed> {
    type Output = crate::nodes::types::NoiseOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("noise_seed".to_string(), self.noise_seed.clone().into());
        output
    }
    const NAME: &'static str = "RandomNoise";
    const DISPLAY_NAME: &'static str = "RandomNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
