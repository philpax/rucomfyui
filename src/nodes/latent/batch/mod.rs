//!batch
///**LatentBatch**
pub struct LatentBatch<Samples1: crate::nodes::Latent, Samples2: crate::nodes::Latent> {
    ///No documentation.
    pub samples1: Samples1,
    ///No documentation.
    pub samples2: Samples2,
}
///**Latent From Batch**
pub struct LatentFromBatch<
    Samples: crate::nodes::Latent,
    BatchIndex: crate::nodes::Int,
    Length: crate::nodes::Int,
> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub batch_index: BatchIndex,
    ///No documentation.
    pub length: Length,
}
///**Rebatch Latents**
pub struct RebatchLatents<Latents: crate::nodes::Latent, BatchSize: crate::nodes::Int> {
    ///No documentation.
    pub latents: Latents,
    ///No documentation.
    pub batch_size: BatchSize,
}
///**Repeat Latent Batch**
pub struct RepeatLatentBatch<Samples: crate::nodes::Latent, Amount: crate::nodes::Int> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub amount: Amount,
}
