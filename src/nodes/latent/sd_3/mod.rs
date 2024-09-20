//!`sd3` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
/// Output types for nodes.
pub mod out {
    ///Output for [`EmptySd3LatentImage`](super::EmptySd3LatentImage).
    #[derive(Clone)]
    pub struct EmptySd3LatentImageOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**EmptySD3LatentImage**: No description.
pub struct EmptySd3LatentImage<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub batch_size: BatchSize,
}
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptySd3LatentImage<Width, Height, BatchSize> {
    type Output = out::EmptySd3LatentImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "EmptySD3LatentImage";
    const DISPLAY_NAME: &'static str = "EmptySD3LatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/sd3";
}
