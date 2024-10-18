//!`preprocessors` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Canny**: No description.
#[derive(Clone)]
pub struct Canny<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 0.4
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub low_threshold: LowThreshold,
    /**No documentation.

**Metadata**:
  - Default: 0.8
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub high_threshold: HighThreshold,
}
impl<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> Canny<Image, LowThreshold, HighThreshold> {
    /// Create a new node.
    pub fn new(
        image: Image,
        low_threshold: LowThreshold,
        high_threshold: HighThreshold,
    ) -> Self {
        Self {
            image,
            low_threshold,
            high_threshold,
        }
    }
}
impl<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> crate::nodes::TypedNode for Canny<Image, LowThreshold, HighThreshold> {
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
