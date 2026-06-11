//!`image` definitions/categories.
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
///**Empty HiDream-O1 Latent Image**: Empty pixel-space latent for HiDream-O1-Image. The model was trained at ~4 megapixels; lower resolutions go off-distribution and quality regresses noticeably. Trained resolutions: 2048x2048, 2304x1728, 1728x2304, 2560x1440, 1440x2560, 2496x1664, 1664x2496, 3104x1312, 1312x3104, 2304x1792, 1792x2304.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyHiDreamO1LatentImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 2048
  - Max: 4096
  - Min: 64
  - Step: 32
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 2048
  - Max: 4096
  - Min: 64
  - Step: 32
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyHiDreamO1LatentImage<WidthParam, HeightParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self { width, height, batch_size }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyHiDreamO1LatentImage<WidthParam, HeightParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyHiDreamO1LatentImage";
    const DISPLAY_NAME: &'static str = "Empty HiDream-O1 Latent Image";
    const DESCRIPTION: &'static str = "Empty pixel-space latent for HiDream-O1-Image. The model was trained at ~4 megapixels; lower resolutions go off-distribution and quality regresses noticeably. Trained resolutions: 2048x2048, 2304x1728, 1728x2304, 2560x1440, 1440x2560, 2496x1664, 1664x2496, 3104x1312, 1312x3104, 2304x1792, 1792x2304.";
    const CATEGORY: &'static str = "model/latent/image";
}
