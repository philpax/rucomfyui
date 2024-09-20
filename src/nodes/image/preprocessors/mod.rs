//!`preprocessors` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
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
///Output for [`Canny`].
#[derive(Clone)]
pub struct CannyOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    LowThreshold: crate::nodes::types::Float,
    HighThreshold: crate::nodes::types::Float,
> crate::nodes::TypedNode for Canny<Image, LowThreshold, HighThreshold> {
    type Output = CannyOutput;
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
