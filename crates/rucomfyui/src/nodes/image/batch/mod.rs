//!`batch` definitions/categories.
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
///**Batch Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BatchImagesNode {}
impl BatchImagesNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for BatchImagesNode {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "BatchImagesNode";
    const DISPLAY_NAME: &'static str = "Batch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Batch Images (DEPRECATED)**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageBatch<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> {
    ///No documentation.
    pub image_1: Image1Param,
    ///No documentation.
    pub image_2: Image2Param,
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> ImageBatch<Image1Param, Image2Param> {
    /// Create a new node.
    pub fn new(image_1: Image1Param, image_2: Image2Param) -> Self {
        Self { image_1, image_2 }
    }
}
impl<
    Image1Param: crate::nodes::types::Image,
    Image2Param: crate::nodes::types::Image,
> crate::nodes::TypedNode for ImageBatch<Image1Param, Image2Param> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image1".to_string(), self.image_1.clone().into());
        output.insert("image2".to_string(), self.image_2.clone().into());
        output
    }
    const NAME: &'static str = "ImageBatch";
    const DISPLAY_NAME: &'static str = "Batch Images (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Deduplicate Images**: Remove duplicate or very similar images from a list.
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
    const DISPLAY_NAME: &'static str = "Deduplicate Images";
    const DESCRIPTION: &'static str = "Remove duplicate or very similar images from a list.";
    const CATEGORY: &'static str = "image/batch";
}
///**Get Image from Batch**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageFromBatch<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: -16384
*/
    pub batch_index: BatchIndexParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub length: LengthParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> ImageFromBatch<ImageParam, BatchIndexParam, LengthParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        batch_index: BatchIndexParam,
        length: LengthParam,
    ) -> Self {
        Self { image, batch_index, length }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    BatchIndexParam: crate::nodes::types::Int,
    LengthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageFromBatch<ImageParam, BatchIndexParam, LengthParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("batch_index".to_string(), self.batch_index.clone().into());
        output.insert("length".to_string(), self.length.clone().into());
        output
    }
    const NAME: &'static str = "ImageFromBatch";
    const DISPLAY_NAME: &'static str = "Get Image from Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Make Image Grid**: Arrange multiple images into a grid layout.
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
    const DISPLAY_NAME: &'static str = "Make Image Grid";
    const DESCRIPTION: &'static str = "Arrange multiple images into a grid layout.";
    const CATEGORY: &'static str = "image/batch";
}
///**Merge List of Tiles to Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageMergeTileList<
    ImageListParam: crate::nodes::types::Image,
    FinalWidthParam: crate::nodes::types::Int,
    FinalHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image_list: ImageListParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 32768
  - Min: 64
*/
    pub final_width: FinalWidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 32768
  - Min: 64
