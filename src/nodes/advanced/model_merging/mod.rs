//!`model_merging` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod model_specific;
///**CLIPMergeAdd**
pub struct ClipMergeAdd<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
> {
    ///No documentation.
    pub clip_1: Clip1,
    ///No documentation.
    pub clip_2: Clip2,
}
///Output for [`ClipMergeAdd`].
#[derive(Clone)]
pub struct ClipMergeAddOutput {
    ///No documentation.
    pub clip: crate::nodes::types::ClipOut,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
> crate::nodes::TypedNode for ClipMergeAdd<Clip1, Clip2> {
    type Output = ClipMergeAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPMergeAdd";
    const DISPLAY_NAME: &'static str = "CLIPMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSimple**
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
///Output for [`ClipMergeSimple`].
#[derive(Clone)]
pub struct ClipMergeSimpleOutput {
    ///No documentation.
    pub clip: crate::nodes::types::ClipOut,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipMergeSimple<Clip1, Clip2, Ratio> {
    type Output = ClipMergeSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPMergeSimple";
    const DISPLAY_NAME: &'static str = "CLIPMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPMergeSubtract**
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
///Output for [`ClipMergeSubtract`].
#[derive(Clone)]
pub struct ClipMergeSubtractOutput {
    ///No documentation.
    pub clip: crate::nodes::types::ClipOut,
}
impl<
    Clip1: crate::nodes::types::Clip,
    Clip2: crate::nodes::types::Clip,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipMergeSubtract<Clip1, Clip2, Multiplier> {
    type Output = ClipMergeSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPMergeSubtract";
    const DISPLAY_NAME: &'static str = "CLIPMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPSave**
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
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "CLIPSave";
    const DISPLAY_NAME: &'static str = "CLIPSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Clip: crate::nodes::types::Clip,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ClipSave<Clip, FilenamePrefix> {}
///**Save Checkpoint**
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
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
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
///**ModelMergeAdd**
pub struct ModelMergeAdd<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
}
///Output for [`ModelMergeAdd`].
#[derive(Clone)]
pub struct ModelMergeAddOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
> crate::nodes::TypedNode for ModelMergeAdd<Model1, Model2> {
    type Output = ModelMergeAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelMergeAdd";
    const DISPLAY_NAME: &'static str = "ModelMergeAdd";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeBlocks**
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
///Output for [`ModelMergeBlocks`].
#[derive(Clone)]
pub struct ModelMergeBlocksOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Input: crate::nodes::types::Float,
    Middle: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeBlocks<Model1, Model2, Input, Middle, Out> {
    type Output = ModelMergeBlocksOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelMergeBlocks";
    const DISPLAY_NAME: &'static str = "ModelMergeBlocks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSimple**
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
///Output for [`ModelMergeSimple`].
#[derive(Clone)]
pub struct ModelMergeSimpleOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Ratio: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeSimple<Model1, Model2, Ratio> {
    type Output = ModelMergeSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelMergeSimple";
    const DISPLAY_NAME: &'static str = "ModelMergeSimple";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeSubtract**
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
///Output for [`ModelMergeSubtract`].
#[derive(Clone)]
pub struct ModelMergeSubtractOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    Multiplier: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeSubtract<Model1, Model2, Multiplier> {
    type Output = ModelMergeSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelMergeSubtract";
    const DISPLAY_NAME: &'static str = "ModelMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelSave**
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
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "ModelSave";
    const DISPLAY_NAME: &'static str = "ModelSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Model: crate::nodes::types::Model,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ModelSave<Model, FilenamePrefix> {}
///**VAESave**
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
    type Output = ();
    fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}
    const NAME: &'static str = "VAESave";
    const DISPLAY_NAME: &'static str = "VAESave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
impl<
    Vae: crate::nodes::types::Vae,
    FilenamePrefix: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for VaeSave<Vae, FilenamePrefix> {}
