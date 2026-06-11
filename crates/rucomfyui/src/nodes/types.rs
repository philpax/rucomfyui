//! Definitions for all ComfyUI types.
use crate::workflow::{WorkflowInput, WorkflowNodeId};
/// Implemented for all `*Out` types.
pub trait Out: Sized {
    /// Create an output from a dynamic node. Use carefully.
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self;
    /// Convert the output to a dynamic input.
    fn into_input(self) -> WorkflowInput;
}
impl<T: Out> From<T> for WorkflowInput {
    fn from(value: T) -> Self {
        value.into_input()
    }
}
/// A generic output with no specific type. Can be used when you
/// don't care about the output type.
///
/// Wrapped by the type-safe output types.
#[derive(Clone, Copy)]
pub struct UntypedOut {
    /// The ID of the node that produced the output.
    pub node_id: WorkflowNodeId,
    /// The node's output slot.
    pub node_slot: u32,
}
impl Out for UntypedOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self { node_id, node_slot }
    }
    fn into_input(self) -> WorkflowInput {
        self.node_id.to_input_with_slot(self.node_slot)
    }
}
///A value of ComfyUI type `ACCESSORIES_OPTIONS`.
pub trait AccessoriesOptions: Clone + Into<WorkflowInput> {}
///A node output of type [`AccessoriesOptions`].
#[derive(Clone, Copy)]
pub struct AccessoriesOptionsOut(pub UntypedOut);
impl Out for AccessoriesOptionsOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl AccessoriesOptions for AccessoriesOptionsOut {}
///A value of ComfyUI type `AUDIO`.
pub trait Audio: Clone + Into<WorkflowInput> {}
///A node output of type [`Audio`].
#[derive(Clone, Copy)]
pub struct AudioOut(pub UntypedOut);
impl Out for AudioOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Audio for AudioOut {}
///A value of ComfyUI type `AUDIO_ENCODER`.
pub trait AudioEncoder: Clone + Into<WorkflowInput> {}
///A node output of type [`AudioEncoder`].
#[derive(Clone, Copy)]
pub struct AudioEncoderOut(pub UntypedOut);
impl Out for AudioEncoderOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl AudioEncoder for AudioEncoderOut {}
///A value of ComfyUI type `AUDIO_ENCODER_OUTPUT`.
pub trait AudioEncoderOutput: Clone + Into<WorkflowInput> {}
///A node output of type [`AudioEncoderOutput`].
#[derive(Clone, Copy)]
pub struct AudioEncoderOutputOut(pub UntypedOut);
impl Out for AudioEncoderOutputOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl AudioEncoderOutput for AudioEncoderOutputOut {}
///A value of ComfyUI type `BACKGROUND_REMOVAL`.
pub trait BackgroundRemoval: Clone + Into<WorkflowInput> {}
///A node output of type [`BackgroundRemoval`].
#[derive(Clone, Copy)]
pub struct BackgroundRemovalOut(pub UntypedOut);
impl Out for BackgroundRemovalOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl BackgroundRemoval for BackgroundRemovalOut {}
///A value of ComfyUI type `BOOLEAN`.
pub trait Boolean: Clone + Into<WorkflowInput> {}
///A node output of type [`Boolean`].
#[derive(Clone, Copy)]
pub struct BooleanOut(pub UntypedOut);
impl Out for BooleanOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Boolean for BooleanOut {}
///A value of ComfyUI type `BOUNDING_BOX`.
pub trait BoundingBox: Clone + Into<WorkflowInput> {}
///A node output of type [`BoundingBox`].
#[derive(Clone, Copy)]
pub struct BoundingBoxOut(pub UntypedOut);
impl Out for BoundingBoxOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl BoundingBox for BoundingBoxOut {}
///A value of ComfyUI type `CAMERA_CONTROL`.
pub trait CameraControl: Clone + Into<WorkflowInput> {}
///A node output of type [`CameraControl`].
#[derive(Clone, Copy)]
pub struct CameraControlOut(pub UntypedOut);
impl Out for CameraControlOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl CameraControl for CameraControlOut {}
///A value of ComfyUI type `CLIP`.
pub trait Clip: Clone + Into<WorkflowInput> {}
///A node output of type [`Clip`].
#[derive(Clone, Copy)]
pub struct ClipOut(pub UntypedOut);
impl Out for ClipOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Clip for ClipOut {}
///A value of ComfyUI type `CLIP_VISION`.
pub trait ClipVision: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVision`].
#[derive(Clone, Copy)]
pub struct ClipVisionOut(pub UntypedOut);
impl Out for ClipVisionOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ClipVision for ClipVisionOut {}
///A value of ComfyUI type `CLIP_VISION_OUTPUT`.
pub trait ClipVisionOutput: Clone + Into<WorkflowInput> {}
///A node output of type [`ClipVisionOutput`].
#[derive(Clone, Copy)]
pub struct ClipVisionOutputOut(pub UntypedOut);
impl Out for ClipVisionOutputOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ClipVisionOutput for ClipVisionOutputOut {}
///A value of ComfyUI type `COLOR`.
pub trait Color: Clone + Into<WorkflowInput> {}
///A node output of type [`Color`].
#[derive(Clone, Copy)]
pub struct ColorOut(pub UntypedOut);
impl Out for ColorOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Color for ColorOut {}
///A value of ComfyUI type `COMFY_MATCH_TYPE_V_3`.
pub trait ComfyMatchTypeV3: Clone + Into<WorkflowInput> {}
///A node output of type [`ComfyMatchTypeV3`].
#[derive(Clone, Copy)]
pub struct ComfyMatchTypeV3Out(pub UntypedOut);
impl Out for ComfyMatchTypeV3Out {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ComfyMatchTypeV3 for ComfyMatchTypeV3Out {}
///A value of ComfyUI type `CONDITIONING`.
pub trait Conditioning: Clone + Into<WorkflowInput> {}
///A node output of type [`Conditioning`].
#[derive(Clone, Copy)]
pub struct ConditioningOut(pub UntypedOut);
impl Out for ConditioningOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Conditioning for ConditioningOut {}
///A value of ComfyUI type `CONTROL_NET`.
pub trait ControlNet: Clone + Into<WorkflowInput> {}
///A node output of type [`ControlNet`].
#[derive(Clone, Copy)]
pub struct ControlNetOut(pub UntypedOut);
impl Out for ControlNetOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ControlNet for ControlNetOut {}
///A value of ComfyUI type `CURVE`.
pub trait Curve: Clone + Into<WorkflowInput> {}
///A node output of type [`Curve`].
#[derive(Clone, Copy)]
pub struct CurveOut(pub UntypedOut);
impl Out for CurveOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Curve for CurveOut {}
///A value of ComfyUI type `DA_3_GEOMETRY`.
pub trait Da3Geometry: Clone + Into<WorkflowInput> {}
///A node output of type [`Da3Geometry`].
#[derive(Clone, Copy)]
pub struct Da3GeometryOut(pub UntypedOut);
impl Out for Da3GeometryOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Da3Geometry for Da3GeometryOut {}
///A value of ComfyUI type `DA_3_MODEL`.
pub trait Da3Model: Clone + Into<WorkflowInput> {}
///A node output of type [`Da3Model`].
#[derive(Clone, Copy)]
pub struct Da3ModelOut(pub UntypedOut);
impl Out for Da3ModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Da3Model for Da3ModelOut {}
///A value of ComfyUI type `ELEVEN_LABS_VOICE`.
pub trait ElevenLabsVoice: Clone + Into<WorkflowInput> {}
///A node output of type [`ElevenLabsVoice`].
#[derive(Clone, Copy)]
pub struct ElevenLabsVoiceOut(pub UntypedOut);
impl Out for ElevenLabsVoiceOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ElevenLabsVoice for ElevenLabsVoiceOut {}
///A value of ComfyUI type `FACE_DETECTION_MODEL`.
pub trait FaceDetectionModel: Clone + Into<WorkflowInput> {}
///A node output of type [`FaceDetectionModel`].
#[derive(Clone, Copy)]
pub struct FaceDetectionModelOut(pub UntypedOut);
impl Out for FaceDetectionModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl FaceDetectionModel for FaceDetectionModelOut {}
///A value of ComfyUI type `FACE_LANDMARKS`.
pub trait FaceLandmarks: Clone + Into<WorkflowInput> {}
///A node output of type [`FaceLandmarks`].
#[derive(Clone, Copy)]
pub struct FaceLandmarksOut(pub UntypedOut);
impl Out for FaceLandmarksOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl FaceLandmarks for FaceLandmarksOut {}
///A value of ComfyUI type `FILE_3_D`.
pub trait File3D: Clone + Into<WorkflowInput> {}
///A node output of type [`File3D`].
#[derive(Clone, Copy)]
pub struct File3DOut(pub UntypedOut);
impl Out for File3DOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3D for File3DOut {}
///A value of ComfyUI type `FILE_3_D_FBX`.
pub trait File3DFbx: Clone + Into<WorkflowInput> {}
///A node output of type [`File3DFbx`].
#[derive(Clone, Copy)]
pub struct File3DFbxOut(pub UntypedOut);
impl Out for File3DFbxOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3DFbx for File3DFbxOut {}
///A value of ComfyUI type `FILE_3_D_GLB`.
pub trait File3DGlb: Clone + Into<WorkflowInput> {}
///A node output of type [`File3DGlb`].
#[derive(Clone, Copy)]
pub struct File3DGlbOut(pub UntypedOut);
impl Out for File3DGlbOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3DGlb for File3DGlbOut {}
///A value of ComfyUI type `FILE_3_D_OBJ`.
pub trait File3DObj: Clone + Into<WorkflowInput> {}
///A node output of type [`File3DObj`].
#[derive(Clone, Copy)]
pub struct File3DObjOut(pub UntypedOut);
impl Out for File3DObjOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3DObj for File3DObjOut {}
///A value of ComfyUI type `FILE_3_D_POINT_CLOUD_ANY`.
pub trait File3DPointCloudAny: Clone + Into<WorkflowInput> {}
///A node output of type [`File3DPointCloudAny`].
#[derive(Clone, Copy)]
pub struct File3DPointCloudAnyOut(pub UntypedOut);
impl Out for File3DPointCloudAnyOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3DPointCloudAny for File3DPointCloudAnyOut {}
///A value of ComfyUI type `FILE_3_D_SPLAT_ANY`.
pub trait File3DSplatAny: Clone + Into<WorkflowInput> {}
///A node output of type [`File3DSplatAny`].
#[derive(Clone, Copy)]
pub struct File3DSplatAnyOut(pub UntypedOut);
impl Out for File3DSplatAnyOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl File3DSplatAny for File3DSplatAnyOut {}
///A value of ComfyUI type `FLOAT`.
pub trait Float: Clone + Into<WorkflowInput> {}
///A node output of type [`Float`].
#[derive(Clone, Copy)]
pub struct FloatOut(pub UntypedOut);
impl Out for FloatOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Float for FloatOut {}
///A value of ComfyUI type `GEMINI_INPUT_FILES`.
pub trait GeminiInputFiles: Clone + Into<WorkflowInput> {}
///A node output of type [`GeminiInputFiles`].
#[derive(Clone, Copy)]
pub struct GeminiInputFilesOut(pub UntypedOut);
impl Out for GeminiInputFilesOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl GeminiInputFiles for GeminiInputFilesOut {}
///A value of ComfyUI type `GLIGEN`.
pub trait Gligen: Clone + Into<WorkflowInput> {}
///A node output of type [`Gligen`].
#[derive(Clone, Copy)]
pub struct GligenOut(pub UntypedOut);
impl Out for GligenOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Gligen for GligenOut {}
///A value of ComfyUI type `GUIDER`.
pub trait Guider: Clone + Into<WorkflowInput> {}
///A node output of type [`Guider`].
#[derive(Clone, Copy)]
pub struct GuiderOut(pub UntypedOut);
impl Out for GuiderOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Guider for GuiderOut {}
///A value of ComfyUI type `HISTOGRAM`.
pub trait Histogram: Clone + Into<WorkflowInput> {}
///A node output of type [`Histogram`].
#[derive(Clone, Copy)]
pub struct HistogramOut(pub UntypedOut);
impl Out for HistogramOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Histogram for HistogramOut {}
///A value of ComfyUI type `HOOK_KEYFRAMES`.
pub trait HookKeyframes: Clone + Into<WorkflowInput> {}
///A node output of type [`HookKeyframes`].
#[derive(Clone, Copy)]
pub struct HookKeyframesOut(pub UntypedOut);
impl Out for HookKeyframesOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl HookKeyframes for HookKeyframesOut {}
///A value of ComfyUI type `HOOKS`.
pub trait Hooks: Clone + Into<WorkflowInput> {}
///A node output of type [`Hooks`].
#[derive(Clone, Copy)]
pub struct HooksOut(pub UntypedOut);
impl Out for HooksOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Hooks for HooksOut {}
///A value of ComfyUI type `IC_LORA_PARAMETERS`.
pub trait IcLoraParameters: Clone + Into<WorkflowInput> {}
///A node output of type [`IcLoraParameters`].
#[derive(Clone, Copy)]
pub struct IcLoraParametersOut(pub UntypedOut);
impl Out for IcLoraParametersOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl IcLoraParameters for IcLoraParametersOut {}
///A value of ComfyUI type `IMAGE`.
pub trait Image: Clone + Into<WorkflowInput> {}
///A node output of type [`Image`].
#[derive(Clone, Copy)]
pub struct ImageOut(pub UntypedOut);
impl Out for ImageOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Image for ImageOut {}
///A value of ComfyUI type `INPAINT_MODEL`.
pub trait InpaintModel: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintModel`].
#[derive(Clone, Copy)]
pub struct InpaintModelOut(pub UntypedOut);
impl Out for InpaintModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl InpaintModel for InpaintModelOut {}
///A value of ComfyUI type `INPAINT_PATCH`.
pub trait InpaintPatch: Clone + Into<WorkflowInput> {}
///A node output of type [`InpaintPatch`].
#[derive(Clone, Copy)]
pub struct InpaintPatchOut(pub UntypedOut);
impl Out for InpaintPatchOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl InpaintPatch for InpaintPatchOut {}
///A value of ComfyUI type `INT`.
pub trait Int: Clone + Into<WorkflowInput> {}
///A node output of type [`Int`].
#[derive(Clone, Copy)]
pub struct IntOut(pub UntypedOut);
impl Out for IntOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Int for IntOut {}
///A value of ComfyUI type `INTERP_MODEL`.
pub trait InterpModel: Clone + Into<WorkflowInput> {}
///A node output of type [`InterpModel`].
#[derive(Clone, Copy)]
pub struct InterpModelOut(pub UntypedOut);
impl Out for InterpModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl InterpModel for InterpModelOut {}
///A value of ComfyUI type `KREA_STYLE_REF`.
pub trait KreaStyleRef: Clone + Into<WorkflowInput> {}
///A node output of type [`KreaStyleRef`].
#[derive(Clone, Copy)]
pub struct KreaStyleRefOut(pub UntypedOut);
impl Out for KreaStyleRefOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl KreaStyleRef for KreaStyleRefOut {}
///A value of ComfyUI type `LATENT`.
pub trait Latent: Clone + Into<WorkflowInput> {}
///A node output of type [`Latent`].
#[derive(Clone, Copy)]
pub struct LatentOut(pub UntypedOut);
impl Out for LatentOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Latent for LatentOut {}
///A value of ComfyUI type `LATENT_OPERATION`.
pub trait LatentOperation: Clone + Into<WorkflowInput> {}
///A node output of type [`LatentOperation`].
#[derive(Clone, Copy)]
pub struct LatentOperationOut(pub UntypedOut);
impl Out for LatentOperationOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LatentOperation for LatentOperationOut {}
///A value of ComfyUI type `LATENT_UPSCALE_MODEL`.
pub trait LatentUpscaleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`LatentUpscaleModel`].
#[derive(Clone, Copy)]
pub struct LatentUpscaleModelOut(pub UntypedOut);
impl Out for LatentUpscaleModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LatentUpscaleModel for LatentUpscaleModelOut {}
///A value of ComfyUI type `LOAD_3_D_CAMERA`.
pub trait Load3DCamera: Clone + Into<WorkflowInput> {}
///A node output of type [`Load3DCamera`].
#[derive(Clone, Copy)]
pub struct Load3DCameraOut(pub UntypedOut);
impl Out for Load3DCameraOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Load3DCamera for Load3DCameraOut {}
///A value of ComfyUI type `LOAD_3_D_MODEL_INFO`.
pub trait Load3DModelInfo: Clone + Into<WorkflowInput> {}
///A node output of type [`Load3DModelInfo`].
#[derive(Clone, Copy)]
pub struct Load3DModelInfoOut(pub UntypedOut);
impl Out for Load3DModelInfoOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Load3DModelInfo for Load3DModelInfoOut {}
///A value of ComfyUI type `LORA_MODEL`.
pub trait LoraModel: Clone + Into<WorkflowInput> {}
///A node output of type [`LoraModel`].
#[derive(Clone, Copy)]
pub struct LoraModelOut(pub UntypedOut);
impl Out for LoraModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LoraModel for LoraModelOut {}
///A value of ComfyUI type `LOSS_MAP`.
pub trait LossMap: Clone + Into<WorkflowInput> {}
///A node output of type [`LossMap`].
#[derive(Clone, Copy)]
pub struct LossMapOut(pub UntypedOut);
impl Out for LossMapOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LossMap for LossMapOut {}
///A value of ComfyUI type `LUMA_CONCEPTS`.
pub trait LumaConcepts: Clone + Into<WorkflowInput> {}
///A node output of type [`LumaConcepts`].
#[derive(Clone, Copy)]
pub struct LumaConceptsOut(pub UntypedOut);
impl Out for LumaConceptsOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LumaConcepts for LumaConceptsOut {}
///A value of ComfyUI type `LUMA_REF`.
pub trait LumaRef: Clone + Into<WorkflowInput> {}
///A node output of type [`LumaRef`].
#[derive(Clone, Copy)]
pub struct LumaRefOut(pub UntypedOut);
impl Out for LumaRefOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl LumaRef for LumaRefOut {}
///A value of ComfyUI type `MASK`.
pub trait Mask: Clone + Into<WorkflowInput> {}
///A node output of type [`Mask`].
#[derive(Clone, Copy)]
pub struct MaskOut(pub UntypedOut);
impl Out for MaskOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Mask for MaskOut {}
///A value of ComfyUI type `MESH`.
pub trait Mesh: Clone + Into<WorkflowInput> {}
///A node output of type [`Mesh`].
#[derive(Clone, Copy)]
pub struct MeshOut(pub UntypedOut);
impl Out for MeshOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Mesh for MeshOut {}
///A value of ComfyUI type `MESHY_RIGGED_TASK_ID`.
pub trait MeshyRiggedTaskId: Clone + Into<WorkflowInput> {}
///A node output of type [`MeshyRiggedTaskId`].
#[derive(Clone, Copy)]
pub struct MeshyRiggedTaskIdOut(pub UntypedOut);
impl Out for MeshyRiggedTaskIdOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl MeshyRiggedTaskId for MeshyRiggedTaskIdOut {}
///A value of ComfyUI type `MESHY_TASK_ID`.
pub trait MeshyTaskId: Clone + Into<WorkflowInput> {}
///A node output of type [`MeshyTaskId`].
#[derive(Clone, Copy)]
pub struct MeshyTaskIdOut(pub UntypedOut);
impl Out for MeshyTaskIdOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl MeshyTaskId for MeshyTaskIdOut {}
///A value of ComfyUI type `MODEL`.
pub trait Model: Clone + Into<WorkflowInput> {}
///A node output of type [`Model`].
#[derive(Clone, Copy)]
pub struct ModelOut(pub UntypedOut);
impl Out for ModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Model for ModelOut {}
///A value of ComfyUI type `MODEL_PATCH`.
pub trait ModelPatch: Clone + Into<WorkflowInput> {}
///A node output of type [`ModelPatch`].
#[derive(Clone, Copy)]
pub struct ModelPatchOut(pub UntypedOut);
impl Out for ModelPatchOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ModelPatch for ModelPatchOut {}
///A value of ComfyUI type `MODEL_TASK_ID`.
pub trait ModelTaskId: Clone + Into<WorkflowInput> {}
///A node output of type [`ModelTaskId`].
#[derive(Clone, Copy)]
pub struct ModelTaskIdOut(pub UntypedOut);
impl Out for ModelTaskIdOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl ModelTaskId for ModelTaskIdOut {}
///A value of ComfyUI type `MOGE_GEOMETRY`.
pub trait MogeGeometry: Clone + Into<WorkflowInput> {}
///A node output of type [`MogeGeometry`].
#[derive(Clone, Copy)]
pub struct MogeGeometryOut(pub UntypedOut);
impl Out for MogeGeometryOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl MogeGeometry for MogeGeometryOut {}
///A value of ComfyUI type `MOGE_MODEL`.
pub trait MogeModel: Clone + Into<WorkflowInput> {}
///A node output of type [`MogeModel`].
#[derive(Clone, Copy)]
pub struct MogeModelOut(pub UntypedOut);
impl Out for MogeModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl MogeModel for MogeModelOut {}
///A value of ComfyUI type `NOISE`.
pub trait Noise: Clone + Into<WorkflowInput> {}
///A node output of type [`Noise`].
#[derive(Clone, Copy)]
pub struct NoiseOut(pub UntypedOut);
impl Out for NoiseOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Noise for NoiseOut {}
///A value of ComfyUI type `OPEN_AI_CHAT_CONFIG`.
pub trait OpenAiChatConfig: Clone + Into<WorkflowInput> {}
///A node output of type [`OpenAiChatConfig`].
#[derive(Clone, Copy)]
pub struct OpenAiChatConfigOut(pub UntypedOut);
impl Out for OpenAiChatConfigOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl OpenAiChatConfig for OpenAiChatConfigOut {}
///A value of ComfyUI type `OPEN_AI_INPUT_FILES`.
pub trait OpenAiInputFiles: Clone + Into<WorkflowInput> {}
///A node output of type [`OpenAiInputFiles`].
#[derive(Clone, Copy)]
pub struct OpenAiInputFilesOut(pub UntypedOut);
impl Out for OpenAiInputFilesOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl OpenAiInputFiles for OpenAiInputFilesOut {}
///A value of ComfyUI type `OPTICAL_FLOW`.
pub trait OpticalFlow: Clone + Into<WorkflowInput> {}
///A node output of type [`OpticalFlow`].
#[derive(Clone, Copy)]
pub struct OpticalFlowOut(pub UntypedOut);
impl Out for OpticalFlowOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl OpticalFlow for OpticalFlowOut {}
///A value of ComfyUI type `PHOTOMAKER`.
pub trait Photomaker: Clone + Into<WorkflowInput> {}
///A node output of type [`Photomaker`].
#[derive(Clone, Copy)]
pub struct PhotomakerOut(pub UntypedOut);
impl Out for PhotomakerOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Photomaker for PhotomakerOut {}
///A value of ComfyUI type `PIXVERSE_TEMPLATE`.
pub trait PixverseTemplate: Clone + Into<WorkflowInput> {}
///A node output of type [`PixverseTemplate`].
#[derive(Clone, Copy)]
pub struct PixverseTemplateOut(pub UntypedOut);
impl Out for PixverseTemplateOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl PixverseTemplate for PixverseTemplateOut {}
///A value of ComfyUI type `POSE_KEYPOINT`.
pub trait PoseKeypoint: Clone + Into<WorkflowInput> {}
///A node output of type [`PoseKeypoint`].
#[derive(Clone, Copy)]
pub struct PoseKeypointOut(pub UntypedOut);
impl Out for PoseKeypointOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl PoseKeypoint for PoseKeypointOut {}
///A value of ComfyUI type `RECRAFT_COLOR`.
pub trait RecraftColor: Clone + Into<WorkflowInput> {}
///A node output of type [`RecraftColor`].
#[derive(Clone, Copy)]
pub struct RecraftColorOut(pub UntypedOut);
impl Out for RecraftColorOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl RecraftColor for RecraftColorOut {}
///A value of ComfyUI type `RECRAFT_CONTROLS`.
pub trait RecraftControls: Clone + Into<WorkflowInput> {}
///A node output of type [`RecraftControls`].
#[derive(Clone, Copy)]
pub struct RecraftControlsOut(pub UntypedOut);
impl Out for RecraftControlsOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl RecraftControls for RecraftControlsOut {}
///A value of ComfyUI type `RECRAFT_V_3_STYLE`.
pub trait RecraftV3Style: Clone + Into<WorkflowInput> {}
///A node output of type [`RecraftV3Style`].
#[derive(Clone, Copy)]
pub struct RecraftV3StyleOut(pub UntypedOut);
impl Out for RecraftV3StyleOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl RecraftV3Style for RecraftV3StyleOut {}
///A value of ComfyUI type `RETARGET_TASK_ID`.
pub trait RetargetTaskId: Clone + Into<WorkflowInput> {}
///A node output of type [`RetargetTaskId`].
#[derive(Clone, Copy)]
pub struct RetargetTaskIdOut(pub UntypedOut);
impl Out for RetargetTaskIdOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl RetargetTaskId for RetargetTaskIdOut {}
///A value of ComfyUI type `RIG_TASK_ID`.
pub trait RigTaskId: Clone + Into<WorkflowInput> {}
///A node output of type [`RigTaskId`].
#[derive(Clone, Copy)]
pub struct RigTaskIdOut(pub UntypedOut);
impl Out for RigTaskIdOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl RigTaskId for RigTaskIdOut {}
///A value of ComfyUI type `SAM_3_TRACK_DATA`.
pub trait Sam3TrackData: Clone + Into<WorkflowInput> {}
///A node output of type [`Sam3TrackData`].
#[derive(Clone, Copy)]
pub struct Sam3TrackDataOut(pub UntypedOut);
impl Out for Sam3TrackDataOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Sam3TrackData for Sam3TrackDataOut {}
///A value of ComfyUI type `SAMPLER`.
pub trait Sampler: Clone + Into<WorkflowInput> {}
///A node output of type [`Sampler`].
#[derive(Clone, Copy)]
pub struct SamplerOut(pub UntypedOut);
impl Out for SamplerOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Sampler for SamplerOut {}
///A value of ComfyUI type `SIGMAS`.
pub trait Sigmas: Clone + Into<WorkflowInput> {}
///A node output of type [`Sigmas`].
#[derive(Clone, Copy)]
pub struct SigmasOut(pub UntypedOut);
impl Out for SigmasOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Sigmas for SigmasOut {}
///A value of ComfyUI type `SPLAT`.
pub trait Splat: Clone + Into<WorkflowInput> {}
///A node output of type [`Splat`].
#[derive(Clone, Copy)]
pub struct SplatOut(pub UntypedOut);
impl Out for SplatOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Splat for SplatOut {}
///A value of ComfyUI type `STRING`.
pub trait String: Clone + Into<WorkflowInput> {}
///A node output of type [`String`].
#[derive(Clone, Copy)]
pub struct StringOut(pub UntypedOut);
impl Out for StringOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl String for StringOut {}
///A value of ComfyUI type `STYLE_MODEL`.
pub trait StyleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`StyleModel`].
#[derive(Clone, Copy)]
pub struct StyleModelOut(pub UntypedOut);
impl Out for StyleModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl StyleModel for StyleModelOut {}
///A value of ComfyUI type `SVG`.
pub trait Svg: Clone + Into<WorkflowInput> {}
///A node output of type [`Svg`].
#[derive(Clone, Copy)]
pub struct SvgOut(pub UntypedOut);
impl Out for SvgOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Svg for SvgOut {}
///A value of ComfyUI type `TIMESTEPS_RANGE`.
pub trait TimestepsRange: Clone + Into<WorkflowInput> {}
///A node output of type [`TimestepsRange`].
#[derive(Clone, Copy)]
pub struct TimestepsRangeOut(pub UntypedOut);
impl Out for TimestepsRangeOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl TimestepsRange for TimestepsRangeOut {}
///A value of ComfyUI type `TRACKS`.
pub trait Tracks: Clone + Into<WorkflowInput> {}
///A node output of type [`Tracks`].
#[derive(Clone, Copy)]
pub struct TracksOut(pub UntypedOut);
impl Out for TracksOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Tracks for TracksOut {}
///A value of ComfyUI type `UPSCALE_MODEL`.
pub trait UpscaleModel: Clone + Into<WorkflowInput> {}
///A node output of type [`UpscaleModel`].
#[derive(Clone, Copy)]
pub struct UpscaleModelOut(pub UntypedOut);
impl Out for UpscaleModelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl UpscaleModel for UpscaleModelOut {}
///A value of ComfyUI type `VAE`.
pub trait Vae: Clone + Into<WorkflowInput> {}
///A node output of type [`Vae`].
#[derive(Clone, Copy)]
pub struct VaeOut(pub UntypedOut);
impl Out for VaeOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Vae for VaeOut {}
///A value of ComfyUI type `VIDEO`.
pub trait Video: Clone + Into<WorkflowInput> {}
///A node output of type [`Video`].
#[derive(Clone, Copy)]
pub struct VideoOut(pub UntypedOut);
impl Out for VideoOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Video for VideoOut {}
///A value of ComfyUI type `VOXEL`.
pub trait Voxel: Clone + Into<WorkflowInput> {}
///A node output of type [`Voxel`].
#[derive(Clone, Copy)]
pub struct VoxelOut(pub UntypedOut);
impl Out for VoxelOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Voxel for VoxelOut {}
///A value of ComfyUI type `WAN_CAMERA_EMBEDDING`.
pub trait WanCameraEmbedding: Clone + Into<WorkflowInput> {}
///A node output of type [`WanCameraEmbedding`].
#[derive(Clone, Copy)]
pub struct WanCameraEmbeddingOut(pub UntypedOut);
impl Out for WanCameraEmbeddingOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl WanCameraEmbedding for WanCameraEmbeddingOut {}
///A value of ComfyUI type `WEBCAM`.
pub trait Webcam: Clone + Into<WorkflowInput> {}
///A node output of type [`Webcam`].
#[derive(Clone, Copy)]
pub struct WebcamOut(pub UntypedOut);
impl Out for WebcamOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Webcam for WebcamOut {}
///A value of ComfyUI type `WILDCARD`.
pub trait Wildcard: Clone + Into<WorkflowInput> {}
///A node output of type [`Wildcard`].
#[derive(Clone, Copy)]
pub struct WildcardOut(pub UntypedOut);
impl Out for WildcardOut {
    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
        Self(UntypedOut::from_dynamic(node_id, node_slot))
    }
    fn into_input(self) -> WorkflowInput {
        self.0.into_input()
    }
}
impl Wildcard for WildcardOut {}
impl String for std::string::String {}
impl String for &str {}
impl Float for f32 {}
impl Float for f64 {}
impl Int for u32 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for u64 {}
impl Boolean for bool {}
