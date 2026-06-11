//!`adjustments` definitions/categories.
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
///**Adjust Brightness**: Adjust the brightness of an image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AdjustBrightness<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Brightness factor. 1.0 = no change, <1.0 = darker, >1.0 = brighter.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: 0
*/
    pub factor: FactorParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> AdjustBrightness<ImagesParam, FactorParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, factor: FactorParam) -> Self {
        Self { images, factor }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for AdjustBrightness<ImagesParam, FactorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("factor".to_string(), self.factor.clone().into());
        output
    }
    const NAME: &'static str = "AdjustBrightness";
    const DISPLAY_NAME: &'static str = "Adjust Brightness";
    const DESCRIPTION: &'static str = "Adjust the brightness of an image.";
    const CATEGORY: &'static str = "image/adjustments";
}
///**Adjust Contrast**: Adjust the contrast of an image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AdjustContrast<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Contrast factor. 1.0 = no change, <1.0 = less contrast, >1.0 = more contrast.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: 0
*/
    pub factor: FactorParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> AdjustContrast<ImagesParam, FactorParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, factor: FactorParam) -> Self {
        Self { images, factor }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for AdjustContrast<ImagesParam, FactorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("factor".to_string(), self.factor.clone().into());
        output
    }
    const NAME: &'static str = "AdjustContrast";
    const DISPLAY_NAME: &'static str = "Adjust Contrast";
    const DESCRIPTION: &'static str = "Adjust the contrast of an image.";
    const CATEGORY: &'static str = "image/adjustments";
}
