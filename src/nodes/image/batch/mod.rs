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
///**Rebatch Images**
pub struct RebatchImages<Images: crate::nodes::Image, BatchSize: crate::nodes::Int> {
    ///No documentation.
    pub images: Images,
    ///No documentation.
    pub batch_size: BatchSize,
}
///**RepeatImageBatch**
pub struct RepeatImageBatch<Image: crate::nodes::Image, Amount: crate::nodes::Int> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub amount: Amount,
}
