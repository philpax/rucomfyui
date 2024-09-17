//!model_merging
pub mod model_specific;
///**CLIPMergeAdd**
pub struct ClipMergeAdd<Clip1: crate::nodes::Clip, Clip2: crate::nodes::Clip> {
    ///No documentation.
    pub clip1: Clip1,
    ///No documentation.
    pub clip2: Clip2,
}
///**CLIPMergeSimple**
pub struct ClipMergeSimple<
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Ratio: crate::nodes::Float,
> {
    ///No documentation.
    pub clip1: Clip1,
    ///No documentation.
    pub clip2: Clip2,
    ///No documentation.
    pub ratio: Ratio,
}
///**CLIPMergeSubtract**
pub struct ClipMergeSubtract<
    Clip1: crate::nodes::Clip,
    Clip2: crate::nodes::Clip,
    Multiplier: crate::nodes::Float,
> {
    ///No documentation.
    pub clip1: Clip1,
    ///No documentation.
    pub clip2: Clip2,
    ///No documentation.
    pub multiplier: Multiplier,
}
///**CLIPSave**
pub struct ClipSave<Clip: crate::nodes::Clip, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///**Save Checkpoint**
pub struct CheckpointSave<
    Model: crate::nodes::Model,
    Clip: crate::nodes::Clip,
    Vae: crate::nodes::Vae,
    FilenamePrefix: crate::nodes::String,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///**ModelMergeAdd**
pub struct ModelMergeAdd<Model1: crate::nodes::Model, Model2: crate::nodes::Model> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
}
///**ModelMergeBlocks**
pub struct ModelMergeBlocks<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Input: crate::nodes::Float,
    Middle: crate::nodes::Float,
    Out: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub input: Input,
    ///No documentation.
    pub middle: Middle,
    ///No documentation.
    pub out: Out,
}
///**ModelMergeSimple**
pub struct ModelMergeSimple<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Ratio: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub ratio: Ratio,
}
///**ModelMergeSubtract**
pub struct ModelMergeSubtract<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    Multiplier: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub multiplier: Multiplier,
}
///**ModelSave**
pub struct ModelSave<Model: crate::nodes::Model, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
///**VAESave**
pub struct VaeSave<Vae: crate::nodes::Vae, FilenamePrefix: crate::nodes::String> {
    ///No documentation.
    pub vae: Vae,
    ///No documentation.
    pub filename_prefix: FilenamePrefix,
}
