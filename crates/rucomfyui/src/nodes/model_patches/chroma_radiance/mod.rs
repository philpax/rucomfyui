//!`chroma_radiance` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ChromaRadianceOptions**: Allows setting advanced options for the Chroma Radiance model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ChromaRadianceOptions<
    ModelParam: crate::nodes::types::Model,
    PreserveWrapperParam: crate::nodes::types::Boolean,
    StartSigmaParam: crate::nodes::types::Float,
    EndSigmaParam: crate::nodes::types::Float,
    NerfTileSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    /**When enabled, will delegate to an existing model function wrapper if it exists. Generally should be left enabled.

**Metadata**:
  - Default: true
*/
    pub preserve_wrapper: PreserveWrapperParam,
    /**First sigma that these options will be in effect.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
*/
    pub start_sigma: StartSigmaParam,
    /**Last sigma that these options will be in effect.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
*/
    pub end_sigma: EndSigmaParam,
    ///Allows overriding the default NeRF tile size. -1 means use the default (32). 0 means use non-tiling mode (may require a lot of VRAM).
    pub nerf_tile_size: NerfTileSizeParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    PreserveWrapperParam: crate::nodes::types::Boolean,
    StartSigmaParam: crate::nodes::types::Float,
    EndSigmaParam: crate::nodes::types::Float,
    NerfTileSizeParam: crate::nodes::types::Int,
> ChromaRadianceOptions<
    ModelParam,
    PreserveWrapperParam,
    StartSigmaParam,
    EndSigmaParam,
    NerfTileSizeParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        preserve_wrapper: PreserveWrapperParam,
        start_sigma: StartSigmaParam,
        end_sigma: EndSigmaParam,
        nerf_tile_size: NerfTileSizeParam,
    ) -> Self {
        Self {
            model,
            preserve_wrapper,
            start_sigma,
            end_sigma,
            nerf_tile_size,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    PreserveWrapperParam: crate::nodes::types::Boolean,
    StartSigmaParam: crate::nodes::types::Float,
    EndSigmaParam: crate::nodes::types::Float,
    NerfTileSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ChromaRadianceOptions<
    ModelParam,
    PreserveWrapperParam,
    StartSigmaParam,
    EndSigmaParam,
    NerfTileSizeParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert(
                "preserve_wrapper".to_string(),
                self.preserve_wrapper.clone().into(),
            );
        output.insert("start_sigma".to_string(), self.start_sigma.clone().into());
        output.insert("end_sigma".to_string(), self.end_sigma.clone().into());
        output.insert("nerf_tile_size".to_string(), self.nerf_tile_size.clone().into());
        output
    }
    const NAME: &'static str = "ChromaRadianceOptions";
    const DISPLAY_NAME: &'static str = "ChromaRadianceOptions";
    const DESCRIPTION: &'static str = "Allows setting advanced options for the Chroma Radiance model.";
    const CATEGORY: &'static str = "model_patches/chroma_radiance";
}
