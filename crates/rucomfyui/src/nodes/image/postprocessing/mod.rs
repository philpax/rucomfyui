//!`postprocessing` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ImageBlend**: No description.
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
    const DISPLAY_NAME: &'static str = "ImageBlend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageBlur**: No description.
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
    const DISPLAY_NAME: &'static str = "ImageBlur";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageQuantize**: No description.
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
    const DISPLAY_NAME: &'static str = "ImageQuantize";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageSharpen**: No description.
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
    const DISPLAY_NAME: &'static str = "ImageSharpen";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageMorphology**: No description.
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
    const DISPLAY_NAME: &'static str = "ImageMorphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
