//!`postprocessing` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ImageBlend**: No description.
#[derive(Clone)]
pub struct ImageBlend<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> {
    ///No documentation.
    pub image_1: Image1,
    ///No documentation.
    pub image_2: Image2,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blend_factor: BlendFactor,
    ///No documentation.
    pub blend_mode: BlendMode,
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> ImageBlend<Image1, Image2, BlendFactor, BlendMode> {
    /// Create a new node.
    pub fn new(
        image_1: Image1,
        image_2: Image2,
        blend_factor: BlendFactor,
        blend_mode: BlendMode,
    ) -> Self {
        Self {
            image_1,
            image_2,
            blend_factor,
            blend_mode,
        }
    }
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageBlend<Image1, Image2, BlendFactor, BlendMode> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output.insert("image2".to_string(), self.image_2.clone().into());
        output.insert("blend_factor".to_string(), self.blend_factor.clone().into());
        output.insert("blend_mode".to_string(), self.blend_mode.clone().into());
        output
    }
    const NAME: &'static str = "ImageBlend";
    const DISPLAY_NAME: &'static str = "ImageBlend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageBlur**: No description.
#[derive(Clone)]
pub struct ImageBlur<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub blur_radius: BlurRadius,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.1
*/
    pub sigma: Sigma,
}
impl<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> ImageBlur<Image, BlurRadius, Sigma> {
    /// Create a new node.
    pub fn new(image: Image, blur_radius: BlurRadius, sigma: Sigma) -> Self {
        Self { image, blur_radius, sigma }
    }
}
impl<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageBlur<Image, BlurRadius, Sigma> {
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
pub struct ImageQuantize<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 256
  - Min: 1
  - Step: 1
*/
    pub colors: Colors,
    ///No documentation.
    pub dither: Dither,
}
impl<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> ImageQuantize<Image, Colors, Dither> {
    /// Create a new node.
    pub fn new(image: Image, colors: Colors, dither: Dither) -> Self {
        Self { image, colors, dither }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageQuantize<Image, Colors, Dither> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("colors".to_string(), self.colors.clone().into());
        output.insert("dither".to_string(), self.dither.clone().into());
        output
    }
    const NAME: &'static str = "ImageQuantize";
    const DISPLAY_NAME: &'static str = "ImageQuantize";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageSharpen**: No description.
#[derive(Clone)]
pub struct ImageSharpen<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 31
  - Min: 1
  - Step: 1
*/
    pub sharpen_radius: SharpenRadius,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0.1
  - Step: 0.01
*/
    pub sigma: Sigma,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 5
  - Min: 0
  - Step: 0.01
*/
    pub alpha: Alpha,
}
impl<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> ImageSharpen<Image, SharpenRadius, Sigma, Alpha> {
    /// Create a new node.
    pub fn new(
        image: Image,
        sharpen_radius: SharpenRadius,
        sigma: Sigma,
        alpha: Alpha,
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
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageSharpen<Image, SharpenRadius, Sigma, Alpha> {
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
pub struct Morphology<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub operation: Operation,
    /**No documentation.

**Metadata**:
  - Default: 3
  - Max: 999
  - Min: 3
  - Step: 1
*/
    pub kernel_size: KernelSize,
}
impl<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> Morphology<Image, Operation, KernelSize> {
    /// Create a new node.
    pub fn new(image: Image, operation: Operation, kernel_size: KernelSize) -> Self {
        Self {
            image,
            operation,
            kernel_size,
        }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for Morphology<Image, Operation, KernelSize> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("operation".to_string(), self.operation.clone().into());
        output.insert("kernel_size".to_string(), self.kernel_size.clone().into());
        output
    }
    const NAME: &'static str = "Morphology";
    const DISPLAY_NAME: &'static str = "ImageMorphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
