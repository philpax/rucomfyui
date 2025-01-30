//!`inpaint` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`InpaintModelConditioning`](super::InpaintModelConditioning).
    #[derive(Clone)]
    pub struct InpaintModelConditioningOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub latent: crate::nodes::types::LatentOut,
    }
}
///**CosmosImageToVideoLatent**: No description.
#[derive(Clone)]
pub struct CosmosImageToVideoLatent<
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    StartImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    EndImage: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub vae: Vae,
    /**No documentation.

**Metadata**:
  - Default: 1280
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 704
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 121
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: Length,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSize,
    ///No documentation.
    pub start_image: Option<StartImage>,
    ///No documentation.
    pub end_image: Option<EndImage>,
}
impl<
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    StartImage: crate::nodes::types::Image,
    EndImage: crate::nodes::types::Image,
> CosmosImageToVideoLatent<Vae, Width, Height, Length, BatchSize, StartImage, EndImage> {
    /// Create a new node.
    pub fn new(
        vae: Vae,
        width: Width,
        height: Height,
        length: Length,
        batch_size: BatchSize,
        start_image: Option<StartImage>,
        end_image: Option<EndImage>,
    ) -> Self {
        Self {
            vae,
            width,
            height,
            length,
            batch_size,
            start_image,
            end_image,
        }
    }
}
impl<
    Vae: crate::nodes::types::Vae,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Length: crate::nodes::types::Int,
    BatchSize: crate::nodes::types::Int,
    StartImage: crate::nodes::types::Image,
    EndImage: crate::nodes::types::Image,
> crate::nodes::TypedNode
for CosmosImageToVideoLatent<
    Vae,
    Width,
    Height,
    Length,
    BatchSize,
    StartImage,
    EndImage,
> {
    type Output = crate::nodes::types::LatentOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        if let Some(v) = &self.start_image {
            output.insert("start_image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.end_image {
            output.insert("end_image".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CosmosImageToVideoLatent";
    const DISPLAY_NAME: &'static str = "CosmosImageToVideoLatent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/inpaint";
}
///**InpaintModelConditioning**: No description.
#[derive(Clone)]
pub struct InpaintModelConditioning<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    NoiseMask: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub pixels: Pixels,
    ///No documentation.
    pub mask: Mask,
    /**Add a noise mask to the latent so sampling will only happen within the mask. Might improve results or completely break things depending on the model.

**Metadata**:
  - Default: true
*/
    pub noise_mask: NoiseMask,
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    NoiseMask: crate::nodes::types::Boolean,
> InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask, NoiseMask> {
    /// Create a new node.
    pub fn new(
        positive: Positive,
        negative: Negative,
        vae: Vae,
        pixels: Pixels,
        mask: Mask,
        noise_mask: NoiseMask,
    ) -> Self {
        Self {
            positive,
            negative,
            vae,
            pixels,
            mask,
            noise_mask,
        }
    }
}
impl<
    Positive: crate::nodes::types::Conditioning,
    Negative: crate::nodes::types::Conditioning,
    Vae: crate::nodes::types::Vae,
    Pixels: crate::nodes::types::Image,
    Mask: crate::nodes::types::Mask,
    NoiseMask: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for InpaintModelConditioning<Positive, Negative, Vae, Pixels, Mask, NoiseMask> {
    type Output = out::InpaintModelConditioningOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
            latent: crate::nodes::types::LatentOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("pixels".to_string(), self.pixels.clone().into());
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("noise_mask".to_string(), self.noise_mask.clone().into());
        output
    }
    const NAME: &'static str = "InpaintModelConditioning";
    const DISPLAY_NAME: &'static str = "InpaintModelConditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/inpaint";
}
