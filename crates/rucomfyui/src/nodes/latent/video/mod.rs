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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1280
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 704
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 121
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyCosmosLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyCosmosLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyHunyuanLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyHunyuanLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 848
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 25
  - Max: 16384
  - Min: 7
  - Step: 6
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyMochiLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyMochiLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
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
