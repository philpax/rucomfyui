//!`geometry estimation` definitions/categories.
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
///**Convert DA3 Geometry to Mesh**: Convert a depth map into a triangulated 3D mesh.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DA3GeometryToMesh<
    Da3GeometryParam: crate::nodes::types::Da3Geometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    ConfidenceThresholdParam: crate::nodes::types::Float,
    UseSkyMaskParam: crate::nodes::types::Boolean,
    TextureParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub da_3_geometry: Da3GeometryParam,
    /**Which image of a batch to convert. Per-image vertex counts differ so batches cannot be stacked.

**Metadata**:
  - Default: 0
  - Max: 4096
  - Min: 0
*/
    pub batch_index: BatchIndexParam,
    /**Vertex stride. 1 = full resolution, 2 = half, etc.

**Metadata**:
  - Default: 1
  - Max: 8
  - Min: 1
*/
    pub decimation: DecimationParam,
    /**Drop triangles whose 3x3 depth span exceeds this fraction. 0 = off.

**Metadata**:
  - Default: 0.04
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub discontinuity_threshold: DiscontinuityThresholdParam,
    /**Exclude pixels whose per-image normalised confidence is below this value (0 = keep all, 1 = keep only the single most confident pixel). Used when the geometry has a confidence map (Small/Base models).

**Metadata**:
  - Default: 0.1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub confidence_threshold: ConfidenceThresholdParam,
    /**Exclude sky-probability pixels (sky >= 0.5) from the mesh. Used when the geometry has a sky map (Mono/Metric models).

**Metadata**:
  - Default: true
*/
    pub use_sky_mask: UseSkyMaskParam,
    /**Use the source image as a base color texture.

**Metadata**:
  - Default: true
*/
    pub texture: TextureParam,
}
impl<
    Da3GeometryParam: crate::nodes::types::Da3Geometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    ConfidenceThresholdParam: crate::nodes::types::Float,
    UseSkyMaskParam: crate::nodes::types::Boolean,
    TextureParam: crate::nodes::types::Boolean,
> DA3GeometryToMesh<
    Da3GeometryParam,
    BatchIndexParam,
    DecimationParam,
    DiscontinuityThresholdParam,
    ConfidenceThresholdParam,
    UseSkyMaskParam,
    TextureParam,
> {
    /// Create a new node.
    pub fn new(
        da_3_geometry: Da3GeometryParam,
        batch_index: BatchIndexParam,
        decimation: DecimationParam,
        discontinuity_threshold: DiscontinuityThresholdParam,
        confidence_threshold: ConfidenceThresholdParam,
        use_sky_mask: UseSkyMaskParam,
        texture: TextureParam,
    ) -> Self {
        Self {
            da_3_geometry,
            batch_index,
            decimation,
            discontinuity_threshold,
            confidence_threshold,
            use_sky_mask,
            texture,
        }
    }
}
impl<
    Da3GeometryParam: crate::nodes::types::Da3Geometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    ConfidenceThresholdParam: crate::nodes::types::Float,
    UseSkyMaskParam: crate::nodes::types::Boolean,
    TextureParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for DA3GeometryToMesh<
    Da3GeometryParam,
    BatchIndexParam,
    DecimationParam,
    DiscontinuityThresholdParam,
    ConfidenceThresholdParam,
    UseSkyMaskParam,
    TextureParam,
> {
    type Output = crate::nodes::types::MeshOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("da3_geometry".to_string(), self.da_3_geometry.clone().into());
        output.insert("batch_index".to_string(), self.batch_index.clone().into());
        output.insert("decimation".to_string(), self.decimation.clone().into());
        output
            .insert(
                "discontinuity_threshold".to_string(),
                self.discontinuity_threshold.clone().into(),
            );
        output
            .insert(
                "confidence_threshold".to_string(),
                self.confidence_threshold.clone().into(),
            );
        output.insert("use_sky_mask".to_string(), self.use_sky_mask.clone().into());
        output.insert("texture".to_string(), self.texture.clone().into());
        output
    }
    const NAME: &'static str = "DA3GeometryToMesh";
    const DISPLAY_NAME: &'static str = "Convert DA3 Geometry to Mesh";
    const DESCRIPTION: &'static str = "Convert a depth map into a triangulated 3D mesh.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Run Depth Anything 3**: Run Depth Anything 3 on an image. In multi-view mode each image is treated as a separate view of the same scene.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DA3Inference<
    Da3ModelParam: crate::nodes::types::Da3Model,
    ImageParam: crate::nodes::types::Image,
    ResolutionParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub da_3_model: Da3ModelParam,
    ///No documentation.
    pub image: ImageParam,
    /**Resolution the model runs at (longest side, multiple of 14).

Lower = faster / less VRAM.

Higher = more detail.

Output is upsampled back to the original size.

**Metadata**:
  - Default: 504
  - Max: 2520
  - Min: 140
  - Step: 14
*/
    pub resolution: ResolutionParam,
}
impl<
    Da3ModelParam: crate::nodes::types::Da3Model,
    ImageParam: crate::nodes::types::Image,
    ResolutionParam: crate::nodes::types::Int,
