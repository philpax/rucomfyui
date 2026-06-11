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
    ///Output for [`LTXVReferenceAudio`](super::LTXVReferenceAudio).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LTXVReferenceAudioOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**LTXV Reference Audio (ID-LoRA)**: Set reference audio for ID-LoRA speaker identity transfer. Encodes a reference audio clip into the conditioning and optionally patches the model with identity guidance (extra forward pass without reference, amplifying the speaker identity effect).
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVReferenceAudio<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ReferenceAudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
    IdentityGuidanceScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///Reference audio clip whose speaker identity to transfer. ~5 seconds recommended (training duration). Shorter or longer clips may degrade voice identity transfer.
    pub reference_audio: ReferenceAudioParam,
    ///LTXV Audio VAE for encoding.
    pub audio_vae: AudioVaeParam,
    /**Strength of identity guidance. Runs an extra forward pass without reference each step to amplify speaker identity. Set to 0 to disable (no extra pass).

**Metadata**:
  - Default: 3
  - Max: 100
  - Min: 0
  - Round: 0.01
  - Step: 0.01
*/
    pub identity_guidance_scale: IdentityGuidanceScaleParam,
    /**Start of the sigma range where identity guidance is active.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start_percent: StartPercentParam,
    /**End of the sigma range where identity guidance is active.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end_percent: EndPercentParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ReferenceAudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
    IdentityGuidanceScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> LTXVReferenceAudio<
    ModelParam,
    PositiveParam,
    NegativeParam,
    ReferenceAudioParam,
    AudioVaeParam,
    IdentityGuidanceScaleParam,
    StartPercentParam,
    EndPercentParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        positive: PositiveParam,
        negative: NegativeParam,
        reference_audio: ReferenceAudioParam,
        audio_vae: AudioVaeParam,
        identity_guidance_scale: IdentityGuidanceScaleParam,
        start_percent: StartPercentParam,
        end_percent: EndPercentParam,
    ) -> Self {
        Self {
            model,
            positive,
            negative,
            reference_audio,
            audio_vae,
            identity_guidance_scale,
            start_percent,
            end_percent,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    ReferenceAudioParam: crate::nodes::types::Audio,
    AudioVaeParam: crate::nodes::types::Vae,
    IdentityGuidanceScaleParam: crate::nodes::types::Float,
    StartPercentParam: crate::nodes::types::Float,
    EndPercentParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LTXVReferenceAudio<
    ModelParam,
    PositiveParam,
    NegativeParam,
    ReferenceAudioParam,
    AudioVaeParam,
    IdentityGuidanceScaleParam,
    StartPercentParam,
    EndPercentParam,
> {
    type Output = out::LTXVReferenceAudioOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output
            .insert("reference_audio".to_string(), self.reference_audio.clone().into());
        output.insert("audio_vae".to_string(), self.audio_vae.clone().into());
        output
            .insert(
                "identity_guidance_scale".to_string(),
                self.identity_guidance_scale.clone().into(),
            );
        output.insert("start_percent".to_string(), self.start_percent.clone().into());
        output.insert("end_percent".to_string(), self.end_percent.clone().into());
        output
    }
    const NAME: &'static str = "LTXVReferenceAudio";
    const DISPLAY_NAME: &'static str = "LTXV Reference Audio (ID-LoRA)";
    const DESCRIPTION: &'static str = "Set reference audio for ID-LoRA speaker identity transfer. Encodes a reference audio clip into the conditioning and optionally patches the model with identity guidance (extra forward pass without reference, amplifying the speaker identity effect).";
    const CATEGORY: &'static str = "model/conditioning/audio";
}
