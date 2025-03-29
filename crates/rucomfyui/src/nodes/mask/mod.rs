//!`mask` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod compositing;
///**CropMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "CropMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**FeatherMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "FeatherMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**GrowMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "GrowMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ImageColorToMask**: No description.
#[derive(Clone)]
pub struct ImageColorToMask<
    ImageParam: crate::nodes::types::Image,
    ColorParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: color
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
    const DISPLAY_NAME: &'static str = "ImageColorToMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Image to Mask**: No description.
#[derive(Clone)]
pub struct ImageToMask<
    ImageParam: crate::nodes::types::Image,
    ChannelParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub channel: ChannelParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    ChannelParam: crate::nodes::types::String,
> ImageToMask<ImageParam, ChannelParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, channel: ChannelParam) -> Self {
        Self { image, channel }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    ChannelParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageToMask<ImageParam, ChannelParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("channel".to_string(), self.channel.clone().into());
        output
    }
    const NAME: &'static str = "ImageToMask";
    const DISPLAY_NAME: &'static str = "Convert Image to Mask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**InvertMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "InvertMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Load Image (as Mask)**: No description.
#[derive(Clone)]
pub struct LoadImageMask<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Image upload: true
*/
    pub image: ImageParam,
    ///No documentation.
    pub channel: ChannelParam,
}
impl<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> LoadImageMask<ImageParam, ChannelParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, channel: ChannelParam) -> Self {
        Self { image, channel }
    }
}
impl<
    ImageParam: crate::nodes::types::String,
    ChannelParam: crate::nodes::types::String,
> crate::nodes::TypedNode for LoadImageMask<ImageParam, ChannelParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("channel".to_string(), self.channel.clone().into());
        output
    }
    const NAME: &'static str = "LoadImageMask";
    const DISPLAY_NAME: &'static str = "Load Image (as Mask)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**MaskComposite**: No description.
#[derive(Clone)]
pub struct MaskComposite<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    OperationParam: crate::nodes::types::String,
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
    ///No documentation.
    pub operation: OperationParam,
}
impl<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    OperationParam: crate::nodes::types::String,
> MaskComposite<DestinationParam, SourceParam, XParam, YParam, OperationParam> {
    /// Create a new node.
    pub fn new(
        destination: DestinationParam,
        source: SourceParam,
        x: XParam,
        y: YParam,
        operation: OperationParam,
    ) -> Self {
        Self {
            destination,
            source,
            x,
            y,
            operation,
        }
    }
}
impl<
    DestinationParam: crate::nodes::types::Mask,
    SourceParam: crate::nodes::types::Mask,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    OperationParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for MaskComposite<DestinationParam, SourceParam, XParam, YParam, OperationParam> {
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
        output.insert("operation".to_string(), self.operation.clone().into());
        output
    }
    const NAME: &'static str = "MaskComposite";
    const DISPLAY_NAME: &'static str = "MaskComposite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**Convert Mask to Image**: No description.
#[derive(Clone)]
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
    const CATEGORY: &'static str = "mask";
}
///**SolidMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "SolidMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
///**ThresholdMask**: No description.
#[derive(Clone)]
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
    const DISPLAY_NAME: &'static str = "ThresholdMask";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask";
}
