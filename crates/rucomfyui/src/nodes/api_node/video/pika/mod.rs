//!`Pika` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Pika Image to Video**: Sends an image and prompt to the Pika API v2.2 to generate a video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PikaImageToVideoNode2_2<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///The image to convert to video
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> PikaImageToVideoNode2_2<ImageParam, PromptTextParam, NegativePromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            prompt_text,
            negative_prompt,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for PikaImageToVideoNode2_2<
    ImageParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "PikaImageToVideoNode2_2";
    const DISPLAY_NAME: &'static str = "Pika Image to Video";
    const DESCRIPTION: &'static str = "Sends an image and prompt to the Pika API v2.2 to generate a video.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pika Scenes (Video Image Composition)**: Combine your images to create a video with the objects in them. Upload multiple images as ingredients and generate a high-quality video that incorporates all of them.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PikaScenesV2_2<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
    ImageIngredient1Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageIngredient2Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageIngredient3Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageIngredient4Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageIngredient5Param: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
    /**Aspect ratio (width / height)

**Metadata**:
  - Default: 1.7777777777777777
  - Max: 2.5
  - Min: 0.4
  - Step: 0.001
*/
    pub aspect_ratio: AspectRatioParam,
    ///Image that will be used as ingredient to create a video.
    pub image_ingredient_1: Option<ImageIngredient1Param>,
    ///Image that will be used as ingredient to create a video.
    pub image_ingredient_2: Option<ImageIngredient2Param>,
    ///Image that will be used as ingredient to create a video.
    pub image_ingredient_3: Option<ImageIngredient3Param>,
    ///Image that will be used as ingredient to create a video.
    pub image_ingredient_4: Option<ImageIngredient4Param>,
    ///Image that will be used as ingredient to create a video.
    pub image_ingredient_5: Option<ImageIngredient5Param>,
}
impl<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
    ImageIngredient1Param: crate::nodes::types::Image,
    ImageIngredient2Param: crate::nodes::types::Image,
    ImageIngredient3Param: crate::nodes::types::Image,
    ImageIngredient4Param: crate::nodes::types::Image,
    ImageIngredient5Param: crate::nodes::types::Image,
> PikaScenesV2_2<
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    AspectRatioParam,
    ImageIngredient1Param,
    ImageIngredient2Param,
    ImageIngredient3Param,
    ImageIngredient4Param,
    ImageIngredient5Param,
> {
    /// Create a new node.
    pub fn new(
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
        aspect_ratio: AspectRatioParam,
        image_ingredient_1: Option<ImageIngredient1Param>,
        image_ingredient_2: Option<ImageIngredient2Param>,
        image_ingredient_3: Option<ImageIngredient3Param>,
        image_ingredient_4: Option<ImageIngredient4Param>,
        image_ingredient_5: Option<ImageIngredient5Param>,
    ) -> Self {
        Self {
            prompt_text,
            negative_prompt,
            seed,
            aspect_ratio,
            image_ingredient_1,
            image_ingredient_2,
            image_ingredient_3,
            image_ingredient_4,
            image_ingredient_5,
        }
    }
}
impl<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
    ImageIngredient1Param: crate::nodes::types::Image,
    ImageIngredient2Param: crate::nodes::types::Image,
    ImageIngredient3Param: crate::nodes::types::Image,
    ImageIngredient4Param: crate::nodes::types::Image,
    ImageIngredient5Param: crate::nodes::types::Image,
