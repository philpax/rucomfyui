//!`ElevenLabs` definitions/categories.
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
    ///Output for [`ElevenLabsSpeechToText`](super::ElevenLabsSpeechToText).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ElevenLabsSpeechToTextOutput {
        ///No documentation.
        pub text: crate::nodes::types::StringOut,
        ///No documentation.
        pub language_code: crate::nodes::types::StringOut,
        ///No documentation.
        pub words_json: crate::nodes::types::StringOut,
    }
}
///**ElevenLabs Voice Isolation**: Remove background noise from audio, isolating vocals or speech.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsAudioIsolation<AudioParam: crate::nodes::types::Audio> {
    ///Audio to process for background noise removal.
    pub audio: AudioParam,
}
impl<AudioParam: crate::nodes::types::Audio> ElevenLabsAudioIsolation<AudioParam> {
    /// Create a new node.
    pub fn new(audio: AudioParam) -> Self {
        Self { audio }
    }
}
impl<AudioParam: crate::nodes::types::Audio> crate::nodes::TypedNode
for ElevenLabsAudioIsolation<AudioParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output
    }
    const NAME: &'static str = "ElevenLabsAudioIsolation";
    const DISPLAY_NAME: &'static str = "ElevenLabs Voice Isolation";
    const DESCRIPTION: &'static str = "Remove background noise from audio, isolating vocals or speech.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Instant Voice Clone**: Create a cloned voice from audio samples. Provide 1-8 audio recordings of the voice to clone.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsInstantVoiceClone<
    RemoveBackgroundNoiseParam: crate::nodes::types::Boolean,
> {
    /**Remove background noise from voice samples using audio isolation.

**Metadata**:
  - Default: false
*/
    pub remove_background_noise: RemoveBackgroundNoiseParam,
}
impl<
    RemoveBackgroundNoiseParam: crate::nodes::types::Boolean,
