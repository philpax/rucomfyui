//!model_merging
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod model_specific;
///**CLIPMergeAdd**
pub struct ClipMergeAdd<Clip1: crate::nodes::Clip, Clip2: crate::nodes::Clip> {
    ///No documentation.
    pub clip_1: Clip1,
    ///No documentation.
    pub clip_2: Clip2,
}
///Output for [`ClipMergeAdd`].
#[derive(Clone)]
pub struct ClipMergeAddOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl<Clip1: crate::nodes::Clip, Clip2: crate::nodes::Clip> crate::nodes::TypedNode
for ClipMergeAdd<Clip1, Clip2> {
    type Output = ClipMergeAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut {
                node_id,
                slot: 0u32,
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
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Ratio: crate::nodes::Float,
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
    pub clip: crate::nodes::ClipOut,
}
impl<
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Ratio: crate::nodes::Float,
> crate::nodes::TypedNode for ClipMergeSimple<Clip1, Clip2, Ratio> {
    type Output = ClipMergeSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut {
                node_id,
                slot: 0u32,
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
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Multiplier: crate::nodes::Float,
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
    pub clip: crate::nodes::ClipOut,
}
impl<
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Multiplier: crate::nodes::Float,
> crate::nodes::TypedNode for ClipMergeSubtract<Clip1, Clip2, Multiplier> {
    type Output = ClipMergeSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPMergeSubtract";
    const DISPLAY_NAME: &'static str = "CLIPMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**CLIPSave**
pub struct ClipSave<Clip: crate::nodes::Clip, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///Output for [`ClipSave`].
#[derive(Clone)]
pub struct ClipSaveOutput {}
impl<
    Clip: crate::nodes::Clip,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for ClipSave<Clip, FilenamePrefix> {
    type Output = ClipSaveOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "CLIPSave";
    const DISPLAY_NAME: &'static str = "CLIPSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**Save Checkpoint**
pub struct CheckpointSave<
    Model: crate::nodes::Model,
    Clip: crate::nodes::Clip,
    Vae: crate::nodes::Vae,
    FilenamePrefix: crate::nodes::String,
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
///Output for [`CheckpointSave`].
#[derive(Clone)]
pub struct CheckpointSaveOutput {}
impl<
    Model: crate::nodes::Model,
    Clip: crate::nodes::Clip,
    Vae: crate::nodes::Vae,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for CheckpointSave<Model, Clip, Vae, FilenamePrefix> {
    type Output = CheckpointSaveOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "CheckpointSave";
    const DISPLAY_NAME: &'static str = "Save Checkpoint";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelMergeAdd**
pub struct ModelMergeAdd<Model1: crate::nodes::Model, Model2: crate::nodes::Model> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
}
///Output for [`ModelMergeAdd`].
#[derive(Clone)]
pub struct ModelMergeAddOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model1: crate::nodes::Model, Model2: crate::nodes::Model> crate::nodes::TypedNode
for ModelMergeAdd<Model1, Model2> {
    type Output = ModelMergeAddOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
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
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Input: crate::nodes::Float,
    Middle: crate::nodes::Float,
    Out: crate::nodes::Float,
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
    pub model: crate::nodes::ModelOut,
}
impl<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Input: crate::nodes::Float,
    Middle: crate::nodes::Float,
    Out: crate::nodes::Float,
> crate::nodes::TypedNode for ModelMergeBlocks<Model1, Model2, Input, Middle, Out> {
    type Output = ModelMergeBlocksOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
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
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Ratio: crate::nodes::Float,
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
    pub model: crate::nodes::ModelOut,
}
impl<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Ratio: crate::nodes::Float,
> crate::nodes::TypedNode for ModelMergeSimple<Model1, Model2, Ratio> {
    type Output = ModelMergeSimpleOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
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
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Multiplier: crate::nodes::Float,
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
    pub model: crate::nodes::ModelOut,
}
impl<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Multiplier: crate::nodes::Float,
> crate::nodes::TypedNode for ModelMergeSubtract<Model1, Model2, Multiplier> {
    type Output = ModelMergeSubtractOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ModelMergeSubtract";
    const DISPLAY_NAME: &'static str = "ModelMergeSubtract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**ModelSave**
pub struct ModelSave<Model: crate::nodes::Model, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///Output for [`ModelSave`].
#[derive(Clone)]
pub struct ModelSaveOutput {}
impl<
    Model: crate::nodes::Model,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for ModelSave<Model, FilenamePrefix> {
    type Output = ModelSaveOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "ModelSave";
    const DISPLAY_NAME: &'static str = "ModelSave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
///**VAESave**
pub struct VaeSave<Vae: crate::nodes::Vae, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///Output for [`VaeSave`].
#[derive(Clone)]
pub struct VaeSaveOutput {}
impl<
    Vae: crate::nodes::Vae,
    FilenamePrefix: crate::nodes::String,
> crate::nodes::TypedNode for VaeSave<Vae, FilenamePrefix> {
    type Output = VaeSaveOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {}
    }
    const NAME: &'static str = "VAESave";
    const DISPLAY_NAME: &'static str = "VAESave";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging";
}
