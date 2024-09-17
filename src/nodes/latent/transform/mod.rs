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
///Output for [`LatentCrop`].
#[derive(Clone)]
pub struct LatentCropOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> crate::nodes::TypedNode for LatentCrop<Samples, Width, Height, X, Y> {
    type Output = LatentCropOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentCrop";
    const DISPLAY_NAME: &'static str = "Crop Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Flip Latent**
pub struct LatentFlip<Samples: crate::nodes::Latent, FlipMethod: crate::nodes::String> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub flip_method: FlipMethod,
}
///Output for [`LatentFlip`].
#[derive(Clone)]
pub struct LatentFlipOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    FlipMethod: crate::nodes::String,
> crate::nodes::TypedNode for LatentFlip<Samples, FlipMethod> {
    type Output = LatentFlipOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentFlip";
    const DISPLAY_NAME: &'static str = "Flip Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
///**Rotate Latent**
pub struct LatentRotate<Samples: crate::nodes::Latent, Rotation: crate::nodes::String> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub rotation: Rotation,
}
///Output for [`LatentRotate`].
#[derive(Clone)]
pub struct LatentRotateOutput {
    ///No documentation.
    pub latent: crate::nodes::LatentOut,
}
impl<
    Samples: crate::nodes::Latent,
    Rotation: crate::nodes::String,
> crate::nodes::TypedNode for LatentRotate<Samples, Rotation> {
    type Output = LatentRotateOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            latent: crate::nodes::LatentOut(0u32),
        }
    }
    const NAME: &'static str = "LatentRotate";
    const DISPLAY_NAME: &'static str = "Rotate Latent";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "latent/transform";
}
