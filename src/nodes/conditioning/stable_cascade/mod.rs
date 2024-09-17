//!stable_cascade
///**StableCascade_StageB_Conditioning**
pub struct StableCascadeStageBConditioning<
    Conditioning: crate::nodes::Conditioning,
    StageC: crate::nodes::Latent,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub stage_c: StageC,
}
