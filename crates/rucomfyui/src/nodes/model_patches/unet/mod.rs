//!`unet` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Epsilon Scaling**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Epsilon_Scaling<
    ModelParam: crate::nodes::types::Model,
    ScalingFactorParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1.005
  - Display: number
  - Max: 1.5
  - Min: 0.5
  - Step: 0.001
*/
    pub scaling_factor: ScalingFactorParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScalingFactorParam: crate::nodes::types::Float,
> Epsilon_Scaling<ModelParam, ScalingFactorParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, scaling_factor: ScalingFactorParam) -> Self {
        Self { model, scaling_factor }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScalingFactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for Epsilon_Scaling<ModelParam, ScalingFactorParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scaling_factor".to_string(), self.scaling_factor.clone().into());
        output
    }
    const NAME: &'static str = "Epsilon Scaling";
    const DISPLAY_NAME: &'static str = "Epsilon Scaling";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**FreeU**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
pub struct FreeU_V2<
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
> FreeU_V2<ModelParam, B1Param, B2Param, S1Param, S2Param> {
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
> crate::nodes::TypedNode for FreeU_V2<ModelParam, B1Param, B2Param, S1Param, S2Param> {
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
pub struct PatchModelAddDownscale<
    ModelParam: crate::nodes::types::Model,
    BlockNumberParam: crate::nodes::types::Int,
    DownscaleFactorParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    DownscaleAfterSkipParam: crate::nodes::types::Boolean,
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
}
impl<
    ModelParam: crate::nodes::types::Model,
    BlockNumberParam: crate::nodes::types::Int,
    DownscaleFactorParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    DownscaleAfterSkipParam: crate::nodes::types::Boolean,
> PatchModelAddDownscale<
    ModelParam,
    BlockNumberParam,
    DownscaleFactorParam,
    StartPercentParam,
    EndPercentParam,
    DownscaleAfterSkipParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        block_number: BlockNumberParam,
        downscale_factor: DownscaleFactorParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        downscale_after_skip: DownscaleAfterSkipParam,
    ) -> Self {
        Self {
            model,
            block_number,
            downscale_factor,
            start_percent,
            end_percent,
            downscale_after_skip,
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
> crate::nodes::TypedNode
for PatchModelAddDownscale<
    ModelParam,
    BlockNumberParam,
    DownscaleFactorParam,
    StartPercentParam,
    EndPercentParam,
    DownscaleAfterSkipParam,
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
    }
    const NAME: &'static str = "PatchModelAddDownscale";
    const DISPLAY_NAME: &'static str = "PatchModelAddDownscale (Kohya Deep Shrink)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**PerturbedAttentionGuidance**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
#[doc = "**TSR - Temporal Score Rescaling**: \\[Post-CFG Function\\]\n\nTSR - Temporal Score Rescaling (2510.01184)\n\n\n\nRescaling the model's score or noise to steer the sampling diversity.\n\n"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TemporalScoreRescaling<
    ModelParam: crate::nodes::types::Model,
    TsrKParam: crate::nodes::types::Float,
    TsrSigmaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**Controls the rescaling strength.

Lower k produces more detailed results; higher k produces smoother results in image generation. Setting k = 1 disables rescaling.

**Metadata**:
  - Default: 0.95
  - Display: number
  - Max: 100
  - Min: 0.01
  - Step: 0.001
*/
    pub tsr_k: TsrKParam,
    /**Controls how early rescaling takes effect.

Larger values take effect earlier.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 100
  - Min: 0.01
  - Step: 0.001
*/
    pub tsr_sigma: TsrSigmaParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    TsrKParam: crate::nodes::types::Float,
    TsrSigmaParam: crate::nodes::types::Float,
> TemporalScoreRescaling<ModelParam, TsrKParam, TsrSigmaParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, tsr_k: TsrKParam, tsr_sigma: TsrSigmaParam) -> Self {
        Self { model, tsr_k, tsr_sigma }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    TsrKParam: crate::nodes::types::Float,
    TsrSigmaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for TemporalScoreRescaling<ModelParam, TsrKParam, TsrSigmaParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("tsr_k".to_string(), self.tsr_k.clone().into());
        output.insert("tsr_sigma".to_string(), self.tsr_sigma.clone().into());
        output
    }
    const NAME: &'static str = "TemporalScoreRescaling";
    const DISPLAY_NAME: &'static str = "TSR - Temporal Score Rescaling";
    const DESCRIPTION: &'static str = "[Post-CFG Function]\nTSR - Temporal Score Rescaling (2510.01184)\n\nRescaling the model's score or noise to steer the sampling diversity.\n";
    const CATEGORY: &'static str = "model_patches/unet";
}
///**TomePatchModel**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
