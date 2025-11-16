//!`save` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**SaveSVGNode**: Save SVG files on disk.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveSVGNode<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub svg: SvgParam,
    /**The prefix for the file to save. This may include formatting information such as %date:yyyy-MM-dd% or %Empty Latent Image.width% to include values from nodes.

**Metadata**:
  - Default: svg/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveSVGNode<SvgParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(svg: SvgParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { svg, filename_prefix }
    }
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveSVGNode<SvgParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("svg".to_string(), self.svg.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveSVGNode";
    const DISPLAY_NAME: &'static str = "SaveSVGNode";
    const DESCRIPTION: &'static str = "Save SVG files on disk.";
    const CATEGORY: &'static str = "image/save";
}
impl<
    SvgParam: crate::nodes::types::Svg,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveSVGNode<SvgParam, FilenamePrefixParam> {}
