//!`conditioning` definitions/categories.
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
    ///Output for [`TripoSplatConditioning`](super::TripoSplatConditioning).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TripoSplatConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///The fixed size noise target (latent +camera).
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**TripoSplat Conditioning**: Encode the image with DINOv3 and the Flux2 VAE into TripoSplat positive/negative conditioning, and create the fixed size noise target (latent + camera) for the KSampler
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoSplatConditioning<
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
> {
    ///DINOv3 ViT-H/16+ image encoder
    pub clip_vision: ClipVisionParam,
    ///Flux2 VAE
    pub vae: VaeParam,
    ///No documentation.
    pub image: ImageParam,
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
> TripoSplatConditioning<ClipVisionParam, VaeParam, ImageParam> {
    /// Create a new node.
    pub fn new(clip_vision: ClipVisionParam, vae: VaeParam, image: ImageParam) -> Self {
        Self { clip_vision, vae, image }
    }
}
impl<
    ClipVisionParam: crate::nodes::types::ClipVision,
    VaeParam: crate::nodes::types::Vae,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for TripoSplatConditioning<ClipVisionParam, VaeParam, ImageParam> {
    type Output = out::TripoSplatConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip_vision".to_string(), self.clip_vision.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "TripoSplatConditioning";
    const DISPLAY_NAME: &'static str = "TripoSplat Conditioning";
    const DESCRIPTION: &'static str = "Encode the image with DINOv3 and the Flux2 VAE into TripoSplat positive/negative conditioning, and create the fixed size noise target (latent + camera) for the KSampler";
    const CATEGORY: &'static str = "3d/conditioning";
}
///**TripoSplat Preprocess Image**: Crop center each image to a square canvas on a black background and add padding.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TripoSplatPreprocessImage<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    ErodeRadiusParam: crate::nodes::types::Int,
    SizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub mask: MaskParam,
    /**Erode the alpha matte by this pixel radius before cropping (avoids border bleed).

**Metadata**:
  - Default: 1
  - Max: 16
  - Min: 0
*/
    pub erode_radius: ErodeRadiusParam,
    /**Square image size. The model is trained at 1024; other sizes run but are off-distribution.

**Metadata**:
  - Default: 1024
  - Max: 4096
  - Min: 256
  - Step: 16
*/
    pub size: SizeParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    ErodeRadiusParam: crate::nodes::types::Int,
    SizeParam: crate::nodes::types::Int,
> TripoSplatPreprocessImage<ImageParam, MaskParam, ErodeRadiusParam, SizeParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        mask: MaskParam,
        erode_radius: ErodeRadiusParam,
        size: SizeParam,
    ) -> Self {
        Self {
            image,
            mask,
            erode_radius,
            size,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    ErodeRadiusParam: crate::nodes::types::Int,
    SizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TripoSplatPreprocessImage<ImageParam, MaskParam, ErodeRadiusParam, SizeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("erode_radius".to_string(), self.erode_radius.clone().into());
        output.insert("size".to_string(), self.size.clone().into());
        output
    }
    const NAME: &'static str = "TripoSplatPreprocessImage";
    const DISPLAY_NAME: &'static str = "TripoSplat Preprocess Image";
    const DESCRIPTION: &'static str = "Crop center each image to a square canvas on a black background and add padding.";
    const CATEGORY: &'static str = "3d/conditioning";
}
