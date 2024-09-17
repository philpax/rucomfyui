//!transform
///**Crop Latent**
pub struct LatentCrop<
    Samples: crate::nodes::Latent,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
///**Flip Latent**
pub struct LatentFlip<Samples: crate::nodes::Latent> {
    ///No documentation.
    pub samples: Samples,
}
///**Rotate Latent**
pub struct LatentRotate<Samples: crate::nodes::Latent> {
    ///No documentation.
    pub samples: Samples,
}
