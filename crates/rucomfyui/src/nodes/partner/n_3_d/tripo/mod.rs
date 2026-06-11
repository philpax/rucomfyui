//!`Tripo` definitions/categories.
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
    ///Output for [`TripoP1ImageToModelNode`](super::TripoP1ImageToModelNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TripoP1ImageToModelNodeOutput {
        ///No documentation.
        pub model_file: crate::nodes::types::StringOut,
        ///No documentation.
        pub model_task_id: crate::nodes::types::ModelTaskIdOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`TripoP1MultiviewToModelNode`](super::TripoP1MultiviewToModelNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TripoP1MultiviewToModelNodeOutput {
        ///No documentation.
        pub model_file: crate::nodes::types::StringOut,
        ///No documentation.
        pub model_task_id: crate::nodes::types::ModelTaskIdOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`TripoP1TextToModelNode`](super::TripoP1TextToModelNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TripoP1TextToModelNodeOutput {
        ///No documentation.
        pub model_file: crate::nodes::types::StringOut,
        ///No documentation.
        pub model_task_id: crate::nodes::types::ModelTaskIdOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
}
///**Tripo: Convert model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoConversionNode<
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSizeParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ForceSymmetryParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FlattenBottomParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FlattenBottomThresholdParam: crate::nodes::types::Float
        = crate::nodes::types::FloatOut,
    PivotToCenterBottomParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    ScaleFactorParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    WithAnimationParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PackUvParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    BakeParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PartNamesParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    ExportVertexColorsParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    AnimateInPlaceParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 2000000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: 4096
  - Max: 4096
  - Min: 128
*/
    pub texture_size: Option<TextureSizeParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub force_symmetry: Option<ForceSymmetryParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub flatten_bottom: Option<FlattenBottomParam>,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
*/
    pub flatten_bottom_threshold: Option<FlattenBottomThresholdParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub pivot_to_center_bottom: Option<PivotToCenterBottomParam>,
    ///No documentation.
    pub scale_factor: Option<ScaleFactorParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub with_animation: Option<WithAnimationParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub pack_uv: Option<PackUvParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub bake: Option<BakeParam>,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub part_names: Option<PartNamesParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub export_vertex_colors: Option<ExportVertexColorsParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub animate_in_place: Option<AnimateInPlaceParam>,
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
    ForceSymmetryParam: crate::nodes::types::Boolean,
    FlattenBottomParam: crate::nodes::types::Boolean,
    FlattenBottomThresholdParam: crate::nodes::types::Float,
    PivotToCenterBottomParam: crate::nodes::types::Boolean,
    ScaleFactorParam: crate::nodes::types::Float,
    WithAnimationParam: crate::nodes::types::Boolean,
    PackUvParam: crate::nodes::types::Boolean,
    BakeParam: crate::nodes::types::Boolean,
    PartNamesParam: crate::nodes::types::String,
    ExportVertexColorsParam: crate::nodes::types::Boolean,
    AnimateInPlaceParam: crate::nodes::types::Boolean,
> TripoConversionNode<
    QuadParam,
    FaceLimitParam,
    TextureSizeParam,
    ForceSymmetryParam,
    FlattenBottomParam,
    FlattenBottomThresholdParam,
    PivotToCenterBottomParam,
    ScaleFactorParam,
    WithAnimationParam,
    PackUvParam,
    BakeParam,
    PartNamesParam,
    ExportVertexColorsParam,
    AnimateInPlaceParam,
> {
    /// Create a new node.
    pub fn new(
        quad: Option<QuadParam>,
        face_limit: Option<FaceLimitParam>,
        texture_size: Option<TextureSizeParam>,
        force_symmetry: Option<ForceSymmetryParam>,
        flatten_bottom: Option<FlattenBottomParam>,
        flatten_bottom_threshold: Option<FlattenBottomThresholdParam>,
        pivot_to_center_bottom: Option<PivotToCenterBottomParam>,
        scale_factor: Option<ScaleFactorParam>,
        with_animation: Option<WithAnimationParam>,
        pack_uv: Option<PackUvParam>,
        bake: Option<BakeParam>,
        part_names: Option<PartNamesParam>,
        export_vertex_colors: Option<ExportVertexColorsParam>,
        animate_in_place: Option<AnimateInPlaceParam>,
    ) -> Self {
        Self {
            quad,
            face_limit,
            texture_size,
            force_symmetry,
            flatten_bottom,
            flatten_bottom_threshold,
            pivot_to_center_bottom,
            scale_factor,
            with_animation,
            pack_uv,
            bake,
            part_names,
            export_vertex_colors,
            animate_in_place,
        }
    }
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
    ForceSymmetryParam: crate::nodes::types::Boolean,
    FlattenBottomParam: crate::nodes::types::Boolean,
    FlattenBottomThresholdParam: crate::nodes::types::Float,
    PivotToCenterBottomParam: crate::nodes::types::Boolean,
    ScaleFactorParam: crate::nodes::types::Float,
    WithAnimationParam: crate::nodes::types::Boolean,
    PackUvParam: crate::nodes::types::Boolean,
    BakeParam: crate::nodes::types::Boolean,
    PartNamesParam: crate::nodes::types::String,
    ExportVertexColorsParam: crate::nodes::types::Boolean,
    AnimateInPlaceParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoConversionNode<
    QuadParam,
    FaceLimitParam,
    TextureSizeParam,
    ForceSymmetryParam,
    FlattenBottomParam,
    FlattenBottomThresholdParam,
    PivotToCenterBottomParam,
    ScaleFactorParam,
    WithAnimationParam,
    PackUvParam,
    BakeParam,
    PartNamesParam,
    ExportVertexColorsParam,
    AnimateInPlaceParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_size {
            output.insert("texture_size".to_string(), v.clone().into());
        }
        if let Some(v) = &self.force_symmetry {
            output.insert("force_symmetry".to_string(), v.clone().into());
        }
        if let Some(v) = &self.flatten_bottom {
            output.insert("flatten_bottom".to_string(), v.clone().into());
        }
        if let Some(v) = &self.flatten_bottom_threshold {
            output.insert("flatten_bottom_threshold".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pivot_to_center_bottom {
            output.insert("pivot_to_center_bottom".to_string(), v.clone().into());
        }
        if let Some(v) = &self.scale_factor {
            output.insert("scale_factor".to_string(), v.clone().into());
        }
        if let Some(v) = &self.with_animation {
            output.insert("with_animation".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pack_uv {
            output.insert("pack_uv".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bake {
            output.insert("bake".to_string(), v.clone().into());
        }
        if let Some(v) = &self.part_names {
            output.insert("part_names".to_string(), v.clone().into());
        }
        if let Some(v) = &self.export_vertex_colors {
            output.insert("export_vertex_colors".to_string(), v.clone().into());
        }
        if let Some(v) = &self.animate_in_place {
            output.insert("animate_in_place".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoConversionNode";
    const DISPLAY_NAME: &'static str = "Tripo: Convert model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
    ForceSymmetryParam: crate::nodes::types::Boolean,
    FlattenBottomParam: crate::nodes::types::Boolean,
    FlattenBottomThresholdParam: crate::nodes::types::Float,
    PivotToCenterBottomParam: crate::nodes::types::Boolean,
    ScaleFactorParam: crate::nodes::types::Float,
    WithAnimationParam: crate::nodes::types::Boolean,
    PackUvParam: crate::nodes::types::Boolean,
    BakeParam: crate::nodes::types::Boolean,
    PartNamesParam: crate::nodes::types::String,
    ExportVertexColorsParam: crate::nodes::types::Boolean,
    AnimateInPlaceParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoConversionNode<
    QuadParam,
    FaceLimitParam,
    TextureSizeParam,
    ForceSymmetryParam,
    FlattenBottomParam,
    FlattenBottomThresholdParam,
    PivotToCenterBottomParam,
    ScaleFactorParam,
    WithAnimationParam,
    PackUvParam,
    BakeParam,
    PartNamesParam,
    ExportVertexColorsParam,
    AnimateInPlaceParam,
> {}
///**Tripo: Image to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoImageToModelNode<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            image,
            texture,
            pbr,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoImageToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Image to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo: Multiview to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoMultiviewToModelNode<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageBackParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageRightParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub image_left: Option<ImageLeftParam>,
    ///No documentation.
    pub image_back: Option<ImageBackParam>,
    ///No documentation.
    pub image_right: Option<ImageRightParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**This parameter is deprecated and does nothing.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        image_left: Option<ImageLeftParam>,
        image_back: Option<ImageBackParam>,
        image_right: Option<ImageRightParam>,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            image,
            image_left,
            image_back,
            image_right,
            texture,
            pbr,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.image_left {
            output.insert("image_left".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_back {
            output.insert("image_back".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_right {
            output.insert("image_right".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoMultiviewToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Multiview to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo P1: Image to Model**: Tripo P1 image-to-3D. Optimized for low-poly, game-ready meshes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoP1ImageToModelNode<
    ImageParam: crate::nodes::types::Image,
    EnableImageAutofixParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    AutoSizeParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ExportUvParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    CompressGeometryParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Pre-process the input image for better generation quality.

**Metadata**:
  - Default: false
*/
    pub enable_image_autofix: Option<EnableImageAutofixParam>,
    /**Target face count, 48-20000. -1 lets Tripo pick adaptively.

**Metadata**:
  - Default: -1
  - Max: 20000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    /**Scale the output to approximate real-world meters.

**Metadata**:
  - Default: false
*/
    pub auto_size: Option<AutoSizeParam>,
    /**UV unwrap during generation. Turn off for faster geometry-only runs.

**Metadata**:
  - Default: true
*/
    pub export_uv: Option<ExportUvParam>,
    /**Apply geometry-based compression. Decompress before editing.

**Metadata**:
  - Default: false
*/
    pub compress_geometry: Option<CompressGeometryParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    EnableImageAutofixParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> TripoP1ImageToModelNode<
    ImageParam,
    EnableImageAutofixParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        enable_image_autofix: Option<EnableImageAutofixParam>,
        face_limit: Option<FaceLimitParam>,
        model_seed: Option<ModelSeedParam>,
        auto_size: Option<AutoSizeParam>,
        export_uv: Option<ExportUvParam>,
        compress_geometry: Option<CompressGeometryParam>,
    ) -> Self {
        Self {
            image,
            enable_image_autofix,
            face_limit,
            model_seed,
            auto_size,
            export_uv,
            compress_geometry,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    EnableImageAutofixParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoP1ImageToModelNode<
    ImageParam,
    EnableImageAutofixParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    type Output = out::TripoP1ImageToModelNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model_file: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            model_task_id: crate::nodes::types::ModelTaskIdOut::from_dynamic(
                node_id,
                1u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.enable_image_autofix {
            output.insert("enable_image_autofix".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.auto_size {
            output.insert("auto_size".to_string(), v.clone().into());
        }
        if let Some(v) = &self.export_uv {
            output.insert("export_uv".to_string(), v.clone().into());
        }
        if let Some(v) = &self.compress_geometry {
            output.insert("compress_geometry".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoP1ImageToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo P1: Image to Model";
    const DESCRIPTION: &'static str = "Tripo P1 image-to-3D. Optimized for low-poly, game-ready meshes.";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
///**Tripo P1: Multiview to Model**: Tripo P1 multiview-to-3D from 2-4 reference images in \[front, left, back, right\] order. Front is required; any combination of the other three may be omitted.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoP1MultiviewToModelNode<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageBackParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageRightParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    AutoSizeParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ExportUvParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    CompressGeometryParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///Front view (0°). Required.
    pub image: ImageParam,
    ///Left view (90°), i.e. the subject's left side.
    pub image_left: Option<ImageLeftParam>,
    ///Back view (180°).
    pub image_back: Option<ImageBackParam>,
    ///Right view (270°), i.e. the subject's right side.
    pub image_right: Option<ImageRightParam>,
    /**Target face count, 48-20000. -1 lets Tripo pick adaptively.

**Metadata**:
  - Default: -1
  - Max: 20000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    /**Scale the output to approximate real-world meters.

**Metadata**:
  - Default: false
*/
    pub auto_size: Option<AutoSizeParam>,
    /**UV unwrap during generation. Turn off for faster geometry-only runs.

**Metadata**:
  - Default: true
*/
    pub export_uv: Option<ExportUvParam>,
    /**Apply geometry-based compression. Decompress before editing.

**Metadata**:
  - Default: false
*/
    pub compress_geometry: Option<CompressGeometryParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> TripoP1MultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        image_left: Option<ImageLeftParam>,
        image_back: Option<ImageBackParam>,
        image_right: Option<ImageRightParam>,
        face_limit: Option<FaceLimitParam>,
        model_seed: Option<ModelSeedParam>,
        auto_size: Option<AutoSizeParam>,
        export_uv: Option<ExportUvParam>,
        compress_geometry: Option<CompressGeometryParam>,
    ) -> Self {
        Self {
            image,
            image_left,
            image_back,
            image_right,
            face_limit,
            model_seed,
            auto_size,
            export_uv,
            compress_geometry,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoP1MultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    type Output = out::TripoP1MultiviewToModelNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model_file: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            model_task_id: crate::nodes::types::ModelTaskIdOut::from_dynamic(
                node_id,
                1u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.image_left {
            output.insert("image_left".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_back {
            output.insert("image_back".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_right {
            output.insert("image_right".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.auto_size {
            output.insert("auto_size".to_string(), v.clone().into());
        }
        if let Some(v) = &self.export_uv {
            output.insert("export_uv".to_string(), v.clone().into());
        }
        if let Some(v) = &self.compress_geometry {
            output.insert("compress_geometry".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoP1MultiviewToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo P1: Multiview to Model";
    const DESCRIPTION: &'static str = "Tripo P1 multiview-to-3D from 2-4 reference images in [front, left, back, right] order. Front is required; any combination of the other three may be omitted.";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
///**Tripo P1: Text to Model**: Tripo P1 text-to-3D. Optimized for low-poly, game-ready meshes with stable topology.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoP1TextToModelNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    ImageSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    AutoSizeParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ExportUvParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    CompressGeometryParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Up to 1024 characters.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Up to 255 characters.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///No documentation.
    pub image_seed: Option<ImageSeedParam>,
    /**Target face count, 48-20000. -1 lets Tripo pick adaptively.

**Metadata**:
  - Default: -1
  - Max: 20000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    /**Scale the output to approximate real-world meters.

**Metadata**:
  - Default: false
*/
    pub auto_size: Option<AutoSizeParam>,
    /**UV unwrap during generation. Turn off for faster geometry-only runs.

**Metadata**:
  - Default: true
*/
    pub export_uv: Option<ExportUvParam>,
    /**Apply geometry-based compression. Decompress before editing.

**Metadata**:
  - Default: false
*/
    pub compress_geometry: Option<CompressGeometryParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> TripoP1TextToModelNode<
    PromptParam,
    NegativePromptParam,
    ImageSeedParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        image_seed: Option<ImageSeedParam>,
        face_limit: Option<FaceLimitParam>,
        model_seed: Option<ModelSeedParam>,
        auto_size: Option<AutoSizeParam>,
        export_uv: Option<ExportUvParam>,
        compress_geometry: Option<CompressGeometryParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            image_seed,
            face_limit,
            model_seed,
            auto_size,
            export_uv,
            compress_geometry,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    ImageSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    AutoSizeParam: crate::nodes::types::Boolean,
    ExportUvParam: crate::nodes::types::Boolean,
    CompressGeometryParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoP1TextToModelNode<
    PromptParam,
    NegativePromptParam,
    ImageSeedParam,
    FaceLimitParam,
    ModelSeedParam,
    AutoSizeParam,
    ExportUvParam,
    CompressGeometryParam,
> {
    type Output = out::TripoP1TextToModelNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model_file: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            model_task_id: crate::nodes::types::ModelTaskIdOut::from_dynamic(
                node_id,
                1u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_seed {
            output.insert("image_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.auto_size {
            output.insert("auto_size".to_string(), v.clone().into());
        }
        if let Some(v) = &self.export_uv {
            output.insert("export_uv".to_string(), v.clone().into());
        }
        if let Some(v) = &self.compress_geometry {
            output.insert("compress_geometry".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoP1TextToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo P1: Text to Model";
    const DESCRIPTION: &'static str = "Tripo P1 text-to-3D. Optimized for low-poly, game-ready meshes with stable topology.";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
///**Tripo: Refine Draft model**: Refine a draft model created by v1.4 Tripo models only.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRefineNode<ModelTaskIdParam: crate::nodes::types::ModelTaskId> {
    ///Must be a v1.4 Tripo model
    pub model_task_id: ModelTaskIdParam,
}
impl<
    ModelTaskIdParam: crate::nodes::types::ModelTaskId,
> TripoRefineNode<ModelTaskIdParam> {
    /// Create a new node.
    pub fn new(model_task_id: ModelTaskIdParam) -> Self {
        Self { model_task_id }
    }
}
impl<ModelTaskIdParam: crate::nodes::types::ModelTaskId> crate::nodes::TypedNode
for TripoRefineNode<ModelTaskIdParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_task_id".to_string(), self.model_task_id.clone().into());
        output
    }
    const NAME: &'static str = "TripoRefineNode";
    const DISPLAY_NAME: &'static str = "Tripo: Refine Draft model";
    const DESCRIPTION: &'static str = "Refine a draft model created by v1.4 Tripo models only.";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<ModelTaskIdParam: crate::nodes::types::ModelTaskId> crate::nodes::TypedOutputNode
for TripoRefineNode<ModelTaskIdParam> {}
///**Tripo: Retarget rigged model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRetargetNode<OriginalModelTaskIdParam: crate::nodes::types::RigTaskId> {
    ///No documentation.
    pub original_model_task_id: OriginalModelTaskIdParam,
}
impl<
    OriginalModelTaskIdParam: crate::nodes::types::RigTaskId,
> TripoRetargetNode<OriginalModelTaskIdParam> {
    /// Create a new node.
    pub fn new(original_model_task_id: OriginalModelTaskIdParam) -> Self {
        Self { original_model_task_id }
    }
}
impl<OriginalModelTaskIdParam: crate::nodes::types::RigTaskId> crate::nodes::TypedNode
for TripoRetargetNode<OriginalModelTaskIdParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "original_model_task_id".to_string(),
                self.original_model_task_id.clone().into(),
            );
        output
    }
    const NAME: &'static str = "TripoRetargetNode";
    const DISPLAY_NAME: &'static str = "Tripo: Retarget rigged model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    OriginalModelTaskIdParam: crate::nodes::types::RigTaskId,
> crate::nodes::TypedOutputNode for TripoRetargetNode<OriginalModelTaskIdParam> {}
///**Tripo: Rig model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRigNode<OriginalModelTaskIdParam: crate::nodes::types::ModelTaskId> {
    ///No documentation.
    pub original_model_task_id: OriginalModelTaskIdParam,
}
impl<
    OriginalModelTaskIdParam: crate::nodes::types::ModelTaskId,
> TripoRigNode<OriginalModelTaskIdParam> {
    /// Create a new node.
    pub fn new(original_model_task_id: OriginalModelTaskIdParam) -> Self {
        Self { original_model_task_id }
    }
}
impl<OriginalModelTaskIdParam: crate::nodes::types::ModelTaskId> crate::nodes::TypedNode
for TripoRigNode<OriginalModelTaskIdParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "original_model_task_id".to_string(),
                self.original_model_task_id.clone().into(),
            );
        output
    }
    const NAME: &'static str = "TripoRigNode";
    const DISPLAY_NAME: &'static str = "Tripo: Rig model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    OriginalModelTaskIdParam: crate::nodes::types::ModelTaskId,
> crate::nodes::TypedOutputNode for TripoRigNode<OriginalModelTaskIdParam> {}
///**Tripo: Text to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoTextToModelNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ImageSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub image_seed: Option<ImageSeedParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 2000000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        image_seed: Option<ImageSeedParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            texture,
            pbr,
            image_seed,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_seed {
            output.insert("image_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoTextToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Text to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo: Texture model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoTextureNode<
    ModelTaskIdParam: crate::nodes::types::ModelTaskId,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub model_task_id: ModelTaskIdParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
}
impl<
    ModelTaskIdParam: crate::nodes::types::ModelTaskId,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> TripoTextureNode<ModelTaskIdParam, TextureParam, PbrParam, TextureSeedParam> {
    /// Create a new node.
    pub fn new(
        model_task_id: ModelTaskIdParam,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        texture_seed: Option<TextureSeedParam>,
    ) -> Self {
        Self {
            model_task_id,
            texture,
            pbr,
            texture_seed,
        }
    }
}
impl<
    ModelTaskIdParam: crate::nodes::types::ModelTaskId,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TripoTextureNode<ModelTaskIdParam, TextureParam, PbrParam, TextureSeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_task_id".to_string(), self.model_task_id.clone().into());
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoTextureNode";
    const DISPLAY_NAME: &'static str = "Tripo: Texture model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Tripo";
}
impl<
    ModelTaskIdParam: crate::nodes::types::ModelTaskId,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for TripoTextureNode<ModelTaskIdParam, TextureParam, PbrParam, TextureSeedParam> {}
