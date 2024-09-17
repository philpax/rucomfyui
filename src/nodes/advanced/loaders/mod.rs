//!loaders
pub mod deprecated;
///**Load CLIP**
pub struct ClipLoader<ClipName: crate::nodes::String, Type: crate::nodes::String> {
    ///No documentation.
    pub clip_name: ClipName,
    ///No documentation.
    pub type_: Type,
}
///Output for [`ClipLoader`].
#[derive(Clone)]
pub struct ClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl<ClipName: crate::nodes::String, Type: crate::nodes::String> crate::nodes::TypedNode
for ClipLoader<ClipName, Type> {
    type Output = ClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Checkpoint With Config (DEPRECATED)**
pub struct CheckpointLoader<
    ConfigName: crate::nodes::String,
    CkptName: crate::nodes::String,
> {
    ///No documentation.
    pub config_name: ConfigName,
    ///No documentation.
    pub ckpt_name: CkptName,
}
///Output for [`CheckpointLoader`].
#[derive(Clone)]
pub struct CheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl<
    ConfigName: crate::nodes::String,
    CkptName: crate::nodes::String,
> crate::nodes::TypedNode for CheckpointLoader<ConfigName, CkptName> {
    type Output = CheckpointLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
            clip: crate::nodes::ClipOut(1u32),
            vae: crate::nodes::VaeOut(2u32),
        }
    }
    const NAME: &'static str = "CheckpointLoader";
    const DISPLAY_NAME: &'static str = "Load Checkpoint With Config (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**DualCLIPLoader**
pub struct DualClipLoader<
    ClipName1: crate::nodes::String,
    ClipName2: crate::nodes::String,
    Type: crate::nodes::String,
> {
    ///No documentation.
    pub clip_name_1: ClipName1,
    ///No documentation.
    pub clip_name_2: ClipName2,
    ///No documentation.
    pub type_: Type,
}
///Output for [`DualClipLoader`].
#[derive(Clone)]
pub struct DualClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl<
    ClipName1: crate::nodes::String,
    ClipName2: crate::nodes::String,
    Type: crate::nodes::String,
> crate::nodes::TypedNode for DualClipLoader<ClipName1, ClipName2, Type> {
    type Output = DualClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0u32),
        }
    }
    const NAME: &'static str = "DualCLIPLoader";
    const DISPLAY_NAME: &'static str = "DualCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**TripleCLIPLoader**
pub struct TripleClipLoader<
    ClipName1: crate::nodes::String,
    ClipName2: crate::nodes::String,
    ClipName3: crate::nodes::String,
> {
    ///No documentation.
    pub clip_name_1: ClipName1,
    ///No documentation.
    pub clip_name_2: ClipName2,
    ///No documentation.
    pub clip_name_3: ClipName3,
}
///Output for [`TripleClipLoader`].
#[derive(Clone)]
pub struct TripleClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl<
    ClipName1: crate::nodes::String,
    ClipName2: crate::nodes::String,
    ClipName3: crate::nodes::String,
> crate::nodes::TypedNode for TripleClipLoader<ClipName1, ClipName2, ClipName3> {
    type Output = TripleClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0u32),
        }
    }
    const NAME: &'static str = "TripleCLIPLoader";
    const DISPLAY_NAME: &'static str = "TripleCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Diffusion Model**
pub struct UnetLoader<
    UnetName: crate::nodes::String,
    WeightDtype: crate::nodes::String,
> {
    ///No documentation.
    pub unet_name: UnetName,
    ///No documentation.
    pub weight_dtype: WeightDtype,
}
///Output for [`UnetLoader`].
#[derive(Clone)]
pub struct UnetLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<
    UnetName: crate::nodes::String,
    WeightDtype: crate::nodes::String,
> crate::nodes::TypedNode for UnetLoader<UnetName, WeightDtype> {
    type Output = UnetLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
        }
    }
    const NAME: &'static str = "UNETLoader";
    const DISPLAY_NAME: &'static str = "Load Diffusion Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
