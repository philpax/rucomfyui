//!`loaders` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod deprecated;
/// Output types for nodes.
pub mod out {
    ///Output for [`ClipLoader`](super::ClipLoader).
    #[derive(Clone)]
    pub struct ClipLoaderOutput {
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`CheckpointLoader`](super::CheckpointLoader).
    #[derive(Clone)]
    pub struct CheckpointLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
        ///No documentation.
        pub vae: crate::nodes::types::VaeOut,
    }
    ///Output for [`DualClipLoader`](super::DualClipLoader).
    #[derive(Clone)]
    pub struct DualClipLoaderOutput {
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`TripleClipLoader`](super::TripleClipLoader).
    #[derive(Clone)]
    pub struct TripleClipLoaderOutput {
        ///No documentation.
        pub clip: crate::nodes::types::ClipOut,
    }
    ///Output for [`UnetLoader`](super::UnetLoader).
    #[derive(Clone)]
    pub struct UnetLoaderOutput {
        ///No documentation.
        pub model: crate::nodes::types::ModelOut,
    }
}
///**Load CLIP**: No description.
pub struct ClipLoader<
    ClipName: crate::nodes::types::String,
    Type: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip_name: ClipName,
    ///No documentation.
    pub type_: Type,
}
impl<
    ClipName: crate::nodes::types::String,
    Type: crate::nodes::types::String,
> crate::nodes::TypedNode for ClipLoader<ClipName, Type> {
    type Output = out::ClipLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name".to_string(), self.clip_name.to_workflow_input());
        output.insert("type".to_string(), self.type_.to_workflow_input());
        output
    }
    const NAME: &'static str = "CLIPLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Checkpoint With Config (DEPRECATED)**: No description.
pub struct CheckpointLoader<
    ConfigName: crate::nodes::types::String,
    CkptName: crate::nodes::types::String,
> {
    ///No documentation.
    pub config_name: ConfigName,
    ///No documentation.
    pub ckpt_name: CkptName,
}
impl<
    ConfigName: crate::nodes::types::String,
    CkptName: crate::nodes::types::String,
> crate::nodes::TypedNode for CheckpointLoader<ConfigName, CkptName> {
    type Output = out::CheckpointLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 1u32,
            },
            vae: crate::nodes::types::VaeOut {
                node_id,
                node_slot: 2u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("config_name".to_string(), self.config_name.to_workflow_input());
        output.insert("ckpt_name".to_string(), self.ckpt_name.to_workflow_input());
        output
    }
    const NAME: &'static str = "CheckpointLoader";
    const DISPLAY_NAME: &'static str = "Load Checkpoint With Config (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**DualCLIPLoader**: No description.
pub struct DualClipLoader<
    ClipName1: crate::nodes::types::String,
    ClipName2: crate::nodes::types::String,
    Type: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip_name_1: ClipName1,
    ///No documentation.
    pub clip_name_2: ClipName2,
    ///No documentation.
    pub type_: Type,
}
impl<
    ClipName1: crate::nodes::types::String,
    ClipName2: crate::nodes::types::String,
    Type: crate::nodes::types::String,
> crate::nodes::TypedNode for DualClipLoader<ClipName1, ClipName2, Type> {
    type Output = out::DualClipLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name1".to_string(), self.clip_name_1.to_workflow_input());
        output.insert("clip_name2".to_string(), self.clip_name_2.to_workflow_input());
        output.insert("type".to_string(), self.type_.to_workflow_input());
        output
    }
    const NAME: &'static str = "DualCLIPLoader";
    const DISPLAY_NAME: &'static str = "DualCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**TripleCLIPLoader**: No description.
pub struct TripleClipLoader<
    ClipName1: crate::nodes::types::String,
    ClipName2: crate::nodes::types::String,
    ClipName3: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip_name_1: ClipName1,
    ///No documentation.
    pub clip_name_2: ClipName2,
    ///No documentation.
    pub clip_name_3: ClipName3,
}
impl<
    ClipName1: crate::nodes::types::String,
    ClipName2: crate::nodes::types::String,
    ClipName3: crate::nodes::types::String,
> crate::nodes::TypedNode for TripleClipLoader<ClipName1, ClipName2, ClipName3> {
    type Output = out::TripleClipLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            clip: crate::nodes::types::ClipOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_name1".to_string(), self.clip_name_1.to_workflow_input());
        output.insert("clip_name2".to_string(), self.clip_name_2.to_workflow_input());
        output.insert("clip_name3".to_string(), self.clip_name_3.to_workflow_input());
        output
    }
    const NAME: &'static str = "TripleCLIPLoader";
    const DISPLAY_NAME: &'static str = "TripleCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Diffusion Model**: No description.
pub struct UnetLoader<
    UnetName: crate::nodes::types::String,
    WeightDtype: crate::nodes::types::String,
> {
    ///No documentation.
    pub unet_name: UnetName,
    ///No documentation.
    pub weight_dtype: WeightDtype,
}
impl<
    UnetName: crate::nodes::types::String,
    WeightDtype: crate::nodes::types::String,
> crate::nodes::TypedNode for UnetLoader<UnetName, WeightDtype> {
    type Output = out::UnetLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::types::ModelOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("unet_name".to_string(), self.unet_name.to_workflow_input());
        output.insert("weight_dtype".to_string(), self.weight_dtype.to_workflow_input());
        output
    }
    const NAME: &'static str = "UNETLoader";
    const DISPLAY_NAME: &'static str = "Load Diffusion Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
