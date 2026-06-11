//!`HitPaw` definitions/categories.
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
///**HitPaw General Image Enhance**: Upscale low-resolution images to super-resolution, eliminate artifacts and noise. Maximum output: 32 megapixels.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HitPawGeneralImageEnhance<
    ImageParam: crate::nodes::types::Image,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Automatically downscale input image if output would exceed the limit.

**Metadata**:
  - Default: false
*/
    pub auto_downscale: AutoDownscaleParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> HitPawGeneralImageEnhance<ImageParam, AutoDownscaleParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, auto_downscale: AutoDownscaleParam) -> Self {
        Self { image, auto_downscale }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for HitPawGeneralImageEnhance<ImageParam, AutoDownscaleParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("auto_downscale".to_string(), self.auto_downscale.clone().into());
        output
    }
    const NAME: &'static str = "HitPawGeneralImageEnhance";
    const DISPLAY_NAME: &'static str = "HitPaw General Image Enhance";
    const DESCRIPTION: &'static str = "Upscale low-resolution images to super-resolution, eliminate artifacts and noise. Maximum output: 32 megapixels.";
    const CATEGORY: &'static str = "partner/image/HitPaw";
}
