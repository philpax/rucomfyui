//!`Magnific` definitions/categories.
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
///**Magnific Image Relight**: Relight an image with lighting adjustments and optional reference-based light transfer.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MagnificImageRelightNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    LightTransferStrengthParam: crate::nodes::types::Int,
    InterpolateFromOriginalParam: crate::nodes::types::Boolean,
    ChangeBackgroundParam: crate::nodes::types::Boolean,
    PreserveDetailsParam: crate::nodes::types::Boolean,
    ReferenceImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///The image to relight.
    pub image: ImageParam,
    /**Descriptive guidance for lighting. Supports emphasis notation (1-1.4).

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Intensity of light transfer application.

**Metadata**:
  - Default: 100
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub light_transfer_strength: LightTransferStrengthParam,
    /**Restricts generation freedom to match original more closely.

**Metadata**:
  - Default: false
*/
    pub interpolate_from_original: InterpolateFromOriginalParam,
    /**Modifies background based on prompt/reference.

**Metadata**:
  - Default: true
*/
    pub change_background: ChangeBackgroundParam,
    /**Maintains texture and fine details from original.

**Metadata**:
  - Default: true
*/
    pub preserve_details: PreserveDetailsParam,
    ///Optional reference image to transfer lighting from.
    pub reference_image: Option<ReferenceImageParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    LightTransferStrengthParam: crate::nodes::types::Int,
    InterpolateFromOriginalParam: crate::nodes::types::Boolean,
    ChangeBackgroundParam: crate::nodes::types::Boolean,
    PreserveDetailsParam: crate::nodes::types::Boolean,
    ReferenceImageParam: crate::nodes::types::Image,
