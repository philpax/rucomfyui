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
    }
    ///Output for [`Load3DAnimation`](super::Load3DAnimation).
    #[derive(Clone)]
    pub struct Load3DAnimationOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
        ///No documentation.
        pub mesh_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub normal: crate::nodes::types::ImageOut,
    }
}
///**Load 3D**: No description.
#[derive(Clone)]
pub struct Load3D<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model_file: ModelFile,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub height: Height,
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> Load3D<ModelFile, Width, Height> {
    /// Create a new node.
    pub fn new(model_file: ModelFile, width: Width, height: Height) -> Self {
        Self { model_file, width, height }
    }
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for Load3D<ModelFile, Width, Height> {
    type Output = out::Load3DOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
            normal: crate::nodes::types::ImageOut::from_dynamic(node_id, 3u32),
            lineart: crate::nodes::types::ImageOut::from_dynamic(node_id, 4u32),
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
pub struct Load3DAnimation<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model_file: ModelFile,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 1
  - Step: 1
*/
    pub height: Height,
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> Load3DAnimation<ModelFile, Width, Height> {
    /// Create a new node.
    pub fn new(model_file: ModelFile, width: Width, height: Height) -> Self {
        Self { model_file, width, height }
    }
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for Load3DAnimation<ModelFile, Width, Height> {
    type Output = out::Load3DAnimationOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
            normal: crate::nodes::types::ImageOut::from_dynamic(node_id, 3u32),
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
pub struct Preview3D<ModelFile: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub model_file: ModelFile,
}
impl<ModelFile: crate::nodes::types::String> Preview3D<ModelFile> {
    /// Create a new node.
    pub fn new(model_file: ModelFile) -> Self {
        Self { model_file }
    }
}
impl<ModelFile: crate::nodes::types::String> crate::nodes::TypedNode
for Preview3D<ModelFile> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output
    }
    const NAME: &'static str = "Preview3D";
    const DISPLAY_NAME: &'static str = "Preview 3D";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<ModelFile: crate::nodes::types::String> crate::nodes::TypedOutputNode
for Preview3D<ModelFile> {}
///**Preview 3D - Animation**: No description.
#[derive(Clone)]
pub struct Preview3DAnimation<ModelFile: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub model_file: ModelFile,
}
impl<ModelFile: crate::nodes::types::String> Preview3DAnimation<ModelFile> {
    /// Create a new node.
    pub fn new(model_file: ModelFile) -> Self {
        Self { model_file }
    }
}
impl<ModelFile: crate::nodes::types::String> crate::nodes::TypedNode
for Preview3DAnimation<ModelFile> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output
    }
    const NAME: &'static str = "Preview3DAnimation";
    const DISPLAY_NAME: &'static str = "Preview 3D - Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<ModelFile: crate::nodes::types::String> crate::nodes::TypedOutputNode
for Preview3DAnimation<ModelFile> {}
///**SaveGLB**: No description.
#[derive(Clone)]
pub struct SaveGlb<
    Mesh: crate::nodes::types::Mesh,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub mesh: Mesh,
    /**No documentation.

**Metadata**:
  - Default: mesh/ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
}
impl<
    Mesh: crate::nodes::types::Mesh,
    FilenamePrefix: crate::nodes::types::String,
> SaveGlb<Mesh, FilenamePrefix> {
    /// Create a new node.
    pub fn new(mesh: Mesh, filename_prefix: FilenamePrefix) -> Self {
        Self { mesh, filename_prefix }
    }
}
impl<
    Mesh: crate::nodes::types::Mesh,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveGlb<Mesh, FilenamePrefix> {
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
    Mesh: crate::nodes::types::Mesh,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveGlb<Mesh, FilenamePrefix> {}
///**VoxelToMeshBasic**: No description.
#[derive(Clone)]
pub struct VoxelToMeshBasic<
    Voxel: crate::nodes::types::Voxel,
    Threshold: crate::nodes::types::Float,
> {
    ///No documentation.
    pub voxel: Voxel,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 1
  - Min: -1
  - Step: 0.01
*/
    pub threshold: Threshold,
}
impl<
    Voxel: crate::nodes::types::Voxel,
    Threshold: crate::nodes::types::Float,
> VoxelToMeshBasic<Voxel, Threshold> {
    /// Create a new node.
    pub fn new(voxel: Voxel, threshold: Threshold) -> Self {
        Self { voxel, threshold }
    }
}
impl<
    Voxel: crate::nodes::types::Voxel,
    Threshold: crate::nodes::types::Float,
> crate::nodes::TypedNode for VoxelToMeshBasic<Voxel, Threshold> {
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
