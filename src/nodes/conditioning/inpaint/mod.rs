//!`inpaint` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`InpaintModelConditioning`](super::InpaintModelConditioning).
    #[derive(Clone)]
    pub struct InpaintModelConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**InpaintModelConditioning**
pub struct InpaintModelConditioning<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub mask: Mask,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask> {
    type Output = out::InpaintModelConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
            negative: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 1u32,
            },
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "InpaintModelConditioning";
    const DISPLAY_NAME: &'static str = "InpaintModelConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/inpaint";
}
