//!`3d` definitions/categories.
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
#[rustfmt::skip]
pub mod conditioning;
#[rustfmt::skip]
pub mod latent;
#[rustfmt::skip]
pub mod splat;
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
        pub camera_info: crate::nodes::types::Load3DCameraOut,
        ///No documentation.
        pub recording_video: crate::nodes::types::VideoOut,
        ///No documentation.
        pub model_3_d: crate::nodes::types::File3DOut,
        ///No documentation.
        pub model_3_d_info: crate::nodes::types::Load3DModelInfoOut,
    }
}
///**Create Camera Info**: Build a camera_infoMode 'orbit' aims with yaw/pitch/distance around the target; 'look_at' places the camera at world position. Coordinates are the viewer's world space (right-handed,Y-up).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CreateCameraInfo<
    TargetXParam: crate::nodes::types::Float,
    TargetYParam: crate::nodes::types::Float,
    TargetZParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    FovParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> {
    /**Look-at point (orbit pivot / aim). In orbit mode, move it to pan/translate the whole camera. Ignored in quaternion mode. Defaults to the origin.

**Metadata**:
  - Default: 0
  - Max: 1000
  - Min: -1000
  - Step: 0.01
*/
    pub target_x: TargetXParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1000
  - Min: -1000
  - Step: 0.01
*/
    pub target_y: TargetYParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1000
  - Min: -1000
  - Step: 0.01
*/
    pub target_z: TargetZParam,
    /**Camera roll about the view axis, degrees.

**Metadata**:
  - Default: 0
  - Max: 180
  - Min: -180
  - Step: 1
*/
    pub roll: RollParam,
    /**Vertical field of view in degrees.

**Metadata**:
  - Default: 35
  - Max: 120
  - Min: 1
  - Step: 1
*/
    pub fov: FovParam,
    /**Digital zoom (focal-length multiplier). >1 zooms in without moving the camera.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0.01
  - Step: 0.01
*/
    pub zoom: ZoomParam,
}
impl<
    TargetXParam: crate::nodes::types::Float,
    TargetYParam: crate::nodes::types::Float,
    TargetZParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    FovParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> CreateCameraInfo<
    TargetXParam,
    TargetYParam,
    TargetZParam,
    RollParam,
    FovParam,
    ZoomParam,
> {
    /// Create a new node.
    pub fn new(
        target_x: TargetXParam,
        target_y: TargetYParam,
        target_z: TargetZParam,
        roll: RollParam,
        fov: FovParam,
        zoom: ZoomParam,
    ) -> Self {
        Self {
            target_x,
            target_y,
            target_z,
            roll,
            fov,
            zoom,
        }
    }
}
impl<
    TargetXParam: crate::nodes::types::Float,
    TargetYParam: crate::nodes::types::Float,
    TargetZParam: crate::nodes::types::Float,
    RollParam: crate::nodes::types::Float,
    FovParam: crate::nodes::types::Float,
    ZoomParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for CreateCameraInfo<
    TargetXParam,
    TargetYParam,
    TargetZParam,
    RollParam,
    FovParam,
    ZoomParam,
