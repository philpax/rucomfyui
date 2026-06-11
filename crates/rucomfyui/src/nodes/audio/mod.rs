//!`audio` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`SplitAudioChannels`](super::SplitAudioChannels).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SplitAudioChannelsOutput {
        ///No documentation.
        pub left: crate::nodes::types::AudioOut,
        ///No documentation.
        pub right: crate::nodes::types::AudioOut,
    }
}
///**Adjust Audio Volume**: Adjust the volume of the audio by a specified amount in decibels (dB).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioAdjustVolume<
    AudioParam: crate::nodes::types::Audio,
    VolumeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**Volume adjustment in decibels (dB). 0 = no change, +6 = double, -6 = half, etc

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: -100
*/
    pub volume: VolumeParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VolumeParam: crate::nodes::types::Int,
> AudioAdjustVolume<AudioParam, VolumeParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, volume: VolumeParam) -> Self {
        Self { audio, volume }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    VolumeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for AudioAdjustVolume<AudioParam, VolumeParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("volume".to_string(), self.volume.clone().into());
        output
    }
    const NAME: &'static str = "AudioAdjustVolume";
    const DISPLAY_NAME: &'static str = "Adjust Audio Volume";
    const DESCRIPTION: &'static str = "Adjust the volume of the audio by a specified amount in decibels (dB).";
    const CATEGORY: &'static str = "audio";
}
///**Concatenate Audio**: Concatenates the audio1 to audio2 in the specified direction.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioConcat<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub audio_1: Audio1Param,
    ///No documentation.
    pub audio_2: Audio2Param,
}
impl<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> AudioConcat<Audio1Param, Audio2Param> {
    /// Create a new node.
    pub fn new(audio_1: Audio1Param, audio_2: Audio2Param) -> Self {
        Self { audio_1, audio_2 }
    }
}
impl<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> crate::nodes::TypedNode for AudioConcat<Audio1Param, Audio2Param> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio1".to_string(), self.audio_1.clone().into());
        output.insert("audio2".to_string(), self.audio_2.clone().into());
        output
    }
    const NAME: &'static str = "AudioConcat";
    const DISPLAY_NAME: &'static str = "Concatenate Audio";
    const DESCRIPTION: &'static str = "Concatenates the audio1 to audio2 in the specified direction.";
    const CATEGORY: &'static str = "audio";
}
///**Audio Equalizer (3-Band)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioEqualizer3Band<
    AudioParam: crate::nodes::types::Audio,
    LowGainDBParam: crate::nodes::types::Float,
    LowFreqParam: crate::nodes::types::Int,
    MidGainDBParam: crate::nodes::types::Float,
    MidFreqParam: crate::nodes::types::Int,
    MidQParam: crate::nodes::types::Float,
    HighGainDBParam: crate::nodes::types::Float,
    HighFreqParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**Gain for Low frequencies (Bass)

**Metadata**:
  - Default: 0
  - Max: 24
  - Min: -24
  - Step: 0.1
*/
    pub low_gain_d_b: LowGainDBParam,
    /**Cutoff frequency for Low shelf

**Metadata**:
  - Default: 100
  - Max: 500
  - Min: 20
*/
    pub low_freq: LowFreqParam,
    /**Gain for Mid frequencies

**Metadata**:
  - Default: 0
  - Max: 24
  - Min: -24
  - Step: 0.1
*/
    pub mid_gain_d_b: MidGainDBParam,
    /**Center frequency for Mids

**Metadata**:
  - Default: 1000
  - Max: 4000
  - Min: 200
*/
    pub mid_freq: MidFreqParam,
    /**Q factor (bandwidth) for Mids

**Metadata**:
  - Default: 0.707
  - Max: 10
  - Min: 0.1
  - Step: 0.1
*/
    pub mid_q: MidQParam,
    /**Gain for High frequencies (Treble)

**Metadata**:
  - Default: 0
  - Max: 24
  - Min: -24
  - Step: 0.1
*/
    pub high_gain_d_b: HighGainDBParam,
    /**Cutoff frequency for High shelf

**Metadata**:
  - Default: 5000
  - Max: 15000
  - Min: 1000
*/
    pub high_freq: HighFreqParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    LowGainDBParam: crate::nodes::types::Float,
    LowFreqParam: crate::nodes::types::Int,
    MidGainDBParam: crate::nodes::types::Float,
    MidFreqParam: crate::nodes::types::Int,
    MidQParam: crate::nodes::types::Float,
    HighGainDBParam: crate::nodes::types::Float,
    HighFreqParam: crate::nodes::types::Int,
