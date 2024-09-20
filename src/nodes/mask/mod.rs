//!`mask` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod compositing;
///**CropMask**: No description.
#[derive(Clone)]
pub struct CropMask<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
impl<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> CropMask<Mask, X, Y, Width, Height> {
    /// Create a new node.
    pub fn new(mask: Mask, x: X, y: Y, width: Width, height: Height) -> Self {
        Self { mask, x, y, width, height }
    }
}
impl<
    Mask: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for CropMask<Mask, X, Y, Width, Height> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub left: Left,
    ///No documentation.
    pub top: Top,
    ///No documentation.
    pub right: Right,
    ///No documentation.
    pub bottom: Bottom,
}
impl<
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> FeatherMask<Mask, Left, Top, Right, Bottom> {
    /// Create a new node.
    pub fn new(mask: Mask, left: Left, top: Top, right: Right, bottom: Bottom) -> Self {
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
    Mask: crate::nodes::types::Mask,
    Left: crate::nodes::types::Int,
    Top: crate::nodes::types::Int,
    Right: crate::nodes::types::Int,
    Bottom: crate::nodes::types::Int,
> crate::nodes::TypedNode for FeatherMask<Mask, Left, Top, Right, Bottom> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub expand: Expand,
    ///No documentation.
    pub tapered_corners: TaperedCorners,
}
impl<
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> GrowMask<Mask, Expand, TaperedCorners> {
    /// Create a new node.
    pub fn new(mask: Mask, expand: Expand, tapered_corners: TaperedCorners) -> Self {
        Self {
            mask,
            expand,
            tapered_corners,
        }
    }
}
impl<
    Mask: crate::nodes::types::Mask,
    Expand: crate::nodes::types::Int,
    TaperedCorners: crate::nodes::types::Boolean,
> crate::nodes::TypedNode for GrowMask<Mask, Expand, TaperedCorners> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub color: Color,
}
impl<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> ImageColorToMask<Image, Color> {
    /// Create a new node.
    pub fn new(image: Image, color: Color) -> Self {
        Self { image, color }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Color: crate::nodes::types::Int,
> crate::nodes::TypedNode for ImageColorToMask<Image, Color> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
impl<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> ImageToMask<Image, Channel> {
    /// Create a new node.
    pub fn new(image: Image, channel: Channel) -> Self {
        Self { image, channel }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageToMask<Image, Channel> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
pub struct InvertMask<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
impl<Mask: crate::nodes::types::Mask> InvertMask<Mask> {
    /// Create a new node.
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for InvertMask<Mask> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub channel: Channel,
}
impl<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> LoadImageMask<Image, Channel> {
    /// Create a new node.
    pub fn new(image: Image, channel: Channel) -> Self {
        Self { image, channel }
    }
}
impl<
    Image: crate::nodes::types::String,
    Channel: crate::nodes::types::String,
> crate::nodes::TypedNode for LoadImageMask<Image, Channel> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> {
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
    ///No documentation.
    pub operation: Operation,
}
impl<
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> MaskComposite<Destination, Source, X, Y, Operation> {
    /// Create a new node.
    pub fn new(
        destination: Destination,
        source: Source,
        x: X,
        y: Y,
        operation: Operation,
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
    Destination: crate::nodes::types::Mask,
    Source: crate::nodes::types::Mask,
    X: crate::nodes::types::Int,
    Y: crate::nodes::types::Int,
    Operation: crate::nodes::types::String,
> crate::nodes::TypedNode for MaskComposite<Destination, Source, X, Y, Operation> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
pub struct MaskToImage<Mask: crate::nodes::types::Mask> {
    ///No documentation.
    pub mask: Mask,
}
impl<Mask: crate::nodes::types::Mask> MaskToImage<Mask> {
    /// Create a new node.
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }
}
impl<Mask: crate::nodes::types::Mask> crate::nodes::TypedNode for MaskToImage<Mask> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> {
    ///No documentation.
    pub value: Value,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
}
impl<
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> SolidMask<Value, Width, Height> {
    /// Create a new node.
    pub fn new(value: Value, width: Width, height: Height) -> Self {
        Self { value, width, height }
    }
}
impl<
    Value: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
> crate::nodes::TypedNode for SolidMask<Value, Width, Height> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> {
    ///No documentation.
    pub mask: Mask,
    ///No documentation.
    pub value: Value,
}
impl<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> ThresholdMask<Mask, Value> {
    /// Create a new node.
    pub fn new(mask: Mask, value: Value) -> Self {
        Self { mask, value }
    }
}
impl<
    Mask: crate::nodes::types::Mask,
    Value: crate::nodes::types::Float,
> crate::nodes::TypedNode for ThresholdMask<Mask, Value> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
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
