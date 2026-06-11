//!`Meshy` definitions/categories.
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
///**Meshy: Animate Model**: Apply a specific animation action to a previously rigged character.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyAnimateModelNode<
    RigTaskIdParam: crate::nodes::types::MeshyRiggedTaskId,
    ActionIdParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub rig_task_id: RigTaskIdParam,
    /**Visit https://docs.meshy.ai/en/api/animation-library for a list of available values.

**Metadata**:
  - Default: 0
  - Max: 696
  - Min: 0
*/
    pub action_id: ActionIdParam,
}
impl<
    RigTaskIdParam: crate::nodes::types::MeshyRiggedTaskId,
    ActionIdParam: crate::nodes::types::Int,
> MeshyAnimateModelNode<RigTaskIdParam, ActionIdParam> {
    /// Create a new node.
    pub fn new(rig_task_id: RigTaskIdParam, action_id: ActionIdParam) -> Self {
        Self { rig_task_id, action_id }
    }
}
impl<
    RigTaskIdParam: crate::nodes::types::MeshyRiggedTaskId,
    ActionIdParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for MeshyAnimateModelNode<RigTaskIdParam, ActionIdParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("rig_task_id".to_string(), self.rig_task_id.clone().into());
        output.insert("action_id".to_string(), self.action_id.clone().into());
        output
    }
    const NAME: &'static str = "MeshyAnimateModelNode";
    const DISPLAY_NAME: &'static str = "Meshy: Animate Model";
    const DESCRIPTION: &'static str = "Apply a specific animation action to a previously rigged character.";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    RigTaskIdParam: crate::nodes::types::MeshyRiggedTaskId,
    ActionIdParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for MeshyAnimateModelNode<RigTaskIdParam, ActionIdParam> {}
///**Meshy: Image to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyImageToModelNode<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> MeshyImageToModelNode<ImageParam, SeedParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, seed: SeedParam) -> Self {
        Self { image, seed }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for MeshyImageToModelNode<ImageParam, SeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "MeshyImageToModelNode";
    const DISPLAY_NAME: &'static str = "Meshy: Image to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    ImageParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode for MeshyImageToModelNode<ImageParam, SeedParam> {}
///**Meshy: Multi-Image to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyMultiImageToModelNode<SeedParam: crate::nodes::types::Int> {
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<SeedParam: crate::nodes::types::Int> MeshyMultiImageToModelNode<SeedParam> {
    /// Create a new node.
    pub fn new(seed: SeedParam) -> Self {
        Self { seed }
    }
}
impl<SeedParam: crate::nodes::types::Int> crate::nodes::TypedNode
for MeshyMultiImageToModelNode<SeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "MeshyMultiImageToModelNode";
    const DISPLAY_NAME: &'static str = "Meshy: Multi-Image to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<SeedParam: crate::nodes::types::Int> crate::nodes::TypedOutputNode
for MeshyMultiImageToModelNode<SeedParam> {}
///**Meshy: Refine Draft Model**: Refine a previously created draft model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyRefineNode<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnablePbrParam: crate::nodes::types::Boolean,
    TexturePromptParam: crate::nodes::types::String,
    TextureImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub meshy_task_id: MeshyTaskIdParam,
    /**Generate PBR Maps (metallic, roughness, normal) in addition to the base color. Note: this should be set to false when using Sculpture style, as Sculpture style generates its own set of PBR maps.

**Metadata**:
  - Default: false
*/
    pub enable_pbr: EnablePbrParam,
    /**Provide a text prompt to guide the texturing process. Maximum 600 characters. Cannot be used at the same time as 'texture_image'.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub texture_prompt: TexturePromptParam,
    ///Only one of 'texture_image' or 'texture_prompt' may be used at the same time.
    pub texture_image: Option<TextureImageParam>,
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnablePbrParam: crate::nodes::types::Boolean,
    TexturePromptParam: crate::nodes::types::String,
    TextureImageParam: crate::nodes::types::Image,
> MeshyRefineNode<
    MeshyTaskIdParam,
    EnablePbrParam,
    TexturePromptParam,
    TextureImageParam,
> {
    /// Create a new node.
    pub fn new(
        meshy_task_id: MeshyTaskIdParam,
        enable_pbr: EnablePbrParam,
        texture_prompt: TexturePromptParam,
        texture_image: Option<TextureImageParam>,
    ) -> Self {
        Self {
            meshy_task_id,
            enable_pbr,
            texture_prompt,
            texture_image,
        }
    }
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnablePbrParam: crate::nodes::types::Boolean,
    TexturePromptParam: crate::nodes::types::String,
    TextureImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for MeshyRefineNode<
    MeshyTaskIdParam,
    EnablePbrParam,
    TexturePromptParam,
    TextureImageParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("meshy_task_id".to_string(), self.meshy_task_id.clone().into());
        output.insert("enable_pbr".to_string(), self.enable_pbr.clone().into());
        output.insert("texture_prompt".to_string(), self.texture_prompt.clone().into());
        if let Some(v) = &self.texture_image {
            output.insert("texture_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MeshyRefineNode";
    const DISPLAY_NAME: &'static str = "Meshy: Refine Draft Model";
    const DESCRIPTION: &'static str = "Refine a previously created draft model.";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnablePbrParam: crate::nodes::types::Boolean,
    TexturePromptParam: crate::nodes::types::String,
    TextureImageParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode
for MeshyRefineNode<
    MeshyTaskIdParam,
    EnablePbrParam,
    TexturePromptParam,
    TextureImageParam,
> {}
///**Meshy: Rig Model**: Provides a rigged character in standard formats. Auto-rigging is currently not suitable for untextured meshes, non-humanoid assets, or humanoid assets with unclear limb and body structure.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyRigModelNode<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    HeightMetersParam: crate::nodes::types::Float,
    TextureImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub meshy_task_id: MeshyTaskIdParam,
    /**The approximate height of the character model in meters. This aids in scaling and rigging accuracy.

**Metadata**:
  - Default: 1.7
  - Max: 15
  - Min: 0.1
*/
    pub height_meters: HeightMetersParam,
    ///The model's UV-unwrapped base color texture image.
    pub texture_image: Option<TextureImageParam>,
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    HeightMetersParam: crate::nodes::types::Float,
    TextureImageParam: crate::nodes::types::Image,
