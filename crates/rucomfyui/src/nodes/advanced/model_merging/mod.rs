//!`model_merging` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod model_specific;
///**CLIPMergeAdd**: No description.
#[derive(Clone)]
pub struct ClipMergeAdd<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
> {
    ///No documentation.
    pub clip_1: Clip1Param,
    ///No documentation.
    pub clip_2: Clip2Param,
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
> ClipMergeAdd<Clip1Param, Clip2Param> {
    /// Create a new node.
    pub fn new(clip_1: Clip1Param, clip_2: Clip2Param) -> Self {
        Self { clip_1, clip_2 }
    }
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
> crate::nodes::TypedNode for ClipMergeAdd<Clip1Param, Clip2Param> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    RatioParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_1: Clip1Param,
    ///No documentation.
    pub clip_2: Clip2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub ratio: RatioParam,
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    RatioParam: crate::nodes::types::Float,
> ClipMergeSimple<Clip1Param, Clip2Param, RatioParam> {
    /// Create a new node.
    pub fn new(clip_1: Clip1Param, clip_2: Clip2Param, ratio: RatioParam) -> Self {
        Self { clip_1, clip_2, ratio }
    }
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    RatioParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ClipMergeSimple<Clip1Param, Clip2Param, RatioParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    MultiplierParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip_1: Clip1Param,
    ///No documentation.
    pub clip_2: Clip2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub multiplier: MultiplierParam,
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    MultiplierParam: crate::nodes::types::Float,
> ClipMergeSubtract<Clip1Param, Clip2Param, MultiplierParam> {
    /// Create a new node.
    pub fn new(
        clip_1: Clip1Param,
        clip_2: Clip2Param,
        multiplier: MultiplierParam,
    ) -> Self {
        Self { clip_1, clip_2, multiplier }
    }
}
impl<
    Clip1Param: crate::nodes::types::Clip,
    Clip2Param: crate::nodes::types::Clip,
    MultiplierParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ClipMergeSubtract<Clip1Param, Clip2Param, MultiplierParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ClipParam: crate::nodes::types::Clip,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: clip/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    FilenamePrefixParam: crate::nodes::types::String,
> ClipSave<ClipParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { clip, filename_prefix }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ClipSave<ClipParam, FilenamePrefixParam> {
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
    ClipParam: crate::nodes::types::Clip,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ClipSave<ClipParam, FilenamePrefixParam> {}
///**Save Checkpoint**: No description.
#[derive(Clone)]
pub struct CheckpointSave<
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub clip: ClipParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: checkpoints/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> CheckpointSave<ModelParam, ClipParam, VaeParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        clip: ClipParam,
        vae: VaeParam,
        filename_prefix: FilenamePrefixParam,
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
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for CheckpointSave<ModelParam, ClipParam, VaeParam, FilenamePrefixParam> {
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
    ModelParam: crate::nodes::types::Model,
    ClipParam: crate::nodes::types::Clip,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for CheckpointSave<ModelParam, ClipParam, VaeParam, FilenamePrefixParam> {}
///**ImageOnlyCheckpointSave**: No description.
#[derive(Clone)]
pub struct ImageOnlyCheckpointSave<
    ModelParam: crate::nodes::types::Model,
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    ///No documentation.
    pub clip_vision: ClipVisionParam,
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: checkpoints/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> ImageOnlyCheckpointSave<ModelParam, ClipVisionParam, VaeParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        clip_vision: ClipVisionParam,
        vae: VaeParam,
        filename_prefix: FilenamePrefixParam,
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
    ModelParam: crate::nodes::types::Model,
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ImageOnlyCheckpointSave<ModelParam, ClipVisionParam, VaeParam, FilenamePrefixParam> {
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
    ModelParam: crate::nodes::types::Model,
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for ImageOnlyCheckpointSave<ModelParam, ClipVisionParam, VaeParam, FilenamePrefixParam> {}
///**ModelMergeAdd**: No description.
#[derive(Clone)]
pub struct ModelMergeAdd<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
> ModelMergeAdd<Model1Param, Model2Param> {
    /// Create a new node.
    pub fn new(model_1: Model1Param, model_2: Model2Param) -> Self {
        Self { model_1, model_2 }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
> crate::nodes::TypedNode for ModelMergeAdd<Model1Param, Model2Param> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InputParam: crate::nodes::types::Float,
    MiddleParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input: InputParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle: MiddleParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InputParam: crate::nodes::types::Float,
    MiddleParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> ModelMergeBlocks<Model1Param, Model2Param, InputParam, MiddleParam, OutParam> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        input: InputParam,
        middle: MiddleParam,
        out: OutParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InputParam: crate::nodes::types::Float,
    MiddleParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeBlocks<Model1Param, Model2Param, InputParam, MiddleParam, OutParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub ratio: RatioParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> ModelMergeSimple<Model1Param, Model2Param, RatioParam> {
    /// Create a new node.
    pub fn new(model_1: Model1Param, model_2: Model2Param, ratio: RatioParam) -> Self {
        Self { model_1, model_2, ratio }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    RatioParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ModelMergeSimple<Model1Param, Model2Param, RatioParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: -10
  - Step: 0.01
*/
    pub multiplier: MultiplierParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> ModelMergeSubtract<Model1Param, Model2Param, MultiplierParam> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        multiplier: MultiplierParam,
    ) -> Self {
        Self {
            model_1,
            model_2,
            multiplier,
        }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    MultiplierParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSubtract<Model1Param, Model2Param, MultiplierParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ModelParam: crate::nodes::types::Model,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: diffusion_models/ComfyUI
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    FilenamePrefixParam: crate::nodes::types::String,
> ModelSave<ModelParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { model, filename_prefix }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ModelSave<ModelParam, FilenamePrefixParam> {
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
    ModelParam: crate::nodes::types::Model,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for ModelSave<ModelParam, FilenamePrefixParam> {}
///**VAESave**: No description.
#[derive(Clone)]
pub struct VaeSave<
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: vae/ComfyUI_vae
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> VaeSave<VaeParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(vae: VaeParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { vae, filename_prefix }
    }
}
impl<
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for VaeSave<VaeParam, FilenamePrefixParam> {
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
    VaeParam: crate::nodes::types::Vae,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for VaeSave<VaeParam, FilenamePrefixParam> {}
