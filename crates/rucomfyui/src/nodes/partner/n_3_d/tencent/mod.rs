//!`Tencent` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`Tencent3DTextureEditNode`](super::Tencent3DTextureEditNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Tencent3DTextureEditNodeOutput {
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
        ///No documentation.
        pub obj: crate::nodes::types::File3DObjOut,
        ///No documentation.
        pub texture_image: crate::nodes::types::ImageOut,
    }
    ///Output for [`TencentModelTo3DUVNode`](super::TencentModelTo3DUVNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TencentModelTo3DUVNodeOutput {
        ///No documentation.
        pub obj: crate::nodes::types::File3DObjOut,
        ///No documentation.
        pub fbx: crate::nodes::types::File3DFbxOut,
        ///No documentation.
        pub uv_image: crate::nodes::types::ImageOut,
    }
}
///**Hunyuan3D: 3D Part**: Automatically perform component identification and generation based on the model structure.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Tencent3DPartNode<SeedParam: crate::nodes::types::Int> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<SeedParam: crate::nodes::types::Int> Tencent3DPartNode<SeedParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam) -> Self {
        Self { seed }
    }
}
impl<SeedParam: crate::nodes::types::Int> crate::nodes::TypedNode
for Tencent3DPartNode<SeedParam> {
    type Output = crate::nodes::types::File3DFbxOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Tencent3DPartNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: 3D Part";
    const DESCRIPTION: &'static str = "Automatically perform component identification and generation based on the model structure.";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
///**Hunyuan3D: 3D Texture Edit**: After inputting the 3D model, perform 3D model texture redrawing.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Tencent3DTextureEditNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**Describes texture editing. Supports up to 1024 UTF-8 characters.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Tencent3DTextureEditNode<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Tencent3DTextureEditNode<PromptParam, SeedParam> {
    type Output = out::Tencent3DTextureEditNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 0u32),
            obj: crate::nodes::types::File3DObjOut::from_dynamic(node_id, 1u32),
            texture_image: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Tencent3DTextureEditNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: 3D Texture Edit";
    const DESCRIPTION: &'static str = "After inputting the 3D model, perform 3D model texture redrawing.";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
///**Hunyuan3D: Image(s) to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TencentImageToModelNode<
    ImageParam: crate::nodes::types::Image,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageLeftParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageRightParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageBackParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 500000
  - Max: 1500000
  - Min: 3000
*/
    pub face_count: FaceCountParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    ///No documentation.
    pub image_left: Option<ImageLeftParam>,
    ///No documentation.
    pub image_right: Option<ImageRightParam>,
    ///No documentation.
    pub image_back: Option<ImageBackParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
> TencentImageToModelNode<
    ImageParam,
    FaceCountParam,
    SeedParam,
    ImageLeftParam,
    ImageRightParam,
    ImageBackParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        face_count: FaceCountParam,
        seed: SeedParam,
        image_left: Option<ImageLeftParam>,
        image_right: Option<ImageRightParam>,
        image_back: Option<ImageBackParam>,
    ) -> Self {
        Self {
            image,
            face_count,
            seed,
            image_left,
            image_right,
            image_back,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for TencentImageToModelNode<
    ImageParam,
    FaceCountParam,
    SeedParam,
    ImageLeftParam,
    ImageRightParam,
    ImageBackParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("face_count".to_string(), self.face_count.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.image_left {
            output.insert("image_left".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_right {
            output.insert("image_right".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_back {
            output.insert("image_back".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TencentImageToModelNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: Image(s) to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
impl<
    ImageParam: crate::nodes::types::Image,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    ImageLeftParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode
for TencentImageToModelNode<
    ImageParam,
    FaceCountParam,
    SeedParam,
    ImageLeftParam,
    ImageRightParam,
    ImageBackParam,
> {}
///**Hunyuan3D: Model to UV**: Perform UV unfolding on a 3D model to generate UV texture. Input model must have less than 30000 faces.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TencentModelTo3DUVNode<SeedParam: crate::nodes::types::Int> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<SeedParam: crate::nodes::types::Int> TencentModelTo3DUVNode<SeedParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam) -> Self {
        Self { seed }
    }
}
impl<SeedParam: crate::nodes::types::Int> crate::nodes::TypedNode
for TencentModelTo3DUVNode<SeedParam> {
    type Output = out::TencentModelTo3DUVNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            obj: crate::nodes::types::File3DObjOut::from_dynamic(node_id, 0u32),
            fbx: crate::nodes::types::File3DFbxOut::from_dynamic(node_id, 1u32),
            uv_image: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "TencentModelTo3DUVNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: Model to UV";
    const DESCRIPTION: &'static str = "Perform UV unfolding on a 3D model to generate UV texture. Input model must have less than 30000 faces.";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
///**Hunyuan3D: Smart Topology**: Perform smart retopology on a 3D model. Supports GLB/OBJ formats; max 200MB; recommended for high-poly models.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TencentSmartTopologyNode<SeedParam: crate::nodes::types::Int> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<SeedParam: crate::nodes::types::Int> TencentSmartTopologyNode<SeedParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam) -> Self {
        Self { seed }
    }
}
impl<SeedParam: crate::nodes::types::Int> crate::nodes::TypedNode
for TencentSmartTopologyNode<SeedParam> {
    type Output = crate::nodes::types::File3DObjOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "TencentSmartTopologyNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: Smart Topology";
    const DESCRIPTION: &'static str = "Perform smart retopology on a 3D model. Supports GLB/OBJ formats; max 200MB; recommended for high-poly models.";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
///**Hunyuan3D: Text to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TencentTextToModelNode<
    PromptParam: crate::nodes::types::String,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    /**Supports up to 1024 characters.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 500000
  - Max: 1500000
  - Min: 3000
*/
    pub face_count: FaceCountParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> TencentTextToModelNode<PromptParam, FaceCountParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        face_count: FaceCountParam,
        seed: SeedParam,
    ) -> Self {
        Self { prompt, face_count, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TencentTextToModelNode<PromptParam, FaceCountParam, SeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("face_count".to_string(), self.face_count.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "TencentTextToModelNode";
    const DISPLAY_NAME: &'static str = "Hunyuan3D: Text to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tencent";
}
impl<
    PromptParam: crate::nodes::types::String,
    FaceCountParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for TencentTextToModelNode<PromptParam, FaceCountParam, SeedParam> {}
