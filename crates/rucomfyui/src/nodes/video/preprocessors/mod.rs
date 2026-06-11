//!`preprocessors` definitions/categories.
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
///**LTXV Preprocess**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVPreprocess<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Amount of compression to apply on image.

**Metadata**:
  - Default: 35
  - Max: 100
  - Min: 0
*/
    pub img_compression: ImgCompressionParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> LTXVPreprocess<ImageParam, ImgCompressionParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, img_compression: ImgCompressionParam) -> Self {
        Self { image, img_compression }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImgCompressionParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for LTXVPreprocess<ImageParam, ImgCompressionParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert("img_compression".to_string(), self.img_compression.clone().into());
        output
    }
    const NAME: &'static str = "LTXVPreprocess";
    const DISPLAY_NAME: &'static str = "LTXV Preprocess";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "video/preprocessors";
}
