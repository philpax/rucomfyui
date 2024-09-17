//!style_model
///**Apply Style Model**
pub struct StyleModelApply<
    Conditioning: crate::nodes::Conditioning,
    StyleModel: crate::nodes::StyleModel,
    ClipVisionOutput: crate::nodes::ClipVisionOutput,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub style_model: StyleModel,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutput,
}
