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
    #[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
pub struct CosmosImageToVideoLatent<
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    EndImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
> {
    ///No documentation.
    pub vae: VaeParam,
    /**No documentation.

**Metadata**:
  - Default: 1280
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 704
  - Max: 16384
  - Min: 16
  - Step: 16
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 121
  - Max: 16384
  - Min: 1
  - Step: 8
*/
    pub length: LengthParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
    ///No documentation.
    pub start_image: Option<StartImageParam>,
    ///No documentation.
    pub end_image: Option<EndImageParam>,
}
impl<
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> CosmosImageToVideoLatent<
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    EndImageParam,
> {
    /// Create a new node.
    pub fn new(
        vae: VaeParam,
        width: WidthParam,
        height: HeightParam,
        length: LengthParam,
        batch_size: BatchSizeParam,
        start_image: Option<StartImageParam>,
        end_image: Option<EndImageParam>,
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
    VaeParam: crate::nodes::types::Vae,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
    BatchSizeParam: crate::nodes::types::Int,
    StartImageParam: crate::nodes::types::Image,
    EndImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode
for CosmosImageToVideoLatent<
    VaeParam,
    WidthParam,
    HeightParam,
    LengthParam,
    BatchSizeParam,
    StartImageParam,
    EndImageParam,
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
#[allow(non_camel_case_types)]
pub struct InpaintModelConditioning<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    NoiseMaskParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
    ///No documentation.
    pub vae: VaeParam,
    ///No documentation.
    pub pixels: PixelsParam,
    ///No documentation.
    pub mask: MaskParam,
    /**Add a noise mask to the latent so sampling will only happen within the mask. Might improve results or completely break things depending on the model.

**Metadata**:
  - Default: true
*/
    pub noise_mask: NoiseMaskParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    NoiseMaskParam: crate::nodes::types::Boolean,
> InpaintModelConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    PixelsParam,
    MaskParam,
    NoiseMaskParam,
> {
    /// Create a new node.
    pub fn new(
        positive: PositiveParam,
        negative: NegativeParam,
        vae: VaeParam,
        pixels: PixelsParam,
        mask: MaskParam,
        noise_mask: NoiseMaskParam,
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
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
    VaeParam: crate::nodes::types::Vae,
    PixelsParam: crate::nodes::types::Image,
    MaskParam: crate::nodes::types::Mask,
    NoiseMaskParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for InpaintModelConditioning<
    PositiveParam,
    NegativeParam,
    VaeParam,
    PixelsParam,
    MaskParam,
    NoiseMaskParam,
> {
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
