//!`transform` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Crop Latent**: No description.
#[derive(Clone)]
pub struct LatentCrop<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: X,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: Y,
}
impl<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> LatentCrop<Samples, Width, Height, X, Y> {
    /// Create a new node.
    pub fn new(samples: Samples, width: Width, height: Height, x: X, y: Y) -> Self {
        Self {
            samples,
            width,
            height,
            x,
            y,
        }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
> crate::nodes::TypedNode for LatentCrop<Samples, Width, Height, X, Y> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output
    }
    const NAME: &'static str = "LatentCrop";
    const DISPLAY_NAME: &'static str = "Crop Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Flip Latent**: No description.
#[derive(Clone)]
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
> LatentFlip<Samples, FlipMethod> {
    /// Create a new node.
    pub fn new(samples: Samples, flip_method: FlipMethod) -> Self {
        Self { samples, flip_method }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    FlipMethod: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentFlip<Samples, FlipMethod> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("flip_method".to_string(), self.flip_method.clone().into());
        output
    }
    const NAME: &'static str = "LatentFlip";
    const DISPLAY_NAME: &'static str = "Flip Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Rotate Latent**: No description.
#[derive(Clone)]
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
> LatentRotate<Samples, Rotation> {
    /// Create a new node.
    pub fn new(samples: Samples, rotation: Rotation) -> Self {
        Self { samples, rotation }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Rotation: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentRotate<Samples, Rotation> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("rotation".to_string(), self.rotation.clone().into());
        output
    }
    const NAME: &'static str = "LatentRotate";
    const DISPLAY_NAME: &'static str = "Rotate Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
