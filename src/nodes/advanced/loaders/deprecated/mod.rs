//!`deprecated` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**DiffusersLoader**
pub struct DiffusersLoader<ModelPath: crate::nodes::types::String> {
    ///No documentation.
    pub model_path: ModelPath,
}
///Output for [`DiffusersLoader`].
#[derive(Clone)]
pub struct DiffusersLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::types::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::types::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::types::VaeOut,
}
impl<ModelPath: crate::nodes::types::String> crate::nodes::TypedNode
for DiffusersLoader<ModelPath> {
    type Output = DiffusersLoaderOutput;
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
    const NAME: &'static str = "DiffusersLoader";
    const DISPLAY_NAME: &'static str = "DiffusersLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders/deprecated";
}
