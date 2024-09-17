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
///**ImageColorToMask**
pub struct ImageColorToMask<Image: crate::nodes::Image, Color: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub color: Color,
}
///**Convert Image to Mask**
pub struct ImageToMask<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
///**InvertMask**
pub struct InvertMask<Mask: crate::nodes::Mask> {
    ///No documentation.
    pub mask: Mask,
}
///**Load Image (as Mask)**
pub struct LoadImageMask {}
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
///**Convert Mask to Image**
pub struct MaskToImage<Mask: crate::nodes::Mask> {
    ///No documentation.
    pub mask: Mask,
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
///**ThresholdMask**
pub struct ThresholdMask<Mask: crate::nodes::Mask, Value: crate::nodes::Float> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub value: Value,
}
