//!`loaders` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod deprecated;
pub mod qwen;
/// Output types for nodes.
pub mod out {
    ///Output for [`CheckpointLoader`](super::CheckpointLoader).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct CheckpointLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
        ///No documentation.
        pub vae: crate::nodes::types::VaeOut,
    }
}
#[doc = "**Load CLIP**: \\[Recipes\\]\n\n\n\nstable_diffusion: clip-l\n\nstable_cascade: clip-g\n\nsd3: t5 xxl/ clip-g / clip-l\n\nstable_audio: t5 base\n\nmochi: t5 xxl\n\ncosmos: old t5 xxl\n\nlumina2: gemma 2 2B\n\nwan: umt5 xxl\n\n hidream: llama-3.1 (Recommend) or t5\n\nomnigen2: qwen vl 2.5 3B"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPLoader<
    ClipNameParam: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub clip_name: ClipNameParam,
    ///No documentation.
    pub type_: TypeParam,
    ///No documentation.
    pub device: Option<DeviceParam>,
}
impl<
    ClipNameParam: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String,
> CLIPLoader<ClipNameParam, TypeParam, DeviceParam> {
    /// Create a new node.
    pub fn new(
        clip_name: ClipNameParam,
        type_: TypeParam,
        device: Option<DeviceParam>,
    ) -> Self {
        Self { clip_name, type_, device }
    }
}
impl<
    ClipNameParam: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String,
> crate::nodes::TypedNode for CLIPLoader<ClipNameParam, TypeParam, DeviceParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name".to_string(), self.clip_name.clone().into());
        output.insert("type".to_string(), self.type_.clone().into());
        if let Some(v) = &self.device {
            output.insert("device".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CLIPLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP";
    const DESCRIPTION: &'static str = "[Recipes]\n\nstable_diffusion: clip-l\nstable_cascade: clip-g\nsd3: t5 xxl/ clip-g / clip-l\nstable_audio: t5 base\nmochi: t5 xxl\ncosmos: old t5 xxl\nlumina2: gemma 2 2B\nwan: umt5 xxl\n hidream: llama-3.1 (Recommend) or t5\nomnigen2: qwen vl 2.5 3B";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Checkpoint With Config (DEPRECATED)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CheckpointLoader<
    ConfigNameParam: crate::nodes::types::String,
    CkptNameParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub config_name: ConfigNameParam,
    ///No documentation.
    pub ckpt_name: CkptNameParam,
}
impl<
    ConfigNameParam: crate::nodes::types::String,
    CkptNameParam: crate::nodes::types::String,
> CheckpointLoader<ConfigNameParam, CkptNameParam> {
    /// Create a new node.
    pub fn new(config_name: ConfigNameParam, ckpt_name: CkptNameParam) -> Self {
        Self { config_name, ckpt_name }
    }
}
impl<
    ConfigNameParam: crate::nodes::types::String,
    CkptNameParam: crate::nodes::types::String,
> crate::nodes::TypedNode for CheckpointLoader<ConfigNameParam, CkptNameParam> {
    type Output = out::CheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            clip: crate::nodes::types::ClipOut::from_dynamic(node_id, 1u32),
            vae: crate::nodes::types::VaeOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("config_name".to_string(), self.config_name.clone().into());
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output
    }
    const NAME: &'static str = "CheckpointLoader";
    const DISPLAY_NAME: &'static str = "Load Checkpoint With Config (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
#[doc = "**DualCLIPLoader**: \\[Recipes\\]\n\n\n\nsdxl: clip-l, clip-g\n\nsd3: clip-l, clip-g / clip-l, t5 / clip-g, t5\n\nflux: clip-l, t5\n\nhidream: at least one of t5 or llama, recommended t5 and llama\n\nhunyuan_image: qwen2.5vl 7b and byt5 small"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DualCLIPLoader<
    ClipName1Param: crate::nodes::types::String,
    ClipName2Param: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///No documentation.
    pub clip_name_1: ClipName1Param,
    ///No documentation.
    pub clip_name_2: ClipName2Param,
    ///No documentation.
    pub type_: TypeParam,
    ///No documentation.
    pub device: Option<DeviceParam>,
}
impl<
    ClipName1Param: crate::nodes::types::String,
    ClipName2Param: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String,
