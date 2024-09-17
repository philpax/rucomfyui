//!postprocessing
///**ImageBlend**
pub struct ImageBlend<
    Image1: crate::nodes::Image,
    Image2: crate::nodes::Image,
    BlendFactor: crate::nodes::Float,
> {
    ///No documentation.
    pub image_1: Image1,
    ///No documentation.
    pub image_2: Image2,
    ///No documentation.
    pub blend_factor: BlendFactor,
}
///Output for [`ImageBlend`].
pub struct ImageBlendOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image1: crate::nodes::Image,
    Image2: crate::nodes::Image,
    BlendFactor: crate::nodes::Float,
> crate::nodes::TypedNode for ImageBlend<Image1, Image2, BlendFactor> {
    type Output = ImageBlendOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageBlend";
    const DISPLAY_NAME: &'static str = "ImageBlend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageBlur**
pub struct ImageBlur<
    Image: crate::nodes::Image,
    BlurRadius: crate::nodes::Int,
    Sigma: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub blur_radius: BlurRadius,
    ///No documentation.
    pub sigma: Sigma,
}
///Output for [`ImageBlur`].
pub struct ImageBlurOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    BlurRadius: crate::nodes::Int,
    Sigma: crate::nodes::Float,
> crate::nodes::TypedNode for ImageBlur<Image, BlurRadius, Sigma> {
    type Output = ImageBlurOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageBlur";
    const DISPLAY_NAME: &'static str = "ImageBlur";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageQuantize**
pub struct ImageQuantize<Image: crate::nodes::Image, Colors: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub colors: Colors,
}
///Output for [`ImageQuantize`].
pub struct ImageQuantizeOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image: crate::nodes::Image, Colors: crate::nodes::Int> crate::nodes::TypedNode
for ImageQuantize<Image, Colors> {
    type Output = ImageQuantizeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageQuantize";
    const DISPLAY_NAME: &'static str = "ImageQuantize";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageSharpen**
pub struct ImageSharpen<
    Image: crate::nodes::Image,
    SharpenRadius: crate::nodes::Int,
    Sigma: crate::nodes::Float,
    Alpha: crate::nodes::Float,
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
pub struct ImageSharpenOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    SharpenRadius: crate::nodes::Int,
    Sigma: crate::nodes::Float,
    Alpha: crate::nodes::Float,
> crate::nodes::TypedNode for ImageSharpen<Image, SharpenRadius, Sigma, Alpha> {
    type Output = ImageSharpenOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "ImageSharpen";
    const DISPLAY_NAME: &'static str = "ImageSharpen";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageMorphology**
pub struct Morphology<Image: crate::nodes::Image, KernelSize: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub kernel_size: KernelSize,
}
///Output for [`Morphology`].
pub struct MorphologyOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image: crate::nodes::Image, KernelSize: crate::nodes::Int> crate::nodes::TypedNode
for Morphology<Image, KernelSize> {
    type Output = MorphologyOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0usize),
        }
    }
    const NAME: &'static str = "Morphology";
    const DISPLAY_NAME: &'static str = "ImageMorphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
