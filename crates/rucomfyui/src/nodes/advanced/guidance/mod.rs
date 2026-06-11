//!`guidance` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**CFGNorm**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CFGNorm<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
    PreCfgParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**If true, rescale the combined noise BEFORE the sampler's CFG combine, without clamping (can amplify). Matches the norm-scaled CFG used by models like Lens. Default false keeps the original post-CFG x0-space attenuate-only behavior.

**Metadata**:
  - Default: false
*/
    pub pre_cfg: Option<PreCfgParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
    PreCfgParam: crate::nodes::types::Boolean,
> CFGNorm<ModelParam, StrengthParam, PreCfgParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        strength: StrengthParam,
        pre_cfg: Option<PreCfgParam>,
    ) -> Self {
        Self { model, strength, pre_cfg }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StrengthParam: crate::nodes::types::Float,
    PreCfgParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for CFGNorm<ModelParam, StrengthParam, PreCfgParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        if let Some(v) = &self.pre_cfg {
            output.insert("pre_cfg".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CFGNorm";
    const DISPLAY_NAME: &'static str = "CFGNorm";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/guidance";
}
///**CFGZeroStar**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CFGZeroStar<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> CFGZeroStar<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for CFGZeroStar<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "CFGZeroStar";
    const DISPLAY_NAME: &'static str = "CFGZeroStar";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/guidance";
}
///**Normalized Attention Guidance**: Applies Normalized Attention Guidance to models, enabling negative prompts on distilled/schnell models.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NAGuidance<
    ModelParam: crate::nodes::types::Model,
    NagScaleParam: crate::nodes::types::Float,
    NagAlphaParam: crate::nodes::types::Float,
    NagTauParam: crate::nodes::types::Float,
> {
    ///The model to apply NAG to.
    pub model: ModelParam,
    /**The guidance scale factor. Higher values push further from the negative prompt.

**Metadata**:
  - Default: 5
  - Max: 50
  - Min: 0
  - Step: 0.1
*/
    pub nag_scale: NagScaleParam,
    /**Blending factor for the normalized attention. 1.0 is full replacement, 0.0 is no effect.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub nag_alpha: NagAlphaParam,
    /**No documentation.

**Metadata**:
  - Default: 1.5
  - Max: 10
  - Min: 1
  - Step: 0.01
*/
    pub nag_tau: NagTauParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    NagScaleParam: crate::nodes::types::Float,
    NagAlphaParam: crate::nodes::types::Float,
    NagTauParam: crate::nodes::types::Float,
> NAGuidance<ModelParam, NagScaleParam, NagAlphaParam, NagTauParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        nag_scale: NagScaleParam,
        nag_alpha: NagAlphaParam,
        nag_tau: NagTauParam,
    ) -> Self {
        Self {
            model,
            nag_scale,
            nag_alpha,
            nag_tau,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    NagScaleParam: crate::nodes::types::Float,
    NagAlphaParam: crate::nodes::types::Float,
    NagTauParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for NAGuidance<ModelParam, NagScaleParam, NagAlphaParam, NagTauParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("nag_scale".to_string(), self.nag_scale.clone().into());
        output.insert("nag_alpha".to_string(), self.nag_alpha.clone().into());
        output.insert("nag_tau".to_string(), self.nag_tau.clone().into());
        output
    }
    const NAME: &'static str = "NAGuidance";
    const DISPLAY_NAME: &'static str = "Normalized Attention Guidance";
    const DESCRIPTION: &'static str = "Applies Normalized Attention Guidance to models, enabling negative prompts on distilled/schnell models.";
    const CATEGORY: &'static str = "advanced/guidance";
}
///**SkipLayerGuidanceDiT**: Generic version of SkipLayerGuidance node that can be used on every DiT model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SkipLayerGuidanceDiT<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    RescalingScaleParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub double_layers: DoubleLayersParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub single_layers: SingleLayersParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub scale: ScaleParam,
    /**No documentation.

**Metadata**:
  - Default: 0.01
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub rescaling_scale: RescalingScaleParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    RescalingScaleParam: crate::nodes::types::Float,
> SkipLayerGuidanceDiT<
    ModelParam,
    DoubleLayersParam,
    SingleLayersParam,
    ScaleParam,
    StartPercentParam,
    EndPercentParam,
    RescalingScaleParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        double_layers: DoubleLayersParam,
        single_layers: SingleLayersParam,
        scale: ScaleParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
        rescaling_scale: RescalingScaleParam,
    ) -> Self {
        Self {
            model,
            double_layers,
            single_layers,
            scale,
            start_percent,
            end_percent,
            rescaling_scale,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
    RescalingScaleParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SkipLayerGuidanceDiT<
    ModelParam,
    DoubleLayersParam,
    SingleLayersParam,
    ScaleParam,
    StartPercentParam,
    EndPercentParam,
    RescalingScaleParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("double_layers".to_string(), self.double_layers.clone().into());
        output.insert("single_layers".to_string(), self.single_layers.clone().into());
        output.insert("scale".to_string(), self.scale.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
            .insert("rescaling_scale".to_string(), self.rescaling_scale.clone().into());
        output
    }
    const NAME: &'static str = "SkipLayerGuidanceDiT";
    const DISPLAY_NAME: &'static str = "SkipLayerGuidanceDiT";
    const DESCRIPTION: &'static str = "Generic version of SkipLayerGuidance node that can be used on every DiT model.";
    const CATEGORY: &'static str = "advanced/guidance";
}
///**SkipLayerGuidanceDiTSimple**: Simple version of the SkipLayerGuidanceDiT node that only modifies the uncond pass.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SkipLayerGuidanceDiTSimple<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub double_layers: DoubleLayersParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub single_layers: SingleLayersParam,
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
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> SkipLayerGuidanceDiTSimple<
    ModelParam,
    DoubleLayersParam,
    SingleLayersParam,
    StartPercentParam,
    EndPercentParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        double_layers: DoubleLayersParam,
        single_layers: SingleLayersParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            model,
            double_layers,
            single_layers,
            start_percent,
            end_percent,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    DoubleLayersParam: crate::nodes::types::String,
    SingleLayersParam: crate::nodes::types::String,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SkipLayerGuidanceDiTSimple<
    ModelParam,
    DoubleLayersParam,
    SingleLayersParam,
    StartPercentParam,
    EndPercentParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("double_layers".to_string(), self.double_layers.clone().into());
        output.insert("single_layers".to_string(), self.single_layers.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "SkipLayerGuidanceDiTSimple";
    const DISPLAY_NAME: &'static str = "SkipLayerGuidanceDiTSimple";
    const DESCRIPTION: &'static str = "Simple version of the SkipLayerGuidanceDiT node that only modifies the uncond pass.";
    const CATEGORY: &'static str = "advanced/guidance";
}
///**SkipLayerGuidanceSD3**: Generic version of SkipLayerGuidance node that can be used on every DiT model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SkipLayerGuidanceSD3<
    ModelParam: crate::nodes::types::Model,
    LayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub layers: LayersParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub scale: ScaleParam,
    /**No documentation.

**Metadata**:
  - Default: 0.01
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**No documentation.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    LayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> SkipLayerGuidanceSD3<
    ModelParam,
    LayersParam,
    ScaleParam,
    StartPercentParam,
    EndPercentParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        layers: LayersParam,
        scale: ScaleParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            model,
            layers,
            scale,
            start_percent,
            end_percent,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    LayersParam: crate::nodes::types::String,
    ScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SkipLayerGuidanceSD3<
    ModelParam,
    LayersParam,
    ScaleParam,
    StartPercentParam,
    EndPercentParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("layers".to_string(), self.layers.clone().into());
        output.insert("scale".to_string(), self.scale.clone().into());
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "SkipLayerGuidanceSD3";
    const DISPLAY_NAME: &'static str = "SkipLayerGuidanceSD3";
    const DESCRIPTION: &'static str = "Generic version of SkipLayerGuidance node that can be used on every DiT model.";
    const CATEGORY: &'static str = "advanced/guidance";
}
#[doc = "**Tangential Damping CFG**: TCFG – Tangential Damping CFG (2503.18137)\n\n\n\nRefine the uncond (negative) to align with the cond (positive) for improving quality."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TCFG<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> TCFG<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for TCFG<ModelParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
    }
    const NAME: &'static str = "TCFG";
    const DISPLAY_NAME: &'static str = "Tangential Damping CFG";
    const DESCRIPTION: &'static str = "TCFG – Tangential Damping CFG (2503.18137)\n\nRefine the uncond (negative) to align with the cond (positive) for improving quality.";
    const CATEGORY: &'static str = "advanced/guidance";
}