> crate::nodes::TypedNode
for PikaScenesV2_2<
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    AspectRatioParam,
    ImageIngredient1Param,
    ImageIngredient2Param,
    ImageIngredient3Param,
    ImageIngredient4Param,
    ImageIngredient5Param,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("aspect_ratio".to_string(), self.aspect_ratio.clone().into());
        if let Some(v) = &self.image_ingredient_1 {
            output.insert("image_ingredient_1".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_ingredient_2 {
            output.insert("image_ingredient_2".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_ingredient_3 {
            output.insert("image_ingredient_3".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_ingredient_4 {
            output.insert("image_ingredient_4".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_ingredient_5 {
            output.insert("image_ingredient_5".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "PikaScenesV2_2";
    const DISPLAY_NAME: &'static str = "Pika Scenes (Video Image Composition)";
    const DESCRIPTION: &'static str = "Combine your images to create a video with the objects in them. Upload multiple images as ingredients and generate a high-quality video that incorporates all of them.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pika Start and End Frame to Video**: Generate a video by combining your first and last frame. Upload two images to define the start and end points, and let the AI create a smooth transition between them.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PikaStartEndFrameNode2_2<
    ImageStartParam: crate::nodes::types::Image,
    ImageEndParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///The first image to combine.
    pub image_start: ImageStartParam,
    ///The last image to combine.
    pub image_end: ImageEndParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
}
impl<
    ImageStartParam: crate::nodes::types::Image,
    ImageEndParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> PikaStartEndFrameNode2_2<
    ImageStartParam,
    ImageEndParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        image_start: ImageStartParam,
        image_end: ImageEndParam,
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image_start,
            image_end,
            prompt_text,
            negative_prompt,
            seed,
        }
    }
}
impl<
    ImageStartParam: crate::nodes::types::Image,
    ImageEndParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for PikaStartEndFrameNode2_2<
    ImageStartParam,
    ImageEndParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_start".to_string(), self.image_start.clone().into());
        output.insert("image_end".to_string(), self.image_end.clone().into());
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "PikaStartEndFrameNode2_2";
    const DISPLAY_NAME: &'static str = "Pika Start and End Frame to Video";
    const DESCRIPTION: &'static str = "Generate a video by combining your first and last frame. Upload two images to define the start and end points, and let the AI create a smooth transition between them.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pika Text to Video**: Sends a text prompt to the Pika API v2.2 to generate a video.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PikaTextToVideoNode2_2<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
    /**Aspect ratio (width / height)

**Metadata**:
  - Default: 1.7777777777777777
  - Max: 2.5
  - Min: 0.4
  - Step: 0.001
*/
    pub aspect_ratio: AspectRatioParam,
}
impl<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
> PikaTextToVideoNode2_2<
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    AspectRatioParam,
> {
    /// Create a new node.
    pub fn new(
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
        aspect_ratio: AspectRatioParam,
    ) -> Self {
        Self {
            prompt_text,
            negative_prompt,
            seed,
            aspect_ratio,
        }
    }
}
impl<
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    AspectRatioParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for PikaTextToVideoNode2_2<
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    AspectRatioParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("aspect_ratio".to_string(), self.aspect_ratio.clone().into());
        output
    }
    const NAME: &'static str = "PikaTextToVideoNode2_2";
    const DISPLAY_NAME: &'static str = "Pika Text to Video";
    const DESCRIPTION: &'static str = "Sends a text prompt to the Pika API v2.2 to generate a video.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pikadditions (Video Object Insertion)**: Add any object or image into your video. Upload a video and specify what you'd like to add to create a seamlessly integrated result.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Pikadditions<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///The video to add an image to.
    pub video: VideoParam,
    ///The image to add to the video.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
}
impl<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Pikadditions<VideoParam, ImageParam, PromptTextParam, NegativePromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        image: ImageParam,
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            video,
            image,
            prompt_text,
            negative_prompt,
            seed,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Pikadditions<
    VideoParam,
    ImageParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Pikadditions";
    const DISPLAY_NAME: &'static str = "Pikadditions (Video Object Insertion)";
    const DESCRIPTION: &'static str = "Add any object or image into your video. Upload a video and specify what you'd like to add to create a seamlessly integrated result.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pikaffects (Video Effects)**: Generate a video with a specific Pikaffect. Supported Pikaffects: Cake-ify, Crumble, Crush, Decapitate, Deflate, Dissolve, Explode, Eye-pop, Inflate, Levitate, Melt, Peel, Poke, Squish, Ta-da, Tear
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Pikaffects<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///The reference image to apply the Pikaffect to.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: PromptTextParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: NegativePromptParam,
    ///No documentation.
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> Pikaffects<ImageParam, PromptTextParam, NegativePromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt_text: PromptTextParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            image,
            prompt_text,
            negative_prompt,
            seed,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Pikaffects<ImageParam, PromptTextParam, NegativePromptParam, SeedParam> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt_text".to_string(), self.prompt_text.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "Pikaffects";
    const DISPLAY_NAME: &'static str = "Pikaffects (Video Effects)";
    const DESCRIPTION: &'static str = "Generate a video with a specific Pikaffect. Supported Pikaffects: Cake-ify, Crumble, Crush, Decapitate, Deflate, Dissolve, Explode, Eye-pop, Inflate, Levitate, Melt, Peel, Poke, Squish, Ta-da, Tear";
    const CATEGORY: &'static str = "api node/video/Pika";
}
///**Pika Swaps (Video Object Replacement)**: Swap out any object or region of your video with a new image or object. Define areas to replace either with a mask or coordinates.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Pikaswaps<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
    PromptTextParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    RegionToModifyParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///The video to swap an object in.
    pub video: VideoParam,
    ///The image used to replace the masked object in the video.
    pub image: Option<ImageParam>,
    ///Use the mask to define areas in the video to replace.
    pub mask: Option<MaskParam>,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt_text: Option<PromptTextParam>,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///No documentation.
    pub seed: Option<SeedParam>,
    /**Plaintext description of the object / region to modify.

**Metadata**:
  - Multiline: true
*/
    pub region_to_modify: Option<RegionToModifyParam>,
}
impl<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    RegionToModifyParam: crate::nodes::types::String,
> Pikaswaps<
    VideoParam,
    ImageParam,
    MaskParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    RegionToModifyParam,
> {
    /// Create a new node.
    pub fn new(
        video: VideoParam,
        image: Option<ImageParam>,
        mask: Option<MaskParam>,
        prompt_text: Option<PromptTextParam>,
        negative_prompt: Option<NegativePromptParam>,
        seed: Option<SeedParam>,
        region_to_modify: Option<RegionToModifyParam>,
    ) -> Self {
        Self {
            video,
            image,
            mask,
            prompt_text,
            negative_prompt,
            seed,
            region_to_modify,
        }
    }
}
impl<
    VideoParam: crate::nodes::types::Video,
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptTextParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    RegionToModifyParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for Pikaswaps<
    VideoParam,
    ImageParam,
    MaskParam,
    PromptTextParam,
    NegativePromptParam,
    SeedParam,
    RegionToModifyParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("video".to_string(), self.video.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        if let Some(v) = &self.prompt_text {
            output.insert("prompt_text".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.region_to_modify {
            output.insert("region_to_modify".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Pikaswaps";
    const DISPLAY_NAME: &'static str = "Pika Swaps (Video Object Replacement)";
    const DESCRIPTION: &'static str = "Swap out any object or region of your video with a new image or object. Define areas to replace either with a mask or coordinates.";
    const CATEGORY: &'static str = "api node/video/Pika";
}
