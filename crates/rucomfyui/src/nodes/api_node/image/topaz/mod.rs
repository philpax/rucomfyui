//!`Topaz` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Topaz Image Enhance**: Industry-standard upscaling and image enhancement.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TopazImageEnhance<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    FaceEnhancementParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FaceEnhancementCreativityParam: crate::nodes::types::Float
        = crate::nodes::types::FloatOut,
    FaceEnhancementStrengthParam: crate::nodes::types::Float
        = crate::nodes::types::FloatOut,
    CropToFillParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    OutputWidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    OutputHeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    CreativityParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FacePreservationParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    ColorPreservationParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Optional text prompt for creative upscaling guidance.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: Option<PromptParam>,
    /**Enhance faces (if present) during processing.

**Metadata**:
  - Default: true
*/
    pub face_enhancement: Option<FaceEnhancementParam>,
    /**Set the creativity level for face enhancement.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub face_enhancement_creativity: Option<FaceEnhancementCreativityParam>,
    /**Controls how sharp enhanced faces are relative to the background.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub face_enhancement_strength: Option<FaceEnhancementStrengthParam>,
    /**By default, the image is letterboxed when the output aspect ratio differs. Enable to crop the image to fill the output dimensions.

**Metadata**:
  - Default: false
*/
    pub crop_to_fill: Option<CropToFillParam>,
    /**Zero value means to calculate automatically (usually it will be original size or output_height if specified).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 32000
  - Min: 0
  - Step: 1
*/
    pub output_width: Option<OutputWidthParam>,
    /**Zero value means to output in the same height as original or output width.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 32000
  - Min: 0
  - Step: 1
*/
    pub output_height: Option<OutputHeightParam>,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Display: slider
  - Max: 9
  - Min: 1
  - Step: 1
*/
    pub creativity: Option<CreativityParam>,
    /**Preserve subjects' facial identity.

**Metadata**:
  - Default: true
*/
    pub face_preservation: Option<FacePreservationParam>,
    /**Preserve the original colors.

**Metadata**:
  - Default: true
*/
    pub color_preservation: Option<ColorPreservationParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    FaceEnhancementParam: crate::nodes::types::Boolean,
    FaceEnhancementCreativityParam: crate::nodes::types::Float,
    FaceEnhancementStrengthParam: crate::nodes::types::Float,
    CropToFillParam: crate::nodes::types::Boolean,
    OutputWidthParam: crate::nodes::types::Int,
    OutputHeightParam: crate::nodes::types::Int,
    CreativityParam: crate::nodes::types::Int,
    FacePreservationParam: crate::nodes::types::Boolean,
    ColorPreservationParam: crate::nodes::types::Boolean,
> TopazImageEnhance<
    ImageParam,
    PromptParam,
    FaceEnhancementParam,
    FaceEnhancementCreativityParam,
    FaceEnhancementStrengthParam,
    CropToFillParam,
    OutputWidthParam,
    OutputHeightParam,
    CreativityParam,
    FacePreservationParam,
    ColorPreservationParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: Option<PromptParam>,
        face_enhancement: Option<FaceEnhancementParam>,
        face_enhancement_creativity: Option<FaceEnhancementCreativityParam>,
        face_enhancement_strength: Option<FaceEnhancementStrengthParam>,
        crop_to_fill: Option<CropToFillParam>,
        output_width: Option<OutputWidthParam>,
        output_height: Option<OutputHeightParam>,
        creativity: Option<CreativityParam>,
        face_preservation: Option<FacePreservationParam>,
        color_preservation: Option<ColorPreservationParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            face_enhancement,
            face_enhancement_creativity,
            face_enhancement_strength,
            crop_to_fill,
            output_width,
            output_height,
            creativity,
            face_preservation,
            color_preservation,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    FaceEnhancementParam: crate::nodes::types::Boolean,
    FaceEnhancementCreativityParam: crate::nodes::types::Float,
    FaceEnhancementStrengthParam: crate::nodes::types::Float,
    CropToFillParam: crate::nodes::types::Boolean,
    OutputWidthParam: crate::nodes::types::Int,
    OutputHeightParam: crate::nodes::types::Int,
    CreativityParam: crate::nodes::types::Int,
    FacePreservationParam: crate::nodes::types::Boolean,
    ColorPreservationParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TopazImageEnhance<
    ImageParam,
    PromptParam,
    FaceEnhancementParam,
    FaceEnhancementCreativityParam,
    FaceEnhancementStrengthParam,
    CropToFillParam,
    OutputWidthParam,
    OutputHeightParam,
    CreativityParam,
    FacePreservationParam,
    ColorPreservationParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.prompt {
            output.insert("prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_enhancement {
            output.insert("face_enhancement".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_enhancement_creativity {
            output.insert("face_enhancement_creativity".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_enhancement_strength {
            output.insert("face_enhancement_strength".to_string(), v.clone().into());
        }
        if let Some(v) = &self.crop_to_fill {
            output.insert("crop_to_fill".to_string(), v.clone().into());
        }
        if let Some(v) = &self.output_width {
            output.insert("output_width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.output_height {
            output.insert("output_height".to_string(), v.clone().into());
        }
        if let Some(v) = &self.creativity {
            output.insert("creativity".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_preservation {
            output.insert("face_preservation".to_string(), v.clone().into());
        }
        if let Some(v) = &self.color_preservation {
            output.insert("color_preservation".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TopazImageEnhance";
    const DISPLAY_NAME: &'static str = "Topaz Image Enhance";
    const DESCRIPTION: &'static str = "Industry-standard upscaling and image enhancement.";
    const CATEGORY: &'static str = "api node/image/Topaz";
}
