//!`animation` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**SaveAnimatedPNG**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAnimatedPNG<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**No documentation.

**Metadata**:
  - Default: 4
  - Max: 9
  - Min: 0
*/
    pub compress_level: CompressLevelParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        fps: FpsParam,
        compress_level: CompressLevelParam,
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
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {
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
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CompressLevelParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for SaveAnimatedPNG<ImagesParam, FilenamePrefixParam, FpsParam, CompressLevelParam> {}
///**SaveAnimatedWEBP**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAnimatedWEBP<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
    MethodParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub lossless: LosslessParam,
    /**No documentation.

**Metadata**:
  - Default: 80
  - Max: 100
  - Min: 0
*/
    pub quality: QualityParam,
    ///No documentation.
    pub method: MethodParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
    MethodParam: crate::nodes::types::String,
> SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
    MethodParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        fps: FpsParam,
        lossless: LosslessParam,
        quality: QualityParam,
        method: MethodParam,
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
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
    MethodParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
    MethodParam,
> {
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
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    LosslessParam: crate::nodes::types::Boolean,
    QualityParam: crate::nodes::types::Int,
    MethodParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveAnimatedWEBP<
    ImagesParam,
    FilenamePrefixParam,
    FpsParam,
    LosslessParam,
    QualityParam,
    MethodParam,
> {}
