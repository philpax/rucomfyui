//!`model_merging` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod model_specific;
///**CLIPMergeAdd**: No description.
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
        output.insert("clip1".to_string(), self.clip_1.to_workflow_input());
        output.insert("clip2".to_string(), self.clip_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPMergeAdd";
    const DISPLAY_NAME: &'static str = "CLIPMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSimple**: No description.
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
        output.insert("clip1".to_string(), self.clip_1.to_workflow_input());
        output.insert("clip2".to_string(), self.clip_2.to_workflow_input());
        output.insert("ratio".to_string(), self.ratio.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPMergeSimple";
    const DISPLAY_NAME: &'static str = "CLIPMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSubtract**: No description.
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
        output.insert("clip1".to_string(), self.clip_1.to_workflow_input());
        output.insert("clip2".to_string(), self.clip_2.to_workflow_input());
        output.insert("multiplier".to_string(), self.multiplier.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPMergeSubtract";
    const DISPLAY_NAME: &'static str = "CLIPMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPSave**: No description.
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
> crate::nodes::TypedNode for ClipSave<Clip, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output
            .insert(
                "filename_prefix".to_string(),
                self.filename_prefix.to_workflow_input(),
            );
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
> crate::nodes::TypedNode for CheckpointSave<Model, Clip, Vae, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output
            .insert(
                "filename_prefix".to_string(),
                self.filename_prefix.to_workflow_input(),
            );
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
///**ModelMergeAdd**: No description.
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
        output.insert("model1".to_string(), self.model_1.to_workflow_input());
        output.insert("model2".to_string(), self.model_2.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelMergeAdd";
    const DISPLAY_NAME: &'static str = "ModelMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeBlocks**: No description.
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
        output.insert("model1".to_string(), self.model_1.to_workflow_input());
        output.insert("model2".to_string(), self.model_2.to_workflow_input());
        output.insert("input".to_string(), self.input.to_workflow_input());
        output.insert("middle".to_string(), self.middle.to_workflow_input());
        output.insert("out".to_string(), self.out.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelMergeBlocks";
    const DISPLAY_NAME: &'static str = "ModelMergeBlocks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSimple**: No description.
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
        output.insert("model1".to_string(), self.model_1.to_workflow_input());
        output.insert("model2".to_string(), self.model_2.to_workflow_input());
        output.insert("ratio".to_string(), self.ratio.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelMergeSimple";
    const DISPLAY_NAME: &'static str = "ModelMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSubtract**: No description.
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
        output.insert("model1".to_string(), self.model_1.to_workflow_input());
        output.insert("model2".to_string(), self.model_2.to_workflow_input());
        output.insert("multiplier".to_string(), self.multiplier.to_workflow_input());
        output
    }
    const NAME: &'static str = "ModelMergeSubtract";
    const DISPLAY_NAME: &'static str = "ModelMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelSave**: No description.
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
> crate::nodes::TypedNode for ModelSave<Model, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.to_workflow_input());
        output
            .insert(
                "filename_prefix".to_string(),
                self.filename_prefix.to_workflow_input(),
            );
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
> crate::nodes::TypedNode for VaeSave<Vae, FilenamePrefix> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae".to_string(), self.vae.to_workflow_input());
        output
            .insert(
                "filename_prefix".to_string(),
                self.filename_prefix.to_workflow_input(),
            );
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
