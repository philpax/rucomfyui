//!animation
///**SaveAnimatedPNG**
pub struct SaveAnimatedPng<
    Images: crate::nodes::Image,
    FilenamePrefix: crate::nodes::String,
    Fps: crate::nodes::Float,
    CompressLevel: crate::nodes::Int,
> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
    ///No documentation.
    pub fps: Fps,
    ///No documentation.
    pub compress_level: CompressLevel,
}
///**SaveAnimatedWEBP**
pub struct SaveAnimatedWebp<
    Images: crate::nodes::Image,
    FilenamePrefix: crate::nodes::String,
    Fps: crate::nodes::Float,
    Lossless: crate::nodes::Boolean,
    Quality: crate::nodes::Int,
> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
    ///No documentation.
    pub fps: Fps,
    ///No documentation.
    pub lossless: Lossless,
    ///No documentation.
    pub quality: Quality,
}
