//!`sigmas` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**FlipSigmas**
pub struct FlipSigmas<Sigmas: crate::nodes::types::Sigmas> {
    ///No documentation.
    pub sigmas: Sigmas,
}
///Output for [`FlipSigmas`].
#[derive(Clone)]
pub struct FlipSigmasOutput {
    ///No documentation.
    pub sigmas: crate::nodes::types::SigmasOut,
}
impl<Sigmas: crate::nodes::types::Sigmas> crate::nodes::TypedNode
for FlipSigmas<Sigmas> {
    type Output = FlipSigmasOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FlipSigmas";
    const DISPLAY_NAME: &'static str = "FlipSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmas**
pub struct SplitSigmas<
    Sigmas: crate::nodes::types::Sigmas,
    Step: crate::nodes::types::Int,
> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub step: Step,
}
///Output for [`SplitSigmas`].
#[derive(Clone)]
pub struct SplitSigmasOutput {
    ///No documentation.
    pub high_sigmas: crate::nodes::types::SigmasOut,
    ///No documentation.
    pub low_sigmas: crate::nodes::types::SigmasOut,
}
impl<
    Sigmas: crate::nodes::types::Sigmas,
    Step: crate::nodes::types::Int,
> crate::nodes::TypedNode for SplitSigmas<Sigmas, Step> {
    type Output = SplitSigmasOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
            low_sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "SplitSigmas";
    const DISPLAY_NAME: &'static str = "SplitSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmasDenoise**
pub struct SplitSigmasDenoise<
    Sigmas: crate::nodes::types::Sigmas,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub denoise: Denoise,
}
///Output for [`SplitSigmasDenoise`].
#[derive(Clone)]
pub struct SplitSigmasDenoiseOutput {
    ///No documentation.
    pub high_sigmas: crate::nodes::types::SigmasOut,
    ///No documentation.
    pub low_sigmas: crate::nodes::types::SigmasOut,
}
impl<
    Sigmas: crate::nodes::types::Sigmas,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SplitSigmasDenoise<Sigmas, Denoise> {
    type Output = SplitSigmasDenoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
            low_sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "SplitSigmasDenoise";
    const DISPLAY_NAME: &'static str = "SplitSigmasDenoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
