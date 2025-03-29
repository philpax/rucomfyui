//!`sd3` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**EmptySD3LatentImage**: No description.
#[derive(Clone)]
pub struct EmptySd3LatentImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
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
    BatchSizeParam: crate::nodes::types::Int,
> EmptySd3LatentImage<WidthParam, HeightParam, BatchSizeParam> {
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
for EmptySd3LatentImage<WidthParam, HeightParam, BatchSizeParam> {
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
    const NAME: &'static str = "EmptySD3LatentImage";
    const DISPLAY_NAME: &'static str = "EmptySD3LatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/sd3";
}
