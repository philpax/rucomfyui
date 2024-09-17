//!preprocessors
///**Canny**
pub struct Canny<
    Image: crate::nodes::Image,
    LowThreshold: crate::nodes::Float,
    HighThreshold: crate::nodes::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub low_threshold: LowThreshold,
    ///No documentation.
    pub high_threshold: HighThreshold,
}
