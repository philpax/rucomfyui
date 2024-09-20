//!`flux` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**CLIPTextEncodeFlux**
pub struct ClipTextEncodeFlux<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub clip_l: ClipL,
    ///No documentation.
    pub t_5_xxl: T5Xxl,
    ///No documentation.
    pub guidance: Guidance,
}
///Output for [`ClipTextEncodeFlux`].
#[derive(Clone)]
pub struct ClipTextEncodeFluxOutput {
    ///No documentation.
    pub conditioning: crate::nodes::types::ConditioningOut,
}
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
    type Output = ClipTextEncodeFluxOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPTextEncodeFlux";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeFlux";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
///**FluxGuidance**
pub struct FluxGuidance<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub guidance: Guidance,
}
///Output for [`FluxGuidance`].
#[derive(Clone)]
pub struct FluxGuidanceOutput {
    ///No documentation.
    pub conditioning: crate::nodes::types::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for FluxGuidance<Conditioning, Guidance> {
    type Output = FluxGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FluxGuidance";
    const DISPLAY_NAME: &'static str = "FluxGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
