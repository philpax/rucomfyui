//!`transform` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`LatentCrop`](super::LatentCrop).
    #[derive(Clone)]
    pub struct LatentCropOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentFlip`](super::LatentFlip).
    #[derive(Clone)]
    pub struct LatentFlipOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
    ///Output for [`LatentRotate`](super::LatentRotate).
    #[derive(Clone)]
    pub struct LatentRotateOutput {
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**Crop Latent**: No description.
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
impl<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentCrop<Samples, Width, Height, X, Y> {
    type Output = out::LatentCropOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("x".to_string(), self.x.to_workflow_input());
        output.insert("y".to_string(), self.y.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentCrop";
    const DISPLAY_NAME: &'static str = "Crop Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Flip Latent**: No description.
pub struct LatentFlip<
    Samples: crate::nodes::types::Latent,
    FlipMethod: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub flip_method: FlipMethod,
}
impl<
    Samples: crate::nodes::types::Latent,
    FlipMethod: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentFlip<Samples, FlipMethod> {
    type Output = out::LatentFlipOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("flip_method".to_string(), self.flip_method.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentFlip";
    const DISPLAY_NAME: &'static str = "Flip Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Rotate Latent**: No description.
pub struct LatentRotate<
    Samples: crate::nodes::types::Latent,
    Rotation: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub rotation: Rotation,
}
impl<
    Samples: crate::nodes::types::Latent,
    Rotation: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentRotate<Samples, Rotation> {
    type Output = out::LatentRotateOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latent: crate::nodes::types::LatentOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.to_workflow_input());
        output.insert("rotation".to_string(), self.rotation.to_workflow_input());
        output
    }
    const NAME: &'static str = "LatentRotate";
    const DISPLAY_NAME: &'static str = "Rotate Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
