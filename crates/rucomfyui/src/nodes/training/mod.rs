//!`training` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`TrainLoraNode`](super::TrainLoraNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TrainLoraNodeOutput {
        ///No documentation.
        pub model_with_lora: crate::nodes::types::ModelOut,
        ///No documentation.
        pub lora: crate::nodes::types::LoraModelOut,
        ///No documentation.
        pub loss: crate::nodes::types::LossMapOut,
        ///No documentation.
        pub steps: crate::nodes::types::IntOut,
    }
}
///**Plot Loss Graph**: Plots the loss graph and saves it to the output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LossGraphNode<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub loss: LossParam,
    /**No documentation.

**Metadata**:
  - Default: loss_graph
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> LossGraphNode<LossParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(loss: LossParam, filename_prefix: FilenamePrefixParam) -> Self {
        Self { loss, filename_prefix }
    }
}
impl<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LossGraphNode<LossParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("loss".to_string(), self.loss.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "LossGraphNode";
    const DISPLAY_NAME: &'static str = "Plot Loss Graph";
    const DESCRIPTION: &'static str = "Plots the loss graph and saves it to the output directory.";
    const CATEGORY: &'static str = "training";
}
impl<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for LossGraphNode<LossParam, FilenamePrefixParam> {}
///**Train LoRA**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TrainLoraNode<
    ModelParam: crate::nodes::types::Model,
    LatentsParam: crate::nodes::types::Latent,
    PositiveParam: crate::nodes::types::Conditioning,
    BatchSizeParam: crate::nodes::types::Int,
    GradAccumulationStepsParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    LearningRateParam: crate::nodes::types::Float,
    RankParam: crate::nodes::types::Int,
    OptimizerParam: crate::nodes::types::String,
    LossFunctionParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    TrainingDtypeParam: crate::nodes::types::String,
    LoraDtypeParam: crate::nodes::types::String,
    AlgorithmParam: crate::nodes::types::String,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    ExistingLoraParam: crate::nodes::types::String,
> {
    ///The model to train the LoRA on.
    pub model: ModelParam,
    ///The Latents to use for training, serve as dataset/input of the model.
    pub latents: LatentsParam,
    ///The positive conditioning to use for training.
    pub positive: PositiveParam,
    /**The batch size to use for training.

**Metadata**:
  - Default: 1
  - Max: 10000
  - Min: 1
  - Step: 1
*/
    pub batch_size: BatchSizeParam,
    /**The number of gradient accumulation steps to use for training.

**Metadata**:
  - Default: 1
  - Max: 1024
  - Min: 1
  - Step: 1
*/
    pub grad_accumulation_steps: GradAccumulationStepsParam,
    /**The number of steps to train the LoRA for.

**Metadata**:
  - Default: 16
  - Max: 100000
  - Min: 1
*/
    pub steps: StepsParam,
    /**The learning rate to use for training.

**Metadata**:
  - Default: 0.0005
  - Max: 1
  - Min: 0.0000001
  - Step: 0.000001
*/
    pub learning_rate: LearningRateParam,
    /**The rank of the LoRA layers.

**Metadata**:
  - Default: 8
  - Max: 128
  - Min: 1
*/
    pub rank: RankParam,
    /**The optimizer to use for training.

**Metadata**:
  - Default: AdamW
*/
    pub optimizer: OptimizerParam,
    /**The loss function to use for training.

**Metadata**:
  - Default: MSE
*/
    pub loss_function: LossFunctionParam,
    /**The seed to use for training (used in generator for LoRA weight initialization and noise sampling)

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**The dtype to use for training.

**Metadata**:
  - Default: bf16
*/
    pub training_dtype: TrainingDtypeParam,
    /**The dtype to use for lora.

**Metadata**:
  - Default: bf16
*/
    pub lora_dtype: LoraDtypeParam,
    /**The algorithm to use for training.

**Metadata**:
  - Default: LoRA
*/
    pub algorithm: AlgorithmParam,
    /**Use gradient checkpointing for training.

**Metadata**:
  - Default: true
*/
    pub gradient_checkpointing: GradientCheckpointingParam,
    /**The existing LoRA to append to. Set to None for new LoRA.

**Metadata**:
  - Default: [None]
*/
    pub existing_lora: ExistingLoraParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    LatentsParam: crate::nodes::types::Latent,
    PositiveParam: crate::nodes::types::Conditioning,
    BatchSizeParam: crate::nodes::types::Int,
    GradAccumulationStepsParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    LearningRateParam: crate::nodes::types::Float,
    RankParam: crate::nodes::types::Int,
    OptimizerParam: crate::nodes::types::String,
    LossFunctionParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    TrainingDtypeParam: crate::nodes::types::String,
    LoraDtypeParam: crate::nodes::types::String,
    AlgorithmParam: crate::nodes::types::String,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    ExistingLoraParam: crate::nodes::types::String,
