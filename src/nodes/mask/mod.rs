//!`mask` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
pub mod compositing;
/// Output types for nodes.
pub mod out {
    ///Output for [`CropMask`](super::CropMask).
    #[derive(Clone)]
    pub struct CropMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`FeatherMask`](super::FeatherMask).
    #[derive(Clone)]
    pub struct FeatherMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`GrowMask`](super::GrowMask).
    #[derive(Clone)]
    pub struct GrowMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`ImageColorToMask`](super::ImageColorToMask).
    #[derive(Clone)]
    pub struct ImageColorToMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`ImageToMask`](super::ImageToMask).
    #[derive(Clone)]
    pub struct ImageToMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`InvertMask`](super::InvertMask).
    #[derive(Clone)]
    pub struct InvertMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`LoadImageMask`](super::LoadImageMask).
    #[derive(Clone)]
    pub struct LoadImageMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`MaskComposite`](super::MaskComposite).
    #[derive(Clone)]
    pub struct MaskCompositeOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`MaskToImage`](super::MaskToImage).
    #[derive(Clone)]
    pub struct MaskToImageOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`SolidMask`](super::SolidMask).
    #[derive(Clone)]
    pub struct SolidMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`ThresholdMask`](super::ThresholdMask).
    #[derive(Clone)]
    pub struct ThresholdMaskOutput {
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**CropMask**: No description.
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
impl<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for CropMask<Mask, X, Y, Width, Height> {
    type Output = out::CropMaskOutput;
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
///**FeatherMask**: No description.
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
impl<
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> crate::nodes::TypedNode for FeatherMask<Mask, Left, Top, Right, Bottom> {
    type Output = out::FeatherMaskOutput;
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
///**GrowMask**: No description.
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
impl<
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for GrowMask<Mask, Expand, TaperedCorners> {
    type Output = out::GrowMaskOutput;
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
///**ImageColorToMask**: No description.
pub struct ImageColorToMask<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub color: Color,
}
impl<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageColorToMask<Image, Color> {
    type Output = out::ImageColorToMaskOutput;
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
///**Convert Image to Mask**: No description.
pub struct ImageToMask<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
impl<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageToMask<Image, Channel> {
    type Output = out::ImageToMaskOutput;
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
///**InvertMask**: No description.
pub struct InvertMask<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for InvertMask<Mask> {
    type Output = out::InvertMaskOutput;
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
///**Load Image (as Mask)**: No description.
pub struct LoadImageMask<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
impl<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for LoadImageMask<Image, Channel> {
    type Output = out::LoadImageMaskOutput;
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
///**MaskComposite**: No description.
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
impl<
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> crate::nodes::TypedNode for MaskComposite<Destination, Source, X, Y, Operation> {
    type Output = out::MaskCompositeOutput;
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
///**Convert Mask to Image**: No description.
pub struct MaskToImage<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for MaskToImage<Mask> {
    type Output = out::MaskToImageOutput;
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
///**SolidMask**: No description.
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
impl<
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for SolidMask<Value, Width, Height> {
    type Output = out::SolidMaskOutput;
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
///**ThresholdMask**: No description.
pub struct ThresholdMask<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub value: Value,
}
impl<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> crate::nodes::TypedNode for ThresholdMask<Mask, Value> {
    type Output = out::ThresholdMaskOutput;
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
