//!audio
///**LoadAudio**
pub struct LoadAudio {}
///**PreviewAudio**
pub struct PreviewAudio<Audio: crate::nodes::Audio> {
    ///No documentation.
    pub audio: Audio,
}
///**SaveAudio**
pub struct SaveAudio<Audio: crate::nodes::Audio, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
