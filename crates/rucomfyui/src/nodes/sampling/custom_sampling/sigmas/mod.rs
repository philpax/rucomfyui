//!`sigmas` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
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
#[derive(Clone)]
pub struct FlipSigmas<SigmasParam: crate::nodes::types::Sigmas> {
    ///No documentation.
    pub sigmas: SigmasParam,
}
impl<SigmasParam: crate::nodes::types::Sigmas> FlipSigmas<SigmasParam> {
    /// Create a new node.
    pub fn new(sigmas: SigmasParam) -> Self {
        Self { sigmas }
    }
}
impl<SigmasParam: crate::nodes::types::Sigmas> crate::nodes::TypedNode
for FlipSigmas<SigmasParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output
    }
    const NAME: &'static str = "FlipSigmas";
    const DISPLAY_NAME: &'static str = "FlipSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SetFirstSigma**: No description.
#[derive(Clone)]
pub struct SetFirstSigma<
    SigmasParam: crate::nodes::types::Sigmas,
    SigmaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub sigmas: SigmasParam,
    /**No documentation.

**Metadata**:
  - Default: 136
  - Max: 20000
  - Min: 0
  - Round: false
  - Step: 0.001
*/
    pub sigma: SigmaParam,
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    SigmaParam: crate::nodes::types::Float,
> SetFirstSigma<SigmasParam, SigmaParam> {
    /// Create a new node.
    pub fn new(sigmas: SigmasParam, sigma: SigmaParam) -> Self {
        Self { sigmas, sigma }
    }
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    SigmaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SetFirstSigma<SigmasParam, SigmaParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("sigma".to_string(), self.sigma.clone().into());
        output
    }
    const NAME: &'static str = "SetFirstSigma";
    const DISPLAY_NAME: &'static str = "SetFirstSigma";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmas**: No description.
#[derive(Clone)]
pub struct SplitSigmas<
    SigmasParam: crate::nodes::types::Sigmas,
    StepParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub sigmas: SigmasParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10000
  - Min: 0
*/
    pub step: StepParam,
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    StepParam: crate::nodes::types::Int,
> SplitSigmas<SigmasParam, StepParam> {
    /// Create a new node.
    pub fn new(sigmas: SigmasParam, step: StepParam) -> Self {
        Self { sigmas, step }
    }
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    StepParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SplitSigmas<SigmasParam, StepParam> {
    type Output = out::SplitSigmasOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::types::SigmasOut::from_dynamic(node_id, 0u32),
            low_sigmas: crate::nodes::types::SigmasOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("step".to_string(), self.step.clone().into());
        output
    }
    const NAME: &'static str = "SplitSigmas";
    const DISPLAY_NAME: &'static str = "SplitSigmas";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
///**SplitSigmasDenoise**: No description.
#[derive(Clone)]
pub struct SplitSigmasDenoise<
    SigmasParam: crate::nodes::types::Sigmas,
    DenoiseParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub sigmas: SigmasParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    DenoiseParam: crate::nodes::types::Float,
> SplitSigmasDenoise<SigmasParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(sigmas: SigmasParam, denoise: DenoiseParam) -> Self {
        Self { sigmas, denoise }
    }
}
impl<
    SigmasParam: crate::nodes::types::Sigmas,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SplitSigmasDenoise<SigmasParam, DenoiseParam> {
    type Output = out::SplitSigmasDenoiseOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            high_sigmas: crate::nodes::types::SigmasOut::from_dynamic(node_id, 0u32),
            low_sigmas: crate::nodes::types::SigmasOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("sigmas".to_string(), self.sigmas.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "SplitSigmasDenoise";
    const DISPLAY_NAME: &'static str = "SplitSigmasDenoise";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/sigmas";
}
