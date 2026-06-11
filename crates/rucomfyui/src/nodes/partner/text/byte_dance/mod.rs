//!`ByteDance` definitions/categories.
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
///**ByteDance Seed**: Generate text responses with ByteDance's Seed 2.0 models. Provide a text prompt and optionally one or more images or videos for multimodal context.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ByteDanceSeedNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    /**Text input to the model.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
    /**Foundational instructions that dictate the model's behavior.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub system_prompt: Option<SystemPromptParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
> ByteDanceSeedNode<PromptParam, SeedParam, SystemPromptParam> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: SeedParam,
        system_prompt: Option<SystemPromptParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            system_prompt,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    SystemPromptParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ByteDanceSeedNode<PromptParam, SeedParam, SystemPromptParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        if let Some(v) = &self.system_prompt {
            output.insert("system_prompt".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ByteDanceSeedNode";
    const DISPLAY_NAME: &'static str = "ByteDance Seed";
    const DESCRIPTION: &'static str = "Generate text responses with ByteDance's Seed 2.0 models. Provide a text prompt and optionally one or more images or videos for multimodal context.";
    const CATEGORY: &'static str = "partner/text/ByteDance";
}
