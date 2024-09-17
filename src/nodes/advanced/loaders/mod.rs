//!loaders
pub mod deprecated;
///**Load CLIP**
pub struct ClipLoader {}
///Output for [`ClipLoader`].
pub struct ClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl crate::nodes::TypedNode for ClipLoader {
    type Output = ClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0usize),
        }
    }
    const NAME: &'static str = "CLIPLoader";
    const DISPLAY_NAME: &'static str = "Load CLIP";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Checkpoint With Config (DEPRECATED)**
pub struct CheckpointLoader {}
///Output for [`CheckpointLoader`].
pub struct CheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl crate::nodes::TypedNode for CheckpointLoader {
    type Output = CheckpointLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0usize),
            clip: crate::nodes::ClipOut(1usize),
            vae: crate::nodes::VaeOut(2usize),
        }
    }
    const NAME: &'static str = "CheckpointLoader";
    const DISPLAY_NAME: &'static str = "Load Checkpoint With Config (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**DualCLIPLoader**
pub struct DualClipLoader {}
///Output for [`DualClipLoader`].
pub struct DualClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl crate::nodes::TypedNode for DualClipLoader {
    type Output = DualClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0usize),
        }
    }
    const NAME: &'static str = "DualCLIPLoader";
    const DISPLAY_NAME: &'static str = "DualCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**TripleCLIPLoader**
pub struct TripleClipLoader {}
///Output for [`TripleClipLoader`].
pub struct TripleClipLoaderOutput {
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
}
impl crate::nodes::TypedNode for TripleClipLoader {
    type Output = TripleClipLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            clip: crate::nodes::ClipOut(0usize),
        }
    }
    const NAME: &'static str = "TripleCLIPLoader";
    const DISPLAY_NAME: &'static str = "TripleCLIPLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
///**Load Diffusion Model**
pub struct UnetLoader {}
///Output for [`UnetLoader`].
pub struct UnetLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl crate::nodes::TypedNode for UnetLoader {
    type Output = UnetLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0usize),
        }
    }
    const NAME: &'static str = "UNETLoader";
    const DISPLAY_NAME: &'static str = "Load Diffusion Model";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders";
}
