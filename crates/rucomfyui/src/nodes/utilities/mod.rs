//!`utilities` definitions/categories.
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
#[rustfmt::skip]
pub mod logic;
#[rustfmt::skip]
pub mod primitive;
/// Output types for nodes.
pub mod out {
    ///Output for [`ColorToRGBInt`](super::ColorToRGBInt).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ColorToRGBIntOutput {
        ///No documentation.
        pub rgb_int: crate::nodes::types::IntOut,
        ///No documentation.
        pub hex: crate::nodes::types::ColorOut,
    }
    ///Output for [`ComfyMathExpression`](super::ComfyMathExpression).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ComfyMathExpressionOutput {
        ///No documentation.
        pub float: crate::nodes::types::FloatOut,
        ///No documentation.
        pub int: crate::nodes::types::IntOut,
        ///No documentation.
        pub bool: crate::nodes::types::BooleanOut,
    }
    ///Output for [`ComfyNumberConvert`](super::ComfyNumberConvert).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ComfyNumberConvertOutput {
        ///No documentation.
        pub float: crate::nodes::types::FloatOut,
        ///No documentation.
        pub int: crate::nodes::types::IntOut,
    }
    ///Output for [`CustomCombo`](super::CustomCombo).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct CustomComboOutput {
        ///No documentation.
        pub string: crate::nodes::types::StringOut,
        ///No documentation.
        pub index: crate::nodes::types::IntOut,
    }
    ///Output for [`ImageHistogram`](super::ImageHistogram).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ImageHistogramOutput {
        ///No documentation.
        pub rgb: crate::nodes::types::HistogramOut,
        ///No documentation.
        pub luminance: crate::nodes::types::HistogramOut,
        ///No documentation.
        pub red: crate::nodes::types::HistogramOut,
        ///No documentation.
        pub green: crate::nodes::types::HistogramOut,
        ///No documentation.
        pub blue: crate::nodes::types::HistogramOut,
    }
    ///Output for [`ResolutionSelector`](super::ResolutionSelector).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct ResolutionSelectorOutput {
        ///Calculated width in pixels multiplied by the selected multiple.
        pub width: crate::nodes::types::IntOut,
        ///Calculated height in pixels multiplied by the selected multiple.
        pub height: crate::nodes::types::IntOut,
    }
}
///**Color Picker**: Return a color RGB integer value and hexadecimal representation.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ColorToRGBInt<ColorParam: crate::nodes::types::Color> {
    /**No documentation.

**Metadata**:
  - Default: #ffffff
*/
    pub color: ColorParam,
}
impl<ColorParam: crate::nodes::types::Color> ColorToRGBInt<ColorParam> {
    /// Create a new node.
    pub fn new(color: ColorParam) -> Self {
        Self { color }
    }
}
impl<ColorParam: crate::nodes::types::Color> crate::nodes::TypedNode
for ColorToRGBInt<ColorParam> {
    type Output = out::ColorToRGBIntOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            rgb_int: crate::nodes::types::IntOut::from_dynamic(node_id, 0u32),
            hex: crate::nodes::types::ColorOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("color".to_string(), self.color.clone().into());
        output
    }
    const NAME: &'static str = "ColorToRGBInt";
    const DISPLAY_NAME: &'static str = "Color Picker";
    const DESCRIPTION: &'static str = "Return a color RGB integer value and hexadecimal representation.";
    const CATEGORY: &'static str = "utilities";
}
///**Math Expression**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfyMathExpression<ExpressionParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default: a + b
*/
    pub expression: ExpressionParam,
}
impl<ExpressionParam: crate::nodes::types::String> ComfyMathExpression<ExpressionParam> {
    /// Create a new node.
    pub fn new(expression: ExpressionParam) -> Self {
        Self { expression }
    }
}
impl<ExpressionParam: crate::nodes::types::String> crate::nodes::TypedNode
for ComfyMathExpression<ExpressionParam> {
    type Output = out::ComfyMathExpressionOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            float: crate::nodes::types::FloatOut::from_dynamic(node_id, 0u32),
            int: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
            bool: crate::nodes::types::BooleanOut::from_dynamic(node_id, 2u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("expression".to_string(), self.expression.clone().into());
        output
    }
    const NAME: &'static str = "ComfyMathExpression";
    const DISPLAY_NAME: &'static str = "Math Expression";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Convert Number**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ComfyNumberConvert {}
impl ComfyNumberConvert {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for ComfyNumberConvert {
    type Output = out::ComfyNumberConvertOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            float: crate::nodes::types::FloatOut::from_dynamic(node_id, 0u32),
            int: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "ComfyNumberConvert";
    const DISPLAY_NAME: &'static str = "Convert Number";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Create List**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CreateList {}
impl CreateList {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for CreateList {
    type Output = crate::nodes::types::ComfyMatchTypeV3Out;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "CreateList";
    const DISPLAY_NAME: &'static str = "Create List";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Curve Editor**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CurveEditor<
    CurveParam: crate::nodes::types::Curve,
    HistogramParam: crate::nodes::types::Histogram = crate::nodes::types::HistogramOut,
> {
    ///No documentation.
    pub curve: CurveParam,
    ///No documentation.
    pub histogram: Option<HistogramParam>,
}
impl<
    CurveParam: crate::nodes::types::Curve,
    HistogramParam: crate::nodes::types::Histogram,
> CurveEditor<CurveParam, HistogramParam> {
    /// Create a new node.
    pub fn new(curve: CurveParam, histogram: Option<HistogramParam>) -> Self {
        Self { curve, histogram }
    }
}
impl<
    CurveParam: crate::nodes::types::Curve,
    HistogramParam: crate::nodes::types::Histogram,
> crate::nodes::TypedNode for CurveEditor<CurveParam, HistogramParam> {
    type Output = crate::nodes::types::CurveOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("curve".to_string(), self.curve.clone().into());
        if let Some(v) = &self.histogram {
            output.insert("histogram".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "CurveEditor";
    const DISPLAY_NAME: &'static str = "Curve Editor";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Custom Combo**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CustomCombo {}
impl CustomCombo {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for CustomCombo {
    type Output = out::CustomComboOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            string: crate::nodes::types::StringOut::from_dynamic(node_id, 0u32),
            index: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "CustomCombo";
    const DISPLAY_NAME: &'static str = "Custom Combo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Image Histogram**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageHistogram<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> ImageHistogram<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for ImageHistogram<ImageParam> {
    type Output = out::ImageHistogramOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            rgb: crate::nodes::types::HistogramOut::from_dynamic(node_id, 0u32),
            luminance: crate::nodes::types::HistogramOut::from_dynamic(node_id, 1u32),
            red: crate::nodes::types::HistogramOut::from_dynamic(node_id, 2u32),
            green: crate::nodes::types::HistogramOut::from_dynamic(node_id, 3u32),
            blue: crate::nodes::types::HistogramOut::from_dynamic(node_id, 4u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "ImageHistogram";
    const DISPLAY_NAME: &'static str = "Image Histogram";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
///**Preview as Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PreviewAny<SourceParam: crate::nodes::types::Wildcard> {
    ///No documentation.
    pub source: SourceParam,
}
impl<SourceParam: crate::nodes::types::Wildcard> PreviewAny<SourceParam> {
    /// Create a new node.
    pub fn new(source: SourceParam) -> Self {
        Self { source }
    }
}
impl<SourceParam: crate::nodes::types::Wildcard> crate::nodes::TypedNode
for PreviewAny<SourceParam> {
    type Output = WorkflowNodeId;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        node_id
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("source".to_string(), self.source.clone().into());
        output
    }
    const NAME: &'static str = "PreviewAny";
    const DISPLAY_NAME: &'static str = "Preview as Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utilities";
}
impl<SourceParam: crate::nodes::types::Wildcard> crate::nodes::TypedOutputNode
for PreviewAny<SourceParam> {}
///**Resolution Selector**: Calculate width and height from aspect ratio and megapixel target. Useful for setting up Empty Latent Image dimensions.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ResolutionSelector<
    MegapixelsParam: crate::nodes::types::Float,
    MultipleParam: crate::nodes::types::Int,
> {
    /**Target total megapixels. 1.0 MP ≈ 1024x1024 for square.

**Metadata**:
  - Default: 1
  - Max: 16
  - Min: 0.1
  - Step: 0.1
*/
    pub megapixels: MegapixelsParam,
    /**Nearest multiple of the result to set the selected resolution to.

**Metadata**:
  - Default: 8
  - Max: 128
  - Min: 8
  - Step: 4
*/
    pub multiple: MultipleParam,
}
impl<
    MegapixelsParam: crate::nodes::types::Float,
    MultipleParam: crate::nodes::types::Int,
> ResolutionSelector<MegapixelsParam, MultipleParam> {
    /// Create a new node.
    pub fn new(megapixels: MegapixelsParam, multiple: MultipleParam) -> Self {
        Self { megapixels, multiple }
    }
}
impl<
    MegapixelsParam: crate::nodes::types::Float,
    MultipleParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for ResolutionSelector<MegapixelsParam, MultipleParam> {
    type Output = out::ResolutionSelectorOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            width: crate::nodes::types::IntOut::from_dynamic(node_id, 0u32),
            height: crate::nodes::types::IntOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("megapixels".to_string(), self.megapixels.clone().into());
        output.insert("multiple".to_string(), self.multiple.clone().into());
        output
    }
    const NAME: &'static str = "ResolutionSelector";
    const DISPLAY_NAME: &'static str = "Resolution Selector";
    const DESCRIPTION: &'static str = "Calculate width and height from aspect ratio and megapixel target. Useful for setting up Empty Latent Image dimensions.";
    const CATEGORY: &'static str = "utilities";
}
