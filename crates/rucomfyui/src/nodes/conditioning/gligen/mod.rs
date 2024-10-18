//!`gligen` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**GLIGENTextBoxApply**: No description.
#[derive(Clone)]
pub struct GligenTextBoxApply<
    ConditioningTo: crate::nodes::types::Conditioning,
    Clip: crate::nodes::types::Clip,
    GligenTextboxModel: crate::nodes::types::Gligen,
    Text: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub gligen_textbox_model: GligenTextboxModel,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: Text,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 8
  - Step: 8
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 8
  - Step: 8
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: X,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: Y,
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    Clip: crate::nodes::types::Clip,
    GligenTextboxModel: crate::nodes::types::Gligen,
    Text: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> GligenTextBoxApply<
    ConditioningTo,
    Clip,
    GligenTextboxModel,
    Text,
    Width,
    Height,
    X,
    Y,
> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningTo,
        clip: Clip,
        gligen_textbox_model: GligenTextboxModel,
        text: Text,
        width: Width,
        height: Height,
        x: X,
        y: Y,
    ) -> Self {
        Self {
            conditioning_to,
            clip,
            gligen_textbox_model,
            text,
            width,
            height,
            x,
            y,
        }
    }
}
impl<
    ConditioningTo: crate::nodes::types::Conditioning,
    Clip: crate::nodes::types::Clip,
    GligenTextboxModel: crate::nodes::types::Gligen,
    Text: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode
for GligenTextBoxApply<
    ConditioningTo,
    Clip,
    GligenTextboxModel,
    Text,
    Width,
    Height,
    X,
    Y,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("conditioning_to".to_string(), self.conditioning_to.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output
            .insert(
                "gligen_textbox_model".to_string(),
                self.gligen_textbox_model.clone().into(),
            );
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output
    }
    const NAME: &'static str = "GLIGENTextBoxApply";
    const DISPLAY_NAME: &'static str = "GLIGENTextBoxApply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/gligen";
}
