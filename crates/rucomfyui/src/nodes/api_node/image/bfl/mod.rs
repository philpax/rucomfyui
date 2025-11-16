//!`BFL` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Flux.1 Kontext \[max\] Image**: Edits images using Flux.1 Kontext \[max\] via api based on prompt and aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxKontextMaxImageNode<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Prompt for the image generation - specify what and how to edit.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Aspect ratio of image; must be between 1:4 and 4:1.

**Metadata**:
  - Multiline: false
  - Default: 16:9
*/
    pub aspect_ratio: AspectRatioParam,
    /**Guidance strength for the image generation process

**Metadata**:
  - Default: 3
  - Max: 99
  - Min: 0.1
  - Step: 0.1
*/
    pub guidance: GuidanceParam,
    /**Number of steps for the image generation process

**Metadata**:
  - Default: 50
  - Max: 150
  - Min: 1
*/
    pub steps: StepsParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 1234
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**Whether to perform upsampling on the prompt. If active, automatically modifies the prompt for more creative generation, but results are nondeterministic (same seed will not produce exactly the same result).

**Metadata**:
  - Default: false
*/
    pub prompt_upsampling: PromptUpsamplingParam,
    ///No documentation.
    pub input_image: Option<InputImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image,
> FluxKontextMaxImageNode<
    PromptParam,
    AspectRatioParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
    PromptUpsamplingParam,
    InputImageParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        aspect_ratio: AspectRatioParam,
        guidance: GuidanceParam,
        steps: StepsParam,
        seed: SeedParam,
        prompt_upsampling: PromptUpsamplingParam,
        input_image: Option<InputImageParam>,
    ) -> Self {
        Self {
            prompt,
            aspect_ratio,
            guidance,
            steps,
            seed,
            prompt_upsampling,
            input_image,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for FluxKontextMaxImageNode<
    PromptParam,
    AspectRatioParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
    PromptUpsamplingParam,
    InputImageParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("aspect_ratio".to_string(), self.aspect_ratio.clone().into());
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "prompt_upsampling".to_string(),
                self.prompt_upsampling.clone().into(),
            );
        if let Some(v) = &self.input_image {
            output.insert("input_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "FluxKontextMaxImageNode";
    const DISPLAY_NAME: &'static str = "Flux.1 Kontext [max] Image";
    const DESCRIPTION: &'static str = "Edits images using Flux.1 Kontext [max] via api based on prompt and aspect ratio.";
    const CATEGORY: &'static str = "api node/image/BFL";
}
///**Flux.1 Kontext \[pro\] Image**: Edits images using Flux.1 Kontext \[pro\] via api based on prompt and aspect ratio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxKontextProImageNode<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**Prompt for the image generation - specify what and how to edit.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Aspect ratio of image; must be between 1:4 and 4:1.

**Metadata**:
  - Multiline: false
  - Default: 16:9
*/
    pub aspect_ratio: AspectRatioParam,
    /**Guidance strength for the image generation process

**Metadata**:
  - Default: 3
  - Max: 99
  - Min: 0.1
  - Step: 0.1
*/
    pub guidance: GuidanceParam,
    /**Number of steps for the image generation process

**Metadata**:
  - Default: 50
  - Max: 150
  - Min: 1
*/
    pub steps: StepsParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 1234
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**Whether to perform upsampling on the prompt. If active, automatically modifies the prompt for more creative generation, but results are nondeterministic (same seed will not produce exactly the same result).

**Metadata**:
  - Default: false
*/
    pub prompt_upsampling: PromptUpsamplingParam,
    ///No documentation.
    pub input_image: Option<InputImageParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image,
> FluxKontextProImageNode<
    PromptParam,
    AspectRatioParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
    PromptUpsamplingParam,
    InputImageParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        aspect_ratio: AspectRatioParam,
        guidance: GuidanceParam,
        steps: StepsParam,
        seed: SeedParam,
        prompt_upsampling: PromptUpsamplingParam,
        input_image: Option<InputImageParam>,
    ) -> Self {
        Self {
            prompt,
            aspect_ratio,
            guidance,
            steps,
            seed,
            prompt_upsampling,
            input_image,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    AspectRatioParam: crate::nodes::types::String,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    InputImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for FluxKontextProImageNode<
    PromptParam,
    AspectRatioParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
    PromptUpsamplingParam,
    InputImageParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("aspect_ratio".to_string(), self.aspect_ratio.clone().into());
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "prompt_upsampling".to_string(),
                self.prompt_upsampling.clone().into(),
            );
        if let Some(v) = &self.input_image {
            output.insert("input_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "FluxKontextProImageNode";
    const DISPLAY_NAME: &'static str = "Flux.1 Kontext [pro] Image";
    const DESCRIPTION: &'static str = "Edits images using Flux.1 Kontext [pro] via api based on prompt and aspect ratio.";
    const CATEGORY: &'static str = "api node/image/BFL";
}
///**Flux.1 Expand Image**: Outpaints image based on prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxProExpandNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    TopParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    LeftParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Whether to perform upsampling on the prompt. If active, automatically modifies the prompt for more creative generation, but results are nondeterministic (same seed will not produce exactly the same result).

**Metadata**:
  - Default: false
*/
    pub prompt_upsampling: PromptUpsamplingParam,
    /**Number of pixels to expand at the top of the image

**Metadata**:
  - Default: 0
  - Max: 2048
  - Min: 0
*/
    pub top: TopParam,
    /**Number of pixels to expand at the bottom of the image

**Metadata**:
  - Default: 0
  - Max: 2048
  - Min: 0
*/
    pub bottom: BottomParam,
    /**Number of pixels to expand at the left of the image

**Metadata**:
  - Default: 0
  - Max: 2048
  - Min: 0
*/
    pub left: LeftParam,
    /**Number of pixels to expand at the right of the image

**Metadata**:
  - Default: 0
  - Max: 2048
  - Min: 0
*/
    pub right: RightParam,
    /**Guidance strength for the image generation process

**Metadata**:
  - Default: 60
  - Max: 100
  - Min: 1.5
*/
    pub guidance: GuidanceParam,
    /**Number of steps for the image generation process

**Metadata**:
  - Default: 50
  - Max: 50
  - Min: 15
*/
    pub steps: StepsParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    TopParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    LeftParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> FluxProExpandNode<
    ImageParam,
    PromptParam,
    PromptUpsamplingParam,
    TopParam,
    BottomParam,
    LeftParam,
    RightParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        prompt_upsampling: PromptUpsamplingParam,
        top: TopParam,
        bottom: BottomParam,
        left: LeftParam,
        right: RightParam,
        guidance: GuidanceParam,
        steps: StepsParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            prompt,
            prompt_upsampling,
            top,
            bottom,
            left,
            right,
            guidance,
            steps,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    TopParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
    LeftParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for FluxProExpandNode<
    ImageParam,
    PromptParam,
    PromptUpsamplingParam,
    TopParam,
    BottomParam,
    LeftParam,
    RightParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
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
                "prompt_upsampling".to_string(),
                self.prompt_upsampling.clone().into(),
            );
        output.insert("top".to_string(), self.top.clone().into());
        output.insert("bottom".to_string(), self.bottom.clone().into());
        output.insert("left".to_string(), self.left.clone().into());
        output.insert("right".to_string(), self.right.clone().into());
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "FluxProExpandNode";
    const DISPLAY_NAME: &'static str = "Flux.1 Expand Image";
    const DESCRIPTION: &'static str = "Outpaints image based on prompt.";
    const CATEGORY: &'static str = "api node/image/BFL";
}
///**Flux.1 Fill Image**: Inpaints image based on mask and prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxProFillNode<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub mask: MaskParam,
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Whether to perform upsampling on the prompt. If active, automatically modifies the prompt for more creative generation, but results are nondeterministic (same seed will not produce exactly the same result).

**Metadata**:
  - Default: false
*/
    pub prompt_upsampling: PromptUpsamplingParam,
    /**Guidance strength for the image generation process

**Metadata**:
  - Default: 60
  - Max: 100
  - Min: 1.5
*/
    pub guidance: GuidanceParam,
    /**Number of steps for the image generation process

**Metadata**:
  - Default: 50
  - Max: 50
  - Min: 15
*/
    pub steps: StepsParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> FluxProFillNode<
    ImageParam,
    MaskParam,
    PromptParam,
    PromptUpsamplingParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        mask: MaskParam,
        prompt: PromptParam,
        prompt_upsampling: PromptUpsamplingParam,
        guidance: GuidanceParam,
        steps: StepsParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            mask,
            prompt,
            prompt_upsampling,
            guidance,
            steps,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    GuidanceParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for FluxProFillNode<
    ImageParam,
    MaskParam,
    PromptParam,
    PromptUpsamplingParam,
    GuidanceParam,
    StepsParam,
    SeedParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "prompt_upsampling".to_string(),
                self.prompt_upsampling.clone().into(),
            );
        output.insert("guidance".to_string(), self.guidance.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "FluxProFillNode";
    const DISPLAY_NAME: &'static str = "Flux.1 Fill Image";
    const DESCRIPTION: &'static str = "Inpaints image based on mask and prompt.";
    const CATEGORY: &'static str = "api node/image/BFL";
}
///**Flux 1.1 \[pro\] Ultra Image**: Generates images using Flux Pro 1.1 Ultra via api based on prompt and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FluxProUltraImageNode<
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::String,
    RawParam: crate::nodes::types::Boolean,
    ImagePromptParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImagePromptStrengthParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**Prompt for the image generation

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Whether to perform upsampling on the prompt. If active, automatically modifies the prompt for more creative generation, but results are nondeterministic (same seed will not produce exactly the same result).

