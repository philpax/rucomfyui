//!batch
///**ImageFromBatch**
pub struct ImageFromBatch<
    Image: crate::nodes::Image,
    BatchIndex: crate::nodes::Int,
    Length: crate::nodes::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub batch_index: BatchIndex,
    ///No documentation.
    pub length: Length,
}
///Output for [`ImageFromBatch`].
pub struct ImageFromBatchOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<
    Image: crate::nodes::Image,
    BatchIndex: crate::nodes::Int,
    Length: crate::nodes::Int,
> crate::nodes::TypedNode for ImageFromBatch<Image, BatchIndex, Length> {
    type Output = ImageFromBatchOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "ImageFromBatch";
    const DISPLAY_NAME: &'static str = "ImageFromBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Rebatch Images**
pub struct RebatchImages<Images: crate::nodes::Image, BatchSize: crate::nodes::Int> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub batch_size: BatchSize,
}
///Output for [`RebatchImages`].
pub struct RebatchImagesOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Images: crate::nodes::Image, BatchSize: crate::nodes::Int> crate::nodes::TypedNode
for RebatchImages<Images, BatchSize> {
    type Output = RebatchImagesOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "RebatchImages";
    const DISPLAY_NAME: &'static str = "Rebatch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**RepeatImageBatch**
pub struct RepeatImageBatch<Image: crate::nodes::Image, Amount: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub amount: Amount,
}
///Output for [`RepeatImageBatch`].
pub struct RepeatImageBatchOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image: crate::nodes::Image, Amount: crate::nodes::Int> crate::nodes::TypedNode
for RepeatImageBatch<Image, Amount> {
    type Output = RepeatImageBatchOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut(0u32),
        }
    }
    const NAME: &'static str = "RepeatImageBatch";
    const DISPLAY_NAME: &'static str = "RepeatImageBatch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
