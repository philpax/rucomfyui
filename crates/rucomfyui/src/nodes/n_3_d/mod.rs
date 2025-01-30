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
    }
}
///**Load 3D**: No description.
#[derive(Clone)]
pub struct Load3D<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
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
    ///No documentation.
    pub material: Material,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: #000000
*/
    pub bg_color: BgColor,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 20
  - Min: 1
  - Step: 1
*/
    pub light_intensity: LightIntensity,
    ///No documentation.
    pub up_direction: UpDirection,
    /**No documentation.

**Metadata**:
  - Default: 75
  - Max: 150
  - Min: 10
  - Step: 1
*/
    pub fov: Fov,
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> Load3D<ModelFile, Width, Height, Material, BgColor, LightIntensity, UpDirection, Fov> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFile,
        width: Width,
        height: Height,
        material: Material,
        bg_color: BgColor,
        light_intensity: LightIntensity,
        up_direction: UpDirection,
        fov: Fov,
    ) -> Self {
        Self {
            model_file,
            width,
            height,
            material,
            bg_color,
            light_intensity,
            up_direction,
            fov,
        }
    }
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Load3D<
    ModelFile,
    Width,
    Height,
    Material,
    BgColor,
    LightIntensity,
    UpDirection,
    Fov,
> {
    type Output = out::Load3DOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("material".to_string(), self.material.clone().into());
        output.insert("bg_color".to_string(), self.bg_color.clone().into());
        output
            .insert("light_intensity".to_string(), self.light_intensity.clone().into());
        output.insert("up_direction".to_string(), self.up_direction.clone().into());
        output.insert("fov".to_string(), self.fov.clone().into());
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
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    AnimationSpeed: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
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
    ///No documentation.
    pub material: Material,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: #000000
*/
    pub bg_color: BgColor,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 20
  - Min: 1
  - Step: 1
*/
    pub light_intensity: LightIntensity,
    ///No documentation.
    pub up_direction: UpDirection,
    /**No documentation.

**Metadata**:
  - Default: 1
*/
    pub animation_speed: AnimationSpeed,
    /**No documentation.

**Metadata**:
  - Default: 75
  - Max: 150
  - Min: 10
  - Step: 1
*/
    pub fov: Fov,
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    AnimationSpeed: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> Load3DAnimation<
    ModelFile,
    Width,
    Height,
    Material,
    BgColor,
    LightIntensity,
    UpDirection,
    AnimationSpeed,
    Fov,
> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFile,
        width: Width,
        height: Height,
        material: Material,
        bg_color: BgColor,
        light_intensity: LightIntensity,
        up_direction: UpDirection,
        animation_speed: AnimationSpeed,
        fov: Fov,
    ) -> Self {
        Self {
            model_file,
            width,
            height,
            material,
            bg_color,
            light_intensity,
            up_direction,
            animation_speed,
            fov,
        }
    }
}
impl<
    ModelFile: crate::nodes::types::String,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    AnimationSpeed: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Load3DAnimation<
    ModelFile,
    Width,
    Height,
    Material,
    BgColor,
    LightIntensity,
    UpDirection,
    AnimationSpeed,
    Fov,
> {
    type Output = out::Load3DAnimationOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            mesh_path: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("material".to_string(), self.material.clone().into());
        output.insert("bg_color".to_string(), self.bg_color.clone().into());
        output
            .insert("light_intensity".to_string(), self.light_intensity.clone().into());
        output.insert("up_direction".to_string(), self.up_direction.clone().into());
        output
            .insert("animation_speed".to_string(), self.animation_speed.clone().into());
        output.insert("fov".to_string(), self.fov.clone().into());
        output
    }
    const NAME: &'static str = "Load3DAnimation";
    const DISPLAY_NAME: &'static str = "Load 3D - Animation";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
///**Preview 3D**: No description.
#[derive(Clone)]
pub struct Preview3D<
    ModelFile: crate::nodes::types::String,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub model_file: ModelFile,
    ///No documentation.
    pub material: Material,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: #000000
*/
    pub bg_color: BgColor,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 20
  - Min: 1
  - Step: 1
*/
    pub light_intensity: LightIntensity,
    ///No documentation.
    pub up_direction: UpDirection,
    /**No documentation.

**Metadata**:
  - Default: 75
  - Max: 150
  - Min: 10
  - Step: 1
*/
    pub fov: Fov,
}
impl<
    ModelFile: crate::nodes::types::String,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> Preview3D<ModelFile, Material, BgColor, LightIntensity, UpDirection, Fov> {
    /// Create a new node.
    pub fn new(
        model_file: ModelFile,
        material: Material,
        bg_color: BgColor,
        light_intensity: LightIntensity,
        up_direction: UpDirection,
        fov: Fov,
    ) -> Self {
        Self {
            model_file,
            material,
            bg_color,
            light_intensity,
            up_direction,
            fov,
        }
    }
}
impl<
    ModelFile: crate::nodes::types::String,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Preview3D<ModelFile, Material, BgColor, LightIntensity, UpDirection, Fov> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_file".to_string(), self.model_file.clone().into());
        output.insert("material".to_string(), self.material.clone().into());
        output.insert("bg_color".to_string(), self.bg_color.clone().into());
        output
            .insert("light_intensity".to_string(), self.light_intensity.clone().into());
        output.insert("up_direction".to_string(), self.up_direction.clone().into());
        output.insert("fov".to_string(), self.fov.clone().into());
        output
    }
    const NAME: &'static str = "Preview3D";
    const DISPLAY_NAME: &'static str = "Preview 3D";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "3d";
}
impl<
    ModelFile: crate::nodes::types::String,
    Material: crate::nodes::types::String,
    BgColor: crate::nodes::types::String,
    LightIntensity: crate::nodes::types::Int,
    UpDirection: crate::nodes::types::String,
    Fov: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for Preview3D<ModelFile, Material, BgColor, LightIntensity, UpDirection, Fov> {}
