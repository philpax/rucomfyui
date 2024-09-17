//!mask
pub mod compositing;
///**CropMask**
pub struct CropMask<
    Mask: crate::nodes::Mask,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
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
pub struct CropMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Mask: crate::nodes::Mask,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> crate::nodes::TypedNode for CropMask<Mask, X, Y, Width, Height> {
    type Output = CropMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "CropMask";
    const DISPLAY_NAME: &'static str = "CropMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**FeatherMask**
pub struct FeatherMask<
    Mask: crate::nodes::Mask,
    Left: crate::nodes::Int,
    Top: crate::nodes::Int,
    Right: crate::nodes::Int,
    Bottom: crate::nodes::Int,
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
pub struct FeatherMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Mask: crate::nodes::Mask,
    Left: crate::nodes::Int,
    Top: crate::nodes::Int,
    Right: crate::nodes::Int,
    Bottom: crate::nodes::Int,
> crate::nodes::TypedNode for FeatherMask<Mask, Left, Top, Right, Bottom> {
    type Output = FeatherMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "FeatherMask";
    const DISPLAY_NAME: &'static str = "FeatherMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**GrowMask**
pub struct GrowMask<
    Mask: crate::nodes::Mask,
    Expand: crate::nodes::Int,
    TaperedCorners: crate::nodes::Boolean,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub expand: Expand,
    ///No documentation.
    pub tapered_corners: TaperedCorners,
}
///Output for [`GrowMask`].
pub struct GrowMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Mask: crate::nodes::Mask,
    Expand: crate::nodes::Int,
    TaperedCorners: crate::nodes::Boolean,
> crate::nodes::TypedNode for GrowMask<Mask, Expand, TaperedCorners> {
    type Output = GrowMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "GrowMask";
    const DISPLAY_NAME: &'static str = "GrowMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ImageColorToMask**
pub struct ImageColorToMask<Image: crate::nodes::Image, Color: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub color: Color,
}
///Output for [`ImageColorToMask`].
pub struct ImageColorToMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<Image: crate::nodes::Image, Color: crate::nodes::Int> crate::nodes::TypedNode
for ImageColorToMask<Image, Color> {
    type Output = ImageColorToMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "ImageColorToMask";
    const DISPLAY_NAME: &'static str = "ImageColorToMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Image to Mask**
pub struct ImageToMask<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
///Output for [`ImageToMask`].
pub struct ImageToMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<Image: crate::nodes::Image> crate::nodes::TypedNode for ImageToMask<Image> {
    type Output = ImageToMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "ImageToMask";
    const DISPLAY_NAME: &'static str = "Convert Image to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**InvertMask**
pub struct InvertMask<Mask: crate::nodes::Mask> {
    ///No documentation.
    pub mask: Mask,
}
///Output for [`InvertMask`].
pub struct InvertMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<Mask: crate::nodes::Mask> crate::nodes::TypedNode for InvertMask<Mask> {
    type Output = InvertMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "InvertMask";
    const DISPLAY_NAME: &'static str = "InvertMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Load Image (as Mask)**
pub struct LoadImageMask {}
///Output for [`LoadImageMask`].
pub struct LoadImageMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl crate::nodes::TypedNode for LoadImageMask {
    type Output = LoadImageMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "LoadImageMask";
    const DISPLAY_NAME: &'static str = "Load Image (as Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**MaskComposite**
pub struct MaskComposite<
    Destination: crate::nodes::Mask,
    Source: crate::nodes::Mask,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
///Output for [`MaskComposite`].
pub struct MaskCompositeOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Destination: crate::nodes::Mask,
    Source: crate::nodes::Mask,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> crate::nodes::TypedNode for MaskComposite<Destination, Source, X, Y> {
    type Output = MaskCompositeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "MaskComposite";
    const DISPLAY_NAME: &'static str = "MaskComposite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Mask to Image**
pub struct MaskToImage<Mask: crate::nodes::Mask> {
    ///No documentation.
    pub mask: Mask,
}
///Output for [`MaskToImage`].
pub struct MaskToImageOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Mask: crate::nodes::Mask> crate::nodes::TypedNode for MaskToImage<Mask> {
    type Output = MaskToImageOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "MaskToImage";
    const DISPLAY_NAME: &'static str = "Convert Mask to Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**SolidMask**
pub struct SolidMask<
    Value: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> {
    ///No documentation.
    pub value: Value,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
///Output for [`SolidMask`].
pub struct SolidMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Value: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
> crate::nodes::TypedNode for SolidMask<Value, Width, Height> {
    type Output = SolidMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "SolidMask";
    const DISPLAY_NAME: &'static str = "SolidMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ThresholdMask**
pub struct ThresholdMask<Mask: crate::nodes::Mask, Value: crate::nodes::Float> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub value: Value,
}
///Output for [`ThresholdMask`].
pub struct ThresholdMaskOutput {
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<Mask: crate::nodes::Mask, Value: crate::nodes::Float> crate::nodes::TypedNode
for ThresholdMask<Mask, Value> {
    type Output = ThresholdMaskOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            mask: crate::nodes::MaskOut(0u32),
        }
    }
    const NAME: &'static str = "ThresholdMask";
    const DISPLAY_NAME: &'static str = "ThresholdMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
