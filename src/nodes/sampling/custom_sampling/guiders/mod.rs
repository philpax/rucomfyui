//!guiders
///**BasicGuider**
pub struct BasicGuider<
    Model: crate::nodes::Model,
    Conditioning: crate::nodes::Conditioning,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub conditioning: Conditioning,
}
///**CFGGuider**
pub struct CfgGuider<
    Model: crate::nodes::Model,
    Positive: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    Cfg: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub positive: Positive,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub cfg: Cfg,
}
///**DualCFGGuider**
pub struct DualCfgGuider<
    Model: crate::nodes::Model,
    Cond1: crate::nodes::Conditioning,
    Cond2: crate::nodes::Conditioning,
    Negative: crate::nodes::Conditioning,
    CfgConds: crate::nodes::Float,
    CfgCond2Negative: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub cond1: Cond1,
    ///No documentation.
    pub cond2: Cond2,
    ///No documentation.
    pub negative: Negative,
    ///No documentation.
    pub cfg_conds: CfgConds,
    ///No documentation.
    pub cfg_cond2_negative: CfgCond2Negative,
}
