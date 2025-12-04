//!`schedulers` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**AlignYourStepsScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AlignYourStepsScheduler<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> AlignYourStepsScheduler<StepsParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(steps: StepsParam, denoise: DenoiseParam) -> Self {
        Self { steps, denoise }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for AlignYourStepsScheduler<StepsParam, DenoiseParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "AlignYourStepsScheduler";
    const DISPLAY_NAME: &'static str = "AlignYourStepsScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**BasicScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BasicScheduler<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> BasicScheduler<ModelParam, StepsParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, steps: StepsParam, denoise: DenoiseParam) -> Self {
        Self { model, steps, denoise }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for BasicScheduler<ModelParam, StepsParam, DenoiseParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "BasicScheduler";
    const DISPLAY_NAME: &'static str = "BasicScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**BetaSamplingScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BetaSamplingScheduler<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    AlphaParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 50
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub alpha: AlphaParam,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 50
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta: BetaParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    AlphaParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> BetaSamplingScheduler<ModelParam, StepsParam, AlphaParam, BetaParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        steps: StepsParam,
        alpha: AlphaParam,
        beta: BetaParam,
    ) -> Self {
        Self { model, steps, alpha, beta }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    AlphaParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for BetaSamplingScheduler<ModelParam, StepsParam, AlphaParam, BetaParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("alpha".to_string(), self.alpha.clone().into());
        output.insert("beta".to_string(), self.beta.clone().into());
        output
    }
    const NAME: &'static str = "BetaSamplingScheduler";
    const DISPLAY_NAME: &'static str = "BetaSamplingScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**ExponentialScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ExponentialScheduler<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMinParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> ExponentialScheduler<StepsParam, SigmaMaxParam, SigmaMinParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
    ) -> Self {
        Self {
            steps,
            sigma_max,
            sigma_min,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ExponentialScheduler<StepsParam, SigmaMaxParam, SigmaMinParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output
    }
    const NAME: &'static str = "ExponentialScheduler";
    const DISPLAY_NAME: &'static str = "ExponentialScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**Flux2Scheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Flux2Scheduler<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 4096
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 16
  - Step: 1
*/
    pub height: HeightParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> Flux2Scheduler<StepsParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(steps: StepsParam, width: WidthParam, height: HeightParam) -> Self {
        Self { steps, width, height }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Flux2Scheduler<StepsParam, WidthParam, HeightParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "Flux2Scheduler";
    const DISPLAY_NAME: &'static str = "Flux2Scheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**GITSScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GITSScheduler<
    CoeffParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 1.2
  - Max: 1.5
  - Min: 0.8
  - Step: 0.05
*/
    pub coeff: CoeffParam,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 1000
  - Min: 2
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    CoeffParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> GITSScheduler<CoeffParam, StepsParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(coeff: CoeffParam, steps: StepsParam, denoise: DenoiseParam) -> Self {
        Self { coeff, steps, denoise }
    }
}
impl<
    CoeffParam: crate::nodes::types::Float,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for GITSScheduler<CoeffParam, StepsParam, DenoiseParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("coeff".to_string(), self.coeff.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "GITSScheduler";
    const DISPLAY_NAME: &'static str = "GITSScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**KarrasScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct KarrasScheduler<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMinParam,
    /**No documentation.

**Metadata**:
  - Default: 7
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub rho: RhoParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> KarrasScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, RhoParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
        rho: RhoParam,
    ) -> Self {
        Self {
            steps,
            sigma_max,
            sigma_min,
            rho,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for KarrasScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, RhoParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output.insert("rho".to_string(), self.rho.clone().into());
        output
    }
    const NAME: &'static str = "KarrasScheduler";
    const DISPLAY_NAME: &'static str = "KarrasScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**LTXVScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LTXVScheduler<
    StepsParam: crate::nodes::types::Int,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    StretchParam: crate::nodes::types::Boolean,
    TerminalParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent = crate::nodes::types::LatentOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 2.05
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub max_shift: MaxShiftParam,
    /**No documentation.

**Metadata**:
  - Default: 0.95
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub base_shift: BaseShiftParam,
    /**Stretch the sigmas to be in the range \[terminal, 1\].

**Metadata**:
  - Default: true
*/
    pub stretch: StretchParam,
    /**The terminal value of the sigmas after stretching.

**Metadata**:
  - Default: 0.1
  - Max: 0.99
  - Min: 0
  - Step: 0.01
*/
    pub terminal: TerminalParam,
    ///No documentation.
    pub latent: Option<LatentParam>,
}
impl<
    StepsParam: crate::nodes::types::Int,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    StretchParam: crate::nodes::types::Boolean,
    TerminalParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent,
> LTXVScheduler<
    StepsParam,
    MaxShiftParam,
    BaseShiftParam,
    StretchParam,
    TerminalParam,
    LatentParam,
> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        max_shift: MaxShiftParam,
        base_shift: BaseShiftParam,
        stretch: StretchParam,
        terminal: TerminalParam,
        latent: Option<LatentParam>,
    ) -> Self {
        Self {
            steps,
            max_shift,
            base_shift,
            stretch,
            terminal,
            latent,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    MaxShiftParam: crate::nodes::types::Float,
    BaseShiftParam: crate::nodes::types::Float,
    StretchParam: crate::nodes::types::Boolean,
    TerminalParam: crate::nodes::types::Float,
    LatentParam: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for LTXVScheduler<
    StepsParam,
    MaxShiftParam,
    BaseShiftParam,
    StretchParam,
    TerminalParam,
    LatentParam,
> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("max_shift".to_string(), self.max_shift.clone().into());
        output.insert("base_shift".to_string(), self.base_shift.clone().into());
        output.insert("stretch".to_string(), self.stretch.clone().into());
        output.insert("terminal".to_string(), self.terminal.clone().into());
        if let Some(v) = &self.latent {
            output.insert("latent".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "LTXVScheduler";
    const DISPLAY_NAME: &'static str = "LTXVScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**LaplaceScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LaplaceScheduler<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    MuParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMinParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: -10
  - Round: false
  - Step: 0.1
*/
    pub mu: MuParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 10
  - Min: 0
  - Round: false
  - Step: 0.1
*/
    pub beta: BetaParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    MuParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> LaplaceScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, MuParam, BetaParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
        mu: MuParam,
        beta: BetaParam,
    ) -> Self {
        Self {
            steps,
            sigma_max,
            sigma_min,
            mu,
            beta,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    MuParam: crate::nodes::types::Float,
    BetaParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for LaplaceScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, MuParam, BetaParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output.insert("mu".to_string(), self.mu.clone().into());
        output.insert("beta".to_string(), self.beta.clone().into());
        output
    }
    const NAME: &'static str = "LaplaceScheduler";
    const DISPLAY_NAME: &'static str = "LaplaceScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**OptimalStepsScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct OptimalStepsScheduler<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 1000
  - Min: 3
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> OptimalStepsScheduler<StepsParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(steps: StepsParam, denoise: DenoiseParam) -> Self {
        Self { steps, denoise }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for OptimalStepsScheduler<StepsParam, DenoiseParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "OptimalStepsScheduler";
    const DISPLAY_NAME: &'static str = "OptimalStepsScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**PolyexponentialScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PolyexponentialScheduler<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMaxParam,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMinParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub rho: RhoParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> PolyexponentialScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, RhoParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        sigma_max: SigmaMaxParam,
        sigma_min: SigmaMinParam,
        rho: RhoParam,
    ) -> Self {
        Self {
            steps,
            sigma_max,
            sigma_min,
            rho,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    SigmaMaxParam: crate::nodes::types::Float,
    SigmaMinParam: crate::nodes::types::Float,
    RhoParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for PolyexponentialScheduler<StepsParam, SigmaMaxParam, SigmaMinParam, RhoParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("sigma_max".to_string(), self.sigma_max.clone().into());
        output.insert("sigma_min".to_string(), self.sigma_min.clone().into());
        output.insert("rho".to_string(), self.rho.clone().into());
        output
    }
    const NAME: &'static str = "PolyexponentialScheduler";
    const DISPLAY_NAME: &'static str = "PolyexponentialScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**SDTurboScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SDTurboScheduler<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: DenoiseParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> SDTurboScheduler<ModelParam, StepsParam, DenoiseParam> {
    /// Create a new node.
    pub fn new(model: ModelParam, steps: StepsParam, denoise: DenoiseParam) -> Self {
        Self { model, steps, denoise }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    StepsParam: crate::nodes::types::Int,
    DenoiseParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for SDTurboScheduler<ModelParam, StepsParam, DenoiseParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("denoise".to_string(), self.denoise.clone().into());
        output
    }
    const NAME: &'static str = "SDTurboScheduler";
    const DISPLAY_NAME: &'static str = "SDTurboScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
///**VPScheduler**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VPScheduler<
    StepsParam: crate::nodes::types::Int,
    BetaDParam: crate::nodes::types::Float,
    BetaMinParam: crate::nodes::types::Float,
    EpsSParam: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: StepsParam,
    /**No documentation.

**Metadata**:
  - Default: 19.9
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta_d: BetaDParam,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta_min: BetaMinParam,
    /**No documentation.

**Metadata**:
  - Default: 0.001
  - Max: 1
  - Min: 0
  - Round: false
  - Step: 0.0001
*/
    pub eps_s: EpsSParam,
}
impl<
    StepsParam: crate::nodes::types::Int,
    BetaDParam: crate::nodes::types::Float,
    BetaMinParam: crate::nodes::types::Float,
    EpsSParam: crate::nodes::types::Float,
> VPScheduler<StepsParam, BetaDParam, BetaMinParam, EpsSParam> {
    /// Create a new node.
    pub fn new(
        steps: StepsParam,
        beta_d: BetaDParam,
        beta_min: BetaMinParam,
        eps_s: EpsSParam,
    ) -> Self {
        Self {
            steps,
            beta_d,
            beta_min,
            eps_s,
        }
    }
}
impl<
    StepsParam: crate::nodes::types::Int,
    BetaDParam: crate::nodes::types::Float,
    BetaMinParam: crate::nodes::types::Float,
    EpsSParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for VPScheduler<StepsParam, BetaDParam, BetaMinParam, EpsSParam> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("beta_d".to_string(), self.beta_d.clone().into());
        output.insert("beta_min".to_string(), self.beta_min.clone().into());
        output.insert("eps_s".to_string(), self.eps_s.clone().into());
        output
    }
    const NAME: &'static str = "VPScheduler";
    const DISPLAY_NAME: &'static str = "VPScheduler";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "sampling/custom_sampling/schedulers";
}
