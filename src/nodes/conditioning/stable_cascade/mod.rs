//!stable_cascade
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**StableCascade_StageB_Conditioning**
pub struct StableCascadeStageBConditioning<
    Conditioning: crate::nodes::Conditioning,
    StageC: crate::nodes::Latent,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub stage_c: StageC,
}
///Output for [`StableCascadeStageBConditioning`].
#[derive(Clone)]
pub struct StableCascadeStageBConditioningOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    StageC: crate::nodes::Latent,
> crate::nodes::TypedNode for StableCascadeStageBConditioning<Conditioning, StageC> {
    type Output = StableCascadeStageBConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DISPLAY_NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/stable_cascade";
}
