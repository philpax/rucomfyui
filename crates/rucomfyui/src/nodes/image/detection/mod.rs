//!`detection` definitions/categories.
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
    ///Output for [`MediaPipeFaceLandmarker`](super::MediaPipeFaceLandmarker).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct MediaPipeFaceLandmarkerOutput {
        ///No documentation.
        pub face_landmarks: crate::nodes::types::FaceLandmarksOut,
        ///No documentation.
        pub bboxes: crate::nodes::types::BoundingBoxOut,
    }
    ///Output for [`SAM3_Detect`](super::SAM3_Detect).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SAM3_DetectOutput {
        ///No documentation.
        pub masks: crate::nodes::types::MaskOut,
        ///No documentation.
        pub bboxes: crate::nodes::types::BoundingBoxOut,
    }
}
///**Draw BBoxes**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DrawBBoxes<
    BboxesParam: crate::nodes::types::BoundingBox,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub bboxes: BboxesParam,
    ///No documentation.
    pub image: Option<ImageParam>,
}
impl<
    BboxesParam: crate::nodes::types::BoundingBox,
    ImageParam: crate::nodes::types::Image,
> DrawBBoxes<BboxesParam, ImageParam> {
    /// Create a new node.
    pub fn new(bboxes: BboxesParam, image: Option<ImageParam>) -> Self {
        Self { bboxes, image }
    }
}
impl<
    BboxesParam: crate::nodes::types::BoundingBox,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for DrawBBoxes<BboxesParam, ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("bboxes".to_string(), self.bboxes.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "DrawBBoxes";
    const DISPLAY_NAME: &'static str = "Draw BBoxes";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**Detect Face Landmarks (MediaPipe)**: Detects facial landmarks using MediaPipe model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MediaPipeFaceLandmarker<
    FaceDetectionModelParam: crate::nodes::types::FaceDetectionModel,
    ImageParam: crate::nodes::types::Image,
    NumFacesParam: crate::nodes::types::Int,
    MinConfidenceParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub face_detection_model: FaceDetectionModelParam,
    ///No documentation.
    pub image: ImageParam,
    /**Maximum faces to return per frame. 0 = no cap (return all detected).

**Metadata**:
  - Default: 1
  - Max: 16
  - Min: 0
  - Step: 1
*/
    pub num_faces: NumFacesParam,
    /**BlazeFace score threshold. Lower to catch small/occluded faces.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub min_confidence: MinConfidenceParam,
}
impl<
    FaceDetectionModelParam: crate::nodes::types::FaceDetectionModel,
    ImageParam: crate::nodes::types::Image,
    NumFacesParam: crate::nodes::types::Int,
    MinConfidenceParam: crate::nodes::types::Float,
> MediaPipeFaceLandmarker<
    FaceDetectionModelParam,
    ImageParam,
    NumFacesParam,
    MinConfidenceParam,
> {
    /// Create a new node.
    pub fn new(
        face_detection_model: FaceDetectionModelParam,
        image: ImageParam,
        num_faces: NumFacesParam,
        min_confidence: MinConfidenceParam,
    ) -> Self {
        Self {
            face_detection_model,
            image,
            num_faces,
            min_confidence,
        }
    }
}
impl<
    FaceDetectionModelParam: crate::nodes::types::FaceDetectionModel,
    ImageParam: crate::nodes::types::Image,
    NumFacesParam: crate::nodes::types::Int,
    MinConfidenceParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for MediaPipeFaceLandmarker<
    FaceDetectionModelParam,
    ImageParam,
    NumFacesParam,
    MinConfidenceParam,
> {
    type Output = out::MediaPipeFaceLandmarkerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            face_landmarks: crate::nodes::types::FaceLandmarksOut::from_dynamic(
                node_id,
                0u32,
            ),
            bboxes: crate::nodes::types::BoundingBoxOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "face_detection_model".to_string(),
                self.face_detection_model.clone().into(),
            );
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("num_faces".to_string(), self.num_faces.clone().into());
        output.insert("min_confidence".to_string(), self.min_confidence.clone().into());
        output
    }
    const NAME: &'static str = "MediaPipeFaceLandmarker";
    const DISPLAY_NAME: &'static str = "Detect Face Landmarks (MediaPipe)";
    const DESCRIPTION: &'static str = "Detects facial landmarks using MediaPipe model.";
    const CATEGORY: &'static str = "image/detection";
}
///**Draw Face Mask (MediaPipe)**: Draws a mask from face landmarks.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MediaPipeFaceMask<FaceLandmarksParam: crate::nodes::types::FaceLandmarks> {
    ///No documentation.
    pub face_landmarks: FaceLandmarksParam,
}
impl<
    FaceLandmarksParam: crate::nodes::types::FaceLandmarks,
> MediaPipeFaceMask<FaceLandmarksParam> {
    /// Create a new node.
    pub fn new(face_landmarks: FaceLandmarksParam) -> Self {
        Self { face_landmarks }
    }
}
impl<FaceLandmarksParam: crate::nodes::types::FaceLandmarks> crate::nodes::TypedNode
for MediaPipeFaceMask<FaceLandmarksParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("face_landmarks".to_string(), self.face_landmarks.clone().into());
        output
    }
    const NAME: &'static str = "MediaPipeFaceMask";
    const DISPLAY_NAME: &'static str = "Draw Face Mask (MediaPipe)";
    const DESCRIPTION: &'static str = "Draws a mask from face landmarks.";
    const CATEGORY: &'static str = "image/detection";
}
///**Visualize Face Landmarks (MediaPipe)**: Draws face landmarks mesh on the input image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MediaPipeFaceMeshVisualize<
    FaceLandmarksParam: crate::nodes::types::FaceLandmarks,
    ColorParam: crate::nodes::types::Color,
    ThicknessParam: crate::nodes::types::Int,
    PointSizeParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub face_landmarks: FaceLandmarksParam,
    /**No documentation.

**Metadata**:
  - Default: #00ff00
*/
    pub color: ColorParam,
    /**Edge line thickness in pixels. 0 disables edge drawing.

**Metadata**:
  - Default: 1
  - Max: 8
  - Min: 0
  - Step: 1
*/
    pub thickness: ThicknessParam,
    /**Landmark dot radius in pixels. 0 disables point drawing.

**Metadata**:
  - Default: 2
  - Max: 16
  - Min: 0
  - Step: 1
*/
    pub point_size: PointSizeParam,
    ///If not connected, a black canvas will be used.
    pub image: Option<ImageParam>,
}
impl<
    FaceLandmarksParam: crate::nodes::types::FaceLandmarks,
    ColorParam: crate::nodes::types::Color,
    ThicknessParam: crate::nodes::types::Int,
    PointSizeParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> MediaPipeFaceMeshVisualize<
    FaceLandmarksParam,
    ColorParam,
    ThicknessParam,
    PointSizeParam,
    ImageParam,
