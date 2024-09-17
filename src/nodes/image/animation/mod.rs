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
///Output for [`SaveAnimatedPng`].
pub struct SaveAnimatedPngOutput {}
impl<
    Images: crate::nodes::Image,
    FilenamePrefix: crate::nodes::String,
    Fps: crate::nodes::Float,
    CompressLevel: crate::nodes::Int,
> crate::nodes::TypedNode
for SaveAnimatedPng<Images, FilenamePrefix, Fps, CompressLevel> {
    type Output = SaveAnimatedPngOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "SaveAnimatedPNG";
    const DISPLAY_NAME: &'static str = "SaveAnimatedPNG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/animation";
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
///Output for [`SaveAnimatedWebp`].
pub struct SaveAnimatedWebpOutput {}
impl<
    Images: crate::nodes::Image,
    FilenamePrefix: crate::nodes::String,
    Fps: crate::nodes::Float,
    Lossless: crate::nodes::Boolean,
    Quality: crate::nodes::Int,
> crate::nodes::TypedNode
for SaveAnimatedWebp<Images, FilenamePrefix, Fps, Lossless, Quality> {
    type Output = SaveAnimatedWebpOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "SaveAnimatedWEBP";
    const DISPLAY_NAME: &'static str = "SaveAnimatedWEBP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/animation";
}