> {
    type Output = crate::nodes::types::Load3DCameraOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("target_x".to_string(), self.target_x.clone().into());
        output.insert("target_y".to_string(), self.target_y.clone().into());
        output.insert("target_z".to_string(), self.target_z.clone().into());
        output.insert("roll".to_string(), self.roll.clone().into());
        output.insert("fov".to_string(), self.fov.clone().into());
        output.insert("zoom".to_string(), self.zoom.clone().into());
        output
    }
    const NAME: &'static str = "CreateCameraInfo";
    const DISPLAY_NAME: &'static str = "Create Camera Info";
    const DESCRIPTION: &'static str = "Build a camera_infoMode 'orbit' aims with yaw/pitch/distance around the target; 'look_at' places the camera at world position. Coordinates are the viewer's world space (right-handed,Y-up).";
    const CATEGORY: &'static str = "3d";
}
///**Load 3D & Animation**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Load3D<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
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
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> Load3D<WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(width: WidthParam, height: HeightParam) -> Self {
        Self { width, height }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Load3D<WidthParam, HeightParam> {
    type Output = out::Load3DOutput;
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
            recording_video: crate::nodes::types::VideoOut::from_dynamic(node_id, 5u32),
            model_3_d: crate::nodes::types::File3DOut::from_dynamic(node_id, 6u32),
            model_3_d_info: crate::nodes::types::Load3DModelInfoOut::from_dynamic(
                node_id,
                7u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "Load3D";
    const DISPLAY_NAME: &'static str = "Load 3D & Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
///**Preview 3D & Animation**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Preview3D<
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
    BgImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
    ///No documentation.
    pub bg_image: Option<BgImageParam>,
}
impl<
    CameraInfoParam: crate::nodes::types::Load3DCamera,
    BgImageParam: crate::nodes::types::Image,
> Preview3D<CameraInfoParam, BgImageParam> {
    /// Create a new node.
    pub fn new(
        camera_info: Option<CameraInfoParam>,
        bg_image: Option<BgImageParam>,
    ) -> Self {
        Self { camera_info, bg_image }
    }
}
impl<
    CameraInfoParam: crate::nodes::types::Load3DCamera,
    BgImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for Preview3D<CameraInfoParam, BgImageParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bg_image {
            output.insert("bg_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Preview3D";
    const DISPLAY_NAME: &'static str = "Preview 3D & Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    CameraInfoParam: crate::nodes::types::Load3DCamera,
    BgImageParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode for Preview3D<CameraInfoParam, BgImageParam> {}
///**Preview 3D (Advanced)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Preview3DAdvanced<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo
        = crate::nodes::types::Load3DModelInfoOut,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
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
    ///No documentation.
    pub model_3_d_info: Option<Model3DInfoParam>,
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> Preview3DAdvanced<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        model_3_d_info: Option<Model3DInfoParam>,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self {
            width,
            height,
            model_3_d_info,
            camera_info,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode
for Preview3DAdvanced<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        if let Some(v) = &self.model_3_d_info {
            output.insert("model_3d_info".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Preview3DAdvanced";
    const DISPLAY_NAME: &'static str = "Preview 3D (Advanced)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedOutputNode
for Preview3DAdvanced<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {}
///**Preview Splat**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewGaussianSplat<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo
        = crate::nodes::types::Load3DModelInfoOut,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
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
    ///No documentation.
    pub model_3_d_info: Option<Model3DInfoParam>,
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> PreviewGaussianSplat<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        model_3_d_info: Option<Model3DInfoParam>,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self {
            width,
            height,
            model_3_d_info,
            camera_info,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode
for PreviewGaussianSplat<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        if let Some(v) = &self.model_3_d_info {
            output.insert("model_3d_info".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PreviewGaussianSplat";
    const DISPLAY_NAME: &'static str = "Preview Splat";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedOutputNode
for PreviewGaussianSplat<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {}
///**Preview Point Cloud**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewPointCloud<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo
        = crate::nodes::types::Load3DModelInfoOut,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
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
    ///No documentation.
    pub model_3_d_info: Option<Model3DInfoParam>,
    ///No documentation.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> PreviewPointCloud<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        model_3_d_info: Option<Model3DInfoParam>,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self {
            width,
            height,
            model_3_d_info,
            camera_info,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode
for PreviewPointCloud<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        if let Some(v) = &self.model_3_d_info {
            output.insert("model_3d_info".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PreviewPointCloud";
    const DISPLAY_NAME: &'static str = "Preview Point Cloud";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    Model3DInfoParam: crate::nodes::types::Load3DModelInfo,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedOutputNode
for PreviewPointCloud<WidthParam, HeightParam, Model3DInfoParam, CameraInfoParam> {}
///**Save 3D Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveGLB<FilenamePrefixParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: 3d/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<FilenamePrefixParam: crate::nodes::types::String> SaveGLB<FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(filename_prefix: FilenamePrefixParam) -> Self {
        Self { filename_prefix }
    }
}
impl<FilenamePrefixParam: crate::nodes::types::String> crate::nodes::TypedNode
for SaveGLB<FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveGLB";
    const DISPLAY_NAME: &'static str = "Save 3D Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<FilenamePrefixParam: crate::nodes::types::String> crate::nodes::TypedOutputNode
for SaveGLB<FilenamePrefixParam> {}
///**Voxel to Mesh**: Converts a voxel grid to a mesh.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VoxelToMesh<
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
> VoxelToMesh<VoxelParam, ThresholdParam> {
    /// Create a new node.
    pub fn new(voxel: VoxelParam, threshold: ThresholdParam) -> Self {
        Self { voxel, threshold }
    }
}
impl<
    VoxelParam: crate::nodes::types::Voxel,
    ThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for VoxelToMesh<VoxelParam, ThresholdParam> {
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
    const NAME: &'static str = "VoxelToMesh";
    const DISPLAY_NAME: &'static str = "Voxel to Mesh";
    const DESCRIPTION: &'static str = "Converts a voxel grid to a mesh.";
    const CATEGORY: &'static str = "3d";
}
///**Voxel to Mesh (Basic) (DEPRECATED)**: Converts a voxel grid to a mesh.
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
    const DISPLAY_NAME: &'static str = "Voxel to Mesh (Basic) (DEPRECATED)";
    const DESCRIPTION: &'static str = "Converts a voxel grid to a mesh.";
    const CATEGORY: &'static str = "3d";
}
