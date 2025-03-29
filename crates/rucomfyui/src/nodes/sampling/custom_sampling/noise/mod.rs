//!`noise` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
        Self::Output::from_dynamic(node_id, 0)
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
pub struct RandomNoise<NoiseSeedParam: crate::nodes::types::Int> {
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub noise_seed: NoiseSeedParam,
}
impl<NoiseSeedParam: crate::nodes::types::Int> RandomNoise<NoiseSeedParam> {
    /// Create a new node.
    pub fn new(noise_seed: NoiseSeedParam) -> Self {
        Self { noise_seed }
    }
}
impl<NoiseSeedParam: crate::nodes::types::Int> crate::nodes::TypedNode
for RandomNoise<NoiseSeedParam> {
    type Output = crate::nodes::types::NoiseOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
