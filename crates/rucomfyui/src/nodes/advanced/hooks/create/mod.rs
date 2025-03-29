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
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub lora_name: LoraNameParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_clip: StrengthClipParam,
    ///No documentation.
    pub prev_hooks: Option<PrevHooksParam>,
}
impl<
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> CreateHookLora<LoraNameParam, StrengthModelParam, StrengthClipParam, PrevHooksParam> {
    /// Create a new node.
    pub fn new(
        lora_name: LoraNameParam,
        strength_model: StrengthModelParam,
        strength_clip: StrengthClipParam,
        prev_hooks: Option<PrevHooksParam>,
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
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookLora<
    LoraNameParam,
    StrengthModelParam,
    StrengthClipParam,
    PrevHooksParam,
> {
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
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub lora_name: LoraNameParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
    ///No documentation.
    pub prev_hooks: Option<PrevHooksParam>,
}
impl<
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> CreateHookLoraModelOnly<LoraNameParam, StrengthModelParam, PrevHooksParam> {
    /// Create a new node.
    pub fn new(
        lora_name: LoraNameParam,
        strength_model: StrengthModelParam,
        prev_hooks: Option<PrevHooksParam>,
    ) -> Self {
        Self {
            lora_name,
            strength_model,
            prev_hooks,
        }
    }
}
impl<
    LoraNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookLoraModelOnly<LoraNameParam, StrengthModelParam, PrevHooksParam> {
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
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub ckpt_name: CkptNameParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_clip: StrengthClipParam,
    ///No documentation.
    pub prev_hooks: Option<PrevHooksParam>,
}
impl<
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> CreateHookModelAsLora<
    CkptNameParam,
    StrengthModelParam,
    StrengthClipParam,
    PrevHooksParam,
> {
    /// Create a new node.
    pub fn new(
        ckpt_name: CkptNameParam,
        strength_model: StrengthModelParam,
        strength_clip: StrengthClipParam,
        prev_hooks: Option<PrevHooksParam>,
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
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    StrengthClipParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookModelAsLora<
    CkptNameParam,
    StrengthModelParam,
    StrengthClipParam,
    PrevHooksParam,
> {
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
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks = crate::nodes::types::HooksOut,
> {
    ///No documentation.
    pub ckpt_name: CkptNameParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 20
  - Min: -20
  - Step: 0.01
*/
    pub strength_model: StrengthModelParam,
    ///No documentation.
    pub prev_hooks: Option<PrevHooksParam>,
}
impl<
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> CreateHookModelAsLoraModelOnly<CkptNameParam, StrengthModelParam, PrevHooksParam> {
    /// Create a new node.
    pub fn new(
        ckpt_name: CkptNameParam,
        strength_model: StrengthModelParam,
        prev_hooks: Option<PrevHooksParam>,
    ) -> Self {
        Self {
            ckpt_name,
            strength_model,
            prev_hooks,
        }
    }
}
impl<
    CkptNameParam: crate::nodes::types::String,
    StrengthModelParam: crate::nodes::types::Float,
    PrevHooksParam: crate::nodes::types::Hooks,
> crate::nodes::TypedNode
for CreateHookModelAsLoraModelOnly<CkptNameParam, StrengthModelParam, PrevHooksParam> {
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
