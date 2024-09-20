//!`audio` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**LoadAudio**
pub struct LoadAudio<Audio: crate::nodes::types::String> {
    ///No documentation.
    pub audio: Audio,
}
///Output for [`LoadAudio`].
#[derive(Clone)]
pub struct LoadAudioOutput {
    ///No documentation.
    pub audio: crate::nodes::types::AudioOut,
}
impl<Audio: crate::nodes::types::String> crate::nodes::TypedNode for LoadAudio<Audio> {
    type Output = LoadAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            audio: crate::nodes::types::AudioOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LoadAudio";
    const DISPLAY_NAME: &'static str = "LoadAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**PreviewAudio**
pub struct PreviewAudio<Audio: crate::nodes::types::Audio> {
    ///No documentation.
    pub audio: Audio,
}
impl<Audio: crate::nodes::types::Audio> crate::nodes::TypedNode for PreviewAudio<Audio> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "PreviewAudio";
    const DISPLAY_NAME: &'static str = "PreviewAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<Audio: crate::nodes::types::Audio> crate::nodes::TypedOutputNode
for PreviewAudio<Audio> {}
///**SaveAudio**
pub struct SaveAudio<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudio<Audio, FilenamePrefix> {
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "SaveAudio";
    const DISPLAY_NAME: &'static str = "SaveAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<
    Audio: crate::nodes::types::Audio,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudio<Audio, FilenamePrefix> {}
