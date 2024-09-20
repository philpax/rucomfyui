//!`preprocessors` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`Canny`](super::Canny).
    #[derive(Clone)]
    pub struct CannyOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
///**Canny**
pub struct Canny<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub low_threshold: LowThreshold,
    ///No documentation.
    pub high_threshold: HighThreshold,
}
impl<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> crate::nodes::TypedNode for Canny<Image, LowThreshold, HighThreshold> {
    type Output = out::CannyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "Canny";
    const DISPLAY_NAME: &'static str = "Canny";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/preprocessors";
}
