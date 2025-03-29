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
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Codec: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Crf: crate::nodes::types::Float,
> {
    ///No documentation.
    pub images: Images,
    /**No documentation.

**Metadata**:
  - Default: ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
    ///No documentation.
    pub codec: Codec,
    /**No documentation.

**Metadata**:
  - Default: 24
  - Max: 1000
  - Min: 0.01
  - Step: 0.01
*/
    pub fps: Fps,
    /**Higher crf means lower quality with a smaller file size, lower crf means higher quality higher filesize.

**Metadata**:
  - Default: 32
  - Max: 63
  - Min: 0
  - Step: 1
*/
    pub crf: Crf,
}
impl<
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Codec: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Crf: crate::nodes::types::Float,
> SaveWebm<Images, FilenamePrefix, Codec, Fps, Crf> {
    /// Create a new node.
    pub fn new(
        images: Images,
        filename_prefix: FilenamePrefix,
        codec: Codec,
        fps: Fps,
        crf: Crf,
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
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Codec: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Crf: crate::nodes::types::Float,
> crate::nodes::TypedNode for SaveWebm<Images, FilenamePrefix, Codec, Fps, Crf> {
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
    Images: crate::nodes::types::Image,
    FilenamePrefix: crate::nodes::types::String,
    Codec: crate::nodes::types::String,
    Fps: crate::nodes::types::Float,
    Crf: crate::nodes::types::Float,
> crate::nodes::TypedOutputNode for SaveWebm<Images, FilenamePrefix, Codec, Fps, Crf> {}
