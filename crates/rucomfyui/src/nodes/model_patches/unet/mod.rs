//!`unet` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**FreeU**: No description.
#[derive(Clone)]
pub struct FreeU<
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1.1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_1: B1Param,
    /**No documentation.

**Metadata**:
  - Default: 1.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_2: B2Param,
    /**No documentation.

**Metadata**:
  - Default: 0.9
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_1: S1Param,
    /**No documentation.

**Metadata**:
  - Default: 0.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_2: S2Param,
}
impl<
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> FreeU<ModelParam, B1Param, B2Param, S1Param, S2Param> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        b_1: B1Param,
        b_2: B2Param,
        s_1: S1Param,
        s_2: S2Param,
    ) -> Self {
        Self { model, b_1, b_2, s_1, s_2 }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeU<ModelParam, B1Param, B2Param, S1Param, S2Param> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1.3
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_1: B1Param,
    /**No documentation.

**Metadata**:
  - Default: 1.4
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub b_2: B2Param,
    /**No documentation.

**Metadata**:
  - Default: 0.9
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_1: S1Param,
    /**No documentation.

**Metadata**:
  - Default: 0.2
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub s_2: S2Param,
}
impl<
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> FreeUV2<ModelParam, B1Param, B2Param, S1Param, S2Param> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        b_1: B1Param,
        b_2: B2Param,
        s_1: S1Param,
        s_2: S2Param,
    ) -> Self {
        Self { model, b_1, b_2, s_1, s_2 }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    B1Param: crate::nodes::types::Float,
    B2Param: crate::nodes::types::Float,
    S1Param: crate::nodes::types::Float,
    S2Param: crate::nodes::types::Float,
> crate::nodes::TypedNode for FreeUV2<ModelParam, B1Param, B2Param, S1Param, S2Param> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    TileSizeParam: crate::nodes::types::Int,
    SwapSizeParam: crate::nodes::types::Int,
    MaxDepthParam: crate::nodes::types::Int,
    ScaleDepthParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 1
*/
    pub tile_size: TileSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 128
  - Min: 1
*/
    pub swap_size: SwapSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
*/
    pub max_depth: MaxDepthParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub scale_depth: ScaleDepthParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    TileSizeParam: crate::nodes::types::Int,
    SwapSizeParam: crate::nodes::types::Int,
    MaxDepthParam: crate::nodes::types::Int,
    ScaleDepthParam: crate::nodes::types::Boolean,
> HyperTile<ModelParam, TileSizeParam, SwapSizeParam, MaxDepthParam, ScaleDepthParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        tile_size: TileSizeParam,
        swap_size: SwapSizeParam,
        max_depth: MaxDepthParam,
        scale_depth: ScaleDepthParam,
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
    ModelParam: crate::nodes::types::Model,
    TileSizeParam: crate::nodes::types::Int,
    SwapSizeParam: crate::nodes::types::Int,
    MaxDepthParam: crate::nodes::types::Int,
    ScaleDepthParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for HyperTile<ModelParam, TileSizeParam, SwapSizeParam, MaxDepthParam, ScaleDepthParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    BlockNumberParam: crate::nodes::types::Int,
    DownscaleFactorParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    DownscaleAfterSkipParam: crate::nodes::types::Boolean,
    DownscaleMethodParam: crate::nodes::types::String,
    UpscaleMethodParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 32
  - Min: 1
  - Step: 1
*/
    pub block_number: BlockNumberParam,
    /**No documentation.

**Metadata**:
  - Default: 2
  - Max: 9
  - Min: 0.1
  - Step: 0.001
*/
    pub downscale_factor: DownscaleFactorParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.35
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub downscale_after_skip: DownscaleAfterSkipParam,
    ///No documentation.
    pub downscale_method: DownscaleMethodParam,
    ///No documentation.
    pub upscale_method: UpscaleMethodParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    BlockNumberParam: crate::nodes::types::Int,
    DownscaleFactorParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    DownscaleAfterSkipParam: crate::nodes::types::Boolean,
    DownscaleMethodParam: crate::nodes::types::String,
    UpscaleMethodParam: crate::nodes::types::String,
> PatchModelAddDownscale<
    ModelParam,
    BlockNumberParam,
    DownscaleFactorParam,
    StartPercentParam,
    EndPercentParam,
    DownscaleAfterSkipParam,
    DownscaleMethodParam,
    UpscaleMethodParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        block_number: BlockNumberParam,
        downscale_factor: DownscaleFactorParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        downscale_after_skip: DownscaleAfterSkipParam,
        downscale_method: DownscaleMethodParam,
        upscale_method: UpscaleMethodParam,
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
    ModelParam: crate::nodes::types::Model,
    BlockNumberParam: crate::nodes::types::Int,
    DownscaleFactorParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    DownscaleAfterSkipParam: crate::nodes::types::Boolean,
    DownscaleMethodParam: crate::nodes::types::String,
    UpscaleMethodParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for PatchModelAddDownscale<
    ModelParam,
    BlockNumberParam,
    DownscaleFactorParam,
    StartPercentParam,
    EndPercentParam,
    DownscaleAfterSkipParam,
    DownscaleMethodParam,
    UpscaleMethodParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.01
*/
    pub scale: ScaleParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
> PerturbedAttentionGuidance<ModelParam, ScaleParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, scale: ScaleParam) -> Self {
        Self { model, scale }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for PerturbedAttentionGuidance<ModelParam, ScaleParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 0.3
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub ratio: RatioParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> TomePatchModel<ModelParam, RatioParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, ratio: RatioParam) -> Self {
        Self { model, ratio }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for TomePatchModel<ModelParam, RatioParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
