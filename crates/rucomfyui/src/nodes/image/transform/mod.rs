//!`transform` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Image Crop**: No description.
#[derive(Clone)]
pub struct ImageCrop<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: YParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> ImageCrop<ImageParam, WidthParam, HeightParam, XParam, YParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
    ) -> Self {
        Self { image, width, height, x, y }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImageCrop<ImageParam, WidthParam, HeightParam, XParam, YParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output
    }
    const NAME: &'static str = "ImageCrop";
    const DISPLAY_NAME: &'static str = "Image Crop";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
