//!`video_models` definitions/categories.
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
#[rustfmt::skip]
pub mod scail;
/// Output types for nodes.
pub mod out {
    ///Output for [`BerniniConditioning`](super::BerniniConditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct BerniniConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**Bernini Conditioning**: Conditioning node for Bernini in-context video/image conditioning. It can be used for the following tasks: t2v (text-to-video), v2v (video-to-video), rv2v (reference-guided video editing), r2v (reference-to-video), ads2v (insert image/video into video).Reference images injected as in-context tokens (r2v, rv2v) are encoded independently at their own native aspect ratio (long edge capped at ref_max_size)
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BerniniConditioning<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    SourceVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ReferenceVideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    RefMaxSizeParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 832
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 480
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 81
  - Max: 8192
  - Min: 1
  - Step: 4
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///Source video to edit or restyle (v2v, rv2v). Resized to width/height and trimmed to length.
    pub source_video: Option<SourceVideoParam>,
    ///Video to insert into the source video (ads2v).
    pub reference_video: Option<ReferenceVideoParam>,
    /**Max size for the long edge of reference_video and reference_images. Resized with preserved aspect ratio and snapped to 16px.

**Metadata**:
  - Default: 848
  - Max: 8192
  - Min: 16
  - Step: 16
*/
    pub ref_max_size: Option<RefMaxSizeParam>,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    SourceVideoParam: crate::nodes::types::Image,
    ReferenceVideoParam: crate::nodes::types::Image,
    RefMaxSizeParam: crate::nodes::types::Int,
> BerniniConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    SourceVideoParam,
    ReferenceVideoParam,
    RefMaxSizeParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        source_video: Option<SourceVideoParam>,
        reference_video: Option<ReferenceVideoParam>,
        ref_max_size: Option<RefMaxSizeParam>,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            width,
            height,
            length,
            batch_size,
            source_video,
            reference_video,
            ref_max_size,
        }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    SourceVideoParam: crate::nodes::types::Image,
    ReferenceVideoParam: crate::nodes::types::Image,
    RefMaxSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for BerniniConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    SourceVideoParam,
    ReferenceVideoParam,
    RefMaxSizeParam,
> {
    type Output = out::BerniniConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.source_video {
            output.insert("source_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.reference_video {
            output.insert("reference_video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ref_max_size {
            output.insert("ref_max_size".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "BerniniConditioning";
    const DISPLAY_NAME: &'static str = "Bernini Conditioning";
    const DESCRIPTION: &'static str = "Conditioning node for Bernini in-context video/image conditioning. It can be used for the following tasks: t2v (text-to-video), v2v (video-to-video), rv2v (reference-guided video editing), r2v (reference-to-video), ads2v (insert image/video into video).Reference images injected as in-context tokens (r2v, rv2v) are encoded independently at their own native aspect ratio (long edge capped at ref_max_size)";
    const CATEGORY: &'static str = "conditioning/video_models";
}
