//!`animation` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**SaveAnimatedPNG**: No description.
#[derive(Clone)]
pub struct SaveAnimatedPng<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: Images,
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: Fps,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 9
  - Min: 0
*/
    pub compress_level: CompressLevel,
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
> SaveAnimatedPng<Images, FilenamePrefix, Fps, CompressLevel> {
    /// Create a new node.
    pub fn new(
        images: Images,
        filename_prefix: FilenamePrefix,
        fps: Fps,
        compress_level: CompressLevel,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            fps,
            compress_level,
        }
    }
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    CompressLevel: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveAnimatedPng<Images, FilenamePrefix, Fps, CompressLevel> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("compress_level".to_string(), self.compress_level.clone().into());
        output
    }
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
///**SaveAnimatedWEBP**: No description.
#[derive(Clone)]
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
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: Fps,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub lossless: Lossless,
    /**No documentation.

**Metadata**:
  - Default: 80
  - Max: 100
  - Min: 0
*/
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
> SaveAnimatedWebp<Images, FilenamePrefix, Fps, Lossless, Quality, Method> {
    /// Create a new node.
    pub fn new(
        images: Images,
        filename_prefix: FilenamePrefix,
        fps: Fps,
        lossless: Lossless,
        quality: Quality,
        method: Method,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            fps,
            lossless,
            quality,
            method,
        }
    }
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
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("lossless".to_string(), self.lossless.clone().into());
        output.insert("quality".to_string(), self.quality.clone().into());
        output.insert("method".to_string(), self.method.clone().into());
        output
    }
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
