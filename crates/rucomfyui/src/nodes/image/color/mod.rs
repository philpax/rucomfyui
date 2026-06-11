//!`color` definitions/categories.
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
///**Invert Image Colors**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageInvert<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageInvert<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageInvert<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageInvert";
    const DISPLAY_NAME: &'static str = "Invert Image Colors";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/color";
}
///**Image RGB to YUV**: No description.
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
    const DISPLAY_NAME: &'static str = "Image RGB to YUV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/color";
}
///**Image YUV to RGB**: No description.
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
    const DISPLAY_NAME: &'static str = "Image YUV to RGB";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/color";
}
///**Normalize Image Colors**: Normalize images using mean and standard deviation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NormalizeImages<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Mean value for normalization.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
*/
    pub mean: MeanParam,
    /**Standard deviation for normalization.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0.001
*/
    pub std: StdParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> NormalizeImages<ImagesParam, MeanParam, StdParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, mean: MeanParam, std: StdParam) -> Self {
        Self { images, mean, std }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for NormalizeImages<ImagesParam, MeanParam, StdParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("mean".to_string(), self.mean.clone().into());
        output.insert("std".to_string(), self.std.clone().into());
        output
    }
    const NAME: &'static str = "NormalizeImages";
    const DISPLAY_NAME: &'static str = "Normalize Image Colors";
    const DESCRIPTION: &'static str = "Normalize images using mean and standard deviation.";
    const CATEGORY: &'static str = "image/color";
}
