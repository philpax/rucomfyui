//!`mask` definitions/categories.
#![allow(unused_imports)]
use crate::WorkflowNodeId;
pub mod compositing;
///**CropMask**
pub struct CropMask<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///Output for [`CropMask`].
#[derive(Clone)]
pub struct CropMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for CropMask<Mask, X, Y, Width, Height> {
    type Output = CropMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CropMask";
    const DISPLAY_NAME: &'static str = "CropMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**FeatherMask**
pub struct FeatherMask<
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub left: Left,
    ///No documentation.
    pub top: Top,
    ///No documentation.
    pub right: Right,
    ///No documentation.
    pub bottom: Bottom,
}
///Output for [`FeatherMask`].
#[derive(Clone)]
pub struct FeatherMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> crate::nodes::TypedNode for FeatherMask<Mask, Left, Top, Right, Bottom> {
    type Output = FeatherMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "FeatherMask";
    const DISPLAY_NAME: &'static str = "FeatherMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**GrowMask**
pub struct GrowMask<
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub expand: Expand,
    ///No documentation.
    pub tapered_corners: TaperedCorners,
}
///Output for [`GrowMask`].
#[derive(Clone)]
pub struct GrowMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for GrowMask<Mask, Expand, TaperedCorners> {
    type Output = GrowMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "GrowMask";
    const DISPLAY_NAME: &'static str = "GrowMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ImageColorToMask**
pub struct ImageColorToMask<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub color: Color,
}
///Output for [`ImageColorToMask`].
#[derive(Clone)]
pub struct ImageColorToMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageColorToMask<Image, Color> {
    type Output = ImageColorToMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageColorToMask";
    const DISPLAY_NAME: &'static str = "ImageColorToMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Image to Mask**
pub struct ImageToMask<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
///Output for [`ImageToMask`].
#[derive(Clone)]
pub struct ImageToMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageToMask<Image, Channel> {
    type Output = ImageToMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ImageToMask";
    const DISPLAY_NAME: &'static str = "Convert Image to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**InvertMask**
pub struct InvertMask<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
///Output for [`InvertMask`].
#[derive(Clone)]
pub struct InvertMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for InvertMask<Mask> {
    type Output = InvertMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "InvertMask";
    const DISPLAY_NAME: &'static str = "InvertMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Load Image (as Mask)**
pub struct LoadImageMask<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
///Output for [`LoadImageMask`].
#[derive(Clone)]
pub struct LoadImageMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for LoadImageMask<Image, Channel> {
    type Output = LoadImageMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "LoadImageMask";
    const DISPLAY_NAME: &'static str = "Load Image (as Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**MaskComposite**
pub struct MaskComposite<
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub operation: Operation,
}
///Output for [`MaskComposite`].
#[derive(Clone)]
pub struct MaskCompositeOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> crate::nodes::TypedNode for MaskComposite<Destination, Source, X, Y, Operation> {
    type Output = MaskCompositeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "MaskComposite";
    const DISPLAY_NAME: &'static str = "MaskComposite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Mask to Image**
pub struct MaskToImage<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
///Output for [`MaskToImage`].
#[derive(Clone)]
pub struct MaskToImageOutput {
    ///No documentation.
    pub image: crate::nodes::types::ImageOut,
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for MaskToImage<Mask> {
    type Output = MaskToImageOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "MaskToImage";
    const DISPLAY_NAME: &'static str = "Convert Mask to Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**SolidMask**
pub struct SolidMask<
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub value: Value,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///Output for [`SolidMask`].
#[derive(Clone)]
pub struct SolidMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for SolidMask<Value, Width, Height> {
    type Output = SolidMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "SolidMask";
    const DISPLAY_NAME: &'static str = "SolidMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ThresholdMask**
pub struct ThresholdMask<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub value: Value,
}
///Output for [`ThresholdMask`].
#[derive(Clone)]
pub struct ThresholdMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::types::MaskOut,
}
impl<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> crate::nodes::TypedNode for ThresholdMask<Mask, Value> {
    type Output = ThresholdMaskOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ThresholdMask";
    const DISPLAY_NAME: &'static str = "ThresholdMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
