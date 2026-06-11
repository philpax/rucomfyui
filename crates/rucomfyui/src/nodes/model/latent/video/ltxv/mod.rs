//!`ltxv` definitions/categories.
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
    ///Output for [`LTXVSeparateAVLatent`](super::LTXVSeparateAVLatent).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVSeparateAVLatentOutput {
        ///No documentation.
        pub video_latent: crate::nodes::types::LatentOut,
        ///No documentation.
        pub audio_latent: crate::nodes::types::LatentOut,
    }
}
///**EmptyLTXVLatentVideo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyLTXVLatentVideo<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 768
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 64
  - Step: 32
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 97
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: LengthParam,
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
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> EmptyLTXVLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
    ) -> Self {
        Self {
            width,
            height,
            length,
            batch_size,
        }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for EmptyLTXVLatentVideo<WidthParam, HeightParam, LengthParam, BatchSizeParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "EmptyLTXVLatentVideo";
    const DISPLAY_NAME: &'static str = "EmptyLTXVLatentVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video/ltxv";
}
///**LTXVConcatAVLatent**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVConcatAVLatent<
    VideoLatentParam: crate::nodes::types::Latent,
    AudioLatentParam: crate::nodes::types::Latent,
> {
    ///No documentation.
    pub video_latent: VideoLatentParam,
    ///No documentation.
    pub audio_latent: AudioLatentParam,
}
impl<
    VideoLatentParam: crate::nodes::types::Latent,
    AudioLatentParam: crate::nodes::types::Latent,
> LTXVConcatAVLatent<VideoLatentParam, AudioLatentParam> {
    /// Create a new node.
    pub fn new(video_latent: VideoLatentParam, audio_latent: AudioLatentParam) -> Self {
        Self { video_latent, audio_latent }
    }
}
impl<
    VideoLatentParam: crate::nodes::types::Latent,
    AudioLatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode for LTXVConcatAVLatent<VideoLatentParam, AudioLatentParam> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video_latent".to_string(), self.video_latent.clone().into());
        output.insert("audio_latent".to_string(), self.audio_latent.clone().into());
        output
    }
    const NAME: &'static str = "LTXVConcatAVLatent";
    const DISPLAY_NAME: &'static str = "LTXVConcatAVLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/latent/video/ltxv";
}
///**LTXVSeparateAVLatent**: LTXV Separate AV Latent
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVSeparateAVLatent<AvLatentParam: crate::nodes::types::Latent> {
    ///No documentation.
    pub av_latent: AvLatentParam,
}
impl<AvLatentParam: crate::nodes::types::Latent> LTXVSeparateAVLatent<AvLatentParam> {
    /// Create a new node.
    pub fn new(av_latent: AvLatentParam) -> Self {
        Self { av_latent }
    }
}
impl<AvLatentParam: crate::nodes::types::Latent> crate::nodes::TypedNode
for LTXVSeparateAVLatent<AvLatentParam> {
    type Output = out::LTXVSeparateAVLatentOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            video_latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            audio_latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("av_latent".to_string(), self.av_latent.clone().into());
        output
    }
    const NAME: &'static str = "LTXVSeparateAVLatent";
    const DISPLAY_NAME: &'static str = "LTXVSeparateAVLatent";
    const DESCRIPTION: &'static str = "LTXV Separate AV Latent";
    const CATEGORY: &'static str = "model/latent/video/ltxv";
}
