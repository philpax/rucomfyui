//!`transform` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**Crop Latent**
pub struct LatentCrop<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
///Output for [`LatentCrop`].
#[derive(Clone)]
pub struct LatentCropOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentCrop<Samples, Width, Height, X, Y> {
    type Output = LatentCropOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentCrop";
    const DISPLAY_NAME: &'static str = "Crop Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Flip Latent**
pub struct LatentFlip<
    Samples: crate::nodes::types::Latent,
    FlipMethod: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub flip_method: FlipMethod,
}
///Output for [`LatentFlip`].
#[derive(Clone)]
pub struct LatentFlipOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    FlipMethod: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentFlip<Samples, FlipMethod> {
    type Output = LatentFlipOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentFlip";
    const DISPLAY_NAME: &'static str = "Flip Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Rotate Latent**
pub struct LatentRotate<
    Samples: crate::nodes::types::Latent,
    Rotation: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub rotation: Rotation,
}
///Output for [`LatentRotate`].
#[derive(Clone)]
pub struct LatentRotateOutput {
    ///No documentation.
    pub latent: crate::nodes::types::LatentOut,
}
impl<
    Samples: crate::nodes::types::Latent,
    Rotation: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentRotate<Samples, Rotation> {
    type Output = LatentRotateOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LatentRotate";
    const DISPLAY_NAME: &'static str = "Rotate Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
