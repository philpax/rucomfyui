//!`latent` definitions/categories.
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
///**TripoSplat Sampling Preview**: Patch the TripoSplat model for the standard Ksampler node to show a live decoded gaussian splat preview at each step.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoSplatSamplingPreview<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    OctreeLevelParam: crate::nodes::types::Int,
    NumGaussiansParam: crate::nodes::types::Int,
    YawParam: crate::nodes::types::Float,
    PitchParam: crate::nodes::types::Float,
    PointSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    ///TripoSplat VAE decoder
    pub vae: VaeParam,
    /**Octree depth for the preview decode (lower = cheaper/coarser).

**Metadata**:
  - Default: 5
  - Max: 8
  - Min: 2
*/
    pub octree_level: OctreeLevelParam,
    /**Number of gaussians to produce for the preview (rounded to a multiple of 32).

**Metadata**:
  - Default: 16384
  - Max: 262144
  - Min: 1024
  - Step: 32
*/
    pub num_gaussians: NumGaussiansParam,
    /**Preview camera yaw in degrees.

**Metadata**:
  - Default: 90
  - Max: 360
  - Min: -360
  - Step: 1
*/
    pub yaw: YawParam,
    /**Preview camera pitch in degrees.

**Metadata**:
  - Default: 15
  - Max: 89
  - Min: -89
  - Step: 1
*/
    pub pitch: PitchParam,
    /**Maximum splat radius in pixels. Each gaussian is sized from its scale and capped here; lower = finer/pointier, higher = chunkier.

**Metadata**:
  - Default: 3
  - Max: 16
  - Min: 1
*/
    pub point_size: PointSizeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    OctreeLevelParam: crate::nodes::types::Int,
    NumGaussiansParam: crate::nodes::types::Int,
    YawParam: crate::nodes::types::Float,
    PitchParam: crate::nodes::types::Float,
    PointSizeParam: crate::nodes::types::Int,
> TripoSplatSamplingPreview<
    ModelParam,
    VaeParam,
    OctreeLevelParam,
    NumGaussiansParam,
    YawParam,
    PitchParam,
    PointSizeParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        vae: VaeParam,
        octree_level: OctreeLevelParam,
        num_gaussians: NumGaussiansParam,
        yaw: YawParam,
        pitch: PitchParam,
        point_size: PointSizeParam,
    ) -> Self {
        Self {
            model,
            vae,
            octree_level,
            num_gaussians,
            yaw,
            pitch,
            point_size,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    OctreeLevelParam: crate::nodes::types::Int,
    NumGaussiansParam: crate::nodes::types::Int,
    YawParam: crate::nodes::types::Float,
    PitchParam: crate::nodes::types::Float,
    PointSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TripoSplatSamplingPreview<
    ModelParam,
    VaeParam,
    OctreeLevelParam,
    NumGaussiansParam,
    YawParam,
    PitchParam,
    PointSizeParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("octree_level".to_string(), self.octree_level.clone().into());
        output.insert("num_gaussians".to_string(), self.num_gaussians.clone().into());
        output.insert("yaw".to_string(), self.yaw.clone().into());
        output.insert("pitch".to_string(), self.pitch.clone().into());
        output.insert("point_size".to_string(), self.point_size.clone().into());
        output
    }
    const NAME: &'static str = "TripoSplatSamplingPreview";
    const DISPLAY_NAME: &'static str = "TripoSplat Sampling Preview";
    const DESCRIPTION: &'static str = "Patch the TripoSplat model for the standard Ksampler node to show a live decoded gaussian splat preview at each step.";
    const CATEGORY: &'static str = "3d/latent";
}
///**TripoSplat Decode**: Decode the sampled TripoSplat latent into a 3D gaussian splat. Modify the number of gaussians to vary the density.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VAEDecodeTripoSplat<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumGaussiansParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///TripoSplat VAE decoder
    pub vae: VaeParam,
    /**Number of gaussians to produce (rounded to a multiple of 32). 262144 matches the octree's point density; higher oversamples the same points (denser, but no new detail) and costs proportionally more VRAM/time.

**Metadata**:
  - Default: 262144
  - Max: 1048576
  - Min: 32768
  - Step: 32
*/
    pub num_gaussians: NumGaussiansParam,
    /**Seeds the octree point sampler (global RNG) for deterministic decodes.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumGaussiansParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> VAEDecodeTripoSplat<SamplesParam, VaeParam, NumGaussiansParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        vae: VaeParam,
        num_gaussians: NumGaussiansParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            samples,
            vae,
            num_gaussians,
            seed,
        }
    }
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumGaussiansParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VAEDecodeTripoSplat<SamplesParam, VaeParam, NumGaussiansParam, SeedParam> {
    type Output = crate::nodes::types::SplatOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("num_gaussians".to_string(), self.num_gaussians.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "VAEDecodeTripoSplat";
    const DISPLAY_NAME: &'static str = "TripoSplat Decode";
    const DESCRIPTION: &'static str = "Decode the sampled TripoSplat latent into a 3D gaussian splat. Modify the number of gaussians to vary the density.";
    const CATEGORY: &'static str = "3d/latent";
}
