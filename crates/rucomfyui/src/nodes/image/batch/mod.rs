//!`batch` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**ImageFromBatch**: No description.
#[derive(Clone)]
pub struct ImageFromBatch<
    Image: crate::nodes::types::Image,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 4095
  - Min: 0
*/
    pub batch_index: BatchIndex,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub length: Length,
}
impl<
    Image: crate::nodes::types::Image,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> ImageFromBatch<Image, BatchIndex, Length> {
    /// Create a new node.
    pub fn new(image: Image, batch_index: BatchIndex, length: Length) -> Self {
        Self { image, batch_index, length }
    }
}
impl<
    Image: crate::nodes::types::Image,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageFromBatch<Image, BatchIndex, Length> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("batch_index".to_string(), self.batch_index.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output
    }
    const NAME: &'static str = "ImageFromBatch";
    const DISPLAY_NAME: &'static str = "ImageFromBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Rebatch Images**: No description.
#[derive(Clone)]
pub struct RebatchImages<
    Images: crate::nodes::types::Image,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: Images,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
}
impl<
    Images: crate::nodes::types::Image,
    BatchSize: crate::nodes::types::Int,
> RebatchImages<Images, BatchSize> {
    /// Create a new node.
    pub fn new(images: Images, batch_size: BatchSize) -> Self {
        Self { images, batch_size }
    }
}
impl<
    Images: crate::nodes::types::Image,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchImages<Images, BatchSize> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "RebatchImages";
    const DISPLAY_NAME: &'static str = "Rebatch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**RepeatImageBatch**: No description.
#[derive(Clone)]
pub struct RepeatImageBatch<
    Image: crate::nodes::types::Image,
    Amount: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub amount: Amount,
}
impl<
    Image: crate::nodes::types::Image,
    Amount: crate::nodes::types::Int,
> RepeatImageBatch<Image, Amount> {
    /// Create a new node.
    pub fn new(image: Image, amount: Amount) -> Self {
        Self { image, amount }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Amount: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatImageBatch<Image, Amount> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("amount".to_string(), self.amount.clone().into());
        output
    }
    const NAME: &'static str = "RepeatImageBatch";
    const DISPLAY_NAME: &'static str = "RepeatImageBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
