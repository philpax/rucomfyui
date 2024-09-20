//!`stable_cascade` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`StableCascadeStageBConditioning`](super::StableCascadeStageBConditioning).
    #[derive(Clone)]
    pub struct StableCascadeStageBConditioningOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
}
///**StableCascade_StageB_Conditioning**: No description.
pub struct StableCascadeStageBConditioning<
    Conditioning: crate::nodes::types::Conditioning,
    StageC: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub stage_c: StageC,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    StageC: crate::nodes::types::Latent,
> crate::nodes::TypedNode for StableCascadeStageBConditioning<Conditioning, StageC> {
    type Output = out::StableCascadeStageBConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DISPLAY_NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/stable_cascade";
}
