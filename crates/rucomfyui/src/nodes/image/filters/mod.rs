//!`filters` definitions/categories.
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
///**Detect Edges (Canny)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Canny<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0.4
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub low_threshold: LowThresholdParam,
    /**No documentation.

**Metadata**:
  - Default: 0.8
  - Max: 0.99
  - Min: 0.01
  - Step: 0.01
*/
    pub high_threshold: HighThresholdParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> Canny<ImageParam, LowThresholdParam, HighThresholdParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        low_threshold: LowThresholdParam,
        high_threshold: HighThresholdParam,
    ) -> Self {
        Self {
            image,
            low_threshold,
            high_threshold,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    LowThresholdParam: crate::nodes::types::Float,
    HighThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for Canny<ImageParam, LowThresholdParam, HighThresholdParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("low_threshold".to_string(), self.low_threshold.clone().into());
        output.insert("high_threshold".to_string(), self.high_threshold.clone().into());
        output
    }
    const NAME: &'static str = "Canny";
    const DISPLAY_NAME: &'static str = "Detect Edges (Canny)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Transfer Color**: Match the colors of one image to another using various algorithms.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ColorTransfer<
    ImageTargetParam: crate::nodes::types::Image,
    ImageRefParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> {
    ///Image(s) to apply the color transform to.
    pub image_target: ImageTargetParam,
    ///Reference image(s) to match colors to.
    pub image_ref: ImageRefParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ImageTargetParam: crate::nodes::types::Image,
    ImageRefParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> ColorTransfer<ImageTargetParam, ImageRefParam, StrengthParam> {
    /// Create a new node.
    pub fn new(
        image_target: ImageTargetParam,
        image_ref: ImageRefParam,
        strength: StrengthParam,
    ) -> Self {
        Self {
            image_target,
            image_ref,
            strength,
        }
    }
}
impl<
    ImageTargetParam: crate::nodes::types::Image,
    ImageRefParam: crate::nodes::types::Image,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ColorTransfer<ImageTargetParam, ImageRefParam, StrengthParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_target".to_string(), self.image_target.clone().into());
        output.insert("image_ref".to_string(), self.image_ref.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ColorTransfer";
    const DISPLAY_NAME: &'static str = "Transfer Color";
    const DESCRIPTION: &'static str = "Match the colors of one image to another using various algorithms.";
    const CATEGORY: &'static str = "image/filters";
}
///**Add Noise to Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageAddNoise<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**The random seed used for creating the noise.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub strength: StrengthParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> ImageAddNoise<ImageParam, SeedParam, StrengthParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, seed: SeedParam, strength: StrengthParam) -> Self {
        Self { image, seed, strength }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageAddNoise<ImageParam, SeedParam, StrengthParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("strength".to_string(), self.strength.clone().into());
        output
    }
    const NAME: &'static str = "ImageAddNoise";
    const DISPLAY_NAME: &'static str = "Add Noise to Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Blend Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageBlend<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
    BlendFactorParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image_1: Image1Param,
    ///No documentation.
    pub image_2: Image2Param,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blend_factor: BlendFactorParam,
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
    BlendFactorParam: crate::nodes::types::Float,
> ImageBlend<Image1Param, Image2Param, BlendFactorParam> {
    /// Create a new node.
    pub fn new(
        image_1: Image1Param,
        image_2: Image2Param,
        blend_factor: BlendFactorParam,
    ) -> Self {
        Self {
            image_1,
            image_2,
            blend_factor,
        }
    }
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
    BlendFactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageBlend<Image1Param, Image2Param, BlendFactorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output.insert("image2".to_string(), self.image_2.clone().into());
        output.insert("blend_factor".to_string(), self.blend_factor.clone().into());
        output
    }
    const NAME: &'static str = "ImageBlend";
    const DISPLAY_NAME: &'static str = "Blend Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Blur Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageBlur<
    ImageParam: crate::nodes::types::Image,
    BlurRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub blur_radius: BlurRadiusParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.1
*/
    pub sigma: SigmaParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    BlurRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
> ImageBlur<ImageParam, BlurRadiusParam, SigmaParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        blur_radius: BlurRadiusParam,
        sigma: SigmaParam,
    ) -> Self {
        Self { image, blur_radius, sigma }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    BlurRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageBlur<ImageParam, BlurRadiusParam, SigmaParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("blur_radius".to_string(), self.blur_radius.clone().into());
        output.insert("sigma".to_string(), self.sigma.clone().into());
        output
    }
    const NAME: &'static str = "ImageBlur";
    const DISPLAY_NAME: &'static str = "Blur Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Quantize Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageQuantize<
    ImageParam: crate::nodes::types::Image,
    ColorsParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 256
  - Min: 1
  - Step: 1
*/
    pub colors: ColorsParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ColorsParam: crate::nodes::types::Int,
> ImageQuantize<ImageParam, ColorsParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, colors: ColorsParam) -> Self {
        Self { image, colors }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ColorsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageQuantize<ImageParam, ColorsParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("colors".to_string(), self.colors.clone().into());
        output
    }
    const NAME: &'static str = "ImageQuantize";
    const DISPLAY_NAME: &'static str = "Quantize Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Sharpen Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageSharpen<
    ImageParam: crate::nodes::types::Image,
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub sharpen_radius: SharpenRadiusParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.01
*/
    pub sigma: SigmaParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 5
  - Min: 0
  - Step: 0.01
*/
    pub alpha: AlphaParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> ImageSharpen<ImageParam, SharpenRadiusParam, SigmaParam, AlphaParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        sharpen_radius: SharpenRadiusParam,
        sigma: SigmaParam,
        alpha: AlphaParam,
    ) -> Self {
        Self {
            image,
            sharpen_radius,
            sigma,
            alpha,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SharpenRadiusParam: crate::nodes::types::Int,
    SigmaParam: crate::nodes::types::Float,
    AlphaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ImageSharpen<ImageParam, SharpenRadiusParam, SigmaParam, AlphaParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("sharpen_radius".to_string(), self.sharpen_radius.clone().into());
        output.insert("sigma".to_string(), self.sigma.clone().into());
        output.insert("alpha".to_string(), self.alpha.clone().into());
        output
    }
    const NAME: &'static str = "ImageSharpen";
    const DISPLAY_NAME: &'static str = "Sharpen Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
///**Apply Morphology**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Morphology<
    ImageParam: crate::nodes::types::Image,
    KernelSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 999
  - Min: 3
  - Step: 1
*/
    pub kernel_size: KernelSizeParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    KernelSizeParam: crate::nodes::types::Int,
> Morphology<ImageParam, KernelSizeParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, kernel_size: KernelSizeParam) -> Self {
        Self { image, kernel_size }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    KernelSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Morphology<ImageParam, KernelSizeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("kernel_size".to_string(), self.kernel_size.clone().into());
        output
    }
    const NAME: &'static str = "Morphology";
    const DISPLAY_NAME: &'static str = "Apply Morphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/filters";
}
