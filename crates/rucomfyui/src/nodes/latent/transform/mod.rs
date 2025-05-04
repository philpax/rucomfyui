//!`transform` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Crop Latent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LatentCrop<
    SamplesParam: crate::nodes::types::Latent,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 8
*/
    pub y: YParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> LatentCrop<SamplesParam, WidthParam, HeightParam, XParam, YParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        width: WidthParam,
        height: HeightParam,
        x: XParam,
        y: YParam,
    ) -> Self {
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
    SamplesParam: crate::nodes::types::Latent,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for LatentCrop<SamplesParam, WidthParam, HeightParam, XParam, YParam> {
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
#[allow(non_camel_case_types)]
pub struct LatentFlip<
    SamplesParam: crate::nodes::types::Latent,
    FlipMethodParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub flip_method: FlipMethodParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    FlipMethodParam: crate::nodes::types::String,
> LatentFlip<SamplesParam, FlipMethodParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, flip_method: FlipMethodParam) -> Self {
        Self { samples, flip_method }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    FlipMethodParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentFlip<SamplesParam, FlipMethodParam> {
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
#[allow(non_camel_case_types)]
pub struct LatentRotate<
    SamplesParam: crate::nodes::types::Latent,
    RotationParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub rotation: RotationParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    RotationParam: crate::nodes::types::String,
> LatentRotate<SamplesParam, RotationParam> {
    /// Create a new node.
    pub fn new(samples: SamplesParam, rotation: RotationParam) -> Self {
        Self { samples, rotation }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    RotationParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LatentRotate<SamplesParam, RotationParam> {
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