> DA3Inference<Da3ModelParam, ImageParam, ResolutionParam> {
    /// Create a new node.
    pub fn new(
        da_3_model: Da3ModelParam,
        image: ImageParam,
        resolution: ResolutionParam,
    ) -> Self {
        Self {
            da_3_model,
            image,
            resolution,
        }
    }
}
impl<
    Da3ModelParam: crate::nodes::types::Da3Model,
    ImageParam: crate::nodes::types::Image,
    ResolutionParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for DA3Inference<Da3ModelParam, ImageParam, ResolutionParam> {
    type Output = crate::nodes::types::Da3GeometryOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("da3_model".to_string(), self.da_3_model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("resolution".to_string(), self.resolution.clone().into());
        output
    }
    const NAME: &'static str = "DA3Inference";
    const DISPLAY_NAME: &'static str = "Run Depth Anything 3";
    const DESCRIPTION: &'static str = "Run Depth Anything 3 on an image. In multi-view mode each image is treated as a separate view of the same scene.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Render Depth Anything 3**: Render a depth map, confidence map, or sky mask from Depth Anything 3 geometry data.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DA3Render<Da3GeometryParam: crate::nodes::types::Da3Geometry> {
    ///No documentation.
    pub da_3_geometry: Da3GeometryParam,
}
impl<Da3GeometryParam: crate::nodes::types::Da3Geometry> DA3Render<Da3GeometryParam> {
    /// Create a new node.
    pub fn new(da_3_geometry: Da3GeometryParam) -> Self {
        Self { da_3_geometry }
    }
}
impl<Da3GeometryParam: crate::nodes::types::Da3Geometry> crate::nodes::TypedNode
for DA3Render<Da3GeometryParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("da3_geometry".to_string(), self.da_3_geometry.clone().into());
        output
    }
    const NAME: &'static str = "DA3Render";
    const DISPLAY_NAME: &'static str = "Render Depth Anything 3";
    const DESCRIPTION: &'static str = "Render a depth map, confidence map, or sky mask from Depth Anything 3 geometry data.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Run MoGe Inference**: Run MoGe on a single image to estimate depth and geometry.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoGeInference<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    FovXDegreesParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
    ForceProjectionParam: crate::nodes::types::Boolean,
    ApplyMaskParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub moge_model: MogeModelParam,
    ///No documentation.
    pub image: ImageParam,
    /**0 = fastest, 9 = most detail.

**Metadata**:
  - Default: 9
  - Max: 9
  - Min: 0
*/
    pub resolution_level: ResolutionLevelParam,
    /**Horizontal field of view of the source camera. Sets the focal length used to unproject the depth map into 3D. 0 = auto-recover from the predicted points.

**Metadata**:
  - Default: 0
  - Max: 170
  - Min: 0
  - Step: 0.1
*/
    pub fov_x_degrees: FovXDegreesParam,
    /**Images per inference call. Lower if you OOM on a long video / image set.

**Metadata**:
  - Default: 4
  - Max: 64
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub force_projection: ForceProjectionParam,
    /**Set masked-out (sky / invalid) pixels to inf in points and depth so meshing culls them. Disable to keep the raw predicted geometry everywhere; the mask is still returned separately.

**Metadata**:
  - Default: true
*/
    pub apply_mask: ApplyMaskParam,
}
impl<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    FovXDegreesParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
    ForceProjectionParam: crate::nodes::types::Boolean,
    ApplyMaskParam: crate::nodes::types::Boolean,
