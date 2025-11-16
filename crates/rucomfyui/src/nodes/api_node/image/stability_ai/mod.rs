//!`Stability AI` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Stability AI Stable Diffusion 3.5 Image**: Generates images synchronously based on prompt and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityStableImageSD_3_5Node<
    PromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    ImageDenoiseParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**What you wish to see in the output image. A strong, descriptive prompt that clearly defines elements, colors, and subjects will lead to better results.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**How strictly the diffusion process adheres to the prompt text (higher values keep your image closer to your prompt)

**Metadata**:
  - Default: 4
  - Max: 10
  - Min: 1
  - Step: 0.1
*/
    pub cfg_scale: CfgScaleParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    /**Keywords of what you do not wish to see in the output image. This is an advanced feature.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Denoise of input image; 0.0 yields image identical to input, 1.0 is as if no image was provided at all.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub image_denoise: Option<ImageDenoiseParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    NegativePromptParam: crate::nodes::types::String,
    ImageDenoiseParam: crate::nodes::types::Float,
> StabilityStableImageSD_3_5Node<
    PromptParam,
    CfgScaleParam,
    SeedParam,
    ImageParam,
    NegativePromptParam,
    ImageDenoiseParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        cfg_scale: CfgScaleParam,
        seed: SeedParam,
        image: Option<ImageParam>,
        negative_prompt: Option<NegativePromptParam>,
        image_denoise: Option<ImageDenoiseParam>,
    ) -> Self {
        Self {
            prompt,
            cfg_scale,
            seed,
            image,
            negative_prompt,
            image_denoise,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    CfgScaleParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    NegativePromptParam: crate::nodes::types::String,
    ImageDenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for StabilityStableImageSD_3_5Node<
    PromptParam,
    CfgScaleParam,
    SeedParam,
    ImageParam,
    NegativePromptParam,
    ImageDenoiseParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("cfg_scale".to_string(), self.cfg_scale.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_denoise {
            output.insert("image_denoise".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityStableImageSD_3_5Node";
    const DISPLAY_NAME: &'static str = "Stability AI Stable Diffusion 3.5 Image";
    const DESCRIPTION: &'static str = "Generates images synchronously based on prompt and resolution.";
    const CATEGORY: &'static str = "api node/image/Stability AI";
}
///**Stability AI Stable Image Ultra**: Generates images synchronously based on prompt and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityStableImageUltraNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    ImageDenoiseParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**What you wish to see in the output image. A strong, descriptive prompt that clearly defineselements, colors, and subjects will lead to better results. To control the weight of a given word use the format `(word:weight)`,where `word` is the word you'd like to control the weight of and `weight`is a value between 0 and 1. For example: `The sky was a crisp (blue:0.3) and (green:0.8)`would convey a sky that was blue and green, but more green than blue.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    /**A blurb of text describing what you do not wish to see in the output image. This is an advanced feature.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**Denoise of input image; 0.0 yields image identical to input, 1.0 is as if no image was provided at all.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub image_denoise: Option<ImageDenoiseParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    NegativePromptParam: crate::nodes::types::String,
    ImageDenoiseParam: crate::nodes::types::Float,
> StabilityStableImageUltraNode<
    PromptParam,
    SeedParam,
    ImageParam,
    NegativePromptParam,
    ImageDenoiseParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        image: Option<ImageParam>,
        negative_prompt: Option<NegativePromptParam>,
        image_denoise: Option<ImageDenoiseParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            image,
            negative_prompt,
            image_denoise,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    NegativePromptParam: crate::nodes::types::String,
    ImageDenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for StabilityStableImageUltraNode<
    PromptParam,
    SeedParam,
    ImageParam,
    NegativePromptParam,
    ImageDenoiseParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_denoise {
            output.insert("image_denoise".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityStableImageUltraNode";
    const DISPLAY_NAME: &'static str = "Stability AI Stable Image Ultra";
    const DESCRIPTION: &'static str = "Generates images synchronously based on prompt and resolution.";
    const CATEGORY: &'static str = "api node/image/Stability AI";
}
///**Stability AI Upscale Conservative**: Upscale image with minimal alterations to 4K resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityUpscaleConservativeNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**What you wish to see in the output image. A strong, descriptive prompt that clearly defines elements, colors, and subjects will lead to better results.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Controls the likelihood of creating additional details not heavily conditioned by the init image.

**Metadata**:
  - Default: 0.35
  - Max: 0.5
  - Min: 0.2
  - Step: 0.01
*/
    pub creativity: CreativityParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Keywords of what you do not wish to see in the output image. This is an advanced feature.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> StabilityUpscaleConservativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    SeedParam,
    NegativePromptParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        creativity: CreativityParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            creativity,
            seed,
            negative_prompt,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for StabilityUpscaleConservativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    SeedParam,
    NegativePromptParam,
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
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityUpscaleConservativeNode";
    const DISPLAY_NAME: &'static str = "Stability AI Upscale Conservative";
    const DESCRIPTION: &'static str = "Upscale image with minimal alterations to 4K resolution.";
    const CATEGORY: &'static str = "api node/image/Stability AI";
}
///**Stability AI Upscale Creative**: Upscale image with minimal alterations to 4K resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityUpscaleCreativeNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**What you wish to see in the output image. A strong, descriptive prompt that clearly defines elements, colors, and subjects will lead to better results.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Controls the likelihood of creating additional details not heavily conditioned by the init image.

**Metadata**:
  - Default: 0.3
  - Max: 0.5
  - Min: 0.1
  - Step: 0.01
*/
    pub creativity: CreativityParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Keywords of what you do not wish to see in the output image. This is an advanced feature.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> StabilityUpscaleCreativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    SeedParam,
    NegativePromptParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        creativity: CreativityParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            creativity,
            seed,
            negative_prompt,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    CreativityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for StabilityUpscaleCreativeNode<
    ImageParam,
    PromptParam,
    CreativityParam,
    SeedParam,
    NegativePromptParam,
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
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityUpscaleCreativeNode";
    const DISPLAY_NAME: &'static str = "Stability AI Upscale Creative";
    const DESCRIPTION: &'static str = "Upscale image with minimal alterations to 4K resolution.";
    const CATEGORY: &'static str = "api node/image/Stability AI";
}
///**Stability AI Upscale Fast**: Quickly upscales an image via Stability API call to 4x its original size; intended for upscaling low-quality/compressed images.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityUpscaleFastNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> StabilityUpscaleFastNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for StabilityUpscaleFastNode<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "StabilityUpscaleFastNode";
    const DISPLAY_NAME: &'static str = "Stability AI Upscale Fast";
    const DESCRIPTION: &'static str = "Quickly upscales an image via Stability API call to 4x its original size; intended for upscaling low-quality/compressed images.";
    const CATEGORY: &'static str = "api node/image/Stability AI";
}
