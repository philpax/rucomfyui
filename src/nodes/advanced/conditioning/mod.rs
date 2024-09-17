//!conditioning
pub mod flux;
///**CLIPTextEncodeHunyuanDiT**
pub struct ClipTextEncodeHunyuanDiT<
    Clip: crate::nodes::Clip,
    Bert: crate::nodes::String,
    Mt5Xl: crate::nodes::String,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub bert: Bert,
    ///No documentation.
    pub mt5xl: Mt5Xl,
}
///**CLIPTextEncodeSD3**
pub struct ClipTextEncodeSd3<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    ClipG: crate::nodes::String,
    T5Xxl: crate::nodes::String,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub clip_l: ClipL,
    ///No documentation.
    pub clip_g: ClipG,
    ///No documentation.
    pub t5xxl: T5Xxl,
}
///**CLIPTextEncodeSDXL**
pub struct ClipTextEncodeSdxl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    CropW: crate::nodes::Int,
    CropH: crate::nodes::Int,
    TargetWidth: crate::nodes::Int,
    TargetHeight: crate::nodes::Int,
    TextG: crate::nodes::String,
    Clip: crate::nodes::Clip,
    TextL: crate::nodes::String,
> {
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub crop_w: CropW,
    ///No documentation.
    pub crop_h: CropH,
    ///No documentation.
    pub target_width: TargetWidth,
    ///No documentation.
    pub target_height: TargetHeight,
    ///No documentation.
    pub text_g: TextG,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub text_l: TextL,
}
///**CLIPTextEncodeSDXLRefiner**
pub struct ClipTextEncodeSdxlRefiner<
    Ascore: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Text: crate::nodes::String,
    Clip: crate::nodes::Clip,
> {
    ///No documentation.
    pub ascore: Ascore,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub text: Text,
    ///No documentation.
    pub clip: Clip,
}
///**ConditioningSetTimestepRange**
pub struct ConditioningSetTimestepRange<
    Conditioning: crate::nodes::Conditioning,
    Start: crate::nodes::Float,
    End: crate::nodes::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub start: Start,
    ///No documentation.
    pub end: End,
}
///**ConditioningZeroOut**
pub struct ConditioningZeroOut<Conditioning: crate::nodes::Conditioning> {
    ///No documentation.
    pub conditioning: Conditioning,
}
