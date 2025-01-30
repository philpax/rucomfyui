//!`ltxv` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**EmptyLTXVLatentVideo**: No description.
#[derive(Clone)]
pub struct EmptyLtxvLatentVideo<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 768
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 97
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> EmptyLtxvLatentVideo<Width, Height, Length, BatchSize> {
    /// Create a new node.
    pub fn new(
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLtxvLatentVideo<Width, Height, Length, BatchSize> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyLTXVLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyLTXVLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/video/ltxv";
}
