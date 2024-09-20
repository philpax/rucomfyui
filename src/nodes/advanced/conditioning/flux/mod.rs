//!flux
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**CLIPTextEncodeFlux**
pub struct ClipTextEncodeFlux<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    T5Xxl: crate::nodes::String,
    Guidance: crate::nodes::Float,
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
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    T5Xxl: crate::nodes::String,
    Guidance: crate::nodes::Float,
> crate::nodes::TypedNode for ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
    type Output = ClipTextEncodeFluxOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
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
    Conditioning: crate::nodes::Conditioning,
    Guidance: crate::nodes::Float,
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
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Guidance: crate::nodes::Float,
> crate::nodes::TypedNode for FluxGuidance<Conditioning, Guidance> {
    type Output = FluxGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FluxGuidance";
    const DISPLAY_NAME: &'static str = "FluxGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning/flux";
}