*/
    pub final_height: FinalHeightParam,
    /**No documentation.

**Metadata**:
  - Default: 128
  - Max: 4096
  - Min: 0
*/
    pub overlap: OverlapParam,
}
impl<
    ImageListParam: crate::nodes::types::Image,
    FinalWidthParam: crate::nodes::types::Int,
    FinalHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> ImageMergeTileList<ImageListParam, FinalWidthParam, FinalHeightParam, OverlapParam> {
    /// Create a new node.
    pub fn new(
        image_list: ImageListParam,
        final_width: FinalWidthParam,
        final_height: FinalHeightParam,
        overlap: OverlapParam,
    ) -> Self {
        Self {
            image_list,
            final_width,
            final_height,
            overlap,
        }
    }
}
impl<
    ImageListParam: crate::nodes::types::Image,
    FinalWidthParam: crate::nodes::types::Int,
    FinalHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for ImageMergeTileList<ImageListParam, FinalWidthParam, FinalHeightParam, OverlapParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_list".to_string(), self.image_list.clone().into());
        output.insert("final_width".to_string(), self.final_width.clone().into());
        output.insert("final_height".to_string(), self.final_height.clone().into());
        output.insert("overlap".to_string(), self.overlap.clone().into());
        output
    }
    const NAME: &'static str = "ImageMergeTileList";
    const DISPLAY_NAME: &'static str = "Merge List of Tiles to Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Merge Image Lists (DEPRECATED)**: Concatenate multiple image lists into one.
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
    const DISPLAY_NAME: &'static str = "Merge Image Lists (DEPRECATED)";
    const DESCRIPTION: &'static str = "Concatenate multiple image lists into one.";
    const CATEGORY: &'static str = "image/batch";
}
///**Rebatch Images**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RebatchImages<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub batch_size: BatchSizeParam,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> RebatchImages<ImagesParam, BatchSizeParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, batch_size: BatchSizeParam) -> Self {
        Self { images, batch_size }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    BatchSizeParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RebatchImages<ImagesParam, BatchSizeParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("images".to_string(), self.images.clone().into());
        output.insert("batch_size".to_string(), self.batch_size.clone().into());
        output
    }
    const NAME: &'static str = "RebatchImages";
    const DISPLAY_NAME: &'static str = "Rebatch Images";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Repeat Image Batch**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RepeatImageBatch<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 4096
  - Min: 1
*/
    pub amount: AmountParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> RepeatImageBatch<ImageParam, AmountParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, amount: AmountParam) -> Self {
        Self { image, amount }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    AmountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for RepeatImageBatch<ImageParam, AmountParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("amount".to_string(), self.amount.clone().into());
        output
    }
    const NAME: &'static str = "RepeatImageBatch";
    const DISPLAY_NAME: &'static str = "Repeat Image Batch";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/batch";
}
///**Shuffle Images List**: Randomly shuffle the order of images in a list.
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
    const DISPLAY_NAME: &'static str = "Shuffle Images List";
    const DESCRIPTION: &'static str = "Randomly shuffle the order of images in a list.";
    const CATEGORY: &'static str = "image/batch";
}
///**Shuffle Pairs of Image-Text**: Randomly shuffle the order of pairs of image-text in a list.
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
    const DISPLAY_NAME: &'static str = "Shuffle Pairs of Image-Text";
    const DESCRIPTION: &'static str = "Randomly shuffle the order of pairs of image-text in a list.";
    const CATEGORY: &'static str = "image/batch";
}
///**Split Image into List of Tiles**: Splits an image into a batched list of tiles with a specified overlap.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SplitImageToTileList<
    ImageParam: crate::nodes::types::Image,
    TileWidthParam: crate::nodes::types::Int,
    TileHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 64
*/
    pub tile_width: TileWidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 64
*/
    pub tile_height: TileHeightParam,
    /**No documentation.

**Metadata**:
  - Default: 128
  - Max: 4096
  - Min: 0
*/
    pub overlap: OverlapParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    TileWidthParam: crate::nodes::types::Int,
    TileHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> SplitImageToTileList<ImageParam, TileWidthParam, TileHeightParam, OverlapParam> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        tile_width: TileWidthParam,
        tile_height: TileHeightParam,
        overlap: OverlapParam,
    ) -> Self {
        Self {
            image,
            tile_width,
            tile_height,
            overlap,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    TileWidthParam: crate::nodes::types::Int,
    TileHeightParam: crate::nodes::types::Int,
    OverlapParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for SplitImageToTileList<ImageParam, TileWidthParam, TileHeightParam, OverlapParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("tile_width".to_string(), self.tile_width.clone().into());
        output.insert("tile_height".to_string(), self.tile_height.clone().into());
        output.insert("overlap".to_string(), self.overlap.clone().into());
        output
    }
    const NAME: &'static str = "SplitImageToTileList";
    const DISPLAY_NAME: &'static str = "Split Image into List of Tiles";
    const DESCRIPTION: &'static str = "Splits an image into a batched list of tiles with a specified overlap.";
    const CATEGORY: &'static str = "image/batch";
}
