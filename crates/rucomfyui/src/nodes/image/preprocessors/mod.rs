//!`preprocessors` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Canny**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Canny<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0.4
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub low_threshold: LowThresholdParam,
    /**No documentation.

**Metadata**:
  - Default: 0.8
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub high_threshold: HighThresholdParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> Canny<ImageParam, LowThresholdParam, HighThresholdParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        low_threshold: LowThresholdParam,
        high_threshold: HighThresholdParam,
    ) -> Self {
        Self {
            image,
            low_threshold,
            high_threshold,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for Canny<ImageParam, LowThresholdParam, HighThresholdParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("low_threshold".to_string(), self.low_threshold.clone().into());
        output.insert("high_threshold".to_string(), self.high_threshold.clone().into());
        output
    }
    const NAME: &'static str = "Canny";
    const DISPLAY_NAME: &'static str = "Canny";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/preprocessors";
}
