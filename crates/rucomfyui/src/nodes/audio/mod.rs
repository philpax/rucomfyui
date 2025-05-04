//!`audio` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**LoadAudio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadAudio<AudioParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Audio upload: true
*/
    pub audio: AudioParam,
}
impl<AudioParam: crate::nodes::types::String> LoadAudio<AudioParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam) -> Self {
        Self { audio }
    }
}
impl<AudioParam: crate::nodes::types::String> crate::nodes::TypedNode
for LoadAudio<AudioParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
#[allow(non_camel_case_types)]
pub struct PreviewAudio<AudioParam: crate::nodes::types::Audio> {
    ///No documentation.
    pub audio: AudioParam,
}
impl<AudioParam: crate::nodes::types::Audio> PreviewAudio<AudioParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam) -> Self {
        Self { audio }
    }
}
impl<AudioParam: crate::nodes::types::Audio> crate::nodes::TypedNode
for PreviewAudio<AudioParam> {
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
impl<AudioParam: crate::nodes::types::Audio> crate::nodes::TypedOutputNode
for PreviewAudio<AudioParam> {}
///**SaveAudio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAudio<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**No documentation.

**Metadata**:
  - Default: audio/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveAudio<AudioParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { audio, filename_prefix }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudio<AudioParam, FilenamePrefixParam> {
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
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudio<AudioParam, FilenamePrefixParam> {}
