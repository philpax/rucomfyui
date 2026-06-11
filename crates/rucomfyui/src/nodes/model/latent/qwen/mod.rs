//!`qwen` definitions/categories.
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
///**Empty Qwen Image Layered Latent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyQwenImageLayeredLatentImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LayersParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 640
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 640
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub layers: LayersParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LayersParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyQwenImageLayeredLatentImage<
    WidthParam,
    HeightParam,
    LayersParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        layers: LayersParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            layers,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LayersParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyQwenImageLayeredLatentImage<
    WidthParam,
    HeightParam,
    LayersParam,
    BatchSizeParam,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("layers".to_string(), self.layers.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyQwenImageLayeredLatentImage";
    const DISPLAY_NAME: &'static str = "Empty Qwen Image Layered Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/qwen";
}
