//!`unet` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**FreeU**: No description.
#[derive(Clone)]
pub struct FreeU<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1.1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_1: B1,
    /**No documentation.

**Metadata**:
  - Default: 1.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_2: B2,
    /**No documentation.

**Metadata**:
  - Default: 0.9
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_1: S1,
    /**No documentation.

**Metadata**:
  - Default: 0.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_2: S2,
}
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> FreeU<Model, B1, B2, S1, S2> {
    /// Create a new node.
    pub fn new(model: Model, b_1: B1, b_2: B2, s_1: S1, s_2: S2) -> Self {
        Self { model, b_1, b_2, s_1, s_2 }
    }
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
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("b1".to_string(), self.b_1.clone().into());
        output.insert("b2".to_string(), self.b_2.clone().into());
        output.insert("s1".to_string(), self.s_1.clone().into());
        output.insert("s2".to_string(), self.s_2.clone().into());
        output
    }
    const NAME: &'static str = "FreeU";
    const DISPLAY_NAME: &'static str = "FreeU";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**FreeU_V2**: No description.
#[derive(Clone)]
pub struct FreeUV2<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1.3
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_1: B1,
    /**No documentation.

**Metadata**:
  - Default: 1.4
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_2: B2,
    /**No documentation.

**Metadata**:
  - Default: 0.9
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_1: S1,
    /**No documentation.

**Metadata**:
  - Default: 0.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_2: S2,
}
impl<
    Model: crate::nodes::types::Model,
    B1: crate::nodes::types::Float,
    B2: crate::nodes::types::Float,
    S1: crate::nodes::types::Float,
    S2: crate::nodes::types::Float,
> FreeUV2<Model, B1, B2, S1, S2> {
    /// Create a new node.
    pub fn new(model: Model, b_1: B1, b_2: B2, s_1: S1, s_2: S2) -> Self {
        Self { model, b_1, b_2, s_1, s_2 }
    }
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
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("b1".to_string(), self.b_1.clone().into());
        output.insert("b2".to_string(), self.b_2.clone().into());
        output.insert("s1".to_string(), self.s_1.clone().into());
        output.insert("s2".to_string(), self.s_2.clone().into());
        output
    }
    const NAME: &'static str = "FreeU_V2";
    const DISPLAY_NAME: &'static str = "FreeU_V2";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**HyperTile**: No description.
#[derive(Clone)]
pub struct HyperTile<
    Model: crate::nodes::types::Model,
    TileSize: crate::nodes::types::Int,
    SwapSize: crate::nodes::types::Int,
    MaxDepth: crate::nodes::types::Int,
    ScaleDepth: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 1
*/
    pub tile_size: TileSize,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 128
  - Min: 1
*/
    pub swap_size: SwapSize,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
*/
    pub max_depth: MaxDepth,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub scale_depth: ScaleDepth,
}
impl<
    Model: crate::nodes::types::Model,
    TileSize: crate::nodes::types::Int,
    SwapSize: crate::nodes::types::Int,
    MaxDepth: crate::nodes::types::Int,
    ScaleDepth: crate::nodes::types::Boolean,
> HyperTile<Model, TileSize, SwapSize, MaxDepth, ScaleDepth> {
    /// Create a new node.
    pub fn new(
        model: Model,
        tile_size: TileSize,
        swap_size: SwapSize,
        max_depth: MaxDepth,
        scale_depth: ScaleDepth,
    ) -> Self {
        Self {
            model,
            tile_size,
            swap_size,
            max_depth,
            scale_depth,
        }
    }
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
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("tile_size".to_string(), self.tile_size.clone().into());
        output.insert("swap_size".to_string(), self.swap_size.clone().into());
        output.insert("max_depth".to_string(), self.max_depth.clone().into());
        output.insert("scale_depth".to_string(), self.scale_depth.clone().into());
        output
    }
    const NAME: &'static str = "HyperTile";
    const DISPLAY_NAME: &'static str = "HyperTile";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**PatchModelAddDownscale (Kohya Deep Shrink)**: No description.
