//!`Recraft` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`RecraftRemoveBackgroundNode`](super::RecraftRemoveBackgroundNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct RecraftRemoveBackgroundNodeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Recraft Color RGB**: Create Recraft Color by choosing specific RGB values.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftColorRGB<
    RParam: crate::nodes::types::Int,
    GParam: crate::nodes::types::Int,
    BParam: crate::nodes::types::Int,
    RecraftColorParam: crate::nodes::types::RecraftColor
        = crate::nodes::types::RecraftColorOut,
> {
    /**Red value of color.

**Metadata**:
  - Default: 0
  - Max: 255
  - Min: 0
*/
    pub r: RParam,
    /**Green value of color.

**Metadata**:
  - Default: 0
  - Max: 255
  - Min: 0
*/
    pub g: GParam,
    /**Blue value of color.

**Metadata**:
  - Default: 0
  - Max: 255
  - Min: 0
*/
    pub b: BParam,
    ///No documentation.
    pub recraft_color: Option<RecraftColorParam>,
}
impl<
    RParam: crate::nodes::types::Int,
    GParam: crate::nodes::types::Int,
    BParam: crate::nodes::types::Int,
    RecraftColorParam: crate::nodes::types::RecraftColor,
> RecraftColorRGB<RParam, GParam, BParam, RecraftColorParam> {
    /// Create a new node.
    pub fn new(
        r: RParam,
        g: GParam,
        b: BParam,
        recraft_color: Option<RecraftColorParam>,
    ) -> Self {
        Self { r, g, b, recraft_color }
    }
}
impl<
    RParam: crate::nodes::types::Int,
    GParam: crate::nodes::types::Int,
    BParam: crate::nodes::types::Int,
    RecraftColorParam: crate::nodes::types::RecraftColor,
