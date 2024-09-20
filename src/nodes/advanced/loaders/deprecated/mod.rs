//!deprecated
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**DiffusersLoader**
pub struct DiffusersLoader<ModelPath: crate::nodes::String> {
    ///No documentation.
    pub model_path: ModelPath,
}
///Output for [`DiffusersLoader`].
#[derive(Clone)]
pub struct DiffusersLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl<ModelPath: crate::nodes::String> crate::nodes::TypedNode
for DiffusersLoader<ModelPath> {
    type Output = DiffusersLoaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut {
                node_id,
                slot: 0u32,
            },
            clip: crate::nodes::ClipOut {
                node_id,
                slot: 1u32,
            },
            vae: crate::nodes::VaeOut {
                node_id,
                slot: 2u32,
            },
        }
    }
    const NAME: &'static str = "DiffusersLoader";
    const DISPLAY_NAME: &'static str = "DiffusersLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders/deprecated";
}
