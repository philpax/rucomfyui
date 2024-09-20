//!`unet` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**FreeU**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeU<Model, B1, B2, S1, S2> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("b1".to_string(), self.b_1.to_workflow_input());
        output.insert("b2".to_string(), self.b_2.to_workflow_input());
        output.insert("s1".to_string(), self.s_1.to_workflow_input());
        output.insert("s2".to_string(), self.s_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "FreeU";
    const DISPLAY_NAME: &'static str = "FreeU";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**FreeU_V2**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeUV2<Model, B1, B2, S1, S2> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("b1".to_string(), self.b_1.to_workflow_input());
        output.insert("b2".to_string(), self.b_2.to_workflow_input());
        output.insert("s1".to_string(), self.s_1.to_workflow_input());
        output.insert("s2".to_string(), self.s_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "FreeU_V2";
    const DISPLAY_NAME: &'static str = "FreeU_V2";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**HyperTile**: No description.
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
impl<
    Model: crate::nodes::types::Model,
    TileSize: crate::nodes::types::Int,
    SwapSize: crate::nodes::types::Int,
    MaxDepth: crate::nodes::types::Int,
    ScaleDepth: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for HyperTile<Model, TileSize, SwapSize, MaxDepth, ScaleDepth> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("tile_size".to_string(), self.tile_size.to_workflow_input());
        output.insert("swap_size".to_string(), self.swap_size.to_workflow_input());
        output.insert("max_depth".to_string(), self.max_depth.to_workflow_input());
        output.insert("scale_depth".to_string(), self.scale_depth.to_workflow_input());
        output
    }
    const NAME: &'static str = "HyperTile";
    const DISPLAY_NAME: &'static str = "HyperTile";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**PerturbedAttentionGuidance**: No description.
pub struct PerturbedAttentionGuidance<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub scale: Scale,
}
impl<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> crate::nodes::TypedNode for PerturbedAttentionGuidance<Model, Scale> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("scale".to_string(), self.scale.to_workflow_input());
        output
    }
    const NAME: &'static str = "PerturbedAttentionGuidance";
    const DISPLAY_NAME: &'static str = "PerturbedAttentionGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