> AudioEqualizer3Band<
    AudioParam,
    LowGainDBParam,
    LowFreqParam,
    MidGainDBParam,
    MidFreqParam,
    MidQParam,
    HighGainDBParam,
    HighFreqParam,
> {
    /// Create a new node.
    pub fn new(
        audio: AudioParam,
        low_gain_d_b: LowGainDBParam,
        low_freq: LowFreqParam,
        mid_gain_d_b: MidGainDBParam,
        mid_freq: MidFreqParam,
        mid_q: MidQParam,
        high_gain_d_b: HighGainDBParam,
        high_freq: HighFreqParam,
    ) -> Self {
        Self {
            audio,
            low_gain_d_b,
            low_freq,
            mid_gain_d_b,
            mid_freq,
            mid_q,
            high_gain_d_b,
            high_freq,
        }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    LowGainDBParam: crate::nodes::types::Float,
    LowFreqParam: crate::nodes::types::Int,
    MidGainDBParam: crate::nodes::types::Float,
    MidFreqParam: crate::nodes::types::Int,
    MidQParam: crate::nodes::types::Float,
    HighGainDBParam: crate::nodes::types::Float,
    HighFreqParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for AudioEqualizer3Band<
    AudioParam,
    LowGainDBParam,
    LowFreqParam,
    MidGainDBParam,
    MidFreqParam,
    MidQParam,
    HighGainDBParam,
    HighFreqParam,
> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("low_gain_dB".to_string(), self.low_gain_d_b.clone().into());
        output.insert("low_freq".to_string(), self.low_freq.clone().into());
        output.insert("mid_gain_dB".to_string(), self.mid_gain_d_b.clone().into());
        output.insert("mid_freq".to_string(), self.mid_freq.clone().into());
        output.insert("mid_q".to_string(), self.mid_q.clone().into());
        output.insert("high_gain_dB".to_string(), self.high_gain_d_b.clone().into());
        output.insert("high_freq".to_string(), self.high_freq.clone().into());
        output
    }
    const NAME: &'static str = "AudioEqualizer3Band";
    const DISPLAY_NAME: &'static str = "Audio Equalizer (3-Band)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**Merge Audio**: Combine two audio tracks by overlaying their waveforms.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AudioMerge<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub audio_1: Audio1Param,
    ///No documentation.
    pub audio_2: Audio2Param,
}
impl<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> AudioMerge<Audio1Param, Audio2Param> {
    /// Create a new node.
    pub fn new(audio_1: Audio1Param, audio_2: Audio2Param) -> Self {
        Self { audio_1, audio_2 }
    }
}
impl<
    Audio1Param: crate::nodes::types::Audio,
    Audio2Param: crate::nodes::types::Audio,
> crate::nodes::TypedNode for AudioMerge<Audio1Param, Audio2Param> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio1".to_string(), self.audio_1.clone().into());
        output.insert("audio2".to_string(), self.audio_2.clone().into());
        output
    }
    const NAME: &'static str = "AudioMerge";
    const DISPLAY_NAME: &'static str = "Merge Audio";
    const DESCRIPTION: &'static str = "Combine two audio tracks by overlaying their waveforms.";
    const CATEGORY: &'static str = "audio";
}
///**Empty Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct EmptyAudio<
    DurationParam: crate::nodes::types::Float,
    SampleRateParam: crate::nodes::types::Int,
    ChannelsParam: crate::nodes::types::Int,
