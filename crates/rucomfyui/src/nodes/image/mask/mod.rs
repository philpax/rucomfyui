//!`mask` definitions/categories.
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
///**Batch Masks**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BatchMasksNode {}
impl BatchMasksNode {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for BatchMasksNode {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "BatchMasksNode";
    const DISPLAY_NAME: &'static str = "Batch Masks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Crop Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CropMask<
    MaskParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
}
impl<
    MaskParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> CropMask<MaskParam, XParam, YParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(
        mask: MaskParam,
        x: XParam,
        y: YParam,
        width: WidthParam,
        height: HeightParam,
    ) -> Self {
        Self { mask, x, y, width, height }
    }
}
impl<
    MaskParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for CropMask<MaskParam, XParam, YParam, WidthParam, HeightParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "CropMask";
    const DISPLAY_NAME: &'static str = "Crop Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Feather Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FeatherMask<
    MaskParam: crate::nodes::types::Mask,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub left: LeftParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub top: TopParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub right: RightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub bottom: BottomParam,
}
impl<
    MaskParam: crate::nodes::types::Mask,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
> FeatherMask<MaskParam, LeftParam, TopParam, RightParam, BottomParam> {
    /// Create a new node.
    pub fn new(
        mask: MaskParam,
        left: LeftParam,
        top: TopParam,
        right: RightParam,
        bottom: BottomParam,
    ) -> Self {
        Self {
            mask,
            left,
            top,
            right,
            bottom,
        }
    }
}
impl<
    MaskParam: crate::nodes::types::Mask,
    LeftParam: crate::nodes::types::Int,
    TopParam: crate::nodes::types::Int,
    RightParam: crate::nodes::types::Int,
    BottomParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for FeatherMask<MaskParam, LeftParam, TopParam, RightParam, BottomParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("left".to_string(), self.left.clone().into());
        output.insert("top".to_string(), self.top.clone().into());
        output.insert("right".to_string(), self.right.clone().into());
        output.insert("bottom".to_string(), self.bottom.clone().into());
        output
    }
    const NAME: &'static str = "FeatherMask";
    const DISPLAY_NAME: &'static str = "Feather Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Grow Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GrowMask<
    MaskParam: crate::nodes::types::Mask,
    ExpandParam: crate::nodes::types::Int,
    TaperedCornersParam: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: -16384
  - Step: 1
*/
    pub expand: ExpandParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub tapered_corners: TaperedCornersParam,
}
impl<
    MaskParam: crate::nodes::types::Mask,
    ExpandParam: crate::nodes::types::Int,
    TaperedCornersParam: crate::nodes::types::Boolean,
> GrowMask<MaskParam, ExpandParam, TaperedCornersParam> {
    /// Create a new node.
    pub fn new(
        mask: MaskParam,
        expand: ExpandParam,
        tapered_corners: TaperedCornersParam,
    ) -> Self {
        Self {
            mask,
            expand,
            tapered_corners,
        }
    }
}
impl<
    MaskParam: crate::nodes::types::Mask,
    ExpandParam: crate::nodes::types::Int,
    TaperedCornersParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for GrowMask<MaskParam, ExpandParam, TaperedCornersParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("expand".to_string(), self.expand.clone().into());
        output
            .insert("tapered_corners".to_string(), self.tapered_corners.clone().into());
        output
    }
    const NAME: &'static str = "GrowMask";
    const DISPLAY_NAME: &'static str = "Grow Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Convert Image Color to Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageColorToMask<
    ImageParam: crate::nodes::types::Image,
    ColorParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 16777215
  - Min: 0
  - Step: 1
*/
    pub color: ColorParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ColorParam: crate::nodes::types::Int,
