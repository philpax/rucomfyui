//!`model_merging` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod model_specific;
///**CLIPMergeAdd**: No description.
#[derive(Clone)]
pub struct ClipMergeAdd<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
> {
    ///No documentation.
    pub clip_1: Clip1,
    ///No documentation.
    pub clip_2: Clip2,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
> ClipMergeAdd<Clip1, Clip2> {
    /// Create a new node.
    pub fn new(clip_1: Clip1, clip_2: Clip2) -> Self {
        Self { clip_1, clip_2 }
    }
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
> crate::nodes::TypedNode for ClipMergeAdd<Clip1, Clip2> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip1".to_string(), self.clip_1.clone().into());
        output.insert("clip2".to_string(), self.clip_2.clone().into());
        output
    }
    const NAME: &'static str = "CLIPMergeAdd";
    const DISPLAY_NAME: &'static str = "CLIPMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSimple**: No description.
#[derive(Clone)]
pub struct ClipMergeSimple<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Ratio: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_1: Clip1,
    ///No documentation.
    pub clip_2: Clip2,
    ///No documentation.
    pub ratio: Ratio,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Ratio: crate::nodes::types::Float,
> ClipMergeSimple<Clip1, Clip2, Ratio> {
    /// Create a new node.
    pub fn new(clip_1: Clip1, clip_2: Clip2, ratio: Ratio) -> Self {
        Self { clip_1, clip_2, ratio }
    }
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipMergeSimple<Clip1, Clip2, Ratio> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip1".to_string(), self.clip_1.clone().into());
        output.insert("clip2".to_string(), self.clip_2.clone().into());
        output.insert("ratio".to_string(), self.ratio.clone().into());
        output
    }
    const NAME: &'static str = "CLIPMergeSimple";
    const DISPLAY_NAME: &'static str = "CLIPMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSubtract**: No description.
#[derive(Clone)]
pub struct ClipMergeSubtract<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_1: Clip1,
    ///No documentation.
    pub clip_2: Clip2,
    ///No documentation.
    pub multiplier: Multiplier,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Multiplier: crate::nodes::types::Float,
> ClipMergeSubtract<Clip1, Clip2, Multiplier> {
    /// Create a new node.
    pub fn new(clip_1: Clip1, clip_2: Clip2, multiplier: Multiplier) -> Self {
        Self { clip_1, clip_2, multiplier }
    }
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipMergeSubtract<Clip1, Clip2, Multiplier> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip1".to_string(), self.clip_1.clone().into());
        output.insert("clip2".to_string(), self.clip_2.clone().into());
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "CLIPMergeSubtract";
    const DISPLAY_NAME: &'static str = "CLIPMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPSave**: No description.
#[derive(Clone)]
pub struct ClipSave<
    Clip: crate::nodes::types::Clip,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Clip: crate::nodes::types::Clip,
    FilenamePrefix: crate::nodes::types::String,
> ClipSave<Clip, FilenamePrefix> {
    /// Create a new node.
    pub fn new(clip: Clip, filename_prefix: FilenamePrefix) -> Self {
        Self { clip, filename_prefix }
    }
}
impl<
    Clip: crate::nodes::types::Clip,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for ClipSave<Clip, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "CLIPSave";
    const DISPLAY_NAME: &'static str = "CLIPSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Clip: crate::nodes::types::Clip,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ClipSave<Clip, FilenamePrefix> {}
///**Save Checkpoint**: No description.
#[derive(Clone)]
pub struct CheckpointSave<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> CheckpointSave<Model, Clip, Vae, FilenamePrefix> {
    /// Create a new node.
    pub fn new(
        model: Model,
        clip: Clip,
        vae: Vae,
        filename_prefix: FilenamePrefix,
    ) -> Self {
        Self {
            model,
            clip,
            vae,
            filename_prefix,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for CheckpointSave<Model, Clip, Vae, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "CheckpointSave";
    const DISPLAY_NAME: &'static str = "Save Checkpoint";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Model: crate::nodes::types::Model,
    Clip: crate::nodes::types::Clip,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for CheckpointSave<Model, Clip, Vae, FilenamePrefix> {}
///**ImageOnlyCheckpointSave**: No description.
#[derive(Clone)]
pub struct ImageOnlyCheckpointSave<
    Model: crate::nodes::types::Model,
    ClipVision: crate::nodes::types::ClipVision,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub clip_vision: ClipVision,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Model: crate::nodes::types::Model,
    ClipVision: crate::nodes::types::ClipVision,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> ImageOnlyCheckpointSave<Model, ClipVision, Vae, FilenamePrefix> {
    /// Create a new node.
    pub fn new(
        model: Model,
        clip_vision: ClipVision,
        vae: Vae,
        filename_prefix: FilenamePrefix,
    ) -> Self {
        Self {
            model,
            clip_vision,
            vae,
            filename_prefix,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    ClipVision: crate::nodes::types::ClipVision,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode
for ImageOnlyCheckpointSave<Model, ClipVision, Vae, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "ImageOnlyCheckpointSave";
    const DISPLAY_NAME: &'static str = "ImageOnlyCheckpointSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Model: crate::nodes::types::Model,
    ClipVision: crate::nodes::types::ClipVision,
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for ImageOnlyCheckpointSave<Model, ClipVision, Vae, FilenamePrefix> {}
///**ModelMergeAdd**: No description.
#[derive(Clone)]
pub struct ModelMergeAdd<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
> ModelMergeAdd<Model1, Model2> {
    /// Create a new node.
    pub fn new(model_1: Model1, model_2: Model2) -> Self {
        Self { model_1, model_2 }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
> crate::nodes::TypedNode for ModelMergeAdd<Model1, Model2> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeAdd";
    const DISPLAY_NAME: &'static str = "ModelMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeBlocks**: No description.
#[derive(Clone)]
pub struct ModelMergeBlocks<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Input: crate::nodes::types::Float,
    Middle: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    ///No documentation.
    pub input: Input,
    ///No documentation.
    pub middle: Middle,
    ///No documentation.
    pub out: Out,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Input: crate::nodes::types::Float,
    Middle: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> ModelMergeBlocks<Model1, Model2, Input, Middle, Out> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        input: Input,
        middle: Middle,
        out: Out,
    ) -> Self {
        Self {
            model_1,
            model_2,
            input,
            middle,
            out,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Input: crate::nodes::types::Float,
    Middle: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeBlocks<Model1, Model2, Input, Middle, Out> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("input".to_string(), self.input.clone().into());
        output.insert("middle".to_string(), self.middle.clone().into());
        output.insert("out".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeBlocks";
    const DISPLAY_NAME: &'static str = "ModelMergeBlocks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSimple**: No description.
#[derive(Clone)]
pub struct ModelMergeSimple<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    ///No documentation.
    pub ratio: Ratio,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> ModelMergeSimple<Model1, Model2, Ratio> {
    /// Create a new node.
    pub fn new(model_1: Model1, model_2: Model2, ratio: Ratio) -> Self {
        Self { model_1, model_2, ratio }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeSimple<Model1, Model2, Ratio> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("ratio".to_string(), self.ratio.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSimple";
    const DISPLAY_NAME: &'static str = "ModelMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSubtract**: No description.
#[derive(Clone)]
pub struct ModelMergeSubtract<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    ///No documentation.
    pub multiplier: Multiplier,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> ModelMergeSubtract<Model1, Model2, Multiplier> {
    /// Create a new node.
    pub fn new(model_1: Model1, model_2: Model2, multiplier: Multiplier) -> Self {
        Self {
            model_1,
            model_2,
            multiplier,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeSubtract<Model1, Model2, Multiplier> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("multiplier".to_string(), self.multiplier.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSubtract";
    const DISPLAY_NAME: &'static str = "ModelMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelSave**: No description.
#[derive(Clone)]
pub struct ModelSave<
    Model: crate::nodes::types::Model,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Model: crate::nodes::types::Model,
    FilenamePrefix: crate::nodes::types::String,
> ModelSave<Model, FilenamePrefix> {
    /// Create a new node.
    pub fn new(model: Model, filename_prefix: FilenamePrefix) -> Self {
        Self { model, filename_prefix }
    }
}
impl<
    Model: crate::nodes::types::Model,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for ModelSave<Model, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "ModelSave";
    const DISPLAY_NAME: &'static str = "ModelSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Model: crate::nodes::types::Model,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ModelSave<Model, FilenamePrefix> {}
///**VAESave**: No description.
#[derive(Clone)]
pub struct VaeSave<
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> {
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
impl<
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> VaeSave<Vae, FilenamePrefix> {
    /// Create a new node.
    pub fn new(vae: Vae, filename_prefix: FilenamePrefix) -> Self {
        Self { vae, filename_prefix }
    }
}
impl<
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedNode for VaeSave<Vae, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae".to_string(), self.vae.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "VAESave";
    const DISPLAY_NAME: &'static str = "VAESave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for VaeSave<Vae, FilenamePrefix> {}