> MagnificImageRelightNode<
    ImageParam,
    PromptParam,
    LightTransferStrengthParam,
    InterpolateFromOriginalParam,
    ChangeBackgroundParam,
    PreserveDetailsParam,
    ReferenceImageParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        light_transfer_strength: LightTransferStrengthParam,
        interpolate_from_original: InterpolateFromOriginalParam,
        change_background: ChangeBackgroundParam,
        preserve_details: PreserveDetailsParam,
        reference_image: Option<ReferenceImageParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            light_transfer_strength,
            interpolate_from_original,
            change_background,
            preserve_details,
            reference_image,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    LightTransferStrengthParam: crate::nodes::types::Int,
    InterpolateFromOriginalParam: crate::nodes::types::Boolean,
    ChangeBackgroundParam: crate::nodes::types::Boolean,
    PreserveDetailsParam: crate::nodes::types::Boolean,
    ReferenceImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for MagnificImageRelightNode<
    ImageParam,
    PromptParam,
    LightTransferStrengthParam,
    InterpolateFromOriginalParam,
    ChangeBackgroundParam,
    PreserveDetailsParam,
    ReferenceImageParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "light_transfer_strength".to_string(),
                self.light_transfer_strength.clone().into(),
            );
        output
            .insert(
                "interpolate_from_original".to_string(),
                self.interpolate_from_original.clone().into(),
            );
        output
            .insert(
                "change_background".to_string(),
                self.change_background.clone().into(),
            );
        output
            .insert(
                "preserve_details".to_string(),
                self.preserve_details.clone().into(),
            );
        if let Some(v) = &self.reference_image {
            output.insert("reference_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MagnificImageRelightNode";
    const DISPLAY_NAME: &'static str = "Magnific Image Relight";
    const DESCRIPTION: &'static str = "Relight an image with lighting adjustments and optional reference-based light transfer.";
    const CATEGORY: &'static str = "partner/image/Magnific";
}
///**Magnific Image Skin Enhancer**: Skin enhancement for portraits with multiple processing modes.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MagnificImageSkinEnhancerNode<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
> {
    ///The portrait image to enhance.
    pub image: ImageParam,
    /**Sharpening intensity level.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub sharpen: SharpenParam,
    /**Smart grain intensity level.

**Metadata**:
  - Default: 2
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub smart_grain: SmartGrainParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
> MagnificImageSkinEnhancerNode<ImageParam, SharpenParam, SmartGrainParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        sharpen: SharpenParam,
        smart_grain: SmartGrainParam,
    ) -> Self {
        Self {
            image,
            sharpen,
            smart_grain,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MagnificImageSkinEnhancerNode<ImageParam, SharpenParam, SmartGrainParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("sharpen".to_string(), self.sharpen.clone().into());
        output.insert("smart_grain".to_string(), self.smart_grain.clone().into());
        output
    }
    const NAME: &'static str = "MagnificImageSkinEnhancerNode";
    const DISPLAY_NAME: &'static str = "Magnific Image Skin Enhancer";
    const DESCRIPTION: &'static str = "Skin enhancement for portraits with multiple processing modes.";
    const CATEGORY: &'static str = "partner/image/Magnific";
}
///**Magnific Image Style Transfer**: Transfer the style from a reference image to your input image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MagnificImageStyleTransferNode<
    ImageParam: crate::nodes::types::Image,
    ReferenceImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    StyleStrengthParam: crate::nodes::types::Int,
    StructureStrengthParam: crate::nodes::types::Int,
    FixedGenerationParam: crate::nodes::types::Boolean,
> {
    ///The image to apply style transfer to.
    pub image: ImageParam,
    ///The reference image to extract style from.
    pub reference_image: ReferenceImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Percentage of style strength.

**Metadata**:
  - Default: 100
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub style_strength: StyleStrengthParam,
    /**Maintains the structure of the original image.

**Metadata**:
  - Default: 50
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub structure_strength: StructureStrengthParam,
    /**When disabled, expect each generation to introduce a degree of randomness, leading to more diverse outcomes.

**Metadata**:
  - Default: true
*/
    pub fixed_generation: FixedGenerationParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ReferenceImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    StyleStrengthParam: crate::nodes::types::Int,
    StructureStrengthParam: crate::nodes::types::Int,
    FixedGenerationParam: crate::nodes::types::Boolean,
> MagnificImageStyleTransferNode<
    ImageParam,
    ReferenceImageParam,
    PromptParam,
    StyleStrengthParam,
    StructureStrengthParam,
    FixedGenerationParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        reference_image: ReferenceImageParam,
        prompt: PromptParam,
        style_strength: StyleStrengthParam,
        structure_strength: StructureStrengthParam,
        fixed_generation: FixedGenerationParam,
    ) -> Self {
        Self {
            image,
            reference_image,
            prompt,
            style_strength,
            structure_strength,
            fixed_generation,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ReferenceImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    StyleStrengthParam: crate::nodes::types::Int,
    StructureStrengthParam: crate::nodes::types::Int,
    FixedGenerationParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MagnificImageStyleTransferNode<
    ImageParam,
    ReferenceImageParam,
    PromptParam,
    StyleStrengthParam,
    StructureStrengthParam,
    FixedGenerationParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
            .insert("reference_image".to_string(), self.reference_image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("style_strength".to_string(), self.style_strength.clone().into());
        output
            .insert(
                "structure_strength".to_string(),
                self.structure_strength.clone().into(),
            );
        output
            .insert(
                "fixed_generation".to_string(),
                self.fixed_generation.clone().into(),
            );
        output
    }
    const NAME: &'static str = "MagnificImageStyleTransferNode";
    const DISPLAY_NAME: &'static str = "Magnific Image Style Transfer";
    const DESCRIPTION: &'static str = "Transfer the style from a reference image to your input image.";
    const CATEGORY: &'static str = "partner/image/Magnific";
}
///**Magnific Image Upscale (Creative)**: Prompt‑guided enhancement, stylization, and 2x/4x/8x/16x upscaling. Maximum output: 25.3 megapixels.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MagnificImageUpscalerCreativeNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Int,
    HdrParam: crate::nodes::types::Int,
    ResemblanceParam: crate::nodes::types::Int,
    FractalityParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
*/
    pub creativity: CreativityParam,
    /**The level of definition and detail.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
*/
    pub hdr: HdrParam,
    /**The level of resemblance to the original image.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
*/
    pub resemblance: ResemblanceParam,
    /**The strength of the prompt and intricacy per square pixel.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 10
  - Min: -10
*/
    pub fractality: FractalityParam,
    /**Automatically downscale input image if output would exceed maximum pixel limit.

**Metadata**:
  - Default: false
*/
    pub auto_downscale: AutoDownscaleParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Int,
    HdrParam: crate::nodes::types::Int,
    ResemblanceParam: crate::nodes::types::Int,
    FractalityParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> MagnificImageUpscalerCreativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    HdrParam,
    ResemblanceParam,
    FractalityParam,
    AutoDownscaleParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        creativity: CreativityParam,
        hdr: HdrParam,
        resemblance: ResemblanceParam,
        fractality: FractalityParam,
        auto_downscale: AutoDownscaleParam,
    ) -> Self {
        Self {
            image,
            prompt,
            creativity,
            hdr,
            resemblance,
            fractality,
            auto_downscale,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Int,
    HdrParam: crate::nodes::types::Int,
    ResemblanceParam: crate::nodes::types::Int,
    FractalityParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MagnificImageUpscalerCreativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    HdrParam,
    ResemblanceParam,
    FractalityParam,
    AutoDownscaleParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("creativity".to_string(), self.creativity.clone().into());
        output.insert("hdr".to_string(), self.hdr.clone().into());
        output.insert("resemblance".to_string(), self.resemblance.clone().into());
        output.insert("fractality".to_string(), self.fractality.clone().into());
        output.insert("auto_downscale".to_string(), self.auto_downscale.clone().into());
        output
    }
    const NAME: &'static str = "MagnificImageUpscalerCreativeNode";
    const DISPLAY_NAME: &'static str = "Magnific Image Upscale (Creative)";
    const DESCRIPTION: &'static str = "Prompt‑guided enhancement, stylization, and 2x/4x/8x/16x upscaling. Maximum output: 25.3 megapixels.";
    const CATEGORY: &'static str = "partner/image/Magnific";
}
///**Magnific Image Upscale (Precise V2)**: High-fidelity upscaling with fine control over sharpness, grain, and detail. Maximum output: 10060×10060 pixels.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MagnificImageUpscalerPreciseV2Node<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
    UltraDetailParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Image sharpness intensity. Higher values increase edge definition and clarity.

**Metadata**:
  - Default: 7
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub sharpen: SharpenParam,
    /**Intelligent grain/texture enhancement to prevent the image from looking too smooth or artificial.

**Metadata**:
  - Default: 7
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub smart_grain: SmartGrainParam,
    /**Controls fine detail, textures, and micro-details added during upscaling.

**Metadata**:
  - Default: 30
  - Display: slider
  - Max: 100
  - Min: 0
*/
    pub ultra_detail: UltraDetailParam,
    /**Automatically downscale input image if output would exceed maximum resolution.

**Metadata**:
  - Default: false
*/
    pub auto_downscale: AutoDownscaleParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
    UltraDetailParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> MagnificImageUpscalerPreciseV2Node<
    ImageParam,
    SharpenParam,
    SmartGrainParam,
    UltraDetailParam,
    AutoDownscaleParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        sharpen: SharpenParam,
        smart_grain: SmartGrainParam,
        ultra_detail: UltraDetailParam,
        auto_downscale: AutoDownscaleParam,
    ) -> Self {
        Self {
            image,
            sharpen,
            smart_grain,
            ultra_detail,
            auto_downscale,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenParam: crate::nodes::types::Int,
    SmartGrainParam: crate::nodes::types::Int,
    UltraDetailParam: crate::nodes::types::Int,
    AutoDownscaleParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for MagnificImageUpscalerPreciseV2Node<
    ImageParam,
    SharpenParam,
    SmartGrainParam,
    UltraDetailParam,
    AutoDownscaleParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("sharpen".to_string(), self.sharpen.clone().into());
        output.insert("smart_grain".to_string(), self.smart_grain.clone().into());
        output.insert("ultra_detail".to_string(), self.ultra_detail.clone().into());
        output.insert("auto_downscale".to_string(), self.auto_downscale.clone().into());
        output
    }
    const NAME: &'static str = "MagnificImageUpscalerPreciseV2Node";
    const DISPLAY_NAME: &'static str = "Magnific Image Upscale (Precise V2)";
    const DESCRIPTION: &'static str = "High-fidelity upscaling with fine control over sharpness, grain, and detail. Maximum output: 10060×10060 pixels.";
    const CATEGORY: &'static str = "partner/image/Magnific";
}
