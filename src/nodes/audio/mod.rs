//!`audio` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
///**LoadAudio**: No description.
#[derive(Clone)]
pub struct LoadAudio<Audio: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Audio upload: true
*/
    pub audio: Audio,
}
impl<Audio: crate::nodes::types::String> LoadAudio<Audio> {
    /// Create a new node.
    pub fn new(audio: Audio) -> Self {
        Self { audio }
    }
}
impl<Audio: crate::nodes::types::String> crate::nodes::TypedNode for LoadAudio<Audio> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "LoadAudio";
    const DISPLAY_NAME: &'static str = "LoadAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**PreviewAudio**: No description.
#[derive(Clone)]
pub struct PreviewAudio<Audio: crate::nodes::types::Audio> {
    ///No documentation.
    pub audio: Audio,
}
impl<Audio: crate::nodes::types::Audio> PreviewAudio<Audio> {
    /// Create a new node.
    pub fn new(audio: Audio) -> Self {
        Self { audio }
    }
}
impl<Audio: crate::nodes::types::Audio> crate::nodes::TypedNode for PreviewAudio<Audio> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "PreviewAudio";
    const DISPLAY_NAME: &'static str = "PreviewAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<Audio: crate::nodes::types::Audio> crate::nodes::TypedOutputNode
for PreviewAudio<Audio> {}
///**SaveAudio**: No description.
#[derive(Clone)]
pub struct SaveAudio<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub audio: Audio,
    /**No documentation.

**Metadata**:
  - Default: audio/ComfyUI
*/
    pub filename_prefix: FilenamePrefix,
}
impl<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> SaveAudio<Audio, FilenamePrefix> {
    /// Create a new node.
    pub fn new(audio: Audio, filename_prefix: FilenamePrefix) -> Self {
        Self { audio, filename_prefix }
    }
}
impl<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudio<Audio, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveAudio";
    const DISPLAY_NAME: &'static str = "SaveAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudio<Audio, FilenamePrefix> {}