> MoGeInference<
    MogeModelParam,
    ImageParam,
    ResolutionLevelParam,
    FovXDegreesParam,
    BatchSizeParam,
    ForceProjectionParam,
    ApplyMaskParam,
> {
    /// Create a new node.
    pub fn new(
        moge_model: MogeModelParam,
        image: ImageParam,
        resolution_level: ResolutionLevelParam,
        fov_x_degrees: FovXDegreesParam,
        batch_size: BatchSizeParam,
        force_projection: ForceProjectionParam,
        apply_mask: ApplyMaskParam,
    ) -> Self {
        Self {
            moge_model,
            image,
            resolution_level,
            fov_x_degrees,
            batch_size,
            force_projection,
            apply_mask,
        }
    }
}
impl<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    FovXDegreesParam: crate::nodes::types::Float,
    BatchSizeParam: crate::nodes::types::Int,
    ForceProjectionParam: crate::nodes::types::Boolean,
    ApplyMaskParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MoGeInference<
    MogeModelParam,
    ImageParam,
    ResolutionLevelParam,
    FovXDegreesParam,
    BatchSizeParam,
    ForceProjectionParam,
    ApplyMaskParam,
> {
    type Output = crate::nodes::types::MogeGeometryOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("moge_model".to_string(), self.moge_model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert(
                "resolution_level".to_string(),
                self.resolution_level.clone().into(),
            );
        output.insert("fov_x_degrees".to_string(), self.fov_x_degrees.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
            .insert(
                "force_projection".to_string(),
                self.force_projection.clone().into(),
            );
        output.insert("apply_mask".to_string(), self.apply_mask.clone().into());
        output
    }
    const NAME: &'static str = "MoGeInference";
    const DISPLAY_NAME: &'static str = "Run MoGe Inference";
    const DESCRIPTION: &'static str = "Run MoGe on a single image to estimate depth and geometry.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Run MoGe Panorama Inference**: Run MoGe on an equirectangular panorama by splitting it into 12 perspective views, running inference on each, and merging the results into a single depth map.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoGePanoramaInference<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    SplitResolutionParam: crate::nodes::types::Int,
    MergeResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub moge_model: MogeModelParam,
    ///Equirectangular panorama (any aspect).
    pub image: ImageParam,
    /**Per-view detail (0 = fastest, 9 = most detailed).

**Metadata**:
  - Default: 9
  - Max: 9
  - Min: 0
*/
    pub resolution_level: ResolutionLevelParam,
    /**Resolution of each perspective split.

**Metadata**:
  - Default: 512
  - Max: 1024
  - Min: 256
*/
    pub split_resolution: SplitResolutionParam,
    /**Long-side resolution of the merged equirect distance map.

**Metadata**:
  - Default: 1920
  - Max: 8192
  - Min: 256
*/
    pub merge_resolution: MergeResolutionParam,
    /**Views per inference batch (12 splits total).

**Metadata**:
  - Default: 4
  - Max: 12
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    SplitResolutionParam: crate::nodes::types::Int,
    MergeResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> MoGePanoramaInference<
    MogeModelParam,
    ImageParam,
    ResolutionLevelParam,
    SplitResolutionParam,
    MergeResolutionParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        moge_model: MogeModelParam,
        image: ImageParam,
        resolution_level: ResolutionLevelParam,
        split_resolution: SplitResolutionParam,
        merge_resolution: MergeResolutionParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            moge_model,
            image,
            resolution_level,
            split_resolution,
            merge_resolution,
            batch_size,
        }
    }
}
impl<
    MogeModelParam: crate::nodes::types::MogeModel,
    ImageParam: crate::nodes::types::Image,
    ResolutionLevelParam: crate::nodes::types::Int,
    SplitResolutionParam: crate::nodes::types::Int,
    MergeResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MoGePanoramaInference<
    MogeModelParam,
    ImageParam,
    ResolutionLevelParam,
    SplitResolutionParam,
    MergeResolutionParam,
    BatchSizeParam,
> {
    type Output = crate::nodes::types::MogeGeometryOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("moge_model".to_string(), self.moge_model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert(
                "resolution_level".to_string(),
                self.resolution_level.clone().into(),
            );
        output
            .insert(
                "split_resolution".to_string(),
                self.split_resolution.clone().into(),
            );
        output
            .insert(
                "merge_resolution".to_string(),
                self.merge_resolution.clone().into(),
            );
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "MoGePanoramaInference";
    const DISPLAY_NAME: &'static str = "Run MoGe Panorama Inference";
    const DESCRIPTION: &'static str = "Run MoGe on an equirectangular panorama by splitting it into 12 perspective views, running inference on each, and merging the results into a single depth map.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Convert MoGe Point Map to Mesh**: Convert a MoGe point map into a 3D mesh.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoGePointMapToMesh<
    MogeGeometryParam: crate::nodes::types::MogeGeometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    TextureParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub moge_geometry: MogeGeometryParam,
    /**Which image of a batched MoGe geometry to mesh. Per-image vertex counts differ, so batches can't be stacked into a single MESH.

**Metadata**:
  - Default: 0
  - Max: 4096
  - Min: 0
*/
    pub batch_index: BatchIndexParam,
    /**Vertex stride; 1 = full resolution.

**Metadata**:
  - Default: 1
  - Max: 8
  - Min: 1
*/
    pub decimation: DecimationParam,
    /**Drop pixels whose 3x3 depth span exceeds this fraction. 0 = off.

**Metadata**:
  - Default: 0.04
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub discontinuity_threshold: DiscontinuityThresholdParam,
    /**Carry the source image through as the baseColor texture.

**Metadata**:
  - Default: true
*/
    pub texture: TextureParam,
}
impl<
    MogeGeometryParam: crate::nodes::types::MogeGeometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    TextureParam: crate::nodes::types::Boolean,
> MoGePointMapToMesh<
    MogeGeometryParam,
    BatchIndexParam,
    DecimationParam,
    DiscontinuityThresholdParam,
    TextureParam,
> {
    /// Create a new node.
    pub fn new(
        moge_geometry: MogeGeometryParam,
        batch_index: BatchIndexParam,
        decimation: DecimationParam,
        discontinuity_threshold: DiscontinuityThresholdParam,
        texture: TextureParam,
    ) -> Self {
        Self {
            moge_geometry,
            batch_index,
            decimation,
            discontinuity_threshold,
            texture,
        }
    }
}
impl<
    MogeGeometryParam: crate::nodes::types::MogeGeometry,
    BatchIndexParam: crate::nodes::types::Int,
    DecimationParam: crate::nodes::types::Int,
    DiscontinuityThresholdParam: crate::nodes::types::Float,
    TextureParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MoGePointMapToMesh<
    MogeGeometryParam,
    BatchIndexParam,
    DecimationParam,
    DiscontinuityThresholdParam,
    TextureParam,
> {
    type Output = crate::nodes::types::MeshOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("moge_geometry".to_string(), self.moge_geometry.clone().into());
        output.insert("batch_index".to_string(), self.batch_index.clone().into());
        output.insert("decimation".to_string(), self.decimation.clone().into());
        output
            .insert(
                "discontinuity_threshold".to_string(),
                self.discontinuity_threshold.clone().into(),
            );
        output.insert("texture".to_string(), self.texture.clone().into());
        output
    }
    const NAME: &'static str = "MoGePointMapToMesh";
    const DISPLAY_NAME: &'static str = "Convert MoGe Point Map to Mesh";
    const DESCRIPTION: &'static str = "Convert a MoGe point map into a 3D mesh.";
    const CATEGORY: &'static str = "image/geometry estimation";
}
///**Render MoGe Geometry**: Render a depth map or normal map from geometry data
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoGeRender<MogeGeometryParam: crate::nodes::types::MogeGeometry> {
    ///No documentation.
    pub moge_geometry: MogeGeometryParam,
}
impl<
    MogeGeometryParam: crate::nodes::types::MogeGeometry,
> MoGeRender<MogeGeometryParam> {
    /// Create a new node.
    pub fn new(moge_geometry: MogeGeometryParam) -> Self {
        Self { moge_geometry }
    }
}
impl<MogeGeometryParam: crate::nodes::types::MogeGeometry> crate::nodes::TypedNode
for MoGeRender<MogeGeometryParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("moge_geometry".to_string(), self.moge_geometry.clone().into());
        output
    }
    const NAME: &'static str = "MoGeRender";
    const DISPLAY_NAME: &'static str = "Render MoGe Geometry";
    const DESCRIPTION: &'static str = "Render a depth map or normal map from geometry data";
    const CATEGORY: &'static str = "image/geometry estimation";
}
