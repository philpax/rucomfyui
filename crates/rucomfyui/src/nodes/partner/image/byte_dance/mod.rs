//!`ByteDance` definitions/categories.
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
    ///Output for [`ByteDanceCreateImageAsset`](super::ByteDanceCreateImageAsset).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ByteDanceCreateImageAssetOutput {
        ///No documentation.
        pub asset_id: crate::nodes::types::StringOut,
        ///No documentation.
        pub group_id: crate::nodes::types::StringOut,
    }
}
///**ByteDance Create Image Asset**: Create a Seedance 2.0 personal image asset. Uploads the input image and registers it in the given asset group. If group_id is empty, runs a real-person H5 authentication flow to create a new group before adding the asset.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceCreateImageAsset<
    ImageParam: crate::nodes::types::Image,
    GroupIdParam: crate::nodes::types::String,
> {
    ///Image to register as a personal asset.
    pub image: ImageParam,
    /**Reuse an existing Seedance asset group ID to skip repeated human verification for the same person. Leave empty to run real-person authentication in the browser and create a new group.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub group_id: GroupIdParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    GroupIdParam: crate::nodes::types::String,
> ByteDanceCreateImageAsset<ImageParam, GroupIdParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, group_id: GroupIdParam) -> Self {
        Self { image, group_id }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    GroupIdParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ByteDanceCreateImageAsset<ImageParam, GroupIdParam> {
    type Output = out::ByteDanceCreateImageAssetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            asset_id: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            group_id: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("group_id".to_string(), self.group_id.clone().into());
        output
    }
    const NAME: &'static str = "ByteDanceCreateImageAsset";
    const DISPLAY_NAME: &'static str = "ByteDance Create Image Asset";
    const DESCRIPTION: &'static str = "Create a Seedance 2.0 personal image asset. Uploads the input image and registers it in the given asset group. If group_id is empty, runs a real-person H5 authentication flow to create a new group before adding the asset.";
    const CATEGORY: &'static str = "partner/image/ByteDance";
}
///**ByteDance Image**: Generate images using ByteDance models via api based on prompt
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceImageNode<
    PromptParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    GuidanceScaleParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**The text prompt used to generate the image

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Custom width for image. Value is working only if `size_preset` is set to `Custom`

**Metadata**:
  - Default: 1024
  - Max: 2048
  - Min: 512
  - Step: 64
*/
    pub width: WidthParam,
    /**Custom height for image. Value is working only if `size_preset` is set to `Custom`

**Metadata**:
  - Default: 1024
  - Max: 2048
  - Min: 512
  - Step: 64
*/
    pub height: HeightParam,
    /**Seed to use for generation

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Higher value makes the image follow the prompt more closely

**Metadata**:
  - Default: 2.5
  - Display: number
  - Max: 10
  - Min: 1
  - Step: 0.01
*/
    pub guidance_scale: Option<GuidanceScaleParam>,
    /**Whether to add an "AI generated" watermark to the image

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    GuidanceScaleParam: crate::nodes::types::Float,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceImageNode<
    PromptParam,
    WidthParam,
    HeightParam,
    SeedParam,
    GuidanceScaleParam,
    WatermarkParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        width: WidthParam,
        height: HeightParam,
        seed: Option<SeedParam>,
        guidance_scale: Option<GuidanceScaleParam>,
        watermark: Option<WatermarkParam>,
    ) -> Self {
        Self {
            prompt,
            width,
            height,
            seed,
            guidance_scale,
            watermark,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    GuidanceScaleParam: crate::nodes::types::Float,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceImageNode<
    PromptParam,
    WidthParam,
    HeightParam,
    SeedParam,
    GuidanceScaleParam,
    WatermarkParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.guidance_scale {
            output.insert("guidance_scale".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceImageNode";
    const DISPLAY_NAME: &'static str = "ByteDance Image";
    const DESCRIPTION: &'static str = "Generate images using ByteDance models via api based on prompt";
    const CATEGORY: &'static str = "partner/image/ByteDance";
}
///**ByteDance Seedream 4.5 & 5.0**: Unified text-to-image generation and precise single-sentence editing at up to 4K resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceSeedreamNode<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    WidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    HeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    MaxImagesParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    WatermarkParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FailOnPartialParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**Text prompt for creating or editing an image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Input image(s) for image-to-image generation. Reference image(s) for single or multi-reference generation.
    pub image: Option<ImageParam>,
    /**Custom width for image. Value is working only if `size_preset` is set to `Custom`

**Metadata**:
  - Default: 2048
  - Max: 6240
  - Min: 1024
  - Step: 2
*/
    pub width: Option<WidthParam>,
    /**Custom height for image. Value is working only if `size_preset` is set to `Custom`

**Metadata**:
  - Default: 2048
  - Max: 4992
  - Min: 1024
  - Step: 2
*/
    pub height: Option<HeightParam>,
    /**Maximum number of images to generate when sequential_image_generation='auto'. Total images (input + generated) cannot exceed 15.

**Metadata**:
  - Default: 1
  - Display: number
  - Max: 15
  - Min: 1
  - Step: 1
*/
    pub max_images: Option<MaxImagesParam>,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Whether to add an "AI generated" watermark to the image.

**Metadata**:
  - Default: false
*/
    pub watermark: Option<WatermarkParam>,
    /**If enabled, abort execution if any requested images are missing or return an error.

**Metadata**:
  - Default: true
*/
    pub fail_on_partial: Option<FailOnPartialParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    MaxImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
    FailOnPartialParam: crate::nodes::types::Boolean,
> ByteDanceSeedreamNode<
    PromptParam,
    ImageParam,
    WidthParam,
    HeightParam,
    MaxImagesParam,
    SeedParam,
    WatermarkParam,
    FailOnPartialParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        image: Option<ImageParam>,
        width: Option<WidthParam>,
        height: Option<HeightParam>,
        max_images: Option<MaxImagesParam>,
        seed: Option<SeedParam>,
        watermark: Option<WatermarkParam>,
        fail_on_partial: Option<FailOnPartialParam>,
    ) -> Self {
        Self {
            prompt,
            image,
            width,
            height,
            max_images,
            seed,
            watermark,
            fail_on_partial,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    ImageParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    MaxImagesParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
    FailOnPartialParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceSeedreamNode<
    PromptParam,
    ImageParam,
    WidthParam,
    HeightParam,
    MaxImagesParam,
    SeedParam,
    WatermarkParam,
    FailOnPartialParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.width {
            output.insert("width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.height {
            output.insert("height".to_string(), v.clone().into());
        }
        if let Some(v) = &self.max_images {
            output.insert("max_images".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.watermark {
            output.insert("watermark".to_string(), v.clone().into());
        }
        if let Some(v) = &self.fail_on_partial {
            output.insert("fail_on_partial".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceSeedreamNode";
    const DISPLAY_NAME: &'static str = "ByteDance Seedream 4.5 & 5.0";
    const DESCRIPTION: &'static str = "Unified text-to-image generation and precise single-sentence editing at up to 4K resolution.";
    const CATEGORY: &'static str = "partner/image/ByteDance";
}
///**ByteDance Seedream 4.5 & 5.0**: Unified text-to-image generation and precise single-sentence editing at up to 4K resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceSeedreamNodeV2<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> {
    /**Text prompt for creating or editing an image.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed to use for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Whether to add an "AI generated" watermark to the image.

**Metadata**:
  - Default: false
*/
    pub watermark: WatermarkParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> ByteDanceSeedreamNodeV2<PromptParam, SeedParam, WatermarkParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam, watermark: WatermarkParam) -> Self {
        Self { prompt, seed, watermark }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    WatermarkParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ByteDanceSeedreamNodeV2<PromptParam, SeedParam, WatermarkParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("watermark".to_string(), self.watermark.clone().into());
        output
    }
    const NAME: &'static str = "ByteDanceSeedreamNodeV2";
    const DISPLAY_NAME: &'static str = "ByteDance Seedream 4.5 & 5.0";
    const DESCRIPTION: &'static str = "Unified text-to-image generation and precise single-sentence editing at up to 4K resolution.";
    const CATEGORY: &'static str = "partner/image/ByteDance";
}
