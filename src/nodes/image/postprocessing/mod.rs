//!`postprocessing` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**ImageBlend**
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
    ///No documentation.
    pub blend_factor: BlendFactor,
    ///No documentation.
    pub blend_mode: BlendMode,
}
///Output for [`ImageBlend`].
#[derive(Clone)]
pub struct ImageBlendOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageBlend<Image1, Image2, BlendFactor, BlendMode> {
    type Output = ImageBlendOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageBlend";
    const DISPLAY_NAME: &'static str = "ImageBlend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageBlur**
pub struct ImageBlur<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub blur_radius: BlurRadius,
    ///No documentation.
    pub sigma: Sigma,
}
///Output for [`ImageBlur`].
#[derive(Clone)]
pub struct ImageBlurOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageBlur<Image, BlurRadius, Sigma> {
    type Output = ImageBlurOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageBlur";
    const DISPLAY_NAME: &'static str = "ImageBlur";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageQuantize**
pub struct ImageQuantize<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub colors: Colors,
    ///No documentation.
    pub dither: Dither,
}
///Output for [`ImageQuantize`].
#[derive(Clone)]
pub struct ImageQuantizeOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageQuantize<Image, Colors, Dither> {
    type Output = ImageQuantizeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageQuantize";
    const DISPLAY_NAME: &'static str = "ImageQuantize";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageSharpen**
pub struct ImageSharpen<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub sharpen_radius: SharpenRadius,
    ///No documentation.
    pub sigma: Sigma,
    ///No documentation.
    pub alpha: Alpha,
}
///Output for [`ImageSharpen`].
#[derive(Clone)]
pub struct ImageSharpenOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageSharpen<Image, SharpenRadius, Sigma, Alpha> {
    type Output = ImageSharpenOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageSharpen";
    const DISPLAY_NAME: &'static str = "ImageSharpen";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageMorphology**
pub struct Morphology<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub operation: Operation,
    ///No documentation.
    pub kernel_size: KernelSize,
}
///Output for [`Morphology`].
#[derive(Clone)]
pub struct MorphologyOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for Morphology<Image, Operation, KernelSize> {
    type Output = MorphologyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "Morphology";
    const DISPLAY_NAME: &'static str = "ImageMorphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
