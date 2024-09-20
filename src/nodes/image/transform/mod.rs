//!`transform` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageCrop`](super::ImageCrop).
    #[derive(Clone)]
    pub struct ImageCropOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
///**ImageCrop**
pub struct ImageCrop<
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
impl<
    Image: crate::nodes::types::Image,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageCrop<Image, Width, Height, X, Y> {
    type Output = out::ImageCropOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageCrop";
    const DISPLAY_NAME: &'static str = "ImageCrop";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/transform";
}
