//!`video` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**SaveWEBM**: No description.
#[derive(Clone)]
pub struct SaveWebm<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    CodecParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
    ///No documentation.
    pub codec: CodecParam,
    /**No documentation.

**Metadata**:
  - Default: 24
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: FpsParam,
    /**Higher crf means lower quality with a smaller file size, lower crf means higher quality higher filesize.

**Metadata**:
  - Default: 32
  - Max: 63
  - Min: 0
  - Step: 1
*/
    pub crf: CrfParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    CodecParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> SaveWebm<ImagesParam, FilenamePrefixParam, CodecParam, FpsParam, CrfParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        filename_prefix: FilenamePrefixParam,
        codec: CodecParam,
        fps: FpsParam,
        crf: CrfParam,
    ) -> Self {
        Self {
            images,
            filename_prefix,
            codec,
            fps,
            crf,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    CodecParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for SaveWebm<ImagesParam, FilenamePrefixParam, CodecParam, FpsParam, CrfParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output.insert("codec".to_string(), self.codec.clone().into());
        output.insert("fps".to_string(), self.fps.clone().into());
        output.insert("crf".to_string(), self.crf.clone().into());
        output
    }
    const NAME: &'static str = "SaveWEBM";
    const DISPLAY_NAME: &'static str = "SaveWEBM";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/video";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FilenamePrefixParam: crate::nodes::types::String,
    CodecParam: crate::nodes::types::String,
    FpsParam: crate::nodes::types::Float,
    CrfParam: crate::nodes::types::Float,
> crate::nodes::TypedOutputNode
for SaveWebm<ImagesParam, FilenamePrefixParam, CodecParam, FpsParam, CrfParam> {}
