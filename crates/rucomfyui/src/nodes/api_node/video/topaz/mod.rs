//!`Topaz` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Topaz Video Enhance**: Breathe new life into video with powerful upscaling and recovery technology.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TopazVideoEnhance<
    VideoParam: crate::nodes::types::Video,
    UpscalerEnabledParam: crate::nodes::types::Boolean,
    InterpolationEnabledParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    InterpolationSlowmoParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    InterpolationFrameRateParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    InterpolationDuplicateParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    InterpolationDuplicateThresholdParam: crate::nodes::types::Float
        = crate::nodes::types::FloatOut,
> {
    ///No documentation.
    pub video: VideoParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub upscaler_enabled: UpscalerEnabledParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub interpolation_enabled: Option<InterpolationEnabledParam>,
    /**Slow-motion factor applied to the input video. For example, 2 makes the output twice as slow and doubles the duration.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 16
  - Min: 1
*/
    pub interpolation_slowmo: Option<InterpolationSlowmoParam>,
    /**Output frame rate.

**Metadata**:
  - Default: 60
  - Display: number
  - Max: 240
  - Min: 15
*/
    pub interpolation_frame_rate: Option<InterpolationFrameRateParam>,
    /**Analyze the input for duplicate frames and remove them.

**Metadata**:
  - Default: false
*/
    pub interpolation_duplicate: Option<InterpolationDuplicateParam>,
    /**Detection sensitivity for duplicate frames.

**Metadata**:
  - Default: 0.01
  - Display: number
  - Max: 0.1
  - Min: 0.001
  - Step: 0.001
*/
    pub interpolation_duplicate_threshold: Option<InterpolationDuplicateThresholdParam>,
}
impl<
    VideoParam: crate::nodes::types::Video,
    UpscalerEnabledParam: crate::nodes::types::Boolean,
    InterpolationEnabledParam: crate::nodes::types::Boolean,
    InterpolationSlowmoParam: crate::nodes::types::Int,
    InterpolationFrameRateParam: crate::nodes::types::Int,
    InterpolationDuplicateParam: crate::nodes::types::Boolean,
    InterpolationDuplicateThresholdParam: crate::nodes::types::Float,
> TopazVideoEnhance<
    VideoParam,
    UpscalerEnabledParam,
    InterpolationEnabledParam,
    InterpolationSlowmoParam,
    InterpolationFrameRateParam,
    InterpolationDuplicateParam,
    InterpolationDuplicateThresholdParam,
> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        upscaler_enabled: UpscalerEnabledParam,
        interpolation_enabled: Option<InterpolationEnabledParam>,
        interpolation_slowmo: Option<InterpolationSlowmoParam>,
        interpolation_frame_rate: Option<InterpolationFrameRateParam>,
        interpolation_duplicate: Option<InterpolationDuplicateParam>,
        interpolation_duplicate_threshold: Option<InterpolationDuplicateThresholdParam>,
    ) -> Self {
        Self {
            video,
            upscaler_enabled,
            interpolation_enabled,
            interpolation_slowmo,
            interpolation_frame_rate,
            interpolation_duplicate,
            interpolation_duplicate_threshold,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    UpscalerEnabledParam: crate::nodes::types::Boolean,
    InterpolationEnabledParam: crate::nodes::types::Boolean,
    InterpolationSlowmoParam: crate::nodes::types::Int,
    InterpolationFrameRateParam: crate::nodes::types::Int,
    InterpolationDuplicateParam: crate::nodes::types::Boolean,
    InterpolationDuplicateThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for TopazVideoEnhance<
    VideoParam,
    UpscalerEnabledParam,
    InterpolationEnabledParam,
    InterpolationSlowmoParam,
    InterpolationFrameRateParam,
    InterpolationDuplicateParam,
    InterpolationDuplicateThresholdParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output
            .insert(
                "upscaler_enabled".to_string(),
                self.upscaler_enabled.clone().into(),
            );
        if let Some(v) = &self.interpolation_enabled {
            output.insert("interpolation_enabled".to_string(), v.clone().into());
        }
        if let Some(v) = &self.interpolation_slowmo {
            output.insert("interpolation_slowmo".to_string(), v.clone().into());
        }
        if let Some(v) = &self.interpolation_frame_rate {
            output.insert("interpolation_frame_rate".to_string(), v.clone().into());
        }
        if let Some(v) = &self.interpolation_duplicate {
            output.insert("interpolation_duplicate".to_string(), v.clone().into());
        }
        if let Some(v) = &self.interpolation_duplicate_threshold {
            output
                .insert(
                    "interpolation_duplicate_threshold".to_string(),
                    v.clone().into(),
                );
        }
        output
    }
    const NAME: &'static str = "TopazVideoEnhance";
    const DISPLAY_NAME: &'static str = "Topaz Video Enhance";
    const DESCRIPTION: &'static str = "Breathe new life into video with powerful upscaling and recovery technology.";
    const CATEGORY: &'static str = "api node/video/Topaz";
}
