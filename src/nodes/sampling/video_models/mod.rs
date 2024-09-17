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