> DualCLIPLoader<ClipName1Param, ClipName2Param, TypeParam, DeviceParam> {
    /// Create a new node.
    pub fn new(
        clip_name_1: ClipName1Param,
        clip_name_2: ClipName2Param,
        type_: TypeParam,
        device: Option<DeviceParam>,
    ) -> Self {
        Self {
            clip_name_1,
            clip_name_2,
            type_,
            device,
        }
    }
}
impl<
    ClipName1Param: crate::nodes::types::String,
    ClipName2Param: crate::nodes::types::String,
    TypeParam: crate::nodes::types::String,
    DeviceParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for DualCLIPLoader<ClipName1Param, ClipName2Param, TypeParam, DeviceParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name1".to_string(), self.clip_name_1.clone().into());
        output.insert("clip_name2".to_string(), self.clip_name_2.clone().into());
        output.insert("type".to_string(), self.type_.clone().into());
        if let Some(v) = &self.device {
            output.insert("device".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "DualCLIPLoader";
    const DISPLAY_NAME: &'static str = "DualCLIPLoader";
    const DESCRIPTION: &'static str = "[Recipes]\n\nsdxl: clip-l, clip-g\nsd3: clip-l, clip-g / clip-l, t5 / clip-g, t5\nflux: clip-l, t5\nhidream: at least one of t5 or llama, recommended t5 and llama\nhunyuan_image: qwen2.5vl 7b and byt5 small";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**ModelPatchLoader**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelPatchLoader<NameParam: crate::nodes::types::String> {
    ///No documentation.
    pub name: NameParam,
}
impl<NameParam: crate::nodes::types::String> ModelPatchLoader<NameParam> {
    /// Create a new node.
    pub fn new(name: NameParam) -> Self {
        Self { name }
    }
}
impl<NameParam: crate::nodes::types::String> crate::nodes::TypedNode
for ModelPatchLoader<NameParam> {
    type Output = crate::nodes::types::ModelPatchOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("name".to_string(), self.name.clone().into());
        output
    }
    const NAME: &'static str = "ModelPatchLoader";
    const DISPLAY_NAME: &'static str = "ModelPatchLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
#[doc = "**QuadrupleCLIPLoader**: \\[Recipes\\]\n\n\n\nhidream: long clip-l, long clip-g, t5xxl, llama_8b_3.1_instruct"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct QuadrupleCLIPLoader {}
impl QuadrupleCLIPLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for QuadrupleCLIPLoader {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "QuadrupleCLIPLoader";
    const DISPLAY_NAME: &'static str = "QuadrupleCLIPLoader";
    const DESCRIPTION: &'static str = "[Recipes]\n\nhidream: long clip-l, long clip-g, t5xxl, llama_8b_3.1_instruct";
    const CATEGORY: &'static str = "advanced/loaders";
}
#[doc = "**TripleCLIPLoader**: \\[Recipes\\]\n\n\n\nsd3: clip-l, clip-g, t5"]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripleCLIPLoader {}
impl TripleCLIPLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for TripleCLIPLoader {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "TripleCLIPLoader";
    const DISPLAY_NAME: &'static str = "TripleCLIPLoader";
    const DESCRIPTION: &'static str = "[Recipes]\n\nsd3: clip-l, clip-g, t5";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Diffusion Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct UNETLoader<
    UnetNameParam: crate::nodes::types::String,
    WeightDtypeParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub unet_name: UnetNameParam,
    ///No documentation.
    pub weight_dtype: WeightDtypeParam,
}
impl<
    UnetNameParam: crate::nodes::types::String,
    WeightDtypeParam: crate::nodes::types::String,
> UNETLoader<UnetNameParam, WeightDtypeParam> {
    /// Create a new node.
    pub fn new(unet_name: UnetNameParam, weight_dtype: WeightDtypeParam) -> Self {
        Self { unet_name, weight_dtype }
    }
}
impl<
    UnetNameParam: crate::nodes::types::String,
    WeightDtypeParam: crate::nodes::types::String,
> crate::nodes::TypedNode for UNETLoader<UnetNameParam, WeightDtypeParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("unet_name".to_string(), self.unet_name.clone().into());
        output.insert("weight_dtype".to_string(), self.weight_dtype.clone().into());
        output
    }
    const NAME: &'static str = "UNETLoader";
    const DISPLAY_NAME: &'static str = "Load Diffusion Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
