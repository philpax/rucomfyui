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
    ConditioningToParam: crate::nodes::types::Conditioning,
    ClipParam: crate::nodes::types::Clip,
    GligenTextboxModelParam: crate::nodes::types::Gligen,
    TextParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub conditioning_to: ConditioningToParam,
    ///No documentation.
    pub clip: ClipParam,
    ///No documentation.
    pub gligen_textbox_model: GligenTextboxModelParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: TextParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 8
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 64
  - Max: 16384
  - Min: 8
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: YParam,
}
impl<
    ConditioningToParam: crate::nodes::types::Conditioning,
    ClipParam: crate::nodes::types::Clip,
    GligenTextboxModelParam: crate::nodes::types::Gligen,
    TextParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> GligenTextBoxApply<
    ConditioningToParam,
    ClipParam,
    GligenTextboxModelParam,
    TextParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
> {
    /// Create a new node.
    pub fn new(
        conditioning_to: ConditioningToParam,
        clip: ClipParam,
        gligen_textbox_model: GligenTextboxModelParam,
        text: TextParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
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
    ConditioningToParam: crate::nodes::types::Conditioning,
    ClipParam: crate::nodes::types::Clip,
    GligenTextboxModelParam: crate::nodes::types::Gligen,
    TextParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for GligenTextBoxApply<
    ConditioningToParam,
    ClipParam,
    GligenTextboxModelParam,
    TextParam,
    WidthParam,
    HeightParam,
    XParam,
    YParam,
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
