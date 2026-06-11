//!`splat` definitions/categories.
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
    ///Output for [`GetSplatCount`](super::GetSplatCount).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GetSplatCountOutput {
        ///No documentation.
        pub splat: crate::nodes::types::SplatOut,
        ///No documentation.
        pub count: crate::nodes::types::IntOut,
    }
    ///Output for [`RenderSplat`](super::RenderSplat).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct RenderSplatOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Get Splat**: Parse a splat File3D into a gaussian splat. Inverse of Create 3D File (from Splat). Supported format:  PLY, SPLAT, KSPLAT, SPZ. PLY carries full spherical harmonics, the other formats are base color only. Format is auto-detected from the file contents.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct File3DToSplat {}
impl File3DToSplat {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for File3DToSplat {
    type Output = crate::nodes::types::SplatOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "File3DToSplat";
    const DISPLAY_NAME: &'static str = "Get Splat";
    const DESCRIPTION: &'static str = "Parse a splat File3D into a gaussian splat. Inverse of Create 3D File (from Splat). Supported format:  PLY, SPLAT, KSPLAT, SPZ. PLY carries full spherical harmonics, the other formats are base color only. Format is auto-detected from the file contents.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Get Splat Count**: Returns the number of splats summed across the batch.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GetSplatCount<SplatParam: crate::nodes::types::Splat> {
    ///No documentation.
    pub splat: SplatParam,
}
impl<SplatParam: crate::nodes::types::Splat> GetSplatCount<SplatParam> {
    /// Create a new node.
    pub fn new(splat: SplatParam) -> Self {
        Self { splat }
    }
}
impl<SplatParam: crate::nodes::types::Splat> crate::nodes::TypedNode
for GetSplatCount<SplatParam> {
    type Output = out::GetSplatCountOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            splat: crate::nodes::types::SplatOut::from_dynamic(node_id, 0u32),
            count: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("splat".to_string(), self.splat.clone().into());
        output
    }
    const NAME: &'static str = "GetSplatCount";
    const DISPLAY_NAME: &'static str = "Get Splat Count";
    const DESCRIPTION: &'static str = "Returns the number of splats summed across the batch.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Merge Splats**: Concatenate any number of gaussian splats into one. Unioning several decodes of the same latent at different seeds densifies the surface, this can improve surface quality when meshing.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MergeSplat {}
impl MergeSplat {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for MergeSplat {
    type Output = crate::nodes::types::SplatOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "MergeSplat";
    const DISPLAY_NAME: &'static str = "Merge Splats";
    const DESCRIPTION: &'static str = "Concatenate any number of gaussian splats into one. Unioning several decodes of the same latent at different seeds densifies the surface, this can improve surface quality when meshing.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Render Splat**: Render a gaussian splat as an image with an anisotropic EWA rasterizer (oriented elliptical splats, antialiased, depth-sorted front-to-back). The camera comes from a camera_info input (Load / Preview 3D, or a Create Camera Info node); leave it empty to auto-frame the splat. Set frames greater than 1 for a turntable batch of images to feed a Video node.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RenderSplat<
    SplatParam: crate::nodes::types::Splat,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    FramesParam: crate::nodes::types::Int,
    SplatScaleParam: crate::nodes::types::Float,
    SharpenParam: crate::nodes::types::Float,
    HeadlightShadingParam: crate::nodes::types::Float,
    OpacityThresholdParam: crate::nodes::types::Float,
    BackgroundParam: crate::nodes::types::Color,
    BgImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    CameraInfoParam: crate::nodes::types::Load3DCamera
        = crate::nodes::types::Load3DCameraOut,
> {
    ///No documentation.
    pub splat: SplatParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 2048
  - Min: 64
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 2048
  - Min: 64
  - Step: 8
*/
    pub height: HeightParam,
    /**-1, 0, 1 = single still image; >1 = turntable, the camera orbits over a full 360 turn (works with any camera_info). Negative value orbits the other way.

**Metadata**:
  - Default: 1
  - Max: 240
  - Min: -240
*/
    pub frames: FramesParam,
    /**Multiplier on each splat's projected footprint (lower = crisper points, higher = softer/fuller surface).

**Metadata**:
  - Default: 1
  - Max: 5
  - Min: 0.1
  - Step: 0.05
*/
    pub splat_scale: SplatScaleParam,
    /**Sharpen overlapping splats: 1.0 = physically-correct blend; higher biases each pixel toward its dominant (nearest) splat for crisper texture, without shrinking splats or opening gaps. Non-physical above 1.

**Metadata**:
  - Default: 2
  - Max: 8
  - Min: 1
  - Step: 0.5
*/
    pub sharpen: SharpenParam,
    /**Diffuse shading from a light at the camera (headlight), using the splat surfel normals: darkens surfaces that turn away from view to reveal form/curvature. 0 = flat albedo, 1 = strongest shading.

**Metadata**:
  - Default: 0
  - Max: 3
  - Min: 0
  - Step: 0.05
*/
    pub headlight_shading: HeadlightShadingParam,
    /**Cull gaussians with opacity below this (removes faint floaters).

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub opacity_threshold: OpacityThresholdParam,
    /**No documentation.

**Metadata**:
  - Default: #000000
*/
    pub background: BackgroundParam,
    ///Optional background plate composited behind the splat (overrides the solid background colour). Resized to the render size; a batch is used per frame, a single image for all. color/clay only.
    pub bg_image: Option<BgImageParam>,
    ///Camera to render from - a Load3D / Preview3D camera or a Create Camera Info node. If empty, the splat is auto-framed from a default 3/4 view.
    pub camera_info: Option<CameraInfoParam>,
}
impl<
    SplatParam: crate::nodes::types::Splat,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    FramesParam: crate::nodes::types::Int,
    SplatScaleParam: crate::nodes::types::Float,
    SharpenParam: crate::nodes::types::Float,
    HeadlightShadingParam: crate::nodes::types::Float,
    OpacityThresholdParam: crate::nodes::types::Float,
    BackgroundParam: crate::nodes::types::Color,
    BgImageParam: crate::nodes::types::Image,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> RenderSplat<
    SplatParam,
    WidthParam,
    HeightParam,
    FramesParam,
    SplatScaleParam,
    SharpenParam,
    HeadlightShadingParam,
    OpacityThresholdParam,
    BackgroundParam,
    BgImageParam,
    CameraInfoParam,
> {
    /// Create a new node.
    pub fn new(
        splat: SplatParam,
        width: WidthParam,
        height: HeightParam,
        frames: FramesParam,
        splat_scale: SplatScaleParam,
        sharpen: SharpenParam,
        headlight_shading: HeadlightShadingParam,
        opacity_threshold: OpacityThresholdParam,
        background: BackgroundParam,
        bg_image: Option<BgImageParam>,
        camera_info: Option<CameraInfoParam>,
    ) -> Self {
        Self {
            splat,
            width,
            height,
            frames,
            splat_scale,
            sharpen,
            headlight_shading,
            opacity_threshold,
            background,
            bg_image,
            camera_info,
        }
    }
}
impl<
    SplatParam: crate::nodes::types::Splat,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    FramesParam: crate::nodes::types::Int,
    SplatScaleParam: crate::nodes::types::Float,
    SharpenParam: crate::nodes::types::Float,
    HeadlightShadingParam: crate::nodes::types::Float,
    OpacityThresholdParam: crate::nodes::types::Float,
    BackgroundParam: crate::nodes::types::Color,
    BgImageParam: crate::nodes::types::Image,
    CameraInfoParam: crate::nodes::types::Load3DCamera,
> crate::nodes::TypedNode
for RenderSplat<
    SplatParam,
    WidthParam,
    HeightParam,
    FramesParam,
    SplatScaleParam,
    SharpenParam,
    HeadlightShadingParam,
    OpacityThresholdParam,
    BackgroundParam,
    BgImageParam,
    CameraInfoParam,
> {
    type Output = out::RenderSplatOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("splat".to_string(), self.splat.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("frames".to_string(), self.frames.clone().into());
        output.insert("splat_scale".to_string(), self.splat_scale.clone().into());
        output.insert("sharpen".to_string(), self.sharpen.clone().into());
        output
            .insert(
                "headlight_shading".to_string(),
                self.headlight_shading.clone().into(),
            );
        output
            .insert(
                "opacity_threshold".to_string(),
                self.opacity_threshold.clone().into(),
            );
        output.insert("background".to_string(), self.background.clone().into());
        if let Some(v) = &self.bg_image {
            output.insert("bg_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.camera_info {
            output.insert("camera_info".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RenderSplat";
    const DISPLAY_NAME: &'static str = "Render Splat";
    const DESCRIPTION: &'static str = "Render a gaussian splat as an image with an anisotropic EWA rasterizer (oriented elliptical splats, antialiased, depth-sorted front-to-back). The camera comes from a camera_info input (Load / Preview 3D, or a Create Camera Info node); leave it empty to auto-frame the splat. Set frames greater than 1 for a turntable batch of images to feed a Video node.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Create 3D File (from Splat)**: Serialize a gaussian splat to a File3D object for Save / Preview 3D nodes. Supports one item per batch only.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SplatToFile3D<SplatParam: crate::nodes::types::Splat> {
    ///No documentation.
    pub splat: SplatParam,
}
impl<SplatParam: crate::nodes::types::Splat> SplatToFile3D<SplatParam> {
    /// Create a new node.
    pub fn new(splat: SplatParam) -> Self {
        Self { splat }
    }
}
impl<SplatParam: crate::nodes::types::Splat> crate::nodes::TypedNode
for SplatToFile3D<SplatParam> {
    type Output = crate::nodes::types::File3DSplatAnyOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("splat".to_string(), self.splat.clone().into());
        output
    }
    const NAME: &'static str = "SplatToFile3D";
    const DISPLAY_NAME: &'static str = "Create 3D File (from Splat)";
    const DESCRIPTION: &'static str = "Serialize a gaussian splat to a File3D object for Save / Preview 3D nodes. Supports one item per batch only.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Extract Mesh from Splat**: Extract a coloured mesh from a gaussian splat.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SplatToMesh<
    SplatParam: crate::nodes::types::Splat,
    ResolutionParam: crate::nodes::types::Int,
    KernelParam: crate::nodes::types::Int,
    SmoothParam: crate::nodes::types::Int,
    LevelParam: crate::nodes::types::Float,
    MinComponentParam: crate::nodes::types::Int,
    MinOpacityParam: crate::nodes::types::Float,
    ColorSharpenParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub splat: SplatParam,
    /**Density-grid resolution along the longest axis. Higher = finer surface, more VRAM/time (grows with resolution^3).

**Metadata**:
  - Default: 384
  - Max: 768
  - Min: 64
  - Step: 16
*/
    pub resolution: ResolutionParam,
    /**Max splat half-width in voxels. Each gaussian is rasterized over a window sized to its own 3-sigma, capped here - small surfels stay cheap, large ones aren't truncated. Raise if sparse splats leave gaps.

**Metadata**:
  - Default: 5
  - Max: 8
  - Min: 1
*/
    pub kernel: KernelParam,
    /**Taubin mesh-smoothing iterations. Smooths the surface without shrinking it (volume-preserving), unlike blurring the density. 0 = raw surface.

**Metadata**:
  - Default: 0
  - Max: 60
  - Min: 0
*/
    pub smooth: SmoothParam,
    /**Iso-surface level. Auto-picked by Otsu; this biases it (1.0 = auto, lower = fatter/more-connected surface, higher = thinner/tighter).

**Metadata**:
  - Default: 0.4
  - Max: 2
  - Min: 0
  - Step: 0.01
*/
    pub level: LevelParam,
    /**Drop connected components smaller than this many vertices (0 = keep all). Removes detached floater blobs and the inner shell of the double wall.

**Metadata**:
  - Default: 500
  - Max: 100000
  - Min: 0
  - Step: 50
*/
    pub min_component: MinComponentParam,
    /**Ignore gaussians fainter than this before meshing.

**Metadata**:
  - Default: 0.02
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub min_opacity: MinOpacityParam,
    /**Crisp up the vertex texture: 1.0 = physically-correct blend; higher biases each voxel's colour toward its dominant gaussian instead of averaging neighbours (de-smears the texture). Colour only - geometry is unchanged.

**Metadata**:
  - Default: 2
  - Max: 8
  - Min: 1
  - Step: 0.5
*/
    pub color_sharpen: ColorSharpenParam,
}
impl<
    SplatParam: crate::nodes::types::Splat,
    ResolutionParam: crate::nodes::types::Int,
    KernelParam: crate::nodes::types::Int,
    SmoothParam: crate::nodes::types::Int,
    LevelParam: crate::nodes::types::Float,
    MinComponentParam: crate::nodes::types::Int,
    MinOpacityParam: crate::nodes::types::Float,
    ColorSharpenParam: crate::nodes::types::Float,
> SplatToMesh<
    SplatParam,
    ResolutionParam,
    KernelParam,
    SmoothParam,
    LevelParam,
    MinComponentParam,
    MinOpacityParam,
    ColorSharpenParam,
> {
    /// Create a new node.
    pub fn new(
        splat: SplatParam,
        resolution: ResolutionParam,
        kernel: KernelParam,
        smooth: SmoothParam,
        level: LevelParam,
        min_component: MinComponentParam,
        min_opacity: MinOpacityParam,
        color_sharpen: ColorSharpenParam,
    ) -> Self {
        Self {
            splat,
            resolution,
            kernel,
            smooth,
            level,
            min_component,
            min_opacity,
            color_sharpen,
        }
    }
}
impl<
    SplatParam: crate::nodes::types::Splat,
    ResolutionParam: crate::nodes::types::Int,
    KernelParam: crate::nodes::types::Int,
    SmoothParam: crate::nodes::types::Int,
    LevelParam: crate::nodes::types::Float,
    MinComponentParam: crate::nodes::types::Int,
    MinOpacityParam: crate::nodes::types::Float,
    ColorSharpenParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SplatToMesh<
    SplatParam,
    ResolutionParam,
    KernelParam,
    SmoothParam,
    LevelParam,
    MinComponentParam,
    MinOpacityParam,
    ColorSharpenParam,
> {
    type Output = crate::nodes::types::MeshOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("splat".to_string(), self.splat.clone().into());
        output.insert("resolution".to_string(), self.resolution.clone().into());
        output.insert("kernel".to_string(), self.kernel.clone().into());
        output.insert("smooth".to_string(), self.smooth.clone().into());
        output.insert("level".to_string(), self.level.clone().into());
        output.insert("min_component".to_string(), self.min_component.clone().into());
        output.insert("min_opacity".to_string(), self.min_opacity.clone().into());
        output.insert("color_sharpen".to_string(), self.color_sharpen.clone().into());
        output
    }
    const NAME: &'static str = "SplatToMesh";
    const DISPLAY_NAME: &'static str = "Extract Mesh from Splat";
    const DESCRIPTION: &'static str = "Extract a coloured mesh from a gaussian splat.";
    const CATEGORY: &'static str = "3d/splat";
}
///**Transform Splat**: Translate, rotate, and scale a gaussian splat. Non-uniform scale also reshapes every individual splat, slower process.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TransformSplat<
    SplatParam: crate::nodes::types::Splat,
    TranslateXParam: crate::nodes::types::Float,
    TranslateYParam: crate::nodes::types::Float,
    TranslateZParam: crate::nodes::types::Float,
    RotateXParam: crate::nodes::types::Float,
    RotateYParam: crate::nodes::types::Float,
    RotateZParam: crate::nodes::types::Float,
    ScaleXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ScaleZParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub splat: SplatParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub translate_x: TranslateXParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub translate_y: TranslateYParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: -100
  - Step: 0.01
*/
    pub translate_z: TranslateZParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 360
  - Min: -360
  - Step: 1
*/
    pub rotate_x: RotateXParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 360
  - Min: -360
  - Step: 1
*/
    pub rotate_y: RotateYParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 360
  - Min: -360
  - Step: 1
*/
    pub rotate_z: RotateZParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0.01
  - Step: 0.01
*/
    pub scale_x: ScaleXParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0.01
  - Step: 0.01
*/
    pub scale_y: ScaleYParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0.01
  - Step: 0.01
*/
    pub scale_z: ScaleZParam,
}
impl<
    SplatParam: crate::nodes::types::Splat,
    TranslateXParam: crate::nodes::types::Float,
    TranslateYParam: crate::nodes::types::Float,
    TranslateZParam: crate::nodes::types::Float,
    RotateXParam: crate::nodes::types::Float,
    RotateYParam: crate::nodes::types::Float,
    RotateZParam: crate::nodes::types::Float,
    ScaleXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ScaleZParam: crate::nodes::types::Float,
> TransformSplat<
    SplatParam,
    TranslateXParam,
    TranslateYParam,
    TranslateZParam,
    RotateXParam,
    RotateYParam,
    RotateZParam,
    ScaleXParam,
    ScaleYParam,
    ScaleZParam,
> {
    /// Create a new node.
    pub fn new(
        splat: SplatParam,
        translate_x: TranslateXParam,
        translate_y: TranslateYParam,
        translate_z: TranslateZParam,
        rotate_x: RotateXParam,
        rotate_y: RotateYParam,
        rotate_z: RotateZParam,
        scale_x: ScaleXParam,
        scale_y: ScaleYParam,
        scale_z: ScaleZParam,
    ) -> Self {
        Self {
            splat,
            translate_x,
            translate_y,
            translate_z,
            rotate_x,
            rotate_y,
            rotate_z,
            scale_x,
            scale_y,
            scale_z,
        }
    }
}
impl<
    SplatParam: crate::nodes::types::Splat,
    TranslateXParam: crate::nodes::types::Float,
    TranslateYParam: crate::nodes::types::Float,
    TranslateZParam: crate::nodes::types::Float,
    RotateXParam: crate::nodes::types::Float,
    RotateYParam: crate::nodes::types::Float,
    RotateZParam: crate::nodes::types::Float,
    ScaleXParam: crate::nodes::types::Float,
    ScaleYParam: crate::nodes::types::Float,
    ScaleZParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for TransformSplat<
    SplatParam,
    TranslateXParam,
    TranslateYParam,
    TranslateZParam,
    RotateXParam,
    RotateYParam,
    RotateZParam,
    ScaleXParam,
    ScaleYParam,
    ScaleZParam,
> {
    type Output = crate::nodes::types::SplatOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("splat".to_string(), self.splat.clone().into());
        output.insert("translate_x".to_string(), self.translate_x.clone().into());
        output.insert("translate_y".to_string(), self.translate_y.clone().into());
        output.insert("translate_z".to_string(), self.translate_z.clone().into());
        output.insert("rotate_x".to_string(), self.rotate_x.clone().into());
        output.insert("rotate_y".to_string(), self.rotate_y.clone().into());
        output.insert("rotate_z".to_string(), self.rotate_z.clone().into());
        output.insert("scale_x".to_string(), self.scale_x.clone().into());
        output.insert("scale_y".to_string(), self.scale_y.clone().into());
        output.insert("scale_z".to_string(), self.scale_z.clone().into());
        output
    }
    const NAME: &'static str = "TransformSplat";
    const DISPLAY_NAME: &'static str = "Transform Splat";
    const DESCRIPTION: &'static str = "Translate, rotate, and scale a gaussian splat. Non-uniform scale also reshapes every individual splat, slower process.";
    const CATEGORY: &'static str = "3d/splat";
}
