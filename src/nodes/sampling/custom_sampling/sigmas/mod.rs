//!`sigmas` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`FlipSigmas`](super::FlipSigmas).
    #[derive(Clone)]
    pub struct FlipSigmasOutput {
        ///No documentation.
        pub sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`SplitSigmas`](super::SplitSigmas).
    #[derive(Clone)]
    pub struct SplitSigmasOutput {
        ///No documentation.
        pub high_sigmas: crate::nodes::types::SigmasOut,
        ///No documentation.
        pub low_sigmas: crate::nodes::types::SigmasOut,
    }
    ///Output for [`SplitSigmasDenoise`](super::SplitSigmasDenoise).
    #[derive(Clone)]
    pub struct SplitSigmasDenoiseOutput {
        ///No documentation.
        pub high_sigmas: crate::nodes::types::SigmasOut,
        ///No documentation.
        pub low_sigmas: crate::nodes::types::SigmasOut,
    }
}
///**FlipSigmas**: No description.
pub struct FlipSigmas<Sigmas: crate::nodes::types::Sigmas> {
    ///No documentation.
    pub sigmas: Sigmas,
}
impl<Sigmas: crate::nodes::types::Sigmas> crate::nodes::TypedNode
for FlipSigmas<Sigmas> {
    type Output = out::FlipSigmasOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            sigmas: crate::nodes::types::SigmasOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.to_workflow_input());
        output
    }
    const NAME: &'static str = "FlipSigmas";
    const DISPLAY_NAME: &'static str = "FlipSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmas**: No description.
pub struct SplitSigmas<
    Sigmas: crate::nodes::types::Sigmas,
    Step: crate::nodes::types::Int,
> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub step: Step,
}
impl<
    Sigmas: crate::nodes::types::Sigmas,
    Step: crate::nodes::types::Int,
> crate::nodes::TypedNode for SplitSigmas<Sigmas, Step> {
    type Output = out::SplitSigmasOutput;
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.to_workflow_input());
        output.insert("step".to_string(), self.step.to_workflow_input());
        output
    }
    const NAME: &'static str = "SplitSigmas";
    const DISPLAY_NAME: &'static str = "SplitSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmasDenoise**: No description.
pub struct SplitSigmasDenoise<
    Sigmas: crate::nodes::types::Sigmas,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub sigmas: Sigmas,
    ///No documentation.
    pub denoise: Denoise,
}
impl<
    Sigmas: crate::nodes::types::Sigmas,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SplitSigmasDenoise<Sigmas, Denoise> {
    type Output = out::SplitSigmasDenoiseOutput;
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
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.to_workflow_input());
        output.insert("denoise".to_string(), self.denoise.to_workflow_input());
        output
    }
    const NAME: &'static str = "SplitSigmasDenoise";
    const DISPLAY_NAME: &'static str = "SplitSigmasDenoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