> {
    /**Duration of the empty audio clip in seconds

**Metadata**:
  - Default: 60
  - Max: 18446744073709551615
  - Min: 0
  - Step: 0.01
*/
    pub duration: DurationParam,
    /**Sample rate of the empty audio clip.

**Metadata**:
  - Default: 44100
  - Max: 192000
  - Min: 1
*/
    pub sample_rate: SampleRateParam,
    /**Number of audio channels (1 for mono, 2 for stereo).

**Metadata**:
  - Default: 2
  - Max: 2
  - Min: 1
*/
    pub channels: ChannelsParam,
}
impl<
    DurationParam: crate::nodes::types::Float,
    SampleRateParam: crate::nodes::types::Int,
    ChannelsParam: crate::nodes::types::Int,
> EmptyAudio<DurationParam, SampleRateParam, ChannelsParam> {
    /// Create a new node.
    pub fn new(
        duration: DurationParam,
        sample_rate: SampleRateParam,
        channels: ChannelsParam,
    ) -> Self {
        Self {
            duration,
            sample_rate,
            channels,
        }
    }
}
impl<
    DurationParam: crate::nodes::types::Float,
    SampleRateParam: crate::nodes::types::Int,
    ChannelsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for EmptyAudio<DurationParam, SampleRateParam, ChannelsParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("duration".to_string(), self.duration.clone().into());
        output.insert("sample_rate".to_string(), self.sample_rate.clone().into());
        output.insert("channels".to_string(), self.channels.clone().into());
        output
    }
    const NAME: &'static str = "EmptyAudio";
    const DISPLAY_NAME: &'static str = "Empty Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**Join Audio Channels**: Joins left and right mono audio channels into a stereo audio.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JoinAudioChannels<
    AudioLeftParam: crate::nodes::types::Audio,
    AudioRightParam: crate::nodes::types::Audio,
> {
    ///No documentation.
    pub audio_left: AudioLeftParam,
    ///No documentation.
    pub audio_right: AudioRightParam,
}
impl<
    AudioLeftParam: crate::nodes::types::Audio,
    AudioRightParam: crate::nodes::types::Audio,
> JoinAudioChannels<AudioLeftParam, AudioRightParam> {
    /// Create a new node.
    pub fn new(audio_left: AudioLeftParam, audio_right: AudioRightParam) -> Self {
        Self { audio_left, audio_right }
    }
}
impl<
    AudioLeftParam: crate::nodes::types::Audio,
    AudioRightParam: crate::nodes::types::Audio,
