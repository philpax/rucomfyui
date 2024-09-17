//!compositing
///**Join Image with Alpha**
pub struct JoinImageWithAlpha<Image: crate::nodes::Image, Alpha: crate::nodes::Mask> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub alpha: Alpha,
}
///**Porter-Duff Image Composite**
pub struct PorterDuffImageComposite<
    Source: crate::nodes::Image,
    SourceAlpha: crate::nodes::Mask,
    Destination: crate::nodes::Image,
    DestinationAlpha: crate::nodes::Mask,
> {
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub source_alpha: SourceAlpha,
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub destination_alpha: DestinationAlpha,
}
///**Split Image with Alpha**
pub struct SplitImageWithAlpha<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
