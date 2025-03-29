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
    ResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 3072
  - Max: 8192
  - Min: 1
*/
    pub resolution: ResolutionParam,
    /**The number of latent images in the batch.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    ResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyLatentHunyuan3Dv2<ResolutionParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(resolution: ResolutionParam, batch_size: BatchSizeParam) -> Self {
        Self { resolution, batch_size }
    }
}
impl<
    ResolutionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyLatentHunyuan3Dv2<ResolutionParam, BatchSizeParam> {
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
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumChunksParam: crate::nodes::types::Int,
    OctreeResolutionParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub samples: SamplesParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 8000
  - Max: 500000
  - Min: 1000
*/
    pub num_chunks: NumChunksParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 512
  - Min: 16
*/
    pub octree_resolution: OctreeResolutionParam,
}
impl<
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumChunksParam: crate::nodes::types::Int,
    OctreeResolutionParam: crate::nodes::types::Int,
> VaeDecodeHunyuan3D<SamplesParam, VaeParam, NumChunksParam, OctreeResolutionParam> {
    /// Create a new node.
    pub fn new(
        samples: SamplesParam,
        vae: VaeParam,
        num_chunks: NumChunksParam,
        octree_resolution: OctreeResolutionParam,
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
    SamplesParam: crate::nodes::types::Latent,
    VaeParam: crate::nodes::types::Vae,
    NumChunksParam: crate::nodes::types::Int,
    OctreeResolutionParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for VaeDecodeHunyuan3D<SamplesParam, VaeParam, NumChunksParam, OctreeResolutionParam> {
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
