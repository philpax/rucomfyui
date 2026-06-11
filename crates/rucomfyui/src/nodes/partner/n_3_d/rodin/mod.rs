//!`Rodin` definitions/categories.
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
/// Output types for nodes.
pub mod out {
    ///Output for [`Rodin3D_Detail`](super::Rodin3D_Detail).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Rodin3D_DetailOutput {
        ///No documentation.
        pub n_3_d_model_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`Rodin3D_Gen2`](super::Rodin3D_Gen2).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Rodin3D_Gen2Output {
        ///No documentation.
        pub n_3_d_model_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`Rodin3D_Regular`](super::Rodin3D_Regular).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Rodin3D_RegularOutput {
        ///No documentation.
        pub n_3_d_model_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`Rodin3D_Sketch`](super::Rodin3D_Sketch).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Rodin3D_SketchOutput {
        ///No documentation.
        pub n_3_d_model_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
    ///Output for [`Rodin3D_Smooth`](super::Rodin3D_Smooth).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Rodin3D_SmoothOutput {
        ///No documentation.
        pub n_3_d_model_path: crate::nodes::types::StringOut,
        ///No documentation.
        pub glb: crate::nodes::types::File3DGlbOut,
    }
}
///**Rodin 3D Generate - Detail Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Detail<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Detail<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Detail<ImagesParam, SeedParam> {
    type Output = out::Rodin3D_DetailOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            n_3_d_model_path: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                0u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Detail";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Detail Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Generate - Gen-2 Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Gen2<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub ta_pose: TaPoseParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Gen2<ImagesParam, TaPoseParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        ta_pose: TaPoseParam,
        seed: Option<SeedParam>,
    ) -> Self {
        Self { images, ta_pose, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Gen2<ImagesParam, TaPoseParam, SeedParam> {
    type Output = out::Rodin3D_Gen2Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            n_3_d_model_path: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                0u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        output.insert("TAPose".to_string(), self.ta_pose.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Gen2";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Gen-2 Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Gen-2.5 - Image to 3D**: Generate a 3D model from 1-5 reference images via Rodin Gen-2.5. Pick a mode (Fast / Regular / Extreme-High) to tune quality vs. cost.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Gen25_Image<
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TaPoseParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    HdTextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    TextureDelightParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    UseOriginalAlphaParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
    AddonHighpackParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    BboxWidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    BboxHeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    BboxLengthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    HeightCmParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
    /**T/A pose for human-like models.

**Metadata**:
  - Default: false
*/
    pub ta_pose: Option<TaPoseParam>,
    /**High-quality texture enhancement.

**Metadata**:
  - Default: false
*/
    pub hd_texture: Option<HdTextureParam>,
    /**Remove baked lighting from textures.

**Metadata**:
  - Default: false
*/
    pub texture_delight: Option<TextureDelightParam>,
    /**Preserve image transparency.

**Metadata**:
  - Default: false
*/
    pub use_original_alpha: Option<UseOriginalAlphaParam>,
    /**HighPack addon: 4K textures and ~16x faces in Quad mode.

**Metadata**:
  - Default: false
*/
    pub addon_highpack: Option<AddonHighpackParam>,
    /**Bounding-box width (Y axis). Set to 0 with the others to skip bbox.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_width: Option<BboxWidthParam>,
    /**Bounding-box height (Z axis).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_height: Option<BboxHeightParam>,
    /**Bounding-box length (X axis).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_length: Option<BboxLengthParam>,
    /**Approximate model height in centimeters (0 to skip).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 10000
  - Min: 0
*/
    pub height_cm: Option<HeightCmParam>,
}
impl<
    SeedParam: crate::nodes::types::Int,
    TaPoseParam: crate::nodes::types::Boolean,
    HdTextureParam: crate::nodes::types::Boolean,
    TextureDelightParam: crate::nodes::types::Boolean,
    UseOriginalAlphaParam: crate::nodes::types::Boolean,
    AddonHighpackParam: crate::nodes::types::Boolean,
    BboxWidthParam: crate::nodes::types::Int,
    BboxHeightParam: crate::nodes::types::Int,
    BboxLengthParam: crate::nodes::types::Int,
    HeightCmParam: crate::nodes::types::Int,
> Rodin3D_Gen25_Image<
    SeedParam,
    TaPoseParam,
    HdTextureParam,
    TextureDelightParam,
    UseOriginalAlphaParam,
    AddonHighpackParam,
    BboxWidthParam,
    BboxHeightParam,
    BboxLengthParam,
    HeightCmParam,
> {
    /// Create a new node.
    pub fn new(
        seed: Option<SeedParam>,
        ta_pose: Option<TaPoseParam>,
        hd_texture: Option<HdTextureParam>,
        texture_delight: Option<TextureDelightParam>,
        use_original_alpha: Option<UseOriginalAlphaParam>,
        addon_highpack: Option<AddonHighpackParam>,
        bbox_width: Option<BboxWidthParam>,
        bbox_height: Option<BboxHeightParam>,
        bbox_length: Option<BboxLengthParam>,
        height_cm: Option<HeightCmParam>,
    ) -> Self {
        Self {
            seed,
            ta_pose,
            hd_texture,
            texture_delight,
            use_original_alpha,
            addon_highpack,
            bbox_width,
            bbox_height,
            bbox_length,
            height_cm,
        }
    }
}
impl<
    SeedParam: crate::nodes::types::Int,
    TaPoseParam: crate::nodes::types::Boolean,
    HdTextureParam: crate::nodes::types::Boolean,
    TextureDelightParam: crate::nodes::types::Boolean,
    UseOriginalAlphaParam: crate::nodes::types::Boolean,
    AddonHighpackParam: crate::nodes::types::Boolean,
    BboxWidthParam: crate::nodes::types::Int,
    BboxHeightParam: crate::nodes::types::Int,
    BboxLengthParam: crate::nodes::types::Int,
    HeightCmParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Rodin3D_Gen25_Image<
    SeedParam,
    TaPoseParam,
    HdTextureParam,
    TextureDelightParam,
    UseOriginalAlphaParam,
    AddonHighpackParam,
    BboxWidthParam,
    BboxHeightParam,
    BboxLengthParam,
    HeightCmParam,
> {
    type Output = crate::nodes::types::File3DOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ta_pose {
            output.insert("TAPose".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hd_texture {
            output.insert("hd_texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_delight {
            output.insert("texture_delight".to_string(), v.clone().into());
        }
        if let Some(v) = &self.use_original_alpha {
            output.insert("use_original_alpha".to_string(), v.clone().into());
        }
        if let Some(v) = &self.addon_highpack {
            output.insert("addon_highpack".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_width {
            output.insert("bbox_width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_height {
            output.insert("bbox_height".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_length {
            output.insert("bbox_length".to_string(), v.clone().into());
        }
        if let Some(v) = &self.height_cm {
            output.insert("height_cm".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Gen25_Image";
    const DISPLAY_NAME: &'static str = "Rodin 3D Gen-2.5 - Image to 3D";
    const DESCRIPTION: &'static str = "Generate a 3D model from 1-5 reference images via Rodin Gen-2.5. Pick a mode (Fast / Regular / Extreme-High) to tune quality vs. cost.";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Gen-2.5 - Text to 3D**: Generate a 3D model from a text prompt via Rodin Gen-2.5. Pick a mode (Fast / Regular / Extreme-High) to tune quality vs. cost.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Gen25_Text<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    TaPoseParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    HdTextureParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    TextureDelightParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    AddonHighpackParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    BboxWidthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    BboxHeightParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    BboxLengthParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
    HeightCmParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Text prompt for the 3D model.

**Metadata**:
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
    /**T/A pose for human-like models.

**Metadata**:
  - Default: false
*/
    pub ta_pose: Option<TaPoseParam>,
    /**High-quality texture enhancement.

**Metadata**:
  - Default: false
*/
    pub hd_texture: Option<HdTextureParam>,
    /**Remove baked lighting from textures.

**Metadata**:
  - Default: false
*/
    pub texture_delight: Option<TextureDelightParam>,
    /**HighPack addon: 4K textures and ~16x faces in Quad mode.

**Metadata**:
  - Default: false
*/
    pub addon_highpack: Option<AddonHighpackParam>,
    /**Bounding-box width (Y axis). Set to 0 with the others to skip bbox.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_width: Option<BboxWidthParam>,
    /**Bounding-box height (Z axis).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_height: Option<BboxHeightParam>,
    /**Bounding-box length (X axis).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 300
  - Min: 0
*/
    pub bbox_length: Option<BboxLengthParam>,
    /**Approximate model height in centimeters (0 to skip).

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 10000
  - Min: 0
*/
    pub height_cm: Option<HeightCmParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    TaPoseParam: crate::nodes::types::Boolean,
    HdTextureParam: crate::nodes::types::Boolean,
    TextureDelightParam: crate::nodes::types::Boolean,
    AddonHighpackParam: crate::nodes::types::Boolean,
    BboxWidthParam: crate::nodes::types::Int,
    BboxHeightParam: crate::nodes::types::Int,
    BboxLengthParam: crate::nodes::types::Int,
    HeightCmParam: crate::nodes::types::Int,
> Rodin3D_Gen25_Text<
    PromptParam,
    SeedParam,
    TaPoseParam,
    HdTextureParam,
    TextureDelightParam,
    AddonHighpackParam,
    BboxWidthParam,
    BboxHeightParam,
    BboxLengthParam,
    HeightCmParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        seed: Option<SeedParam>,
        ta_pose: Option<TaPoseParam>,
        hd_texture: Option<HdTextureParam>,
        texture_delight: Option<TextureDelightParam>,
        addon_highpack: Option<AddonHighpackParam>,
        bbox_width: Option<BboxWidthParam>,
        bbox_height: Option<BboxHeightParam>,
        bbox_length: Option<BboxLengthParam>,
        height_cm: Option<HeightCmParam>,
    ) -> Self {
        Self {
            prompt,
            seed,
            ta_pose,
            hd_texture,
            texture_delight,
            addon_highpack,
            bbox_width,
            bbox_height,
            bbox_length,
            height_cm,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    TaPoseParam: crate::nodes::types::Boolean,
    HdTextureParam: crate::nodes::types::Boolean,
    TextureDelightParam: crate::nodes::types::Boolean,
    AddonHighpackParam: crate::nodes::types::Boolean,
    BboxWidthParam: crate::nodes::types::Int,
    BboxHeightParam: crate::nodes::types::Int,
    BboxLengthParam: crate::nodes::types::Int,
    HeightCmParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for Rodin3D_Gen25_Text<
    PromptParam,
    SeedParam,
    TaPoseParam,
    HdTextureParam,
    TextureDelightParam,
    AddonHighpackParam,
    BboxWidthParam,
    BboxHeightParam,
    BboxLengthParam,
    HeightCmParam,
> {
    type Output = crate::nodes::types::File3DOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        if let Some(v) = &self.seed {
            output.insert("seed".to_string(), v.clone().into());
        }
        if let Some(v) = &self.ta_pose {
            output.insert("TAPose".to_string(), v.clone().into());
        }
        if let Some(v) = &self.hd_texture {
            output.insert("hd_texture".to_string(), v.clone().into());
        }
        if let Some(v) = &self.texture_delight {
            output.insert("texture_delight".to_string(), v.clone().into());
        }
        if let Some(v) = &self.addon_highpack {
            output.insert("addon_highpack".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_width {
            output.insert("bbox_width".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_height {
            output.insert("bbox_height".to_string(), v.clone().into());
        }
        if let Some(v) = &self.bbox_length {
            output.insert("bbox_length".to_string(), v.clone().into());
        }
        if let Some(v) = &self.height_cm {
            output.insert("height_cm".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Gen25_Text";
    const DISPLAY_NAME: &'static str = "Rodin 3D Gen-2.5 - Text to 3D";
    const DESCRIPTION: &'static str = "Generate a 3D model from a text prompt via Rodin Gen-2.5. Pick a mode (Fast / Regular / Extreme-High) to tune quality vs. cost.";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Generate - Regular Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Regular<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Regular<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Regular<ImagesParam, SeedParam> {
    type Output = out::Rodin3D_RegularOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            n_3_d_model_path: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                0u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Regular";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Regular Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Generate - Sketch Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Sketch<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Sketch<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Sketch<ImagesParam, SeedParam> {
    type Output = out::Rodin3D_SketchOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            n_3_d_model_path: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                0u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Sketch";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Sketch Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
///**Rodin 3D Generate - Smooth Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Smooth<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Smooth<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Smooth<ImagesParam, SeedParam> {
    type Output = out::Rodin3D_SmoothOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            n_3_d_model_path: crate::nodes::types::StringOut::from_dynamic(
                node_id,
                0u32,
            ),
            glb: crate::nodes::types::File3DGlbOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Smooth";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Smooth Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "partner/3d/Rodin";
}
