//!postprocessing
///**ImageBlend**
pub struct ImageBlend<
    Image1: crate::nodes::Image,
    Image2: crate::nodes::Image,
    BlendFactor: crate::nodes::Float,
> {
    ///No documentation.
    pub image1: Image1,
    ///No documentation.
    pub image2: Image2,
    ///No documentation.
    pub blend_factor: BlendFactor,
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
///**ImageQuantize**
pub struct ImageQuantize<Image: crate::nodes::Image, Colors: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub colors: Colors,
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
///**ImageMorphology**
pub struct Morphology<Image: crate::nodes::Image, KernelSize: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub kernel_size: KernelSize,
}
