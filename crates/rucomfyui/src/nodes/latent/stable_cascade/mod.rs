//!`stable_cascade` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`StableCascade_EmptyLatentImage`](super::StableCascade_EmptyLatentImage).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct StableCascade_EmptyLatentImageOutput {
        ///No documentation.
        pub stage_c: crate::nodes::types::LatentOut,
        ///No documentation.
        pub stage_b: crate::nodes::types::LatentOut,
    }
    ///Output for [`StableCascade_StageC_VAEEncode`](super::StableCascade_StageC_VAEEncode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct StableCascade_StageC_VAEEncodeOutput {
        ///No documentation.
        pub stage_c: crate::nodes::types::LatentOut,
        ///No documentation.
        pub stage_b: crate::nodes::types::LatentOut,
    }
}
///**StableCascade_EmptyLatentImage**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StableCascade_EmptyLatentImage<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CompressionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 256
  - Step: 8
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 256
  - Step: 8
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 42
  - Max: 128
  - Min: 4
  - Step: 1
*/
    pub compression: CompressionParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CompressionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> StableCascade_EmptyLatentImage<
    WidthParam,
    HeightParam,
    CompressionParam,
    BatchSizeParam,
> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        compression: CompressionParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            compression,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CompressionParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for StableCascade_EmptyLatentImage<
    WidthParam,
    HeightParam,
    CompressionParam,
    BatchSizeParam,
> {
    type Output = out::StableCascade_EmptyLatentImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            stage_b: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("compression".to_string(), self.compression.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DISPLAY_NAME: &'static str = "StableCascade_EmptyLatentImage";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
///**StableCascade_StageC_VAEEncode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StableCascade_StageC_VAEEncode<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    CompressionParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 42
  - Max: 128
  - Min: 4
  - Step: 1
*/
    pub compression: CompressionParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    CompressionParam: crate::nodes::types::Int,
> StableCascade_StageC_VAEEncode<ImageParam, VaeParam, CompressionParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, vae: VaeParam, compression: CompressionParam) -> Self {
        Self { image, vae, compression }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    CompressionParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for StableCascade_StageC_VAEEncode<ImageParam, VaeParam, CompressionParam> {
    type Output = out::StableCascade_StageC_VAEEncodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            stage_c: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            stage_b: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("compression".to_string(), self.compression.clone().into());
        output
    }
    const NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DISPLAY_NAME: &'static str = "StableCascade_StageC_VAEEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/stable_cascade";
}
