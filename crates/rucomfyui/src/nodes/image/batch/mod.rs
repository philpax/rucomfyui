//!`batch` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageRGBToYUV`](super::ImageRGBToYUV).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ImageRGBToYUVOutput {
        ///No documentation.
        pub y: crate::nodes::types::ImageOut,
        ///No documentation.
        pub u: crate::nodes::types::ImageOut,
        ///No documentation.
        pub v: crate::nodes::types::ImageOut,
    }
}
///**ImageFromBatch**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageFromBatch<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 4095
  - Min: 0
*/
    pub batch_index: BatchIndexParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub length: LengthParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> ImageFromBatch<ImageParam, BatchIndexParam, LengthParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        batch_index: BatchIndexParam,
        length: LengthParam,
    ) -> Self {
        Self { image, batch_index, length }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageFromBatch<ImageParam, BatchIndexParam, LengthParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
///**ImageRGBToYUV**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageRGBToYUV<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageRGBToYUV<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageRGBToYUV<ImageParam> {
    type Output = out::ImageRGBToYUVOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            y: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            u: crate::nodes::types::ImageOut::from_dynamic(node_id, 1u32),
            v: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageRGBToYUV";
    const DISPLAY_NAME: &'static str = "ImageRGBToYUV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**ImageYUVToRGB**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageYUVToRGB<
    YParam: crate::nodes::types::Image,
    UParam: crate::nodes::types::Image,
    VParam: crate::nodes::types::Image,
> {
    ///No documentation.
    pub y: YParam,
    ///No documentation.
    pub u: UParam,
    ///No documentation.
    pub v: VParam,
}
impl<
    YParam: crate::nodes::types::Image,
    UParam: crate::nodes::types::Image,
    VParam: crate::nodes::types::Image,
> ImageYUVToRGB<YParam, UParam, VParam> {
    /// Create a new node.
    pub fn new(y: YParam, u: UParam, v: VParam) -> Self {
        Self { y, u, v }
    }
}
impl<
    YParam: crate::nodes::types::Image,
    UParam: crate::nodes::types::Image,
    VParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageYUVToRGB<YParam, UParam, VParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Y".to_string(), self.y.clone().into());
        output.insert("U".to_string(), self.u.clone().into());
        output.insert("V".to_string(), self.v.clone().into());
        output
    }
    const NAME: &'static str = "ImageYUVToRGB";
    const DISPLAY_NAME: &'static str = "ImageYUVToRGB";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Rebatch Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RebatchImages<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> RebatchImages<ImagesParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, batch_size: BatchSizeParam) -> Self {
        Self { images, batch_size }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchImages<ImagesParam, BatchSizeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct RepeatImageBatch<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub amount: AmountParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> RepeatImageBatch<ImageParam, AmountParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, amount: AmountParam) -> Self {
        Self { image, amount }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatImageBatch<ImageParam, AmountParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
