//!`Tripo` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Tripo: Convert model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoConversionNode<
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSizeParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: 4096
  - Max: 4096
  - Min: 128
*/
    pub texture_size: Option<TextureSizeParam>,
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
> TripoConversionNode<QuadParam, FaceLimitParam, TextureSizeParam> {
    /// Create a new node.
    pub fn new(
        quad: Option<QuadParam>,
        face_limit: Option<FaceLimitParam>,
        texture_size: Option<TextureSizeParam>,
    ) -> Self {
        Self {
            quad,
            face_limit,
            texture_size,
        }
    }
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TripoConversionNode<QuadParam, FaceLimitParam, TextureSizeParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_size {
            output.insert("texture_size".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoConversionNode";
    const DISPLAY_NAME: &'static str = "Tripo: Convert model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl<
    QuadParam: crate::nodes::types::Boolean,
    FaceLimitParam: crate::nodes::types::Int,
    TextureSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for TripoConversionNode<QuadParam, FaceLimitParam, TextureSizeParam> {}
///**Tripo: Image to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoImageToModelNode<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            image,
            texture,
            pbr,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoImageToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Image to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl<
    ImageParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoImageToModelNode<
    ImageParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo: Multiview to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoMultiviewToModelNode<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageBackParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    ImageRightParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub image_left: Option<ImageLeftParam>,
    ///No documentation.
    pub image_back: Option<ImageBackParam>,
    ///No documentation.
    pub image_right: Option<ImageRightParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        image_left: Option<ImageLeftParam>,
        image_back: Option<ImageBackParam>,
        image_right: Option<ImageRightParam>,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            image,
            image_left,
            image_back,
            image_right,
            texture,
            pbr,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        if let Some(v) = &self.image_left {
            output.insert("image_left".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_back {
            output.insert("image_back".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_right {
            output.insert("image_right".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoMultiviewToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Multiview to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl<
    ImageParam: crate::nodes::types::Image,
    ImageLeftParam: crate::nodes::types::Image,
    ImageBackParam: crate::nodes::types::Image,
    ImageRightParam: crate::nodes::types::Image,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoMultiviewToModelNode<
    ImageParam,
    ImageLeftParam,
    ImageBackParam,
    ImageRightParam,
    TextureParam,
    PbrParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo: Refine Draft model**: Refine a draft model created by v1.4 Tripo models only.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRefineNode {}
impl TripoRefineNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for TripoRefineNode {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "TripoRefineNode";
    const DISPLAY_NAME: &'static str = "Tripo: Refine Draft model";
    const DESCRIPTION: &'static str = "Refine a draft model created by v1.4 Tripo models only.";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl crate::nodes::TypedOutputNode for TripoRefineNode {}
///**Tripo: Retarget rigged model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRetargetNode {}
impl TripoRetargetNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for TripoRetargetNode {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "TripoRetargetNode";
    const DISPLAY_NAME: &'static str = "Tripo: Retarget rigged model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl crate::nodes::TypedOutputNode for TripoRetargetNode {}
///**Tripo: Rig model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoRigNode {}
impl TripoRigNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for TripoRigNode {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "TripoRigNode";
    const DISPLAY_NAME: &'static str = "Tripo: Rig model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl crate::nodes::TypedOutputNode for TripoRigNode {}
///**Tripo: Text to Model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoTextToModelNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String = crate::nodes::types::StringOut,
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    ImageSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    ModelSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    FaceLimitParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    QuadParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub negative_prompt: Option<NegativePromptParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub image_seed: Option<ImageSeedParam>,
    ///No documentation.
    pub model_seed: Option<ModelSeedParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
    /**No documentation.

**Metadata**:
  - Default: -1
  - Max: 500000
  - Min: -1
*/
    pub face_limit: Option<FaceLimitParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub quad: Option<QuadParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: Option<NegativePromptParam>,
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        image_seed: Option<ImageSeedParam>,
        model_seed: Option<ModelSeedParam>,
        texture_seed: Option<TextureSeedParam>,
        face_limit: Option<FaceLimitParam>,
        quad: Option<QuadParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            texture,
            pbr,
            image_seed,
            model_seed,
            texture_seed,
            face_limit,
            quad,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.negative_prompt {
            output.insert("negative_prompt".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.image_seed {
            output.insert("image_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.model_seed {
            output.insert("model_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.face_limit {
            output.insert("face_limit".to_string(), v.clone().into());
        }
        if let Some(v) = &self.quad {
            output.insert("quad".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoTextToModelNode";
    const DISPLAY_NAME: &'static str = "Tripo: Text to Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    ImageSeedParam: crate::nodes::types::Int,
    ModelSeedParam: crate::nodes::types::Int,
    TextureSeedParam: crate::nodes::types::Int,
    FaceLimitParam: crate::nodes::types::Int,
    QuadParam: crate::nodes::types::Boolean,
> crate::nodes::TypedOutputNode
for TripoTextToModelNode<
    PromptParam,
    NegativePromptParam,
    TextureParam,
    PbrParam,
    ImageSeedParam,
    ModelSeedParam,
    TextureSeedParam,
    FaceLimitParam,
    QuadParam,
> {}
///**Tripo: Texture model**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoTextureNode<
    TextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    PbrParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    TextureSeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub texture: Option<TextureParam>,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub pbr: Option<PbrParam>,
    ///No documentation.
    pub texture_seed: Option<TextureSeedParam>,
}
impl<
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> TripoTextureNode<TextureParam, PbrParam, TextureSeedParam> {
    /// Create a new node.
    pub fn new(
        texture: Option<TextureParam>,
        pbr: Option<PbrParam>,
        texture_seed: Option<TextureSeedParam>,
    ) -> Self {
        Self { texture, pbr, texture_seed }
    }
}
impl<
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TripoTextureNode<TextureParam, PbrParam, TextureSeedParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.texture {
            output.insert("texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.pbr {
            output.insert("pbr".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_seed {
            output.insert("texture_seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TripoTextureNode";
    const DISPLAY_NAME: &'static str = "Tripo: Texture model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/3d/Tripo";
}
impl<
    TextureParam: crate::nodes::types::Boolean,
    PbrParam: crate::nodes::types::Boolean,
    TextureSeedParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for TripoTextureNode<TextureParam, PbrParam, TextureSeedParam> {}
