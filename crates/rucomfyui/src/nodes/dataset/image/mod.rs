//!`image` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`ShuffleImageTextDataset`](super::ShuffleImageTextDataset).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ShuffleImageTextDatasetOutput {
        ///Shuffled images
        pub images: crate::nodes::types::ImageOut,
        ///Shuffled texts
        pub texts: crate::nodes::types::StringOut,
    }
}
///**Adjust Brightness**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AdjustBrightness<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Brightness factor. 1.0 = no change, <1.0 = darker, >1.0 = brighter.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: 0
*/
    pub factor: FactorParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> AdjustBrightness<ImagesParam, FactorParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, factor: FactorParam) -> Self {
        Self { images, factor }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for AdjustBrightness<ImagesParam, FactorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("factor".to_string(), self.factor.clone().into());
        output
    }
    const NAME: &'static str = "AdjustBrightness";
    const DISPLAY_NAME: &'static str = "Adjust Brightness";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Adjust Contrast**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AdjustContrast<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Contrast factor. 1.0 = no change, <1.0 = less contrast, >1.0 = more contrast.

**Metadata**:
  - Default: 1
  - Max: 2
  - Min: 0
*/
    pub factor: FactorParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> AdjustContrast<ImagesParam, FactorParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, factor: FactorParam) -> Self {
        Self { images, factor }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    FactorParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for AdjustContrast<ImagesParam, FactorParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("factor".to_string(), self.factor.clone().into());
        output
    }
    const NAME: &'static str = "AdjustContrast";
    const DISPLAY_NAME: &'static str = "Adjust Contrast";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Center Crop Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CenterCropImages<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Crop width.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub width: WidthParam,
    /**Crop height.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub height: HeightParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> CenterCropImages<ImagesParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, width: WidthParam, height: HeightParam) -> Self {
        Self { images, width, height }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for CenterCropImages<ImagesParam, WidthParam, HeightParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "CenterCropImages";
    const DISPLAY_NAME: &'static str = "Center Crop Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Image Deduplication**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageDeduplication<
    ImagesParam: crate::nodes::types::Image,
    SimilarityThresholdParam: crate::nodes::types::Float,
> {
    ///List of images to process.
    pub images: ImagesParam,
    /**Similarity threshold (0-1). Higher means more similar. Images above this threshold are considered duplicates.

**Metadata**:
  - Default: 0.95
  - Max: 1
  - Min: 0
*/
    pub similarity_threshold: SimilarityThresholdParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SimilarityThresholdParam: crate::nodes::types::Float,
> ImageDeduplication<ImagesParam, SimilarityThresholdParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        similarity_threshold: SimilarityThresholdParam,
    ) -> Self {
        Self {
            images,
            similarity_threshold,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SimilarityThresholdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageDeduplication<ImagesParam, SimilarityThresholdParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
            .insert(
                "similarity_threshold".to_string(),
                self.similarity_threshold.clone().into(),
            );
        output
    }
    const NAME: &'static str = "ImageDeduplication";
    const DISPLAY_NAME: &'static str = "Image Deduplication";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Image Grid**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageGrid<
    ImagesParam: crate::nodes::types::Image,
    ColumnsParam: crate::nodes::types::Int,
    CellWidthParam: crate::nodes::types::Int,
    CellHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> {
    ///List of images to process.
    pub images: ImagesParam,
    /**Number of columns in the grid.

**Metadata**:
  - Default: 4
  - Max: 20
  - Min: 1
*/
    pub columns: ColumnsParam,
    /**Width of each cell in the grid.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 32
*/
    pub cell_width: CellWidthParam,
    /**Height of each cell in the grid.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 32
*/
    pub cell_height: CellHeightParam,
    /**Padding between images.

**Metadata**:
  - Default: 4
  - Max: 50
  - Min: 0
*/
    pub padding: PaddingParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ColumnsParam: crate::nodes::types::Int,
    CellWidthParam: crate::nodes::types::Int,
    CellHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> ImageGrid<ImagesParam, ColumnsParam, CellWidthParam, CellHeightParam, PaddingParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        columns: ColumnsParam,
        cell_width: CellWidthParam,
        cell_height: CellHeightParam,
        padding: PaddingParam,
    ) -> Self {
        Self {
            images,
            columns,
            cell_width,
            cell_height,
            padding,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ColumnsParam: crate::nodes::types::Int,
    CellWidthParam: crate::nodes::types::Int,
    CellHeightParam: crate::nodes::types::Int,
    PaddingParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImageGrid<ImagesParam, ColumnsParam, CellWidthParam, CellHeightParam, PaddingParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("columns".to_string(), self.columns.clone().into());
        output.insert("cell_width".to_string(), self.cell_width.clone().into());
        output.insert("cell_height".to_string(), self.cell_height.clone().into());
        output.insert("padding".to_string(), self.padding.clone().into());
        output
    }
    const NAME: &'static str = "ImageGrid";
    const DISPLAY_NAME: &'static str = "Image Grid";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Merge Image Lists**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MergeImageLists<ImagesParam: crate::nodes::types::Image> {
    ///List of images to process.
    pub images: ImagesParam,
}
impl<ImagesParam: crate::nodes::types::Image> MergeImageLists<ImagesParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam) -> Self {
        Self { images }
    }
}
impl<ImagesParam: crate::nodes::types::Image> crate::nodes::TypedNode
for MergeImageLists<ImagesParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output
    }
    const NAME: &'static str = "MergeImageLists";
    const DISPLAY_NAME: &'static str = "Merge Image Lists";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Normalize Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NormalizeImages<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Mean value for normalization.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
