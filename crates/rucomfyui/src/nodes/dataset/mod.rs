//!`dataset` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod image;
#[rustfmt::skip]
pub mod text;
/// Output types for nodes.
pub mod out {
    ///Output for [`LoadImageTextDataSetFromFolder`](super::LoadImageTextDataSetFromFolder).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct LoadImageTextDataSetFromFolderOutput {
        ///List of loaded images
        pub images: crate::nodes::types::ImageOut,
        ///List of text captions
        pub texts: crate::nodes::types::StringOut,
    }
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
}
///**Load Image Dataset from Folder**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageDataSetFromFolder {}
impl LoadImageDataSetFromFolder {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadImageDataSetFromFolder {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadImageDataSetFromFolder";
    const DISPLAY_NAME: &'static str = "Load Image Dataset from Folder";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
///**Load Image and Text Dataset from Folder**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LoadImageTextDataSetFromFolder {}
impl LoadImageTextDataSetFromFolder {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for LoadImageTextDataSetFromFolder {
    type Output = out::LoadImageTextDataSetFromFolderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            images: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            texts: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "LoadImageTextDataSetFromFolder";
    const DISPLAY_NAME: &'static str = "Load Image and Text Dataset from Folder";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
///**Load Training Dataset**: No description.
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
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
///**Make Training Dataset**: No description.
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
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
///**Save Image Dataset to Folder**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImageDataSetToFolder<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///List of images to save.
    pub images: ImagesParam,
    /**Name of the folder to save images to (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: dataset
*/
    pub folder_name: FolderNameParam,
    /**Prefix for saved image filenames.

**Metadata**:
  - Multiline: false
  - Default: image
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        folder_name: FolderNameParam,
        filename_prefix: FilenamePrefixParam,
    ) -> Self {
        Self {
            images,
            folder_name,
            filename_prefix,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveImageDataSetToFolder";
    const DISPLAY_NAME: &'static str = "Save Image Dataset to Folder";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveImageDataSetToFolder<ImagesParam, FolderNameParam, FilenamePrefixParam> {}
///**Save Image and Text Dataset to Folder**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SaveImageTextDataSetToFolder<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> {
    ///List of images to save.
    pub images: ImagesParam,
    /**List of text captions to save.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Name of the folder to save images to (inside output directory).

**Metadata**:
  - Multiline: false
  - Default: dataset
*/
    pub folder_name: FolderNameParam,
    /**Prefix for saved image filenames.

**Metadata**:
  - Multiline: false
  - Default: image
*/
    pub filename_prefix: FilenamePrefixParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> SaveImageTextDataSetToFolder<
    ImagesParam,
    TextsParam,
    FolderNameParam,
    FilenamePrefixParam,
> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        texts: TextsParam,
        folder_name: FolderNameParam,
        filename_prefix: FilenamePrefixParam,
    ) -> Self {
        Self {
            images,
            texts,
            folder_name,
            filename_prefix,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for SaveImageTextDataSetToFolder<
    ImagesParam,
    TextsParam,
    FolderNameParam,
    FilenamePrefixParam,
> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("folder_name".to_string(), self.folder_name.clone().into());
        output
            .insert("filename_prefix".to_string(), self.filename_prefix.clone().into());
        output
    }
    const NAME: &'static str = "SaveImageTextDataSetToFolder";
    const DISPLAY_NAME: &'static str = "Save Image and Text Dataset to Folder";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    FolderNameParam: crate::nodes::types::String,
    FilenamePrefixParam: crate::nodes::types::String,
> crate::nodes::TypedOutputNode
for SaveImageTextDataSetToFolder<
    ImagesParam,
    TextsParam,
    FolderNameParam,
    FilenamePrefixParam,
> {}
///**Save Training Dataset**: No description.
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
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset";
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
