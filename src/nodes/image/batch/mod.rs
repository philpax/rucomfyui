//!`batch` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageFromBatch`](super::ImageFromBatch).
    #[derive(Clone)]
    pub struct ImageFromBatchOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`RebatchImages`](super::RebatchImages).
    #[derive(Clone)]
    pub struct RebatchImagesOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`RepeatImageBatch`](super::RepeatImageBatch).
    #[derive(Clone)]
    pub struct RepeatImageBatchOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
///**ImageFromBatch**: No description.
pub struct ImageFromBatch<
    Image: crate::nodes::types::Image,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub batch_index: BatchIndex,
    ///No documentation.
    pub length: Length,
}
impl<
    Image: crate::nodes::types::Image,
    BatchIndex: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageFromBatch<Image, BatchIndex, Length> {
    type Output = out::ImageFromBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageFromBatch";
    const DISPLAY_NAME: &'static str = "ImageFromBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Rebatch Images**: No description.
pub struct RebatchImages<
    Images: crate::nodes::types::Image,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub batch_size: BatchSize,
}
impl<
    Images: crate::nodes::types::Image,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchImages<Images, BatchSize> {
    type Output = out::RebatchImagesOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RebatchImages";
    const DISPLAY_NAME: &'static str = "Rebatch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**RepeatImageBatch**: No description.
pub struct RepeatImageBatch<
    Image: crate::nodes::types::Image,
    Amount: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub amount: Amount,
}
impl<
    Image: crate::nodes::types::Image,
    Amount: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatImageBatch<Image, Amount> {
    type Output = out::RepeatImageBatchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "RepeatImageBatch";
    const DISPLAY_NAME: &'static str = "RepeatImageBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
