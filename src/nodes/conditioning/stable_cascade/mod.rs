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
///Output for [`StableCascadeStageBConditioning`].
pub struct StableCascadeStageBConditioningOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    StageC: crate::nodes::Latent,
> crate::nodes::TypedNode for StableCascadeStageBConditioning<Conditioning, StageC> {
    type Output = StableCascadeStageBConditioningOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0usize),
        }
    }
    const NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DISPLAY_NAME: &'static str = "StableCascade_StageB_Conditioning";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "conditioning/stable_cascade";
}