#[derive(Clone)]
pub struct PatchModelAddDownscale<
    Model: crate::nodes::types::Model,
    BlockNumber: crate::nodes::types::Int,
    DownscaleFactor: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    DownscaleAfterSkip: crate::nodes::types::Boolean,
    DownscaleMethod: crate::nodes::types::String,
    UpscaleMethod: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 32
  - Min: 1
  - Step: 1
*/
    pub block_number: BlockNumber,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 9
  - Min: 0.1
  - Step: 0.001
*/
    pub downscale_factor: DownscaleFactor,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 0.35
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub downscale_after_skip: DownscaleAfterSkip,
    ///No documentation.
    pub downscale_method: DownscaleMethod,
    ///No documentation.
    pub upscale_method: UpscaleMethod,
}
impl<
    Model: crate::nodes::types::Model,
    BlockNumber: crate::nodes::types::Int,
    DownscaleFactor: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    DownscaleAfterSkip: crate::nodes::types::Boolean,
    DownscaleMethod: crate::nodes::types::String,
    UpscaleMethod: crate::nodes::types::String,
> PatchModelAddDownscale<
    Model,
    BlockNumber,
    DownscaleFactor,
    StartPercent,
    EndPercent,
    DownscaleAfterSkip,
    DownscaleMethod,
    UpscaleMethod,
> {
    /// Create a new node.
    pub fn new(
        model: Model,
        block_number: BlockNumber,
        downscale_factor: DownscaleFactor,
        start_percent: StartPercent,
        end_percent: EndPercent,
        downscale_after_skip: DownscaleAfterSkip,
        downscale_method: DownscaleMethod,
        upscale_method: UpscaleMethod,
    ) -> Self {
        Self {
            model,
            block_number,
            downscale_factor,
            start_percent,
            end_percent,
            downscale_after_skip,
            downscale_method,
            upscale_method,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    BlockNumber: crate::nodes::types::Int,
    DownscaleFactor: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    DownscaleAfterSkip: crate::nodes::types::Boolean,
    DownscaleMethod: crate::nodes::types::String,
    UpscaleMethod: crate::nodes::types::String,
> crate::nodes::TypedNode
for PatchModelAddDownscale<
    Model,
    BlockNumber,
    DownscaleFactor,
    StartPercent,
    EndPercent,
    DownscaleAfterSkip,
    DownscaleMethod,
    UpscaleMethod,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("block_number".to_string(), self.block_number.clone().into());
        output
            .insert(
                "downscale_factor".to_string(),
                self.downscale_factor.clone().into(),
            );
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
            .insert(
                "downscale_after_skip".to_string(),
                self.downscale_after_skip.clone().into(),
            );
        output
            .insert(
                "downscale_method".to_string(),
                self.downscale_method.clone().into(),
            );
        output.insert("upscale_method".to_string(), self.upscale_method.clone().into());
        output
    }
    const NAME: &'static str = "PatchModelAddDownscale";
    const DISPLAY_NAME: &'static str = "PatchModelAddDownscale (Kohya Deep Shrink)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**PerturbedAttentionGuidance**: No description.
#[derive(Clone)]
pub struct PerturbedAttentionGuidance<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.01
*/
    pub scale: Scale,
}
impl<
    Model: crate::nodes::types::Model,
    Scale: crate::nodes::types::Float,
> PerturbedAttentionGuidance<Model, Scale> {
    /// Create a new node.
    pub fn new(model: Model, scale: Scale) -> Self {
        Self { model, scale }
    }
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
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scale".to_string(), self.scale.clone().into());
        output
    }
    const NAME: &'static str = "PerturbedAttentionGuidance";
    const DISPLAY_NAME: &'static str = "PerturbedAttentionGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**TomePatchModel**: No description.
#[derive(Clone)]
pub struct TomePatchModel<
    Model: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 0.3
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub ratio: Ratio,
}
impl<
    Model: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> TomePatchModel<Model, Ratio> {
    /// Create a new node.
    pub fn new(model: Model, ratio: Ratio) -> Self {
        Self { model, ratio }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for TomePatchModel<Model, Ratio> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("ratio".to_string(), self.ratio.clone().into());
        output
    }
    const NAME: &'static str = "TomePatchModel";
    const DISPLAY_NAME: &'static str = "TomePatchModel";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
