//!`flux` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`ClipTextEncodeFlux`](super::ClipTextEncodeFlux).
    #[derive(Clone)]
    pub struct ClipTextEncodeFluxOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`FluxGuidance`](super::FluxGuidance).
    #[derive(Clone)]
    pub struct FluxGuidanceOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
}
///**CLIPTextEncodeFlux**: No description.
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
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipTextEncodeFlux<Clip, ClipL, T5Xxl, Guidance> {
    type Output = out::ClipTextEncodeFluxOutput;
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
///**FluxGuidance**: No description.
pub struct FluxGuidance<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub guidance: Guidance,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Guidance: crate::nodes::types::Float,
> crate::nodes::TypedNode for FluxGuidance<Conditioning, Guidance> {
    type Output = out::FluxGuidanceOutput;
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