> {
    /// Create a new node.
    pub fn new(
        face_landmarks: FaceLandmarksParam,
        color: ColorParam,
        thickness: ThicknessParam,
        point_size: PointSizeParam,
        image: Option<ImageParam>,
    ) -> Self {
        Self {
            face_landmarks,
            color,
            thickness,
            point_size,
            image,
        }
    }
}
impl<
    FaceLandmarksParam: crate::nodes::types::FaceLandmarks,
    ColorParam: crate::nodes::types::Color,
    ThicknessParam: crate::nodes::types::Int,
    PointSizeParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for MediaPipeFaceMeshVisualize<
    FaceLandmarksParam,
    ColorParam,
    ThicknessParam,
    PointSizeParam,
    ImageParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("face_landmarks".to_string(), self.face_landmarks.clone().into());
        output.insert("color".to_string(), self.color.clone().into());
        output.insert("thickness".to_string(), self.thickness.clone().into());
        output.insert("point_size".to_string(), self.point_size.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MediaPipeFaceMeshVisualize";
    const DISPLAY_NAME: &'static str = "Visualize Face Landmarks (MediaPipe)";
    const DESCRIPTION: &'static str = "Draws face landmarks mesh on the input image.";
    const CATEGORY: &'static str = "image/detection";
}
///**RT-DETR Detect**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RTDETR_detect<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    MaxDetectionsParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub threshold: ThresholdParam,
    ///Maximum number of detections to return per image. In order of descending confidence score.
    pub max_detections: MaxDetectionsParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    MaxDetectionsParam: crate::nodes::types::Int,
