//!audio
///**LoadAudio**
pub struct LoadAudio {}
///Output for [`LoadAudio`].
pub struct LoadAudioOutput {
    ///No documentation.
    pub audio: crate::nodes::AudioOut,
}
impl crate::nodes::TypedNode for LoadAudio {
    type Output = LoadAudioOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            audio: crate::nodes::AudioOut(0usize),
        }
    }
    const NAME: &'static str = "LoadAudio";
    const DISPLAY_NAME: &'static str = "LoadAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**PreviewAudio**
pub struct PreviewAudio<Audio: crate::nodes::Audio> {
    ///No documentation.
    pub audio: Audio,
}
///Output for [`PreviewAudio`].
pub struct PreviewAudioOutput {}
impl<Audio: crate::nodes::Audio> crate::nodes::TypedNode for PreviewAudio<Audio> {
    type Output = PreviewAudioOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "PreviewAudio";
    const DISPLAY_NAME: &'static str = "PreviewAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**SaveAudio**
pub struct SaveAudio<Audio: crate::nodes::Audio, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///Output for [`SaveAudio`].
pub struct SaveAudioOutput {}
impl<
    Audio: crate::nodes::Audio,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for SaveAudio<Audio, FilenamePrefix> {
    type Output = SaveAudioOutput;
    fn output(&self) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "SaveAudio";
    const DISPLAY_NAME: &'static str = "SaveAudio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
