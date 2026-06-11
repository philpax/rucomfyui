//!`video` definitions/categories.
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
    ///Output for [`WanDancerPadKeyframes`](super::WanDancerPadKeyframes).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanDancerPadKeyframesOutput {
        ///Padded keyframe sequence
        pub keyframes_sequence: crate::nodes::types::ImageOut,
        ///Mask indicating valid frames
        pub keyframes_mask: crate::nodes::types::MaskOut,
        ///Audio segment for this video segment
        pub audio_segment: crate::nodes::types::AudioOut,
    }
    ///Output for [`WanDancerPadKeyframesList`](super::WanDancerPadKeyframesList).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct WanDancerPadKeyframesListOutput {
        ///Padded keyframe sequences
        pub keyframes_sequence: crate::nodes::types::ImageOut,
        ///Masks indicating valid frames
        pub keyframes_mask: crate::nodes::types::MaskOut,
        ///Audio segment for each video segment
        pub audio_segment: crate::nodes::types::AudioOut,
    }
}
///**WanDancerPadKeyframes**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanDancerPadKeyframes<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    SegmentIndexParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**Length of this segment (usually 149 frames)

**Metadata**:
  - Default: 149
  - Max: 10000
  - Min: 1
*/
    pub segment_length: SegmentLengthParam,
    /**Which segment this is (0 for first, 1 for second, etc.)

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: 0
*/
    pub segment_index: SegmentIndexParam,
    ///Audio to calculate total output frames from and extract segment audio.
    pub audio: AudioParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    SegmentIndexParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> WanDancerPadKeyframes<ImagesParam, SegmentLengthParam, SegmentIndexParam, AudioParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        segment_length: SegmentLengthParam,
        segment_index: SegmentIndexParam,
        audio: AudioParam,
    ) -> Self {
        Self {
            images,
            segment_length,
            segment_index,
            audio,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    SegmentIndexParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode
for WanDancerPadKeyframes<
    ImagesParam,
    SegmentLengthParam,
    SegmentIndexParam,
    AudioParam,
> {
    type Output = out::WanDancerPadKeyframesOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            keyframes_sequence: crate::nodes::types::ImageOut::from_dynamic(
                node_id,
                0u32,
            ),
            keyframes_mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            audio_segment: crate::nodes::types::AudioOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("segment_length".to_string(), self.segment_length.clone().into());
        output.insert("segment_index".to_string(), self.segment_index.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "WanDancerPadKeyframes";
    const DISPLAY_NAME: &'static str = "WanDancerPadKeyframes";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/video";
}
///**WanDancerPadKeyframesList**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WanDancerPadKeyframesList<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    NumSegmentsParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**Length of each segment (usually 149 frames)

**Metadata**:
  - Default: 149
  - Max: 10000
  - Min: 1
*/
    pub segment_length: SegmentLengthParam,
    /**How many padded segments to emit as lists.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 1
*/
    pub num_segments: NumSegmentsParam,
    ///Audio to slice for each emitted segment.
    pub audio: AudioParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    NumSegmentsParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> WanDancerPadKeyframesList<
    ImagesParam,
    SegmentLengthParam,
    NumSegmentsParam,
    AudioParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        segment_length: SegmentLengthParam,
        num_segments: NumSegmentsParam,
        audio: AudioParam,
    ) -> Self {
        Self {
            images,
            segment_length,
            num_segments,
            audio,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SegmentLengthParam: crate::nodes::types::Int,
    NumSegmentsParam: crate::nodes::types::Int,
    AudioParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode
for WanDancerPadKeyframesList<
    ImagesParam,
    SegmentLengthParam,
    NumSegmentsParam,
    AudioParam,
> {
    type Output = out::WanDancerPadKeyframesListOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            keyframes_sequence: crate::nodes::types::ImageOut::from_dynamic(
                node_id,
                0u32,
            ),
            keyframes_mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
            audio_segment: crate::nodes::types::AudioOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("segment_length".to_string(), self.segment_length.clone().into());
        output.insert("num_segments".to_string(), self.num_segments.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "WanDancerPadKeyframesList";
    const DISPLAY_NAME: &'static str = "WanDancerPadKeyframesList";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/video";
}
