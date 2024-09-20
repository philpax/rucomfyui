//!`noise` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**DisableNoise**
pub struct DisableNoise {}
///Output for [`DisableNoise`].
#[derive(Clone)]
pub struct DisableNoiseOutput {
    ///No documentation.
    pub noise: crate::nodes::NoiseOut,
}
impl crate::nodes::TypedNode for DisableNoise {
    type Output = DisableNoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            noise: crate::nodes::NoiseOut {
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
pub struct RandomNoise<NoiseSeed: crate::nodes::Int> {
    ///No documentation.
    pub noise_seed: NoiseSeed,
}
///Output for [`RandomNoise`].
#[derive(Clone)]
pub struct RandomNoiseOutput {
    ///No documentation.
    pub noise: crate::nodes::NoiseOut,
}
impl<NoiseSeed: crate::nodes::Int> crate::nodes::TypedNode for RandomNoise<NoiseSeed> {
    type Output = RandomNoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            noise: crate::nodes::NoiseOut {
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
