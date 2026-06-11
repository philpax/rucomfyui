//!`scail` definitions/categories.
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
    ///Output for [`SCAIL2ColoredMask`](super::SCAIL2ColoredMask).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SCAIL2ColoredMaskOutput {
        ///No documentation.
        pub pose_video_mask: crate::nodes::types::ImageOut,
        ///No documentation.
        pub reference_image_mask: crate::nodes::types::ImageOut,
    }
}
///**Create SCAIL-2 Colored Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SCAIL2ColoredMask<
    DrivingTrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
    ReplacementModeParam: crate::nodes::types::Boolean,
    RefTrackDataParam: crate::nodes::types::Sam3TrackData
        = crate::nodes::types::Sam3TrackDataOut,
> {
    ///SAM3 track of the driving pose video. Will be rendered into the pose_video_mask output.
    pub driving_track_data: DrivingTrackDataParam,
    /**Comma-separated list of person indices to include (e.g. '0,2,3'). Applied to both reference and pose video masks. Empty = all.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub object_indices: ObjectIndicesParam,
    /**False = Animation Mode (pose_video_mask has black background, reference_image_mask has white background). True = Replacement Mode (pose_video_mask has white background, reference_image_mask has black background).

**Metadata**:
  - Default: false
*/
    pub replacement_mode: ReplacementModeParam,
    ///SAM3 track of the reference image.
    pub ref_track_data: Option<RefTrackDataParam>,
}
impl<
    DrivingTrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
    ReplacementModeParam: crate::nodes::types::Boolean,
    RefTrackDataParam: crate::nodes::types::Sam3TrackData,
> SCAIL2ColoredMask<
    DrivingTrackDataParam,
    ObjectIndicesParam,
    ReplacementModeParam,
    RefTrackDataParam,
> {
    /// Create a new node.
    pub fn new(
        driving_track_data: DrivingTrackDataParam,
        object_indices: ObjectIndicesParam,
        replacement_mode: ReplacementModeParam,
        ref_track_data: Option<RefTrackDataParam>,
    ) -> Self {
        Self {
            driving_track_data,
            object_indices,
            replacement_mode,
            ref_track_data,
        }
    }
}
impl<
    DrivingTrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
    ReplacementModeParam: crate::nodes::types::Boolean,
    RefTrackDataParam: crate::nodes::types::Sam3TrackData,
> crate::nodes::TypedNode
for SCAIL2ColoredMask<
    DrivingTrackDataParam,
    ObjectIndicesParam,
    ReplacementModeParam,
    RefTrackDataParam,
> {
    type Output = out::SCAIL2ColoredMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            pose_video_mask: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            reference_image_mask: crate::nodes::types::ImageOut::from_dynamic(
                node_id,
                1u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "driving_track_data".to_string(),
                self.driving_track_data.clone().into(),
            );
        output.insert("object_indices".to_string(), self.object_indices.clone().into());
        output
            .insert(
                "replacement_mode".to_string(),
                self.replacement_mode.clone().into(),
            );
        if let Some(v) = &self.ref_track_data {
            output.insert("ref_track_data".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SCAIL2ColoredMask";
    const DISPLAY_NAME: &'static str = "Create SCAIL-2 Colored Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/video_models/scail";
}