> ImageColorToMask<ImageParam, ColorParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, color: ColorParam) -> Self {
        Self { image, color }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ColorParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageColorToMask<ImageParam, ColorParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("color".to_string(), self.color.clone().into());
        output
    }
    const NAME: &'static str = "ImageColorToMask";
    const DISPLAY_NAME: &'static str = "Convert Image Color to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Convert Image to Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageToMask<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageToMask<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageToMask<ImageParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageToMask";
    const DISPLAY_NAME: &'static str = "Convert Image to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Invert Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct InvertMask<MaskParam: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: MaskParam,
}
impl<MaskParam: crate::nodes::types::Mask> InvertMask<MaskParam> {
    /// Create a new node.
    pub fn new(mask: MaskParam) -> Self {
        Self { mask }
    }
}
impl<MaskParam: crate::nodes::types::Mask> crate::nodes::TypedNode
for InvertMask<MaskParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output
    }
    const NAME: &'static str = "InvertMask";
    const DISPLAY_NAME: &'static str = "Invert Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Combine Masks**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MaskComposite<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub destination: DestinationParam,
    ///No documentation.
    pub source: SourceParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: YParam,
}
impl<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> MaskComposite<DestinationParam, SourceParam, XParam, YParam> {
    /// Create a new node.
    pub fn new(
        destination: DestinationParam,
        source: SourceParam,
        x: XParam,
        y: YParam,
    ) -> Self {
        Self { destination, source, x, y }
    }
}
impl<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MaskComposite<DestinationParam, SourceParam, XParam, YParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("destination".to_string(), self.destination.clone().into());
        output.insert("source".to_string(), self.source.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output
    }
    const NAME: &'static str = "MaskComposite";
    const DISPLAY_NAME: &'static str = "Combine Masks";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Preview Mask**: Saves the input images to your ComfyUI output directory.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MaskPreview<MaskParam: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: MaskParam,
}
impl<MaskParam: crate::nodes::types::Mask> MaskPreview<MaskParam> {
    /// Create a new node.
    pub fn new(mask: MaskParam) -> Self {
        Self { mask }
    }
}
impl<MaskParam: crate::nodes::types::Mask> crate::nodes::TypedNode
for MaskPreview<MaskParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output
    }
    const NAME: &'static str = "MaskPreview";
    const DISPLAY_NAME: &'static str = "Preview Mask";
    const DESCRIPTION: &'static str = "Saves the input images to your ComfyUI output directory.";
    const CATEGORY: &'static str = "image/mask";
}
impl<MaskParam: crate::nodes::types::Mask> crate::nodes::TypedOutputNode
for MaskPreview<MaskParam> {}
///**Convert Mask to Image**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MaskToImage<MaskParam: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: MaskParam,
}
impl<MaskParam: crate::nodes::types::Mask> MaskToImage<MaskParam> {
    /// Create a new node.
    pub fn new(mask: MaskParam) -> Self {
        Self { mask }
    }
}
impl<MaskParam: crate::nodes::types::Mask> crate::nodes::TypedNode
for MaskToImage<MaskParam> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output
    }
    const NAME: &'static str = "MaskToImage";
    const DISPLAY_NAME: &'static str = "Convert Mask to Image";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Create Solid Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SolidMask<
    ValueParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub value: ValueParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 512
  - Max: 16384
  - Min: 1
  - Step: 1
*/
    pub height: HeightParam,
}
impl<
    ValueParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> SolidMask<ValueParam, WidthParam, HeightParam> {
    /// Create a new node.
    pub fn new(value: ValueParam, width: WidthParam, height: HeightParam) -> Self {
        Self { value, width, height }
    }
}
impl<
    ValueParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for SolidMask<ValueParam, WidthParam, HeightParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("value".to_string(), self.value.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output
    }
    const NAME: &'static str = "SolidMask";
    const DISPLAY_NAME: &'static str = "Create Solid Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**Threshold Mask**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ThresholdMask<
    MaskParam: crate::nodes::types::Mask,
    ValueParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub mask: MaskParam,
    /**No documentation.

**Metadata**:
  - Default: 0.5
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub value: ValueParam,
}
impl<
    MaskParam: crate::nodes::types::Mask,
    ValueParam: crate::nodes::types::Float,
> ThresholdMask<MaskParam, ValueParam> {
    /// Create a new node.
    pub fn new(mask: MaskParam, value: ValueParam) -> Self {
        Self { mask, value }
    }
}
impl<
    MaskParam: crate::nodes::types::Mask,
    ValueParam: crate::nodes::types::Float,
> crate::nodes::TypedNode for ThresholdMask<MaskParam, ValueParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("value".to_string(), self.value.clone().into());
        output
    }
    const NAME: &'static str = "ThresholdMask";
    const DISPLAY_NAME: &'static str = "Threshold Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
///**VOID Quadmask Preprocessor**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct VOIDQuadmaskPreprocess<
    MaskParam: crate::nodes::types::Mask,
    DilateWidthParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: MaskParam,
    /**Dilation radius for the primary mask region (0 = no dilation)

**Metadata**:
  - Default: 0
  - Max: 50
  - Min: 0
  - Step: 1
*/
    pub dilate_width: DilateWidthParam,
}
impl<
    MaskParam: crate::nodes::types::Mask,
    DilateWidthParam: crate::nodes::types::Int,
> VOIDQuadmaskPreprocess<MaskParam, DilateWidthParam> {
    /// Create a new node.
    pub fn new(mask: MaskParam, dilate_width: DilateWidthParam) -> Self {
        Self { mask, dilate_width }
    }
}
impl<
    MaskParam: crate::nodes::types::Mask,
    DilateWidthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for VOIDQuadmaskPreprocess<MaskParam, DilateWidthParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("mask".to_string(), self.mask.clone().into());
        output.insert("dilate_width".to_string(), self.dilate_width.clone().into());
        output
    }
    const NAME: &'static str = "VOIDQuadmaskPreprocess";
    const DISPLAY_NAME: &'static str = "VOID Quadmask Preprocessor";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/mask";
}