> crate::nodes::TypedNode
for RecraftColorRGB<RParam, GParam, BParam, RecraftColorParam> {
    type Output = crate::nodes::types::RecraftColorOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("r".to_string(), self.r.clone().into());
        output.insert("g".to_string(), self.g.clone().into());
        output.insert("b".to_string(), self.b.clone().into());
        if let Some(v) = &self.recraft_color {
            output.insert("recraft_color".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftColorRGB";
    const DISPLAY_NAME: &'static str = "Recraft Color RGB";
    const DESCRIPTION: &'static str = "Create Recraft Color by choosing specific RGB values.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Controls**: Create Recraft Controls for customizing Recraft generation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftControls<
    ColorsParam: crate::nodes::types::RecraftColor
        = crate::nodes::types::RecraftColorOut,
    BackgroundColorParam: crate::nodes::types::RecraftColor
        = crate::nodes::types::RecraftColorOut,
> {
    ///No documentation.
    pub colors: Option<ColorsParam>,
    ///No documentation.
    pub background_color: Option<BackgroundColorParam>,
}
impl<
    ColorsParam: crate::nodes::types::RecraftColor,
    BackgroundColorParam: crate::nodes::types::RecraftColor,
> RecraftControls<ColorsParam, BackgroundColorParam> {
    /// Create a new node.
    pub fn new(
        colors: Option<ColorsParam>,
        background_color: Option<BackgroundColorParam>,
    ) -> Self {
        Self { colors, background_color }
    }
}
impl<
    ColorsParam: crate::nodes::types::RecraftColor,
    BackgroundColorParam: crate::nodes::types::RecraftColor,
> crate::nodes::TypedNode for RecraftControls<ColorsParam, BackgroundColorParam> {
    type Output = crate::nodes::types::RecraftControlsOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.colors {
            output.insert("colors".to_string(), v.clone().into());
        }
        if let Some(v) = &self.background_color {
            output.insert("background_color".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftControls";
    const DISPLAY_NAME: &'static str = "Recraft Controls";
    const DESCRIPTION: &'static str = "Create Recraft Controls for customizing Recraft generation.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
#[doc = "**Recraft Creative Upscale Image**: Upscale image synchronously.\n\nEnhances a given raster image using ‘creative upscale’ tool, boosting resolution with a focus on refining small details and faces."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftCreativeUpscaleNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> RecraftCreativeUpscaleNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for RecraftCreativeUpscaleNode<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "RecraftCreativeUpscaleNode";
    const DISPLAY_NAME: &'static str = "Recraft Creative Upscale Image";
    const DESCRIPTION: &'static str = "Upscale image synchronously.\nEnhances a given raster image using ‘creative upscale’ tool, boosting resolution with a focus on refining small details and faces.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
#[doc = "**Recraft Crisp Upscale Image**: Upscale image synchronously.\n\nEnhances a given raster image using ‘crisp upscale’ tool, increasing image resolution, making the image sharper and cleaner."]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftCrispUpscaleNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> RecraftCrispUpscaleNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for RecraftCrispUpscaleNode<ImageParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "RecraftCrispUpscaleNode";
    const DISPLAY_NAME: &'static str = "Recraft Crisp Upscale Image";
    const DESCRIPTION: &'static str = "Upscale image synchronously.\nEnhances a given raster image using ‘crisp upscale’ tool, increasing image resolution, making the image sharper and cleaner.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Image Inpainting**: Modify image based on prompt and mask.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftImageInpaintingNode<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style
        = crate::nodes::types::RecraftV3StyleOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub mask: MaskParam,
    /**Prompt for the image generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The number of images to generate.

**Metadata**:
  - Default: 1
  - Max: 6
  - Min: 1
*/
    pub n: NParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///No documentation.
    pub recraft_style: Option<RecraftStyleParam>,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
> RecraftImageInpaintingNode<
    ImageParam,
    MaskParam,
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        mask: MaskParam,
        prompt: PromptParam,
        n: NParam,
        seed: SeedParam,
        recraft_style: Option<RecraftStyleParam>,
        negative_prompt: Option<NegativePromptParam>,
    ) -> Self {
        Self {
            image,
            mask,
            prompt,
            n,
            seed,
            recraft_style,
            negative_prompt,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for RecraftImageInpaintingNode<
    ImageParam,
    MaskParam,
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
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
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.recraft_style {
            output.insert("recraft_style".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftImageInpaintingNode";
    const DISPLAY_NAME: &'static str = "Recraft Image Inpainting";
    const DESCRIPTION: &'static str = "Modify image based on prompt and mask.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Image to Image**: Modify image based on prompt and strength.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftImageToImageNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style
        = crate::nodes::types::RecraftV3StyleOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    RecraftControlsParam: crate::nodes::types::RecraftControls
        = crate::nodes::types::RecraftControlsOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt for the image generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The number of images to generate.

**Metadata**:
  - Default: 1
  - Max: 6
  - Min: 1
*/
    pub n: NParam,
    /**Defines the difference with the original image, should lie in \[0, 1\], where 0 means almost identical, and 1 means miserable similarity.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///No documentation.
    pub recraft_style: Option<RecraftStyleParam>,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///Optional additional controls over the generation via the Recraft Controls node.
    pub recraft_controls: Option<RecraftControlsParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> RecraftImageToImageNode<
    ImageParam,
    PromptParam,
    NParam,
    StrengthParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        n: NParam,
        strength: StrengthParam,
        seed: SeedParam,
        recraft_style: Option<RecraftStyleParam>,
        negative_prompt: Option<NegativePromptParam>,
        recraft_controls: Option<RecraftControlsParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            n,
            strength,
            seed,
            recraft_style,
            negative_prompt,
            recraft_controls,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> crate::nodes::TypedNode
for RecraftImageToImageNode<
    ImageParam,
    PromptParam,
    NParam,
    StrengthParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.recraft_style {
            output.insert("recraft_style".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.recraft_controls {
            output.insert("recraft_controls".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftImageToImageNode";
    const DISPLAY_NAME: &'static str = "Recraft Image to Image";
    const DESCRIPTION: &'static str = "Modify image based on prompt and strength.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Remove Background**: Remove background from image, and return processed image and mask.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftRemoveBackgroundNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> RecraftRemoveBackgroundNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for RecraftRemoveBackgroundNode<ImageParam> {
    type Output = out::RecraftRemoveBackgroundNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "RecraftRemoveBackgroundNode";
    const DISPLAY_NAME: &'static str = "Recraft Remove Background";
    const DESCRIPTION: &'static str = "Remove background from image, and return processed image and mask.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Replace Background**: Replace background on image, based on provided prompt.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftReplaceBackgroundNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style
        = crate::nodes::types::RecraftV3StyleOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Prompt for the image generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The number of images to generate.

**Metadata**:
  - Default: 1
  - Max: 6
  - Min: 1
*/
    pub n: NParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///No documentation.
    pub recraft_style: Option<RecraftStyleParam>,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
> RecraftReplaceBackgroundNode<
    ImageParam,
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        n: NParam,
        seed: SeedParam,
        recraft_style: Option<RecraftStyleParam>,
        negative_prompt: Option<NegativePromptParam>,
    ) -> Self {
        Self {
            image,
            prompt,
            n,
            seed,
            recraft_style,
            negative_prompt,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for RecraftReplaceBackgroundNode<
    ImageParam,
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
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
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.recraft_style {
            output.insert("recraft_style".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftReplaceBackgroundNode";
    const DISPLAY_NAME: &'static str = "Recraft Replace Background";
    const DESCRIPTION: &'static str = "Replace background on image, based on provided prompt.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Style - Digital Illustration**: Select realistic_image style and optional substyle.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftStyleV3DigitalIllustration {}
impl RecraftStyleV3DigitalIllustration {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for RecraftStyleV3DigitalIllustration {
    type Output = crate::nodes::types::RecraftV3StyleOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "RecraftStyleV3DigitalIllustration";
    const DISPLAY_NAME: &'static str = "Recraft Style - Digital Illustration";
    const DESCRIPTION: &'static str = "Select realistic_image style and optional substyle.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Style - Infinite Style Library**: Select style based on preexisting UUID from Recraft's Infinite Style Library.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftStyleV3InfiniteStyleLibrary<
    StyleIdParam: crate::nodes::types::String,
> {
    /**UUID of style from Infinite Style Library.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub style_id: StyleIdParam,
}
impl<
    StyleIdParam: crate::nodes::types::String,
> RecraftStyleV3InfiniteStyleLibrary<StyleIdParam> {
    /// Create a new node.
    pub fn new(style_id: StyleIdParam) -> Self {
        Self { style_id }
    }
}
impl<StyleIdParam: crate::nodes::types::String> crate::nodes::TypedNode
for RecraftStyleV3InfiniteStyleLibrary<StyleIdParam> {
    type Output = crate::nodes::types::RecraftV3StyleOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("style_id".to_string(), self.style_id.clone().into());
        output
    }
    const NAME: &'static str = "RecraftStyleV3InfiniteStyleLibrary";
    const DISPLAY_NAME: &'static str = "Recraft Style - Infinite Style Library";
    const DESCRIPTION: &'static str = "Select style based on preexisting UUID from Recraft's Infinite Style Library.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Style - Logo Raster**: Select realistic_image style and optional substyle.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftStyleV3LogoRaster {}
impl RecraftStyleV3LogoRaster {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for RecraftStyleV3LogoRaster {
    type Output = crate::nodes::types::RecraftV3StyleOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "RecraftStyleV3LogoRaster";
    const DISPLAY_NAME: &'static str = "Recraft Style - Logo Raster";
    const DESCRIPTION: &'static str = "Select realistic_image style and optional substyle.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Style - Realistic Image**: Select realistic_image style and optional substyle.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftStyleV3RealisticImage {}
impl RecraftStyleV3RealisticImage {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for RecraftStyleV3RealisticImage {
    type Output = crate::nodes::types::RecraftV3StyleOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "RecraftStyleV3RealisticImage";
    const DISPLAY_NAME: &'static str = "Recraft Style - Realistic Image";
    const DESCRIPTION: &'static str = "Select realistic_image style and optional substyle.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Text to Image**: Generates images synchronously based on prompt and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftTextToImageNode<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style
        = crate::nodes::types::RecraftV3StyleOut,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    RecraftControlsParam: crate::nodes::types::RecraftControls
        = crate::nodes::types::RecraftControlsOut,
> {
    /**Prompt for the image generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The number of images to generate.

**Metadata**:
  - Default: 1
  - Max: 6
  - Min: 1
*/
    pub n: NParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    ///No documentation.
    pub recraft_style: Option<RecraftStyleParam>,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///Optional additional controls over the generation via the Recraft Controls node.
    pub recraft_controls: Option<RecraftControlsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> RecraftTextToImageNode<
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        n: NParam,
        seed: SeedParam,
        recraft_style: Option<RecraftStyleParam>,
        negative_prompt: Option<NegativePromptParam>,
        recraft_controls: Option<RecraftControlsParam>,
    ) -> Self {
        Self {
            prompt,
            n,
            seed,
            recraft_style,
            negative_prompt,
            recraft_controls,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    RecraftStyleParam: crate::nodes::types::RecraftV3Style,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> crate::nodes::TypedNode
for RecraftTextToImageNode<
    PromptParam,
    NParam,
    SeedParam,
    RecraftStyleParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.recraft_style {
            output.insert("recraft_style".to_string(), v.clone().into());
        }
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.recraft_controls {
            output.insert("recraft_controls".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftTextToImageNode";
    const DISPLAY_NAME: &'static str = "Recraft Text to Image";
    const DESCRIPTION: &'static str = "Generates images synchronously based on prompt and resolution.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Text to Vector**: Generates SVG synchronously based on prompt and resolution.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftTextToVectorNode<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    RecraftControlsParam: crate::nodes::types::RecraftControls
        = crate::nodes::types::RecraftControlsOut,
> {
    /**Prompt for the image generation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**The number of images to generate.

**Metadata**:
  - Default: 1
  - Max: 6
  - Min: 1
*/
    pub n: NParam,
    /**Seed to determine if node should re-run; actual results are nondeterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**An optional text description of undesired elements on an image.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub negative_prompt: Option<NegativePromptParam>,
    ///Optional additional controls over the generation via the Recraft Controls node.
    pub recraft_controls: Option<RecraftControlsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> RecraftTextToVectorNode<
    PromptParam,
    NParam,
    SeedParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        n: NParam,
        seed: SeedParam,
        negative_prompt: Option<NegativePromptParam>,
        recraft_controls: Option<RecraftControlsParam>,
    ) -> Self {
        Self {
            prompt,
            n,
            seed,
            negative_prompt,
            recraft_controls,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    NegativePromptParam: crate::nodes::types::String,
    RecraftControlsParam: crate::nodes::types::RecraftControls,
> crate::nodes::TypedNode
for RecraftTextToVectorNode<
    PromptParam,
    NParam,
    SeedParam,
    NegativePromptParam,
    RecraftControlsParam,
> {
    type Output = crate::nodes::types::SvgOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("n".to_string(), self.n.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.recraft_controls {
            output.insert("recraft_controls".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RecraftTextToVectorNode";
    const DISPLAY_NAME: &'static str = "Recraft Text to Vector";
    const DESCRIPTION: &'static str = "Generates SVG synchronously based on prompt and resolution.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
///**Recraft Vectorize Image**: Generates SVG synchronously from an input image.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecraftVectorizeImageNode<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> RecraftVectorizeImageNode<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for RecraftVectorizeImageNode<ImageParam> {
    type Output = crate::nodes::types::SvgOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "RecraftVectorizeImageNode";
    const DISPLAY_NAME: &'static str = "Recraft Vectorize Image";
    const DESCRIPTION: &'static str = "Generates SVG synchronously from an input image.";
    const CATEGORY: &'static str = "api node/image/Recraft";
}
