//!`3d` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`Load3D`](super::Load3D).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Load3DOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
        ///No documentation.
        pub mesh_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub normal: crate::nodes::types::ImageOut,
        ///No documentation.
        pub lineart: crate::nodes::types::ImageOut,
        ///No documentation.
        pub camera_info: crate::nodes::types::Load3DCameraOut,
    }
    ///Output for [`Load3DAnimation`](super::Load3DAnimation).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Load3DAnimationOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
        ///No documentation.
        pub mesh_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub normal: crate::nodes::types::ImageOut,
        ///No documentation.
        pub camera_info: crate::nodes::types::Load3DCameraOut,
    }
}
///**Load 3D**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Load3D<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model_file: ModelFileParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
}
impl<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> Load3D<ModelFileParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFileParam,
        width: WidthParam,
        height: HeightParam,
    ) -> Self {
        Self { model_file, width, height }
    }
}
impl<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Load3D<ModelFileParam, WidthParam, HeightParam> {
    type Output = out::Load3DOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
            normal: crate::nodes::types::ImageOut::from_dynamic(node_id, 3u32),
            lineart: crate::nodes::types::ImageOut::from_dynamic(node_id, 4u32),
            camera_info: crate::nodes::types::Load3DCameraOut::from_dynamic(
                node_id,
                5u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "Load3D";
    const DISPLAY_NAME: &'static str = "Load 3D";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
///**Load 3D - Animation**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Load3DAnimation<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model_file: ModelFileParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
}
impl<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> Load3DAnimation<ModelFileParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFileParam,
        width: WidthParam,
        height: HeightParam,
    ) -> Self {
        Self { model_file, width, height }
    }
}
impl<
    ModelFileParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Load3DAnimation<ModelFileParam, WidthParam, HeightParam> {
    type Output = out::Load3DAnimationOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
            normal: crate::nodes::types::ImageOut::from_dynamic(node_id, 3u32),
            camera_info: crate::nodes::types::Load3DCameraOut::from_dynamic(
                node_id,
                4u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "Load3DAnimation";
    const DISPLAY_NAME: &'static str = "Load 3D - Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
///**Preview 3D**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Preview3D<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub model_file: ModelFileParam,
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> Preview3D<ModelFileParam, CameraInfoParam> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFileParam,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self { model_file, camera_info }
    }
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode for Preview3D<ModelFileParam, CameraInfoParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Preview3D";
    const DISPLAY_NAME: &'static str = "Preview 3D";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedOutputNode for Preview3D<ModelFileParam, CameraInfoParam> {}
///**Preview 3D - Animation**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Preview3DAnimation<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub model_file: ModelFileParam,
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> Preview3DAnimation<ModelFileParam, CameraInfoParam> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFileParam,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self { model_file, camera_info }
    }
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode for Preview3DAnimation<ModelFileParam, CameraInfoParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Preview3DAnimation";
    const DISPLAY_NAME: &'static str = "Preview 3D - Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    ModelFileParam: crate::nodes::types::String,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedOutputNode for Preview3DAnimation<ModelFileParam, CameraInfoParam> {}
///**SaveGLB**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveGLB<
    MeshParam: crate::nodes::types::Mesh,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub mesh: MeshParam,
    /**No documentation.

**Metadata**:
  - Default: mesh/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    MeshParam: crate::nodes::types::Mesh,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveGLB<MeshParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(mesh: MeshParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { mesh, filename_prefix }
    }
}
impl<
    MeshParam: crate::nodes::types::Mesh,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveGLB<MeshParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mesh".to_string(), self.mesh.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveGLB";
    const DISPLAY_NAME: &'static str = "SaveGLB";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    MeshParam: crate::nodes::types::Mesh,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveGLB<MeshParam, FilenamePrefixParam> {}
///**VoxelToMesh**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VoxelToMesh<
    VoxelParam: crate::nodes::types::Voxel,
    AlgorithmParam: crate::nodes::types::String,
    ThresholdParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub voxel: VoxelParam,
    ///No documentation.
    pub algorithm: AlgorithmParam,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 1
  - Min: -1
  - Step: 0.01
*/
    pub threshold: ThresholdParam,
}
impl<
    VoxelParam: crate::nodes::types::Voxel,
    AlgorithmParam: crate::nodes::types::String,
    ThresholdParam: crate::nodes::types::Float,
> VoxelToMesh<VoxelParam, AlgorithmParam, ThresholdParam> {
    /// Create a new node.
    pub fn new(
        voxel: VoxelParam,
        algorithm: AlgorithmParam,
        threshold: ThresholdParam,
    ) -> Self {
        Self {
            voxel,
            algorithm,
            threshold,
        }
    }
}
impl<
    VoxelParam: crate::nodes::types::Voxel,
    AlgorithmParam: crate::nodes::types::String,
    ThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VoxelToMesh<VoxelParam, AlgorithmParam, ThresholdParam> {
    type Output = crate::nodes::types::MeshOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("voxel".to_string(), self.voxel.clone().into());
        output.insert("algorithm".to_string(), self.algorithm.clone().into());
        output.insert("threshold".to_string(), self.threshold.clone().into());
        output
    }
    const NAME: &'static str = "VoxelToMesh";
    const DISPLAY_NAME: &'static str = "VoxelToMesh";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
///**VoxelToMeshBasic**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VoxelToMeshBasic<
    VoxelParam: crate::nodes::types::Voxel,
    ThresholdParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub voxel: VoxelParam,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 1
  - Min: -1
  - Step: 0.01
*/
    pub threshold: ThresholdParam,
}
impl<
    VoxelParam: crate::nodes::types::Voxel,
    ThresholdParam: crate::nodes::types::Float,
> VoxelToMeshBasic<VoxelParam, ThresholdParam> {
    /// Create a new node.
    pub fn new(voxel: VoxelParam, threshold: ThresholdParam) -> Self {
        Self { voxel, threshold }
    }
}
impl<
    VoxelParam: crate::nodes::types::Voxel,
    ThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VoxelToMeshBasic<VoxelParam, ThresholdParam> {
    type Output = crate::nodes::types::MeshOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("voxel".to_string(), self.voxel.clone().into());
        output.insert("threshold".to_string(), self.threshold.clone().into());
        output
    }
    const NAME: &'static str = "VoxelToMeshBasic";
    const DISPLAY_NAME: &'static str = "VoxelToMeshBasic";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
