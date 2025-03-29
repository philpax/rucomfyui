//!`guidance` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**CFGZeroStar**: No description.
#[derive(Clone)]
pub struct CfgZeroStar<Model: crate::nodes::types::Model> {
    ///No documentation.
    pub model: Model,
}
impl<Model: crate::nodes::types::Model> CfgZeroStar<Model> {
    /// Create a new node.
    pub fn new(model: Model) -> Self {
        Self { model }
    }
}
impl<Model: crate::nodes::types::Model> crate::nodes::TypedNode for CfgZeroStar<Model> {
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
    Model: crate::nodes::types::Model,
    DoubleLayers: crate::nodes::types::String,
    SingleLayers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    RescalingScale: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub double_layers: DoubleLayers,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub single_layers: SingleLayers,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub scale: Scale,
    /**No documentation.

**Metadata**:
  - Default: 0.01
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub rescaling_scale: RescalingScale,
}
impl<
    Model: crate::nodes::types::Model,
    DoubleLayers: crate::nodes::types::String,
    SingleLayers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    RescalingScale: crate::nodes::types::Float,
> SkipLayerGuidanceDiT<
    Model,
    DoubleLayers,
    SingleLayers,
    Scale,
    StartPercent,
    EndPercent,
    RescalingScale,
> {
    /// Create a new node.
    pub fn new(
        model: Model,
        double_layers: DoubleLayers,
        single_layers: SingleLayers,
        scale: Scale,
        start_percent: StartPercent,
        end_percent: EndPercent,
        rescaling_scale: RescalingScale,
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
    Model: crate::nodes::types::Model,
    DoubleLayers: crate::nodes::types::String,
    SingleLayers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
    RescalingScale: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SkipLayerGuidanceDiT<
    Model,
    DoubleLayers,
    SingleLayers,
    Scale,
    StartPercent,
    EndPercent,
    RescalingScale,
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
    Model: crate::nodes::types::Model,
    Layers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 7, 8, 9
*/
    pub layers: Layers,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub scale: Scale,
    /**No documentation.

**Metadata**:
  - Default: 0.01
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercent,
    /**No documentation.

**Metadata**:
  - Default: 0.15
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercent,
}
impl<
    Model: crate::nodes::types::Model,
    Layers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> SkipLayerGuidanceSd3<Model, Layers, Scale, StartPercent, EndPercent> {
    /// Create a new node.
    pub fn new(
        model: Model,
        layers: Layers,
        scale: Scale,
        start_percent: StartPercent,
        end_percent: EndPercent,
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
    Model: crate::nodes::types::Model,
    Layers: crate::nodes::types::String,
    Scale: crate::nodes::types::Float,
    StartPercent: crate::nodes::types::Float,
    EndPercent: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SkipLayerGuidanceSd3<Model, Layers, Scale, StartPercent, EndPercent> {
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