> ElevenLabsInstantVoiceClone<RemoveBackgroundNoiseParam> {
    /// Create a new node.
    pub fn new(remove_background_noise: RemoveBackgroundNoiseParam) -> Self {
        Self { remove_background_noise }
    }
}
impl<RemoveBackgroundNoiseParam: crate::nodes::types::Boolean> crate::nodes::TypedNode
for ElevenLabsInstantVoiceClone<RemoveBackgroundNoiseParam> {
    type Output = crate::nodes::types::ElevenLabsVoiceOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "remove_background_noise".to_string(),
                self.remove_background_noise.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ElevenLabsInstantVoiceClone";
    const DISPLAY_NAME: &'static str = "ElevenLabs Instant Voice Clone";
    const DESCRIPTION: &'static str = "Create a cloned voice from audio samples. Provide 1-8 audio recordings of the voice to clone.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Speech to Speech**: Transform speech from one voice to another while preserving the original content and emotion.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsSpeechToSpeech<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    AudioParam: crate::nodes::types::Audio,
    StabilityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RemoveBackgroundNoiseParam: crate::nodes::types::Boolean,
> {
    ///Target voice for the transformation. Connect from Voice Selector or Instant Voice Clone.
    pub voice: VoiceParam,
    ///Source audio to transform.
    pub audio: AudioParam,
    /**Voice stability. Lower values give broader emotional range, higher values produce more consistent but potentially monotonous speech.

**Metadata**:
  - Default: 0.5
  - Display: slider
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub stability: StabilityParam,
    /**Seed for reproducibility.

**Metadata**:
  - Default: 0
  - Max: 4294967295
  - Min: 0
*/
    pub seed: SeedParam,
    /**Remove background noise from input audio using audio isolation.

**Metadata**:
  - Default: false
*/
    pub remove_background_noise: RemoveBackgroundNoiseParam,
}
impl<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    AudioParam: crate::nodes::types::Audio,
    StabilityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RemoveBackgroundNoiseParam: crate::nodes::types::Boolean,
> ElevenLabsSpeechToSpeech<
    VoiceParam,
    AudioParam,
    StabilityParam,
    SeedParam,
    RemoveBackgroundNoiseParam,
> {
    /// Create a new node.
    pub fn new(
        voice: VoiceParam,
        audio: AudioParam,
        stability: StabilityParam,
        seed: SeedParam,
        remove_background_noise: RemoveBackgroundNoiseParam,
    ) -> Self {
        Self {
            voice,
            audio,
            stability,
            seed,
            remove_background_noise,
        }
    }
}
impl<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    AudioParam: crate::nodes::types::Audio,
    StabilityParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    RemoveBackgroundNoiseParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for ElevenLabsSpeechToSpeech<
    VoiceParam,
    AudioParam,
    StabilityParam,
    SeedParam,
    RemoveBackgroundNoiseParam,
> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("voice".to_string(), self.voice.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("stability".to_string(), self.stability.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "remove_background_noise".to_string(),
                self.remove_background_noise.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ElevenLabsSpeechToSpeech";
    const DISPLAY_NAME: &'static str = "ElevenLabs Speech to Speech";
    const DESCRIPTION: &'static str = "Transform speech from one voice to another while preserving the original content and emotion.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Speech to Text**: Transcribe audio to text. Supports automatic language detection, speaker diarization, and audio event tagging.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsSpeechToText<
    AudioParam: crate::nodes::types::Audio,
    LanguageCodeParam: crate::nodes::types::String,
    NumSpeakersParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///Audio to transcribe.
    pub audio: AudioParam,
    /**ISO-639-1 or ISO-639-3 language code (e.g., 'en', 'es', 'fra'). Leave empty for automatic detection.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub language_code: LanguageCodeParam,
    /**Maximum number of speakers to predict. Set to 0 for automatic detection.

**Metadata**:
  - Default: 0
  - Display: slider
  - Max: 32
  - Min: 0
*/
    pub num_speakers: NumSpeakersParam,
    /**Seed for reproducibility (determinism not guaranteed).

**Metadata**:
  - Default: 1
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    AudioParam: crate::nodes::types::Audio,
    LanguageCodeParam: crate::nodes::types::String,
    NumSpeakersParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> ElevenLabsSpeechToText<AudioParam, LanguageCodeParam, NumSpeakersParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        audio: AudioParam,
        language_code: LanguageCodeParam,
        num_speakers: NumSpeakersParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            audio,
            language_code,
            num_speakers,
            seed,
        }
    }
}
impl<
    AudioParam: crate::nodes::types::Audio,
    LanguageCodeParam: crate::nodes::types::String,
    NumSpeakersParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ElevenLabsSpeechToText<AudioParam, LanguageCodeParam, NumSpeakersParam, SeedParam> {
    type Output = out::ElevenLabsSpeechToTextOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            text: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            language_code: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
            words_json: crate::nodes::types::StringOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("audio".to_string(), self.audio.clone().into());
        output.insert("language_code".to_string(), self.language_code.clone().into());
        output.insert("num_speakers".to_string(), self.num_speakers.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ElevenLabsSpeechToText";
    const DISPLAY_NAME: &'static str = "ElevenLabs Speech to Text";
    const DESCRIPTION: &'static str = "Transcribe audio to text. Supports automatic language detection, speaker diarization, and audio event tagging.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Text to Dialogue**: Generate multi-speaker dialogue from text. Each dialogue entry has its own text and voice.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsTextToDialogue<
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**Voice stability. Lower values give broader emotional range, higher values produce more consistent but potentially monotonous speech.

**Metadata**:
  - Default: 0.5
  - Display: slider
  - Max: 1
  - Min: 0
  - Step: 0.5
*/
    pub stability: StabilityParam,
    /**ISO-639-1 or ISO-639-3 language code (e.g., 'en', 'es', 'fra'). Leave empty for automatic detection.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub language_code: LanguageCodeParam,
    /**Seed for reproducibility.

**Metadata**:
  - Default: 1
  - Max: 4294967295
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> ElevenLabsTextToDialogue<StabilityParam, LanguageCodeParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        stability: StabilityParam,
        language_code: LanguageCodeParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            stability,
            language_code,
            seed,
        }
    }
}
impl<
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ElevenLabsTextToDialogue<StabilityParam, LanguageCodeParam, SeedParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("stability".to_string(), self.stability.clone().into());
        output.insert("language_code".to_string(), self.language_code.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ElevenLabsTextToDialogue";
    const DISPLAY_NAME: &'static str = "ElevenLabs Text to Dialogue";
    const DESCRIPTION: &'static str = "Generate multi-speaker dialogue from text. Each dialogue entry has its own text and voice.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Text to Sound Effects**: Generate sound effects from text descriptions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsTextToSoundEffects<TextParam: crate::nodes::types::String> {
    /**Text description of the sound effect to generate.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub text: TextParam,
}
impl<TextParam: crate::nodes::types::String> ElevenLabsTextToSoundEffects<TextParam> {
    /// Create a new node.
    pub fn new(text: TextParam) -> Self {
        Self { text }
    }
}
impl<TextParam: crate::nodes::types::String> crate::nodes::TypedNode
for ElevenLabsTextToSoundEffects<TextParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("text".to_string(), self.text.clone().into());
        output
    }
    const NAME: &'static str = "ElevenLabsTextToSoundEffects";
    const DISPLAY_NAME: &'static str = "ElevenLabs Text to Sound Effects";
    const DESCRIPTION: &'static str = "Generate sound effects from text descriptions.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Text to Speech**: Convert text to speech.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsTextToSpeech<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    TextParam: crate::nodes::types::String,
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///Voice to use for speech synthesis. Connect from Voice Selector or Instant Voice Clone.
    pub voice: VoiceParam,
    /**The text to convert to speech.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub text: TextParam,
    /**Voice stability. Lower values give broader emotional range, higher values produce more consistent but potentially monotonous speech.

**Metadata**:
  - Default: 0.5
  - Display: slider
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub stability: StabilityParam,
    /**ISO-639-1 or ISO-639-3 language code (e.g., 'en', 'es', 'fra'). Leave empty for automatic detection.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub language_code: LanguageCodeParam,
    /**Seed for reproducibility (determinism not guaranteed).

**Metadata**:
  - Default: 1
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    TextParam: crate::nodes::types::String,
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> ElevenLabsTextToSpeech<
    VoiceParam,
    TextParam,
    StabilityParam,
    LanguageCodeParam,
    SeedParam,
> {
    /// Create a new node.
    pub fn new(
        voice: VoiceParam,
        text: TextParam,
        stability: StabilityParam,
        language_code: LanguageCodeParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            voice,
            text,
            stability,
            language_code,
            seed,
        }
    }
}
impl<
    VoiceParam: crate::nodes::types::ElevenLabsVoice,
    TextParam: crate::nodes::types::String,
    StabilityParam: crate::nodes::types::Float,
    LanguageCodeParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ElevenLabsTextToSpeech<
    VoiceParam,
    TextParam,
    StabilityParam,
    LanguageCodeParam,
    SeedParam,
> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("voice".to_string(), self.voice.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("stability".to_string(), self.stability.clone().into());
        output.insert("language_code".to_string(), self.language_code.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ElevenLabsTextToSpeech";
    const DISPLAY_NAME: &'static str = "ElevenLabs Text to Speech";
    const DESCRIPTION: &'static str = "Convert text to speech.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
///**ElevenLabs Voice Selector**: Select a predefined ElevenLabs voice for text-to-speech generation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ElevenLabsVoiceSelector {}
impl ElevenLabsVoiceSelector {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for ElevenLabsVoiceSelector {
    type Output = crate::nodes::types::ElevenLabsVoiceOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "ElevenLabsVoiceSelector";
    const DISPLAY_NAME: &'static str = "ElevenLabs Voice Selector";
    const DESCRIPTION: &'static str = "Select a predefined ElevenLabs voice for text-to-speech generation.";
    const CATEGORY: &'static str = "partner/audio/ElevenLabs";
}
