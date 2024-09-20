//!`noise` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`DisableNoise`](super::DisableNoise).
    #[derive(Clone)]
    pub struct DisableNoiseOutput {
        ///No documentation.
        pub noise: crate::nodes::types::NoiseOut,
    }
    ///Output for [`RandomNoise`](super::RandomNoise).
    #[derive(Clone)]
    pub struct RandomNoiseOutput {
        ///No documentation.
        pub noise: crate::nodes::types::NoiseOut,
    }
}
///**DisableNoise**
pub struct DisableNoise {}
impl crate::nodes::TypedNode for DisableNoise {
    type Output = out::DisableNoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            noise: crate::nodes::types::NoiseOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "DisableNoise";
    const DISPLAY_NAME: &'static str = "DisableNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
///**RandomNoise**
pub struct RandomNoise<NoiseSeed: crate::nodes::types::Int> {
    ///No documentation.
    pub noise_seed: NoiseSeed,
}
impl<NoiseSeed: crate::nodes::types::Int> crate::nodes::TypedNode
for RandomNoise<NoiseSeed> {
    type Output = out::RandomNoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            noise: crate::nodes::types::NoiseOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RandomNoise";
    const DISPLAY_NAME: &'static str = "RandomNoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/noise";
}
