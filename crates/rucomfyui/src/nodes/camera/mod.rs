//!`camera` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`WanCameraEmbedding`](super::WanCameraEmbedding).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanCameraEmbeddingOutput {
        ///No documentation.
        pub camera_embedding: crate::nodes::types::WanCameraEmbeddingOut,
        ///No documentation.
        pub width: crate::nodes::types::IntOut,
        ///No documentation.
        pub height: crate::nodes::types::IntOut,
        ///No documentation.
        pub length: crate::nodes::types::IntOut,
    }
}
///**WanCameraEmbedding**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanCameraEmbedding<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    FxParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    FyParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    CxParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    CyParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 16384
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.1
*/
    pub speed: Option<SpeedParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.000000001
*/
    pub fx: Option<FxParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.000000001
*/
    pub fy: Option<FyParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cx: Option<CxParam>,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cy: Option<CyParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float,
    FxParam: crate::nodes::types::Float,
    FyParam: crate::nodes::types::Float,
    CxParam: crate::nodes::types::Float,
    CyParam: crate::nodes::types::Float,
> WanCameraEmbedding<
    WidthParam,
    HeightParam,
    LengthParam,
    SpeedParam,
    FxParam,
    FyParam,
    CxParam,
    CyParam,
> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        speed: Option<SpeedParam>,
        fx: Option<FxParam>,
        fy: Option<FyParam>,
        cx: Option<CxParam>,
        cy: Option<CyParam>,
    ) -> Self {
        Self {
            width,
            height,
            length,
            speed,
            fx,
            fy,
            cx,
            cy,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    SpeedParam: crate::nodes::types::Float,
    FxParam: crate::nodes::types::Float,
    FyParam: crate::nodes::types::Float,
    CxParam: crate::nodes::types::Float,
    CyParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for WanCameraEmbedding<
    WidthParam,
    HeightParam,
    LengthParam,
    SpeedParam,
    FxParam,
    FyParam,
    CxParam,
    CyParam,
> {
    type Output = out::WanCameraEmbeddingOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            camera_embedding: crate::nodes::types::WanCameraEmbeddingOut::from_dynamic(
                node_id,
                0u32,
            ),
            width: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
            height: crate::nodes::types::IntOut::from_dynamic(node_id, 2u32),
            length: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        if let Some(v) = &self.speed {
            output.insert("speed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.fx {
            output.insert("fx".to_string(), v.clone().into());
        }
        if let Some(v) = &self.fy {
            output.insert("fy".to_string(), v.clone().into());
        }
        if let Some(v) = &self.cx {
            output.insert("cx".to_string(), v.clone().into());
        }
        if let Some(v) = &self.cy {
            output.insert("cy".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "WanCameraEmbedding";
    const DISPLAY_NAME: &'static str = "WanCameraEmbedding";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "camera";
}
