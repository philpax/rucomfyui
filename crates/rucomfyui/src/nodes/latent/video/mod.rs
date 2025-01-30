//!`video` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod ltxv;
///**EmptyCosmosLatentVideo**: No description.
#[derive(Clone)]
pub struct EmptyCosmosLatentVideo<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1280
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 704
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 121
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
> EmptyCosmosLatentVideo<Width, Height, Length, BatchSize> {
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
> crate::nodes::TypedNode for EmptyCosmosLatentVideo<Width, Height, Length, BatchSize> {
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
    const NAME: &'static str = "EmptyCosmosLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyCosmosLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/video";
}
///**EmptyHunyuanLatentVideo**: No description.
#[derive(Clone)]
pub struct EmptyHunyuanLatentVideo<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 1
  - Step: 4
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
> EmptyHunyuanLatentVideo<Width, Height, Length, BatchSize> {
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
> crate::nodes::TypedNode for EmptyHunyuanLatentVideo<Width, Height, Length, BatchSize> {
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
    const NAME: &'static str = "EmptyHunyuanLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyHunyuanLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/video";
}
///**EmptyMochiLatentVideo**: No description.
#[derive(Clone)]
pub struct EmptyMochiLatentVideo<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 7
  - Step: 6
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
> EmptyMochiLatentVideo<Width, Height, Length, BatchSize> {
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
> crate::nodes::TypedNode for EmptyMochiLatentVideo<Width, Height, Length, BatchSize> {
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
    const NAME: &'static str = "EmptyMochiLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyMochiLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/video";
}