> crate::nodes::TypedNode for JoinAudioChannels<AudioLeftParam, AudioRightParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio_left".to_string(), self.audio_left.clone().into());
        output.insert("audio_right".to_string(), self.audio_right.clone().into());
        output
    }
    const NAME: &'static str = "JoinAudioChannels";
    const DISPLAY_NAME: &'static str = "Join Audio Channels";
    const DESCRIPTION: &'static str = "Joins left and right mono audio channels into a stereo audio.";
    const CATEGORY: &'static str = "audio";
}
///**Load Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadAudio {}
impl LoadAudio {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadAudio {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadAudio";
    const DISPLAY_NAME: &'static str = "Load Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**Preview Audio**: No description.
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
    const DISPLAY_NAME: &'static str = "Preview Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<AudioParam: crate::nodes::types::Audio> crate::nodes::TypedOutputNode
for PreviewAudio<AudioParam> {}
///**Record Audio**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RecordAudio {}
impl RecordAudio {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for RecordAudio {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "RecordAudio";
    const DISPLAY_NAME: &'static str = "Record Audio";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
///**Save Audio (FLAC) (Deprecated)**: No description.
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
  - Multiline: false
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
    const DISPLAY_NAME: &'static str = "Save Audio (FLAC) (Deprecated)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudio<AudioParam, FilenamePrefixParam> {}
///**Save Audio (Advanced)**: Saves the input audio to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAudioAdvanced<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///The audio to save.
    pub audio: AudioParam,
    /**The prefix for the file to save. May include formatting tokens such as %date:yyyy-MM-dd%.

**Metadata**:
  - Multiline: false
  - Default: audio/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveAudioAdvanced<AudioParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { audio, filename_prefix }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudioAdvanced<AudioParam, FilenamePrefixParam> {
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
    const NAME: &'static str = "SaveAudioAdvanced";
    const DISPLAY_NAME: &'static str = "Save Audio (Advanced)";
    const DESCRIPTION: &'static str = "Saves the input audio to your ComfyUI output directory.";
    const CATEGORY: &'static str = "audio";
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudioAdvanced<AudioParam, FilenamePrefixParam> {}
///**Save Audio (MP3) (Deprecated)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAudioMP3<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: audio/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveAudioMP3<AudioParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { audio, filename_prefix }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudioMP3<AudioParam, FilenamePrefixParam> {
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
    const NAME: &'static str = "SaveAudioMP3";
    const DISPLAY_NAME: &'static str = "Save Audio (MP3) (Deprecated)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudioMP3<AudioParam, FilenamePrefixParam> {}
///**Save Audio (Opus) (Deprecated)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveAudioOpus<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default: audio/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveAudioOpus<AudioParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { audio, filename_prefix }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for SaveAudioOpus<AudioParam, FilenamePrefixParam> {
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
    const NAME: &'static str = "SaveAudioOpus";
    const DISPLAY_NAME: &'static str = "Save Audio (Opus) (Deprecated)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "audio";
}
impl<
    AudioParam: crate::nodes::types::Audio,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for SaveAudioOpus<AudioParam, FilenamePrefixParam> {}
///**Split Audio Channels**: Separates the audio into left and right channels.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SplitAudioChannels<AudioParam: crate::nodes::types::Audio> {
    ///No documentation.
    pub audio: AudioParam,
}
impl<AudioParam: crate::nodes::types::Audio> SplitAudioChannels<AudioParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam) -> Self {
        Self { audio }
    }
}
impl<AudioParam: crate::nodes::types::Audio> crate::nodes::TypedNode
for SplitAudioChannels<AudioParam> {
    type Output = out::SplitAudioChannelsOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            left: crate::nodes::types::AudioOut::from_dynamic(node_id, 0u32),
            right: crate::nodes::types::AudioOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "SplitAudioChannels";
    const DISPLAY_NAME: &'static str = "Split Audio Channels";
    const DESCRIPTION: &'static str = "Separates the audio into left and right channels.";
    const CATEGORY: &'static str = "audio";
}
///**Trim Audio Duration**: Trim audio tensor into chosen time range.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TrimAudioDuration<
    AudioParam: crate::nodes::types::Audio,
    StartIndexParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub audio: AudioParam,
    /**Start time in seconds, can be negative to count from the end (supports sub-seconds).

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: -18446744073709552000
  - Step: 0.01
*/
    pub start_index: StartIndexParam,
    ///Duration in seconds
    pub duration: DurationParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    StartIndexParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
> TrimAudioDuration<AudioParam, StartIndexParam, DurationParam> {
    /// Create a new node.
    pub fn new(
        audio: AudioParam,
        start_index: StartIndexParam,
        duration: DurationParam,
    ) -> Self {
        Self {
            audio,
            start_index,
            duration,
        }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    StartIndexParam: crate::nodes::types::Float,
    DurationParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for TrimAudioDuration<AudioParam, StartIndexParam, DurationParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("start_index".to_string(), self.start_index.clone().into());
        output.insert("duration".to_string(), self.duration.clone().into());
        output
    }
    const NAME: &'static str = "TrimAudioDuration";
    const DISPLAY_NAME: &'static str = "Trim Audio Duration";
    const DESCRIPTION: &'static str = "Trim audio tensor into chosen time range.";
    const CATEGORY: &'static str = "audio";
}