*/
    pub mean: MeanParam,
    /**Standard deviation for normalization.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0.001
*/
    pub std: StdParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> NormalizeImages<ImagesParam, MeanParam, StdParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, mean: MeanParam, std: StdParam) -> Self {
        Self { images, mean, std }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    MeanParam: crate::nodes::types::Float,
    StdParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for NormalizeImages<ImagesParam, MeanParam, StdParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("mean".to_string(), self.mean.clone().into());
        output.insert("std".to_string(), self.std.clone().into());
        output
    }
    const NAME: &'static str = "NormalizeImages";
    const DISPLAY_NAME: &'static str = "Normalize Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Random Crop Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RandomCropImages<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Crop width.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub width: WidthParam,
    /**Crop height.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub height: HeightParam,
    /**Random seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> RandomCropImages<ImagesParam, WidthParam, HeightParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        width: WidthParam,
        height: HeightParam,
        seed: SeedParam,
    ) -> Self {
        Self {
            images,
            width,
            height,
            seed,
        }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RandomCropImages<ImagesParam, WidthParam, HeightParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "RandomCropImages";
    const DISPLAY_NAME: &'static str = "Random Crop Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Resize Images by Longer Edge**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeImagesByLongerEdge<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Target length for the longer edge.

**Metadata**:
  - Default: 1024
  - Max: 8192
  - Min: 1
*/
    pub longer_edge: LongerEdgeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> ResizeImagesByLongerEdge<ImagesParam, LongerEdgeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, longer_edge: LongerEdgeParam) -> Self {
        Self { images, longer_edge }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    LongerEdgeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ResizeImagesByLongerEdge<ImagesParam, LongerEdgeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("longer_edge".to_string(), self.longer_edge.clone().into());
        output
    }
    const NAME: &'static str = "ResizeImagesByLongerEdge";
    const DISPLAY_NAME: &'static str = "Resize Images by Longer Edge";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Resize Images by Shorter Edge**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResizeImagesByShorterEdge<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> {
    ///Image to process.
    pub images: ImagesParam,
    /**Target length for the shorter edge.

**Metadata**:
  - Default: 512
  - Max: 8192
  - Min: 1
*/
    pub shorter_edge: ShorterEdgeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> ResizeImagesByShorterEdge<ImagesParam, ShorterEdgeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, shorter_edge: ShorterEdgeParam) -> Self {
        Self { images, shorter_edge }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    ShorterEdgeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ResizeImagesByShorterEdge<ImagesParam, ShorterEdgeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("shorter_edge".to_string(), self.shorter_edge.clone().into());
        output
    }
    const NAME: &'static str = "ResizeImagesByShorterEdge";
    const DISPLAY_NAME: &'static str = "Resize Images by Shorter Edge";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Shuffle Image Dataset**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ShuffleDataset<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> {
    ///List of images to process.
    pub images: ImagesParam,
    /**Random seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> ShuffleDataset<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: SeedParam) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ShuffleDataset<ImagesParam, SeedParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ShuffleDataset";
    const DISPLAY_NAME: &'static str = "Shuffle Image Dataset";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
///**Shuffle Image-Text Dataset**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ShuffleImageTextDataset<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> {
    ///List of images to shuffle.
    pub images: ImagesParam,
    /**List of texts to shuffle.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Random seed.

**Metadata**:
  - Default: 0
  - Max: 18446744073709551615
  - Min: 0
*/
    pub seed: SeedParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> ShuffleImageTextDataset<ImagesParam, TextsParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, texts: TextsParam, seed: SeedParam) -> Self {
        Self { images, texts, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TextsParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ShuffleImageTextDataset<ImagesParam, TextsParam, SeedParam> {
    type Output = out::ShuffleImageTextDatasetOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            images: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            texts: crate::nodes::types::StringOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output
    }
    const NAME: &'static str = "ShuffleImageTextDataset";
    const DISPLAY_NAME: &'static str = "Shuffle Image-Text Dataset";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/image";
}
