//!`training` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`LoadTrainingDataset`](super::LoadTrainingDataset).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoadTrainingDatasetOutput {
        ///List of latent dicts
        pub latents: crate::nodes::types::LatentOut,
        ///List of conditioning lists
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`MakeTrainingDataset`](super::MakeTrainingDataset).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct MakeTrainingDatasetOutput {
        ///List of latent dicts
        pub latents: crate::nodes::types::LatentOut,
        ///List of conditioning lists
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ResolutionBucket`](super::ResolutionBucket).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ResolutionBucketOutput {
        ///List of batched latent dicts, one per resolution bucket.
        pub latents: crate::nodes::types::LatentOut,
        ///List of condition lists, one per resolution bucket.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`TrainLoraNode`](super::TrainLoraNode).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct TrainLoraNodeOutput {
        ///LoRA weights
        pub lora: crate::nodes::types::LoraModelOut,
        ///Loss history
        pub loss_map: crate::nodes::types::LossMapOut,
        ///Total training steps
        pub steps: crate::nodes::types::IntOut,
    }
}
///**Load Training Dataset**: Load encoded training dataset (latents + conditioning) from disk for use in training.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadTrainingDataset<FolderNameParam: crate::nodes::types::String> {
    /**Name of folder containing the saved dataset (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: training_dataset
*/
    pub folder_name: FolderNameParam,
}
impl<FolderNameParam: crate::nodes::types::String> LoadTrainingDataset<FolderNameParam> {
    /// Create a new node.
    pub fn new(folder_name: FolderNameParam) -> Self {
        Self { folder_name }
    }
}
impl<FolderNameParam: crate::nodes::types::String> crate::nodes::TypedNode
for LoadTrainingDataset<FolderNameParam> {
    type Output = out::LoadTrainingDatasetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latents: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            conditioning: crate::nodes::types::ConditioningOut::from_dynamic(
                node_id,
                1u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output
    }
    const NAME: &'static str = "LoadTrainingDataset";
    const DISPLAY_NAME: &'static str = "Load Training Dataset";
    const DESCRIPTION: &'static str = "Load encoded training dataset (latents + conditioning) from disk for use in training.";
    const CATEGORY: &'static str = "model/training";
}
///**Plot Loss Graph**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LossGraphNode<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///Loss map from training node.
    pub loss: LossParam,
    /**Prefix for the saved loss graph image.

**Metadata**:
  - Multiline: false
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
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/training";
}
impl<
    LossParam: crate::nodes::types::LossMap,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode for LossGraphNode<LossParam, FilenamePrefixParam> {}
///**Make Training Dataset**: Encode images with VAE and texts with CLIP to create a training dataset of latents and conditionings.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MakeTrainingDataset<
    ImagesParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    ClipParam: crate::nodes::types::Clip,
    TextsParam: crate::nodes::types::String = crate::nodes::types::StringOut,
> {
    ///List of images to encode.
    pub images: ImagesParam,
    ///VAE model for encoding images to latents.
    pub vae: VaeParam,
    ///CLIP model for encoding text to conditioning.
    pub clip: ClipParam,
    /**List of text captions. Can be length n (matching images), 1 (repeated for all), or omitted (uses empty string).

**Metadata**:
  - Multiline: false
*/
    pub texts: Option<TextsParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    ClipParam: crate::nodes::types::Clip,
    TextsParam: crate::nodes::types::String,
> MakeTrainingDataset<ImagesParam, VaeParam, ClipParam, TextsParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        vae: VaeParam,
        clip: ClipParam,
        texts: Option<TextsParam>,
    ) -> Self {
        Self { images, vae, clip, texts }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    VaeParam: crate::nodes::types::Vae,
    ClipParam: crate::nodes::types::Clip,
    TextsParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for MakeTrainingDataset<ImagesParam, VaeParam, ClipParam, TextsParam> {
    type Output = out::MakeTrainingDatasetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latents: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            conditioning: crate::nodes::types::ConditioningOut::from_dynamic(
                node_id,
                1u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("vae".to_string(), self.vae.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        if let Some(v) = &self.texts {
            output.insert("texts".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MakeTrainingDataset";
    const DISPLAY_NAME: &'static str = "Make Training Dataset";
    const DESCRIPTION: &'static str = "Encode images with VAE and texts with CLIP to create a training dataset of latents and conditionings.";
    const CATEGORY: &'static str = "model/training";
}
///**Resolution Bucket**: Group latents and conditionings into buckets
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResolutionBucket<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
> {
    ///List of latent dicts to bucket by resolution.
    pub latents: LatentsParam,
    ///List of conditioning lists (must match latents length).
    pub conditioning: ConditioningParam,
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
> ResolutionBucket<LatentsParam, ConditioningParam> {
    /// Create a new node.
    pub fn new(latents: LatentsParam, conditioning: ConditioningParam) -> Self {
        Self { latents, conditioning }
    }
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for ResolutionBucket<LatentsParam, ConditioningParam> {
    type Output = out::ResolutionBucketOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            latents: crate::nodes::types::LatentOut::from_dynamic(node_id, 0u32),
            conditioning: crate::nodes::types::ConditioningOut::from_dynamic(
                node_id,
                1u32,
            ),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("latents".to_string(), self.latents.clone().into());
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output
    }
    const NAME: &'static str = "ResolutionBucket";
    const DISPLAY_NAME: &'static str = "Resolution Bucket";
    const DESCRIPTION: &'static str = "Group latents and conditionings into buckets";
    const CATEGORY: &'static str = "model/training";
}
///**Save Training Dataset**: Save encoded training dataset (latents + conditioning) to disk for efficient loading during training.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveTrainingDataset<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
    FolderNameParam: crate::nodes::types::String,
    ShardSizeParam: crate::nodes::types::Int,
> {
    ///List of latent dicts from MakeTrainingDataset.
    pub latents: LatentsParam,
    ///List of conditioning lists from MakeTrainingDataset.
    pub conditioning: ConditioningParam,
    /**Name of folder to save dataset (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: training_dataset
*/
    pub folder_name: FolderNameParam,
    /**Number of samples per shard file.

**Metadata**:
  - Default: 1000
  - Max: 100000
  - Min: 1
*/
    pub shard_size: ShardSizeParam,
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
    FolderNameParam: crate::nodes::types::String,
    ShardSizeParam: crate::nodes::types::Int,
> SaveTrainingDataset<LatentsParam, ConditioningParam, FolderNameParam, ShardSizeParam> {
    /// Create a new node.
    pub fn new(
        latents: LatentsParam,
        conditioning: ConditioningParam,
        folder_name: FolderNameParam,
        shard_size: ShardSizeParam,
    ) -> Self {
        Self {
            latents,
            conditioning,
            folder_name,
            shard_size,
        }
    }
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
    FolderNameParam: crate::nodes::types::String,
    ShardSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SaveTrainingDataset<
    LatentsParam,
    ConditioningParam,
    FolderNameParam,
    ShardSizeParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("latents".to_string(), self.latents.clone().into());
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output.insert("shard_size".to_string(), self.shard_size.clone().into());
        output
    }
    const NAME: &'static str = "SaveTrainingDataset";
    const DISPLAY_NAME: &'static str = "Save Training Dataset";
    const DESCRIPTION: &'static str = "Save encoded training dataset (latents + conditioning) to disk for efficient loading during training.";
    const CATEGORY: &'static str = "model/training";
}
impl<
    LatentsParam: crate::nodes::types::Latent,
    ConditioningParam: crate::nodes::types::Conditioning,
    FolderNameParam: crate::nodes::types::String,
    ShardSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedOutputNode
for SaveTrainingDataset<
    LatentsParam,
    ConditioningParam,
    FolderNameParam,
    ShardSizeParam,
> {}
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
    SeedParam: crate::nodes::types::Int,
    QuantizedBackwardParam: crate::nodes::types::Boolean,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    CheckpointDepthParam: crate::nodes::types::Int,
    OffloadingParam: crate::nodes::types::Boolean,
    BucketModeParam: crate::nodes::types::Boolean,
    BypassModeParam: crate::nodes::types::Boolean,
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
*/
    pub batch_size: BatchSizeParam,
    /**The number of gradient accumulation steps to use for training.

**Metadata**:
  - Default: 1
  - Max: 1024
  - Min: 1
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
  - Step: 0.0000001
*/
    pub learning_rate: LearningRateParam,
    /**The rank of the LoRA layers.

**Metadata**:
  - Default: 8
  - Max: 128
  - Min: 1
*/
    pub rank: RankParam,
    /**The seed to use for training (used in generator for LoRA weight initialization and noise sampling)

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
    /**When using training_dtype 'none' and training on quantized model, doing backward with quantized matmul when enabled.

**Metadata**:
  - Default: false
*/
    pub quantized_backward: QuantizedBackwardParam,
    /**Use gradient checkpointing for training.

**Metadata**:
  - Default: true
*/
    pub gradient_checkpointing: GradientCheckpointingParam,
    /**Depth level for gradient checkpointing.

**Metadata**:
  - Default: 1
  - Max: 5
  - Min: 1
*/
    pub checkpoint_depth: CheckpointDepthParam,
    /**Offload model weights to CPU during training to save GPU memory.

**Metadata**:
  - Default: false
*/
    pub offloading: OffloadingParam,
    /**Enable resolution bucket mode. When enabled, expects pre-bucketed latents from ResolutionBucket node.

**Metadata**:
  - Default: false
*/
    pub bucket_mode: BucketModeParam,
    /**Enable bypass mode for training. When enabled, adapters are applied via forward hooks instead of weight modification. Useful for quantized models where weights cannot be directly modified.

**Metadata**:
  - Default: false
*/
    pub bypass_mode: BypassModeParam,
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
    SeedParam: crate::nodes::types::Int,
    QuantizedBackwardParam: crate::nodes::types::Boolean,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    CheckpointDepthParam: crate::nodes::types::Int,
    OffloadingParam: crate::nodes::types::Boolean,
    BucketModeParam: crate::nodes::types::Boolean,
    BypassModeParam: crate::nodes::types::Boolean,
> TrainLoraNode<
    ModelParam,
    LatentsParam,
    PositiveParam,
    BatchSizeParam,
    GradAccumulationStepsParam,
    StepsParam,
    LearningRateParam,
    RankParam,
    SeedParam,
    QuantizedBackwardParam,
    GradientCheckpointingParam,
    CheckpointDepthParam,
    OffloadingParam,
    BucketModeParam,
    BypassModeParam,
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
        seed: SeedParam,
        quantized_backward: QuantizedBackwardParam,
        gradient_checkpointing: GradientCheckpointingParam,
        checkpoint_depth: CheckpointDepthParam,
        offloading: OffloadingParam,
        bucket_mode: BucketModeParam,
        bypass_mode: BypassModeParam,
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
            seed,
            quantized_backward,
            gradient_checkpointing,
            checkpoint_depth,
            offloading,
            bucket_mode,
            bypass_mode,
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
    SeedParam: crate::nodes::types::Int,
    QuantizedBackwardParam: crate::nodes::types::Boolean,
    GradientCheckpointingParam: crate::nodes::types::Boolean,
    CheckpointDepthParam: crate::nodes::types::Int,
    OffloadingParam: crate::nodes::types::Boolean,
    BucketModeParam: crate::nodes::types::Boolean,
    BypassModeParam: crate::nodes::types::Boolean,
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
    SeedParam,
    QuantizedBackwardParam,
    GradientCheckpointingParam,
    CheckpointDepthParam,
    OffloadingParam,
    BucketModeParam,
    BypassModeParam,
> {
    type Output = out::TrainLoraNodeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            lora: crate::nodes::types::LoraModelOut::from_dynamic(node_id, 0u32),
            loss_map: crate::nodes::types::LossMapOut::from_dynamic(node_id, 1u32),
            steps: crate::nodes::types::IntOut::from_dynamic(node_id, 2u32),
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
        output.insert("seed".to_string(), self.seed.clone().into());
        output
            .insert(
                "quantized_backward".to_string(),
                self.quantized_backward.clone().into(),
            );
        output
            .insert(
                "gradient_checkpointing".to_string(),
                self.gradient_checkpointing.clone().into(),
            );
        output
            .insert(
                "checkpoint_depth".to_string(),
                self.checkpoint_depth.clone().into(),
            );
        output.insert("offloading".to_string(), self.offloading.clone().into());
        output.insert("bucket_mode".to_string(), self.bucket_mode.clone().into());
        output.insert("bypass_mode".to_string(), self.bypass_mode.clone().into());
        output
    }
    const NAME: &'static str = "TrainLoraNode";
    const DISPLAY_NAME: &'static str = "Train LoRA";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "model/training";
}
