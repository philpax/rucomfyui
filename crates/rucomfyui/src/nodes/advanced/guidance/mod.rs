//!`guidance` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**CFGZeroStar**: No description.
#[derive(Clone)]
pub struct CfgZeroStar<ModelParam: crate::nodes::types::Model> {
    ///No documentation.
    pub model: ModelParam,
}
impl<ModelParam: crate::nodes::types::Model> CfgZeroStar<ModelParam> {
    /// Create a new node.
    pub fn new(model: ModelParam) -> Self {
        Self { model }
    }
}
impl<ModelParam: crate::nodes::types::Model> crate::nodes::TypedNode
for CfgZeroStar<ModelParam> {
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
///**SkipLayerGuidanceDiT**: Generic version of SkipLayerGuidance node that can be used on every DiT model.
#[derive(Clone)]
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
///**SkipLayerGuidanceSD3**: Generic version of SkipLayerGuidance node that can be used on every DiT model.
#[derive(Clone)]
pub struct SkipLayerGuidanceSd3<
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
> SkipLayerGuidanceSd3<
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
for SkipLayerGuidanceSd3<
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