> RTDETR_detect<ModelParam, ImageParam, ThresholdParam, MaxDetectionsParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        image: ImageParam,
        threshold: ThresholdParam,
        max_detections: MaxDetectionsParam,
    ) -> Self {
        Self {
            model,
            image,
            threshold,
            max_detections,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    MaxDetectionsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RTDETR_detect<ModelParam, ImageParam, ThresholdParam, MaxDetectionsParam> {
    type Output = crate::nodes::types::BoundingBoxOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("threshold".to_string(), self.threshold.clone().into());
        output.insert("max_detections".to_string(), self.max_detections.clone().into());
        output
    }
    const NAME: &'static str = "RTDETR_detect";
    const DISPLAY_NAME: &'static str = "RT-DETR Detect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SAM3 Detect**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SAM3_Detect<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    RefineIterationsParam: crate::nodes::types::Int,
    IndividualMasksParam: crate::nodes::types::Boolean,
    ConditioningParam: crate::nodes::types::Conditioning
        = crate::nodes::types::ConditioningOut,
    BboxesParam: crate::nodes::types::BoundingBox = crate::nodes::types::BoundingBoxOut,
    PositiveCoordsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    NegativeCoordsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub threshold: ThresholdParam,
    /**SAM decoder refinement passes (0=use raw detector masks)

**Metadata**:
  - Default: 2
  - Max: 5
  - Min: 0
*/
    pub refine_iterations: RefineIterationsParam,
    /**Output per-object masks instead of union

**Metadata**:
  - Default: false
*/
    pub individual_masks: IndividualMasksParam,
    ///Text conditioning from CLIPTextEncode
    pub conditioning: Option<ConditioningParam>,
    ///Bounding boxes to segment within
    pub bboxes: Option<BboxesParam>,
    /**Positive point prompts as JSON \[{"x": int, "y": int}, ...\] (pixel coords)

**Metadata**:
  - Multiline: false
*/
    pub positive_coords: Option<PositiveCoordsParam>,
    /**Negative point prompts as JSON \[{"x": int, "y": int}, ...\] (pixel coords)

**Metadata**:
  - Multiline: false
*/
    pub negative_coords: Option<NegativeCoordsParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    RefineIterationsParam: crate::nodes::types::Int,
    IndividualMasksParam: crate::nodes::types::Boolean,
    ConditioningParam: crate::nodes::types::Conditioning,
    BboxesParam: crate::nodes::types::BoundingBox,
    PositiveCoordsParam: crate::nodes::types::String,
    NegativeCoordsParam: crate::nodes::types::String,
> SAM3_Detect<
    ModelParam,
    ImageParam,
    ThresholdParam,
    RefineIterationsParam,
    IndividualMasksParam,
    ConditioningParam,
    BboxesParam,
    PositiveCoordsParam,
    NegativeCoordsParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        image: ImageParam,
        threshold: ThresholdParam,
        refine_iterations: RefineIterationsParam,
        individual_masks: IndividualMasksParam,
        conditioning: Option<ConditioningParam>,
        bboxes: Option<BboxesParam>,
        positive_coords: Option<PositiveCoordsParam>,
        negative_coords: Option<NegativeCoordsParam>,
    ) -> Self {
        Self {
            model,
            image,
            threshold,
            refine_iterations,
            individual_masks,
            conditioning,
            bboxes,
            positive_coords,
            negative_coords,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    ImageParam: crate::nodes::types::Image,
    ThresholdParam: crate::nodes::types::Float,
    RefineIterationsParam: crate::nodes::types::Int,
    IndividualMasksParam: crate::nodes::types::Boolean,
    ConditioningParam: crate::nodes::types::Conditioning,
    BboxesParam: crate::nodes::types::BoundingBox,
    PositiveCoordsParam: crate::nodes::types::String,
    NegativeCoordsParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SAM3_Detect<
    ModelParam,
    ImageParam,
    ThresholdParam,
    RefineIterationsParam,
    IndividualMasksParam,
    ConditioningParam,
    BboxesParam,
    PositiveCoordsParam,
    NegativeCoordsParam,
> {
    type Output = out::SAM3_DetectOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            masks: crate::nodes::types::MaskOut::from_dynamic(node_id, 0u32),
            bboxes: crate::nodes::types::BoundingBoxOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("threshold".to_string(), self.threshold.clone().into());
        output
            .insert(
                "refine_iterations".to_string(),
                self.refine_iterations.clone().into(),
            );
        output
            .insert(
                "individual_masks".to_string(),
                self.individual_masks.clone().into(),
            );
        if let Some(v) = &self.conditioning {
            output.insert("conditioning".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bboxes {
            output.insert("bboxes".to_string(), v.clone().into());
        }
        if let Some(v) = &self.positive_coords {
            output.insert("positive_coords".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_coords {
            output.insert("negative_coords".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SAM3_Detect";
    const DISPLAY_NAME: &'static str = "SAM3 Detect";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SAM3 Track Preview**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SAM3_TrackPreview<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    OpacityParam: crate::nodes::types::Float,
    FpsParam: crate::nodes::types::Float,
    ImagesParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub track_data: TrackDataParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.05
*/
    pub opacity: OpacityParam,
    /**No documentation.

**Metadata**:
  - Default: 24
  - Max: 120
  - Min: 1
  - Step: 1
*/
    pub fps: FpsParam,
    ///No documentation.
    pub images: Option<ImagesParam>,
}
impl<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    OpacityParam: crate::nodes::types::Float,
    FpsParam: crate::nodes::types::Float,
    ImagesParam: crate::nodes::types::Image,
> SAM3_TrackPreview<TrackDataParam, OpacityParam, FpsParam, ImagesParam> {
    /// Create a new node.
    pub fn new(
        track_data: TrackDataParam,
        opacity: OpacityParam,
        fps: FpsParam,
        images: Option<ImagesParam>,
    ) -> Self {
        Self {
            track_data,
            opacity,
            fps,
            images,
        }
    }
}
impl<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    OpacityParam: crate::nodes::types::Float,
    FpsParam: crate::nodes::types::Float,
    ImagesParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for SAM3_TrackPreview<TrackDataParam, OpacityParam, FpsParam, ImagesParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("track_data".to_string(), self.track_data.clone().into());
        output.insert("opacity".to_string(), self.opacity.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        if let Some(v) = &self.images {
            output.insert("images".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SAM3_TrackPreview";
    const DISPLAY_NAME: &'static str = "SAM3 Track Preview";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
impl<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    OpacityParam: crate::nodes::types::Float,
    FpsParam: crate::nodes::types::Float,
    ImagesParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode
for SAM3_TrackPreview<TrackDataParam, OpacityParam, FpsParam, ImagesParam> {}
///**SAM3 Track to Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SAM3_TrackToMask<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub track_data: TrackDataParam,
    /**Comma-separated object indices to include (e.g. '0,2,3'). Empty = all objects.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub object_indices: ObjectIndicesParam,
}
impl<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
> SAM3_TrackToMask<TrackDataParam, ObjectIndicesParam> {
    /// Create a new node.
    pub fn new(track_data: TrackDataParam, object_indices: ObjectIndicesParam) -> Self {
        Self { track_data, object_indices }
    }
}
impl<
    TrackDataParam: crate::nodes::types::Sam3TrackData,
    ObjectIndicesParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SAM3_TrackToMask<TrackDataParam, ObjectIndicesParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("track_data".to_string(), self.track_data.clone().into());
        output.insert("object_indices".to_string(), self.object_indices.clone().into());
        output
    }
    const NAME: &'static str = "SAM3_TrackToMask";
    const DISPLAY_NAME: &'static str = "SAM3 Track to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SAM3 Video Track**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SAM3_VideoTrack<
    ImagesParam: crate::nodes::types::Image,
    ModelParam: crate::nodes::types::Model,
    DetectionThresholdParam: crate::nodes::types::Float,
    MaxObjectsParam: crate::nodes::types::Int,
    DetectIntervalParam: crate::nodes::types::Int,
    InitialMaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    ConditioningParam: crate::nodes::types::Conditioning
        = crate::nodes::types::ConditioningOut,
> {
    ///Video frames as batched images
    pub images: ImagesParam,
    ///No documentation.
    pub model: ModelParam,
    /**Score threshold for text-prompted detection.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub detection_threshold: DetectionThresholdParam,
    /**Max tracked objects. Initial masks count toward this limit. 0 uses the internal cap of 64.

**Metadata**:
  - Default: 4
  - Max: 64
  - Min: 0
*/
    pub max_objects: MaxObjectsParam,
    ///Run detection every N frames (1=every frame). Higher values save compute.
    pub detect_interval: DetectIntervalParam,
    ///Mask(s) for the first frame to track (one per object)
    pub initial_mask: Option<InitialMaskParam>,
    ///Text conditioning for detecting new objects during tracking
    pub conditioning: Option<ConditioningParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ModelParam: crate::nodes::types::Model,
    DetectionThresholdParam: crate::nodes::types::Float,
    MaxObjectsParam: crate::nodes::types::Int,
    DetectIntervalParam: crate::nodes::types::Int,
    InitialMaskParam: crate::nodes::types::Mask,
    ConditioningParam: crate::nodes::types::Conditioning,
> SAM3_VideoTrack<
    ImagesParam,
    ModelParam,
    DetectionThresholdParam,
    MaxObjectsParam,
    DetectIntervalParam,
    InitialMaskParam,
    ConditioningParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        model: ModelParam,
        detection_threshold: DetectionThresholdParam,
        max_objects: MaxObjectsParam,
        detect_interval: DetectIntervalParam,
        initial_mask: Option<InitialMaskParam>,
        conditioning: Option<ConditioningParam>,
    ) -> Self {
        Self {
            images,
            model,
            detection_threshold,
            max_objects,
            detect_interval,
            initial_mask,
            conditioning,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ModelParam: crate::nodes::types::Model,
    DetectionThresholdParam: crate::nodes::types::Float,
    MaxObjectsParam: crate::nodes::types::Int,
    DetectIntervalParam: crate::nodes::types::Int,
    InitialMaskParam: crate::nodes::types::Mask,
    ConditioningParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode
for SAM3_VideoTrack<
    ImagesParam,
    ModelParam,
    DetectionThresholdParam,
    MaxObjectsParam,
    DetectIntervalParam,
    InitialMaskParam,
    ConditioningParam,
> {
    type Output = crate::nodes::types::Sam3TrackDataOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert(
                "detection_threshold".to_string(),
                self.detection_threshold.clone().into(),
            );
        output.insert("max_objects".to_string(), self.max_objects.clone().into());
        output
            .insert("detect_interval".to_string(), self.detect_interval.clone().into());
        if let Some(v) = &self.initial_mask {
            output.insert("initial_mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.conditioning {
            output.insert("conditioning".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SAM3_VideoTrack";
    const DISPLAY_NAME: &'static str = "SAM3 Video Track";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SDPose Draw Keypoints**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SDPoseDrawKeypoints<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    DrawBodyParam: crate::nodes::types::Boolean,
    DrawHandsParam: crate::nodes::types::Boolean,
    DrawFaceParam: crate::nodes::types::Boolean,
    DrawFeetParam: crate::nodes::types::Boolean,
    StickWidthParam: crate::nodes::types::Int,
    FacePointSizeParam: crate::nodes::types::Int,
    ScoreThresholdParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub keypoints: KeypointsParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub draw_body: DrawBodyParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub draw_hands: DrawHandsParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub draw_face: DrawFaceParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub draw_feet: DrawFeetParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub stick_width: StickWidthParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 10
  - Min: 1
  - Step: 1
*/
    pub face_point_size: FacePointSizeParam,
    /**No documentation.

**Metadata**:
  - Default: 0.3
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub score_threshold: ScoreThresholdParam,
}
impl<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    DrawBodyParam: crate::nodes::types::Boolean,
    DrawHandsParam: crate::nodes::types::Boolean,
    DrawFaceParam: crate::nodes::types::Boolean,
    DrawFeetParam: crate::nodes::types::Boolean,
    StickWidthParam: crate::nodes::types::Int,
    FacePointSizeParam: crate::nodes::types::Int,
    ScoreThresholdParam: crate::nodes::types::Float,
> SDPoseDrawKeypoints<
    KeypointsParam,
    DrawBodyParam,
    DrawHandsParam,
    DrawFaceParam,
    DrawFeetParam,
    StickWidthParam,
    FacePointSizeParam,
    ScoreThresholdParam,
> {
    /// Create a new node.
    pub fn new(
        keypoints: KeypointsParam,
        draw_body: DrawBodyParam,
        draw_hands: DrawHandsParam,
        draw_face: DrawFaceParam,
        draw_feet: DrawFeetParam,
        stick_width: StickWidthParam,
        face_point_size: FacePointSizeParam,
        score_threshold: ScoreThresholdParam,
    ) -> Self {
        Self {
            keypoints,
            draw_body,
            draw_hands,
            draw_face,
            draw_feet,
            stick_width,
            face_point_size,
            score_threshold,
        }
    }
}
impl<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    DrawBodyParam: crate::nodes::types::Boolean,
    DrawHandsParam: crate::nodes::types::Boolean,
    DrawFaceParam: crate::nodes::types::Boolean,
    DrawFeetParam: crate::nodes::types::Boolean,
    StickWidthParam: crate::nodes::types::Int,
    FacePointSizeParam: crate::nodes::types::Int,
    ScoreThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SDPoseDrawKeypoints<
    KeypointsParam,
    DrawBodyParam,
    DrawHandsParam,
    DrawFaceParam,
    DrawFeetParam,
    StickWidthParam,
    FacePointSizeParam,
    ScoreThresholdParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("keypoints".to_string(), self.keypoints.clone().into());
        output.insert("draw_body".to_string(), self.draw_body.clone().into());
        output.insert("draw_hands".to_string(), self.draw_hands.clone().into());
        output.insert("draw_face".to_string(), self.draw_face.clone().into());
        output.insert("draw_feet".to_string(), self.draw_feet.clone().into());
        output.insert("stick_width".to_string(), self.stick_width.clone().into());
        output
            .insert("face_point_size".to_string(), self.face_point_size.clone().into());
        output
            .insert("score_threshold".to_string(), self.score_threshold.clone().into());
        output
    }
    const NAME: &'static str = "SDPoseDrawKeypoints";
    const DISPLAY_NAME: &'static str = "SDPose Draw Keypoints";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SDPose Face Bounding Boxes**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SDPoseFaceBBoxes<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    ScaleParam: crate::nodes::types::Float,
    ForceSquareParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub keypoints: KeypointsParam,
    /**Multiplier for the bounding box area around each detected face.

**Metadata**:
  - Default: 1.5
  - Max: 10
  - Min: 1
  - Step: 0.1
*/
    pub scale: ScaleParam,
    /**Expand the shorter bbox axis so the crop region is always square.

**Metadata**:
  - Default: true
*/
    pub force_square: ForceSquareParam,
}
impl<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    ScaleParam: crate::nodes::types::Float,
    ForceSquareParam: crate::nodes::types::Boolean,
> SDPoseFaceBBoxes<KeypointsParam, ScaleParam, ForceSquareParam> {
    /// Create a new node.
    pub fn new(
        keypoints: KeypointsParam,
        scale: ScaleParam,
        force_square: ForceSquareParam,
    ) -> Self {
        Self {
            keypoints,
            scale,
            force_square,
        }
    }
}
impl<
    KeypointsParam: crate::nodes::types::PoseKeypoint,
    ScaleParam: crate::nodes::types::Float,
    ForceSquareParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for SDPoseFaceBBoxes<KeypointsParam, ScaleParam, ForceSquareParam> {
    type Output = crate::nodes::types::BoundingBoxOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("keypoints".to_string(), self.keypoints.clone().into());
        output.insert("scale".to_string(), self.scale.clone().into());
        output.insert("force_square".to_string(), self.force_square.clone().into());
        output
    }
    const NAME: &'static str = "SDPoseFaceBBoxes";
    const DISPLAY_NAME: &'static str = "SDPose Face Bounding Boxes";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/detection";
}
///**SDPose Keypoint Extractor**: Extract pose keypoints from images using the SDPose model: https://huggingface.co/Comfy-Org/SDPose/tree/main/checkpoints
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SDPoseKeypointExtractor<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
    BboxesParam: crate::nodes::types::BoundingBox = crate::nodes::types::BoundingBoxOut,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 16
  - Max: 10000
  - Min: 1
  - Step: 1
*/
    pub batch_size: BatchSizeParam,
    ///Optional bounding boxes for more accurate detections. Required for multi-person detection.
    pub bboxes: Option<BboxesParam>,
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
    BboxesParam: crate::nodes::types::BoundingBox,
> SDPoseKeypointExtractor<
    ModelParam,
    VaeParam,
    ImageParam,
    BatchSizeParam,
    BboxesParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        vae: VaeParam,
        image: ImageParam,
        batch_size: BatchSizeParam,
        bboxes: Option<BboxesParam>,
    ) -> Self {
        Self {
            model,
            vae,
            image,
            batch_size,
            bboxes,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
    BboxesParam: crate::nodes::types::BoundingBox,
> crate::nodes::TypedNode
for SDPoseKeypointExtractor<
    ModelParam,
    VaeParam,
    ImageParam,
    BatchSizeParam,
    BboxesParam,
> {
    type Output = crate::nodes::types::PoseKeypointOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.bboxes {
            output.insert("bboxes".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "SDPoseKeypointExtractor";
    const DISPLAY_NAME: &'static str = "SDPose Keypoint Extractor";
    const DESCRIPTION: &'static str = "Extract pose keypoints from images using the SDPose model: https://huggingface.co/Comfy-Org/SDPose/tree/main/checkpoints";
    const CATEGORY: &'static str = "image/detection";
}
