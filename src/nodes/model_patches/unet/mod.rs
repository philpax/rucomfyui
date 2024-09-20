//!`unet` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**FreeU**
pub struct FreeU<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub b_1: B1,
    ///No documentation.
    pub b_2: B2,
    ///No documentation.
    pub s_1: S1,
    ///No documentation.
    pub s_2: S2,
}
///Output for [`FreeU`].
#[derive(Clone)]
pub struct FreeUOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeU<Model, B1, B2, S1, S2> {
    type Output = FreeUOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FreeU";
    const DISPLAY_NAME: &'static str = "FreeU";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**FreeU_V2**
pub struct FreeUV2<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub b_1: B1,
    ///No documentation.
    pub b_2: B2,
    ///No documentation.
    pub s_1: S1,
    ///No documentation.
    pub s_2: S2,
}
///Output for [`FreeUV2`].
#[derive(Clone)]
pub struct FreeUV2Output {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeUV2<Model, B1, B2, S1, S2> {
    type Output = FreeUV2Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FreeU_V2";
    const DISPLAY_NAME: &'static str = "FreeU_V2";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**HyperTile**
pub struct HyperTile<
    Model: crate::nodes::types::Model,
    TileSize: crate::nodes::types::Int,
    SwapSize: crate::nodes::types::Int,
    MaxDepth: crate::nodes::types::Int,
    ScaleDepth: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub tile_size: TileSize,
    ///No documentation.
    pub swap_size: SwapSize,
    ///No documentation.
    pub max_depth: MaxDepth,
    ///No documentation.
    pub scale_depth: ScaleDepth,
}
///Output for [`HyperTile`].
#[derive(Clone)]
pub struct HyperTileOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    TileSize: crate::nodes::types::Int,
    SwapSize: crate::nodes::types::Int,
    MaxDepth: crate::nodes::types::Int,
    ScaleDepth: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for HyperTile<Model, TileSize, SwapSize, MaxDepth, ScaleDepth> {
    type Output = HyperTileOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "HyperTile";
    const DISPLAY_NAME: &'static str = "HyperTile";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**PerturbedAttentionGuidance**
pub struct PerturbedAttentionGuidance<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub scale: Scale,
}
///Output for [`PerturbedAttentionGuidance`].
#[derive(Clone)]
pub struct PerturbedAttentionGuidanceOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> crate::nodes::TypedNode for PerturbedAttentionGuidance<Model, Scale> {
    type Output = PerturbedAttentionGuidanceOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "PerturbedAttentionGuidance";
    const DISPLAY_NAME: &'static str = "PerturbedAttentionGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
