//!`3d` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**EmptyLatentHunyuan3Dv2**: No description.
#[derive(Clone)]
pub struct EmptyLatentHunyuan3Dv2<
    Resolution: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 3072
  - Max: 8192
  - Min: 1
*/
    pub resolution: Resolution,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
}
impl<
    Resolution: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> EmptyLatentHunyuan3Dv2<Resolution, BatchSize> {
    /// Create a new node.
    pub fn new(resolution: Resolution, batch_size: BatchSize) -> Self {
        Self { resolution, batch_size }
    }
}
impl<
    Resolution: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentHunyuan3Dv2<Resolution, BatchSize> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("resolution".to_string(), self.resolution.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyLatentHunyuan3Dv2";
    const DISPLAY_NAME: &'static str = "EmptyLatentHunyuan3Dv2";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/3d";
}
///**VAEDecodeHunyuan3D**: No description.
#[derive(Clone)]
pub struct VaeDecodeHunyuan3D<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
    NumChunks: crate::nodes::types::Int,
    OctreeResolution: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 8000
  - Max: 500000
  - Min: 1000
*/
    pub num_chunks: NumChunks,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 512
  - Min: 16
*/
    pub octree_resolution: OctreeResolution,
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
    NumChunks: crate::nodes::types::Int,
    OctreeResolution: crate::nodes::types::Int,
> VaeDecodeHunyuan3D<Samples, Vae, NumChunks, OctreeResolution> {
    /// Create a new node.
    pub fn new(
        samples: Samples,
        vae: Vae,
        num_chunks: NumChunks,
        octree_resolution: OctreeResolution,
    ) -> Self {
        Self {
            samples,
            vae,
            num_chunks,
            octree_resolution,
        }
    }
}
impl<
    Samples: crate::nodes::types::Latent,
    Vae: crate::nodes::types::Vae,
    NumChunks: crate::nodes::types::Int,
    OctreeResolution: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VaeDecodeHunyuan3D<Samples, Vae, NumChunks, OctreeResolution> {
    type Output = crate::nodes::types::VoxelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("samples".to_string(), self.samples.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("num_chunks".to_string(), self.num_chunks.clone().into());
        output
            .insert(
                "octree_resolution".to_string(),
                self.octree_resolution.clone().into(),
            );
        output
    }
    const NAME: &'static str = "VAEDecodeHunyuan3D";
    const DISPLAY_NAME: &'static str = "VAEDecodeHunyuan3D";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/3d";
}