> TrainLoraNode<
    ModelParam,
    LatentsParam,
    PositiveParam,
    BatchSizeParam,
    GradAccumulationStepsParam,
    StepsParam,
    LearningRateParam,
    RankParam,
    OptimizerParam,
    LossFunctionParam,
    SeedParam,
    TrainingDtypeParam,
    LoraDtypeParam,
    AlgorithmParam,
    GradientCheckpointingParam,
    ExistingLoraParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        latents: LatentsParam,
        positive: PositiveParam,
        batch_size: BatchSizeParam,
        grad_accumulation_steps: GradAccumulationStepsParam,
        steps: StepsParam,
        learning_rate: LearningRateParam,
        rank: RankParam,
        optimizer: OptimizerParam,
        loss_function: LossFunctionParam,
        seed: SeedParam,
        training_dtype: TrainingDtypeParam,
        lora_dtype: LoraDtypeParam,
        algorithm: AlgorithmParam,
        gradient_checkpointing: GradientCheckpointingParam,
        existing_lora: ExistingLoraParam,
    ) -> Self {
        Self {
            model,
            latents,
            positive,
            batch_size,
            grad_accumulation_steps,
            steps,
            learning_rate,
            rank,
            optimizer,
            loss_function,
            seed,
            training_dtype,
            lora_dtype,
            algorithm,
            gradient_checkpointing,
            existing_lora,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    LatentsParam: crate::nodes::types::Latent,
    PositiveParam: crate::nodes::types::Conditioning,
    BatchSizeParam: crate::nodes::types::Int,
    GradAccumulationStepsParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
    LearningRateParam: crate::nodes::types::Float,
    RankParam: crate::nodes::types::Int,
    OptimizerParam: crate::nodes::types::String,
    LossFunctionParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    TrainingDtypeParam: crate::nodes::types::String,
    LoraDtypeParam: crate::nodes::types::String,
    AlgorithmParam: crate::nodes::types::String,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    ExistingLoraParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for TrainLoraNode<
    ModelParam,
    LatentsParam,
    PositiveParam,
    BatchSizeParam,
    GradAccumulationStepsParam,
    StepsParam,
    LearningRateParam,
    RankParam,
    OptimizerParam,
    LossFunctionParam,
    SeedParam,
    TrainingDtypeParam,
    LoraDtypeParam,
    AlgorithmParam,
    GradientCheckpointingParam,
    ExistingLoraParam,
> {
    type Output = out::TrainLoraNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            model_with_lora: crate::nodes::types::ModelOut::from_dynamic(node_id, 0u32),
            lora: crate::nodes::types::LoraModelOut::from_dynamic(node_id, 1u32),
            loss: crate::nodes::types::LossMapOut::from_dynamic(node_id, 2u32),
            steps: crate::nodes::types::IntOut::from_dynamic(node_id, 3u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("latents".to_string(), self.latents.clone().into());
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
            .insert(
                "grad_accumulation_steps".to_string(),
                self.grad_accumulation_steps.clone().into(),
            );
        output.insert("steps".to_string(), self.steps.clone().into());
        output.insert("learning_rate".to_string(), self.learning_rate.clone().into());
        output.insert("rank".to_string(), self.rank.clone().into());
        output.insert("optimizer".to_string(), self.optimizer.clone().into());
        output.insert("loss_function".to_string(), self.loss_function.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("training_dtype".to_string(), self.training_dtype.clone().into());
        output.insert("lora_dtype".to_string(), self.lora_dtype.clone().into());
        output.insert("algorithm".to_string(), self.algorithm.clone().into());
        output
            .insert(
                "gradient_checkpointing".to_string(),
                self.gradient_checkpointing.clone().into(),
            );
        output.insert("existing_lora".to_string(), self.existing_lora.clone().into());
        output
    }
    const NAME: &'static str = "TrainLoraNode";
    const DISPLAY_NAME: &'static str = "Train LoRA";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "training";
}