**Metadata**:
  - Default: false
*/
    pub prompt_upsampling: PromptUpsamplingParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**Aspect ratio of image; must be between 1:4 and 4:1.

**Metadata**:
  - Multiline: false
  - Default: 16:9
*/
    pub aspect_ratio: AspectRatioParam,
    /**When True, generate less processed, more natural-looking images.

**Metadata**:
  - Default: false
*/
    pub raw: RawParam,
    ///No documentation.
    pub image_prompt: Option<ImagePromptParam>,
    /**Blend between the prompt and the image prompt.

**Metadata**:
  - Default: 0.1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub image_prompt_strength: Option<ImagePromptStrengthParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::String,
    RawParam: crate::nodes::types::Boolean,
    ImagePromptParam: crate::nodes::types::Image,
    ImagePromptStrengthParam: crate::nodes::types::Float,
> FluxProUltraImageNode<
    PromptParam,
    PromptUpsamplingParam,
    SeedParam,
    AspectRatioParam,
    RawParam,
    ImagePromptParam,
    ImagePromptStrengthParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        prompt_upsampling: PromptUpsamplingParam,
        seed: SeedParam,
        aspect_ratio: AspectRatioParam,
        raw: RawParam,
        image_prompt: Option<ImagePromptParam>,
        image_prompt_strength: Option<ImagePromptStrengthParam>,
    ) -> Self {
        Self {
            prompt,
            prompt_upsampling,
            seed,
            aspect_ratio,
            raw,
            image_prompt,
            image_prompt_strength,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    PromptUpsamplingParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::String,
    RawParam: crate::nodes::types::Boolean,
    ImagePromptParam: crate::nodes::types::Image,
    ImagePromptStrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for FluxProUltraImageNode<
    PromptParam,
    PromptUpsamplingParam,
    SeedParam,
    AspectRatioParam,
    RawParam,
    ImagePromptParam,
    ImagePromptStrengthParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "prompt_upsampling".to_string(),
                self.prompt_upsampling.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("aspect_ratio".to_string(), self.aspect_ratio.clone().into());
        output.insert("raw".to_string(), self.raw.clone().into());
        if let Some(v) = &self.image_prompt {
            output.insert("image_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_prompt_strength {
            output.insert("image_prompt_strength".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "FluxProUltraImageNode";
    const DISPLAY_NAME: &'static str = "Flux 1.1 [pro] Ultra Image";
    const DESCRIPTION: &'static str = "Generates images using Flux Pro 1.1 Ultra via api based on prompt and resolution.";
    const CATEGORY: &'static str = "api node/image/BFL";
}
