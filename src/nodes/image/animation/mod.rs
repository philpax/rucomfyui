//!`animation` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
/// Output types for nodes.
pub mod out {}
///**SaveAnimatedPNG**
pub struct SaveAnimatedPng<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
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
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveAnimatedPng<Images, FilenamePrefix, Fps, CompressLevel> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "SaveAnimatedPNG";
    const DISPLAY_NAME: &'static str = "SaveAnimatedPNG";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/animation";
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for SaveAnimatedPng<Images, FilenamePrefix, Fps, CompressLevel> {}
///**SaveAnimatedWEBP**
pub struct SaveAnimatedWebp<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Lossless: crate::nodes::types::Boolean,
    Quality: crate::nodes::types::Int,
    Method: crate::nodes::types::String,
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
    ///No documentation.
    pub method: Method,
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Lossless: crate::nodes::types::Boolean,
    Quality: crate::nodes::types::Int,
    Method: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveAnimatedWebp<Images, FilenamePrefix, Fps, Lossless, Quality, Method> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "SaveAnimatedWEBP";
    const DISPLAY_NAME: &'static str = "SaveAnimatedWEBP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/animation";
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Lossless: crate::nodes::types::Boolean,
    Quality: crate::nodes::types::Int,
    Method: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveAnimatedWebp<Images, FilenamePrefix, Fps, Lossless, Quality, Method> {}
