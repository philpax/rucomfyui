//!`model_patches` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod flux;
///**ScaleROPE**: Scale and shift the ROPE of the model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ScaleROPE<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_x: ScaleXParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_x: ShiftXParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_y: ScaleYParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_y: ShiftYParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Step: 0.1
*/
    pub scale_t: ScaleTParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 256
  - Min: -256
  - Step: 0.1
*/
    pub shift_t: ShiftTParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> ScaleROPE<
    ModelParam,
    ScaleXParam,
    ShiftXParam,
    ScaleYParam,
    ShiftYParam,
    ScaleTParam,
    ShiftTParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        scale_x: ScaleXParam,
        shift_x: ShiftXParam,
        scale_y: ScaleYParam,
        shift_y: ShiftYParam,
        scale_t: ScaleTParam,
        shift_t: ShiftTParam,
    ) -> Self {
        Self {
            model,
            scale_x,
            shift_x,
            scale_y,
            shift_y,
            scale_t,
            shift_t,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ScaleXParam: crate::nodes::types::Float,
    ShiftXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ShiftYParam: crate::nodes::types::Float,
    ScaleTParam: crate::nodes::types::Float,
    ShiftTParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ScaleROPE<
    ModelParam,
    ScaleXParam,
    ShiftXParam,
    ScaleYParam,
    ShiftYParam,
    ScaleTParam,
    ShiftTParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scale_x".to_string(), self.scale_x.clone().into());
        output.insert("shift_x".to_string(), self.shift_x.clone().into());
        output.insert("scale_y".to_string(), self.scale_y.clone().into());
        output.insert("shift_y".to_string(), self.shift_y.clone().into());
        output.insert("scale_t".to_string(), self.scale_t.clone().into());
        output.insert("shift_t".to_string(), self.shift_t.clone().into());
        output
    }
    const NAME: &'static str = "ScaleROPE";
    const DISPLAY_NAME: &'static str = "ScaleROPE";
    const DESCRIPTION: &'static str = "Scale and shift the ROPE of the model.";
    const CATEGORY: &'static str = "advanced/model_patches";
}
