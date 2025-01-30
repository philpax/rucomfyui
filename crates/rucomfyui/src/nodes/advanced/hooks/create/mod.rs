//!`create` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Create Hook LoRA**: No description.
#[derive(Clone)]
pub struct CreateHookLora<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub lora_name: LoraName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_clip: StrengthClip,
    ///No documentation.
    pub prev_hooks: Option<PrevHooks>,
}
impl<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> CreateHookLora<LoraName, StrengthModel, StrengthClip, PrevHooks> {
    /// Create a new node.
    pub fn new(
        lora_name: LoraName,
        strength_model: StrengthModel,
        strength_clip: StrengthClip,
        prev_hooks: Option<PrevHooks>,
    ) -> Self {
        Self {
            lora_name,
            strength_model,
            strength_clip,
            prev_hooks,
        }
    }
}
impl<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookLora<LoraName, StrengthModel, StrengthClip, PrevHooks> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("lora_name".to_string(), self.lora_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output.insert("strength_clip".to_string(), self.strength_clip.clone().into());
        if let Some(v) = &self.prev_hooks {
            output.insert("prev_hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookLora";
    const DISPLAY_NAME: &'static str = "Create Hook LoRA";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/create";
}
///**Create Hook LoRA (MO)**: No description.
#[derive(Clone)]
pub struct CreateHookLoraModelOnly<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub lora_name: LoraName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
    ///No documentation.
    pub prev_hooks: Option<PrevHooks>,
}
impl<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> CreateHookLoraModelOnly<LoraName, StrengthModel, PrevHooks> {
    /// Create a new node.
    pub fn new(
        lora_name: LoraName,
        strength_model: StrengthModel,
        prev_hooks: Option<PrevHooks>,
    ) -> Self {
        Self {
            lora_name,
            strength_model,
            prev_hooks,
        }
    }
}
impl<
    LoraName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookLoraModelOnly<LoraName, StrengthModel, PrevHooks> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("lora_name".to_string(), self.lora_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        if let Some(v) = &self.prev_hooks {
            output.insert("prev_hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookLoraModelOnly";
    const DISPLAY_NAME: &'static str = "Create Hook LoRA (MO)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/create";
}
///**Create Hook Model as LoRA**: No description.
#[derive(Clone)]
pub struct CreateHookModelAsLora<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub ckpt_name: CkptName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_clip: StrengthClip,
    ///No documentation.
    pub prev_hooks: Option<PrevHooks>,
}
impl<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> CreateHookModelAsLora<CkptName, StrengthModel, StrengthClip, PrevHooks> {
    /// Create a new node.
    pub fn new(
        ckpt_name: CkptName,
        strength_model: StrengthModel,
        strength_clip: StrengthClip,
        prev_hooks: Option<PrevHooks>,
    ) -> Self {
        Self {
            ckpt_name,
            strength_model,
            strength_clip,
            prev_hooks,
        }
    }
}
impl<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    StrengthClip: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookModelAsLora<CkptName, StrengthModel, StrengthClip, PrevHooks> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        output.insert("strength_clip".to_string(), self.strength_clip.clone().into());
        if let Some(v) = &self.prev_hooks {
            output.insert("prev_hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookModelAsLora";
    const DISPLAY_NAME: &'static str = "Create Hook Model as LoRA";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/create";
}
///**Create Hook Model as LoRA (MO)**: No description.
#[derive(Clone)]
pub struct CreateHookModelAsLoraModelOnly<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub ckpt_name: CkptName,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModel,
    ///No documentation.
    pub prev_hooks: Option<PrevHooks>,
}
impl<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> CreateHookModelAsLoraModelOnly<CkptName, StrengthModel, PrevHooks> {
    /// Create a new node.
    pub fn new(
        ckpt_name: CkptName,
        strength_model: StrengthModel,
        prev_hooks: Option<PrevHooks>,
    ) -> Self {
        Self {
            ckpt_name,
            strength_model,
            prev_hooks,
        }
    }
}
impl<
    CkptName: crate::nodes::types::String,
    StrengthModel: crate::nodes::types::Float,
    PrevHooks: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookModelAsLoraModelOnly<CkptName, StrengthModel, PrevHooks> {
    type Output = crate::nodes::types::HooksOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ckpt_name".to_string(), self.ckpt_name.clone().into());
        output.insert("strength_model".to_string(), self.strength_model.clone().into());
        if let Some(v) = &self.prev_hooks {
            output.insert("prev_hooks".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CreateHookModelAsLoraModelOnly";
    const DISPLAY_NAME: &'static str = "Create Hook Model as LoRA (MO)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/hooks/create";
}
