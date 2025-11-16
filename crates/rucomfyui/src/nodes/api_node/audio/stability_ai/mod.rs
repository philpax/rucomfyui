//!`Stability AI` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Stability AI Audio Inpaint**: Transforms part of existing audio sample using text instructions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityAudioInpaint<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    StepsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    MaskStartParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    MaskEndParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Audio must be between 6 and 190 seconds long.
    pub audio: AudioParam,
    /**Controls the duration in seconds of the generated audio.

**Metadata**:
  - Default: 190
  - Max: 190
  - Min: 1
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**The random seed used for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Controls the number of sampling steps.

**Metadata**:
  - Default: 8
  - Max: 8
  - Min: 4
  - Step: 1
*/
    pub steps: Option<StepsParam>,
    /**No documentation.

**Metadata**:
  - Default: 30
  - Max: 190
  - Min: 0
  - Step: 1
*/
    pub mask_start: Option<MaskStartParam>,
    /**No documentation.

**Metadata**:
  - Default: 190
  - Max: 190
  - Min: 0
  - Step: 1
*/
    pub mask_end: Option<MaskEndParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    MaskStartParam: crate::nodes::types::Int,
    MaskEndParam: crate::nodes::types::Int,
> StabilityAudioInpaint<
    PromptParam,
    AudioParam,
    DurationParam,
    SeedParam,
    StepsParam,
    MaskStartParam,
    MaskEndParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        audio: AudioParam,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
        steps: Option<StepsParam>,
        mask_start: Option<MaskStartParam>,
        mask_end: Option<MaskEndParam>,
    ) -> Self {
        Self {
            prompt,
            audio,
            duration,
            seed,
            steps,
            mask_start,
            mask_end,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    MaskStartParam: crate::nodes::types::Int,
    MaskEndParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for StabilityAudioInpaint<
    PromptParam,
    AudioParam,
    DurationParam,
    SeedParam,
    StepsParam,
    MaskStartParam,
    MaskEndParam,
> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.steps {
            output.insert("steps".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask_start {
            output.insert("mask_start".to_string(), v.clone().into());
        }
        if let Some(v) = &self.mask_end {
            output.insert("mask_end".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityAudioInpaint";
    const DISPLAY_NAME: &'static str = "Stability AI Audio Inpaint";
    const DESCRIPTION: &'static str = "Transforms part of existing audio sample using text instructions.";
    const CATEGORY: &'static str = "api node/audio/Stability AI";
}
///**Stability AI Audio To Audio**: Transforms existing audio samples into new high-quality compositions using text instructions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityAudioToAudio<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    StepsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    StrengthParam: crate::nodes::types::Float = crate::nodes::types::FloatOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    ///Audio must be between 6 and 190 seconds long.
    pub audio: AudioParam,
    /**Controls the duration in seconds of the generated audio.

**Metadata**:
  - Default: 190
  - Max: 190
  - Min: 1
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**The random seed used for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Controls the number of sampling steps.

**Metadata**:
  - Default: 8
  - Max: 8
  - Min: 4
  - Step: 1
*/
    pub steps: Option<StepsParam>,
    /**Parameter controls how much influence the audio parameter has on the generated audio.

**Metadata**:
  - Default: 1
  - Display: slider
  - Max: 1
  - Min: 0.01
  - Step: 0.01
*/
    pub strength: Option<StrengthParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> StabilityAudioToAudio<
    PromptParam,
    AudioParam,
    DurationParam,
    SeedParam,
    StepsParam,
    StrengthParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        audio: AudioParam,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
        steps: Option<StepsParam>,
        strength: Option<StrengthParam>,
    ) -> Self {
        Self {
            prompt,
            audio,
            duration,
            seed,
            steps,
            strength,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    AudioParam: crate::nodes::types::Audio,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    StrengthParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for StabilityAudioToAudio<
    PromptParam,
    AudioParam,
    DurationParam,
    SeedParam,
    StepsParam,
    StrengthParam,
> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("audio".to_string(), self.audio.clone().into());
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.steps {
            output.insert("steps".to_string(), v.clone().into());
        }
        if let Some(v) = &self.strength {
            output.insert("strength".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityAudioToAudio";
    const DISPLAY_NAME: &'static str = "Stability AI Audio To Audio";
    const DESCRIPTION: &'static str = "Transforms existing audio samples into new high-quality compositions using text instructions.";
    const CATEGORY: &'static str = "api node/audio/Stability AI";
}
///**Stability AI Text To Audio**: Generates high-quality music and sound effects from text descriptions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StabilityTextToAudio<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    StepsParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Controls the duration in seconds of the generated audio.

**Metadata**:
  - Default: 190
  - Max: 190
  - Min: 1
  - Step: 1
*/
    pub duration: Option<DurationParam>,
    /**The random seed used for generation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 4294967294
  - Min: 0
  - Step: 1
*/
    pub seed: Option<SeedParam>,
    /**Controls the number of sampling steps.

**Metadata**:
  - Default: 8
  - Max: 8
  - Min: 4
  - Step: 1
*/
    pub steps: Option<StepsParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> StabilityTextToAudio<PromptParam, DurationParam, SeedParam, StepsParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        duration: Option<DurationParam>,
        seed: Option<SeedParam>,
        steps: Option<StepsParam>,
    ) -> Self {
        Self {
            prompt,
            duration,
            seed,
            steps,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    DurationParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for StabilityTextToAudio<PromptParam, DurationParam, SeedParam, StepsParam> {
    type Output = crate::nodes::types::AudioOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.duration {
            output.insert("duration".to_string(), v.clone().into());
        }
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.steps {
            output.insert("steps".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "StabilityTextToAudio";
    const DISPLAY_NAME: &'static str = "Stability AI Text To Audio";
    const DESCRIPTION: &'static str = "Generates high-quality music and sound effects from text descriptions.";
    const CATEGORY: &'static str = "api node/audio/Stability AI";
}
