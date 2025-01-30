//!`schedulers` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**AlignYourStepsScheduler**: No description.
#[derive(Clone)]
pub struct AlignYourStepsScheduler<
    ModelType: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_type: ModelType,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: Denoise,
}
impl<
    ModelType: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> AlignYourStepsScheduler<ModelType, Steps, Denoise> {
    /// Create a new node.
    pub fn new(model_type: ModelType, steps: Steps, denoise: Denoise) -> Self {
        Self { model_type, steps, denoise }
    }
}
impl<
    ModelType: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for AlignYourStepsScheduler<ModelType, Steps, Denoise> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model_type".to_string(), self.model_type.clone().into());
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
pub struct BasicScheduler<
    Model: crate::nodes::types::Model,
    Scheduler: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub scheduler: Scheduler,
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: Denoise,
}
impl<
    Model: crate::nodes::types::Model,
    Scheduler: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> BasicScheduler<Model, Scheduler, Steps, Denoise> {
    /// Create a new node.
    pub fn new(
        model: Model,
        scheduler: Scheduler,
        steps: Steps,
        denoise: Denoise,
    ) -> Self {
        Self {
            model,
            scheduler,
            steps,
            denoise,
        }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Scheduler: crate::nodes::types::String,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for BasicScheduler<Model, Scheduler, Steps, Denoise> {
    type Output = crate::nodes::types::SigmasOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("scheduler".to_string(), self.scheduler.clone().into());
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
pub struct BetaSamplingScheduler<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Alpha: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 50
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub alpha: Alpha,
    /**No documentation.

**Metadata**:
  - Default: 0.6
  - Max: 50
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta: Beta,
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Alpha: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> BetaSamplingScheduler<Model, Steps, Alpha, Beta> {
    /// Create a new node.
    pub fn new(model: Model, steps: Steps, alpha: Alpha, beta: Beta) -> Self {
        Self { model, steps, alpha, beta }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Alpha: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> crate::nodes::TypedNode for BetaSamplingScheduler<Model, Steps, Alpha, Beta> {
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
pub struct ExponentialScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMin,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> ExponentialScheduler<Steps, SigmaMax, SigmaMin> {
    /// Create a new node.
    pub fn new(steps: Steps, sigma_max: SigmaMax, sigma_min: SigmaMin) -> Self {
        Self {
            steps,
            sigma_max,
            sigma_min,
        }
    }
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
> crate::nodes::TypedNode for ExponentialScheduler<Steps, SigmaMax, SigmaMin> {
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
///**GITSScheduler**: No description.
#[derive(Clone)]
pub struct GitsScheduler<
    Coeff: crate::nodes::types::Float,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 1.2
  - Max: 1.5
  - Min: 0.8
  - Step: 0.05
*/
    pub coeff: Coeff,
    /**No documentation.

**Metadata**:
  - Default: 10
  - Max: 1000
  - Min: 2
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: Denoise,
}
impl<
    Coeff: crate::nodes::types::Float,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> GitsScheduler<Coeff, Steps, Denoise> {
    /// Create a new node.
    pub fn new(coeff: Coeff, steps: Steps, denoise: Denoise) -> Self {
        Self { coeff, steps, denoise }
    }
}
impl<
    Coeff: crate::nodes::types::Float,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for GitsScheduler<Coeff, Steps, Denoise> {
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
pub struct KarrasScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMin,
    /**No documentation.

**Metadata**:
  - Default: 7
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub rho: Rho,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> KarrasScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    /// Create a new node.
    pub fn new(
        steps: Steps,
        sigma_max: SigmaMax,
        sigma_min: SigmaMin,
        rho: Rho,
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
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> crate::nodes::TypedNode for KarrasScheduler<Steps, SigmaMax, SigmaMin, Rho> {
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
pub struct LtxvScheduler<
    Steps: crate::nodes::types::Int,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Stretch: crate::nodes::types::Boolean,
    Terminal: crate::nodes::types::Float,
    Latent: crate::nodes::types::Latent = crate::nodes::types::LatentOut,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 2.05
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub max_shift: MaxShift,
    /**No documentation.

**Metadata**:
  - Default: 0.95
  - Max: 100
  - Min: 0
  - Step: 0.01
*/
    pub base_shift: BaseShift,
    /**Stretch the sigmas to be in the range [terminal, 1].

**Metadata**:
  - Default: true
*/
    pub stretch: Stretch,
    /**The terminal value of the sigmas after stretching.

**Metadata**:
  - Default: 0.1
  - Max: 0.99
  - Min: 0
  - Step: 0.01
*/
    pub terminal: Terminal,
    ///No documentation.
    pub latent: Option<Latent>,
}
impl<
    Steps: crate::nodes::types::Int,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Stretch: crate::nodes::types::Boolean,
    Terminal: crate::nodes::types::Float,
    Latent: crate::nodes::types::Latent,
> LtxvScheduler<Steps, MaxShift, BaseShift, Stretch, Terminal, Latent> {
    /// Create a new node.
    pub fn new(
        steps: Steps,
        max_shift: MaxShift,
        base_shift: BaseShift,
        stretch: Stretch,
        terminal: Terminal,
        latent: Option<Latent>,
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
    Steps: crate::nodes::types::Int,
    MaxShift: crate::nodes::types::Float,
    BaseShift: crate::nodes::types::Float,
    Stretch: crate::nodes::types::Boolean,
    Terminal: crate::nodes::types::Float,
    Latent: crate::nodes::types::Latent,
> crate::nodes::TypedNode
for LtxvScheduler<Steps, MaxShift, BaseShift, Stretch, Terminal, Latent> {
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
pub struct LaplaceScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Mu: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMin,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 10
  - Min: -10
  - Round: false
  - Step: 0.1
*/
    pub mu: Mu,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 10
  - Min: 0
  - Round: false
  - Step: 0.1
*/
    pub beta: Beta,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Mu: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> LaplaceScheduler<Steps, SigmaMax, SigmaMin, Mu, Beta> {
    /// Create a new node.
    pub fn new(
        steps: Steps,
        sigma_max: SigmaMax,
        sigma_min: SigmaMin,
        mu: Mu,
        beta: Beta,
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
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Mu: crate::nodes::types::Float,
    Beta: crate::nodes::types::Float,
> crate::nodes::TypedNode for LaplaceScheduler<Steps, SigmaMax, SigmaMin, Mu, Beta> {
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
///**PolyexponentialScheduler**: No description.
#[derive(Clone)]
pub struct PolyexponentialScheduler<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 14.614642
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_max: SigmaMax,
    /**No documentation.

**Metadata**:
  - Default: 0.0291675
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub sigma_min: SigmaMin,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub rho: Rho,
}
impl<
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> PolyexponentialScheduler<Steps, SigmaMax, SigmaMin, Rho> {
    /// Create a new node.
    pub fn new(
        steps: Steps,
        sigma_max: SigmaMax,
        sigma_min: SigmaMin,
        rho: Rho,
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
    Steps: crate::nodes::types::Int,
    SigmaMax: crate::nodes::types::Float,
    SigmaMin: crate::nodes::types::Float,
    Rho: crate::nodes::types::Float,
> crate::nodes::TypedNode for PolyexponentialScheduler<Steps, SigmaMax, SigmaMin, Rho> {
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
pub struct SdTurboScheduler<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: Model,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub denoise: Denoise,
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> SdTurboScheduler<Model, Steps, Denoise> {
    /// Create a new node.
    pub fn new(model: Model, steps: Steps, denoise: Denoise) -> Self {
        Self { model, steps, denoise }
    }
}
impl<
    Model: crate::nodes::types::Model,
    Steps: crate::nodes::types::Int,
    Denoise: crate::nodes::types::Float,
> crate::nodes::TypedNode for SdTurboScheduler<Model, Steps, Denoise> {
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
pub struct VpScheduler<
    Steps: crate::nodes::types::Int,
    BetaD: crate::nodes::types::Float,
    BetaMin: crate::nodes::types::Float,
    EpsS: crate::nodes::types::Float,
> {
    /**No documentation.

**Metadata**:
  - Default: 20
  - Max: 10000
  - Min: 1
*/
    pub steps: Steps,
    /**No documentation.

**Metadata**:
  - Default: 19.9
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta_d: BetaD,
    /**No documentation.

**Metadata**:
  - Default: 0.1
  - Max: 5000
  - Min: 0
  - Round: false
  - Step: 0.01
*/
    pub beta_min: BetaMin,
    /**No documentation.

**Metadata**:
  - Default: 0.001
  - Max: 1
  - Min: 0
  - Round: false
  - Step: 0.0001
*/
    pub eps_s: EpsS,
}
impl<
    Steps: crate::nodes::types::Int,
    BetaD: crate::nodes::types::Float,
    BetaMin: crate::nodes::types::Float,
    EpsS: crate::nodes::types::Float,
> VpScheduler<Steps, BetaD, BetaMin, EpsS> {
    /// Create a new node.
    pub fn new(steps: Steps, beta_d: BetaD, beta_min: BetaMin, eps_s: EpsS) -> Self {
        Self {
            steps,
            beta_d,
            beta_min,
            eps_s,
        }
    }
}
impl<
    Steps: crate::nodes::types::Int,
    BetaD: crate::nodes::types::Float,
    BetaMin: crate::nodes::types::Float,
    EpsS: crate::nodes::types::Float,
> crate::nodes::TypedNode for VpScheduler<Steps, BetaD, BetaMin, EpsS> {
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
