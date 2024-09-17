//!video_models
///**VideoLinearCFGGuidance**
pub struct VideoLinearCfgGuidance<
    Model: crate::nodes::Model,
    MinCfg: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
}
///Output for [`VideoLinearCfgGuidance`].
pub struct VideoLinearCfgGuidanceOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, MinCfg: crate::nodes::Float> crate::nodes::TypedNode
for VideoLinearCfgGuidance<Model, MinCfg> {
    type Output = VideoLinearCfgGuidanceOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0usize),
        }
    }
    const NAME: &'static str = "VideoLinearCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoLinearCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
///**VideoTriangleCFGGuidance**
pub struct VideoTriangleCfgGuidance<
    Model: crate::nodes::Model,
    MinCfg: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub min_cfg: MinCfg,
}
///Output for [`VideoTriangleCfgGuidance`].
pub struct VideoTriangleCfgGuidanceOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
}
impl<Model: crate::nodes::Model, MinCfg: crate::nodes::Float> crate::nodes::TypedNode
for VideoTriangleCfgGuidance<Model, MinCfg> {
    type Output = VideoTriangleCfgGuidanceOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0usize),
        }
    }
    const NAME: &'static str = "VideoTriangleCFGGuidance";
    const DISPLAY_NAME: &'static str = "VideoTriangleCFGGuidance";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/video_models";
}