> MeshyRigModelNode<MeshyTaskIdParam, HeightMetersParam, TextureImageParam> {
    /// Create a new node.
    pub fn new(
        meshy_task_id: MeshyTaskIdParam,
        height_meters: HeightMetersParam,
        texture_image: Option<TextureImageParam>,
    ) -> Self {
        Self {
            meshy_task_id,
            height_meters,
            texture_image,
        }
    }
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    HeightMetersParam: crate::nodes::types::Float,
    TextureImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for MeshyRigModelNode<MeshyTaskIdParam, HeightMetersParam, TextureImageParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("meshy_task_id".to_string(), self.meshy_task_id.clone().into());
        output.insert("height_meters".to_string(), self.height_meters.clone().into());
        if let Some(v) = &self.texture_image {
            output.insert("texture_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MeshyRigModelNode";
    const DISPLAY_NAME: &'static str = "Meshy: Rig Model";
    const DESCRIPTION: &'static str = "Provides a rigged character in standard formats. Auto-rigging is currently not suitable for untextured meshes, non-humanoid assets, or humanoid assets with unclear limb and body structure.";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    HeightMetersParam: crate::nodes::types::Float,
    TextureImageParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode
for MeshyRigModelNode<MeshyTaskIdParam, HeightMetersParam, TextureImageParam> {}
///**Meshy: Text to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyTextToModelNode<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**Seed controls whether the node should re-run; results are non-deterministic regardless of seed.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 2147483647
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> MeshyTextToModelNode<PromptParam, SeedParam> {
    /// Create a new node.
    pub fn new(prompt: PromptParam, seed: SeedParam) -> Self {
        Self { prompt, seed }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for MeshyTextToModelNode<PromptParam, SeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "MeshyTextToModelNode";
    const DISPLAY_NAME: &'static str = "Meshy: Text to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode for MeshyTextToModelNode<PromptParam, SeedParam> {}
///**Meshy: Texture Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MeshyTextureNode<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnableOriginalUvParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextStylePromptParam: crate::nodes::types::String,
    ImageStyleParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub meshy_task_id: MeshyTaskIdParam,
    /**Use the original UV of the model instead of generating new UVs. When enabled, Meshy preserves existing textures from the uploaded model. If the model has no original UV, the quality of the output might not be as good.

**Metadata**:
  - Default: true
*/
    pub enable_original_uv: EnableOriginalUvParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub pbr: PbrParam,
    /**Describe your desired texture style of the object using text. Maximum 600 characters.Maximum 600 characters. Cannot be used at the same time as 'image_style'.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub text_style_prompt: TextStylePromptParam,
    ///A 2d image to guide the texturing process. Can not be used at the same time with 'text_style_prompt'.
    pub image_style: Option<ImageStyleParam>,
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnableOriginalUvParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextStylePromptParam: crate::nodes::types::String,
    ImageStyleParam: crate::nodes::types::Image,
> MeshyTextureNode<
    MeshyTaskIdParam,
    EnableOriginalUvParam,
    PbrParam,
    TextStylePromptParam,
    ImageStyleParam,
> {
    /// Create a new node.
    pub fn new(
        meshy_task_id: MeshyTaskIdParam,
        enable_original_uv: EnableOriginalUvParam,
        pbr: PbrParam,
        text_style_prompt: TextStylePromptParam,
        image_style: Option<ImageStyleParam>,
    ) -> Self {
        Self {
            meshy_task_id,
            enable_original_uv,
            pbr,
            text_style_prompt,
            image_style,
        }
    }
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnableOriginalUvParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextStylePromptParam: crate::nodes::types::String,
    ImageStyleParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for MeshyTextureNode<
    MeshyTaskIdParam,
    EnableOriginalUvParam,
    PbrParam,
    TextStylePromptParam,
    ImageStyleParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("meshy_task_id".to_string(), self.meshy_task_id.clone().into());
        output
            .insert(
                "enable_original_uv".to_string(),
                self.enable_original_uv.clone().into(),
            );
        output.insert("pbr".to_string(), self.pbr.clone().into());
        output
            .insert(
                "text_style_prompt".to_string(),
                self.text_style_prompt.clone().into(),
            );
        if let Some(v) = &self.image_style {
            output.insert("image_style".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MeshyTextureNode";
    const DISPLAY_NAME: &'static str = "Meshy: Texture Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "partner/3d/Meshy";
}
impl<
    MeshyTaskIdParam: crate::nodes::types::MeshyTaskId,
    EnableOriginalUvParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextStylePromptParam: crate::nodes::types::String,
    ImageStyleParam: crate::nodes::types::Image,
> crate::nodes::TypedOutputNode
for MeshyTextureNode<
    MeshyTaskIdParam,
    EnableOriginalUvParam,
    PbrParam,
    TextStylePromptParam,
    ImageStyleParam,
> {}
