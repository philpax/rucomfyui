//!`noise` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**DisableNoise**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
    const CATEGORY: &'static str = "model/sampling/noise";
}
///**RandomNoise**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
    const CATEGORY: &'static str = "model/sampling/noise";
}
///**VOIDWarpedNoiseSource**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VOIDWarpedNoiseSource<WarpedNoiseParam: crate::nodes::types::Latent> {
    ///Warped noise latent from VOIDWarpedNoise
    pub warped_noise: WarpedNoiseParam,
}
impl<
    WarpedNoiseParam: crate::nodes::types::Latent,
> VOIDWarpedNoiseSource<WarpedNoiseParam> {
    /// Create a new node.
    pub fn new(warped_noise: WarpedNoiseParam) -> Self {
        Self { warped_noise }
    }
}
impl<WarpedNoiseParam: crate::nodes::types::Latent> crate::nodes::TypedNode
for VOIDWarpedNoiseSource<WarpedNoiseParam> {
    type Output = crate::nodes::types::NoiseOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("warped_noise".to_string(), self.warped_noise.clone().into());
        output
    }
    const NAME: &'static str = "VOIDWarpedNoiseSource";
    const DISPLAY_NAME: &'static str = "VOIDWarpedNoiseSource";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/sampling/noise";
}
