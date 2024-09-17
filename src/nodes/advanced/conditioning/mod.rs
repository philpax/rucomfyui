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
    pub mt_5_xl: Mt5Xl,
}
///Output for [`ClipTextEncodeHunyuanDiT`].
#[derive(Clone)]
pub struct ClipTextEncodeHunyuanDiTOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Clip: crate::nodes::Clip,
    Bert: crate::nodes::String,
    Mt5Xl: crate::nodes::String,
> crate::nodes::TypedNode for ClipTextEncodeHunyuanDiT<Clip, Bert, Mt5Xl> {
    type Output = ClipTextEncodeHunyuanDiTOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSD3**
pub struct ClipTextEncodeSd3<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    ClipG: crate::nodes::String,
    T5Xxl: crate::nodes::String,
    EmptyPadding: crate::nodes::String,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub clip_l: ClipL,
    ///No documentation.
    pub clip_g: ClipG,
    ///No documentation.
    pub t_5_xxl: T5Xxl,
    ///No documentation.
    pub empty_padding: EmptyPadding,
}
///Output for [`ClipTextEncodeSd3`].
#[derive(Clone)]
pub struct ClipTextEncodeSd3Output {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Clip: crate::nodes::Clip,
    ClipL: crate::nodes::String,
    ClipG: crate::nodes::String,
    T5Xxl: crate::nodes::String,
    EmptyPadding: crate::nodes::String,
> crate::nodes::TypedNode
for ClipTextEncodeSd3<Clip, ClipL, ClipG, T5Xxl, EmptyPadding> {
    type Output = ClipTextEncodeSd3Output;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSD3";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
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
///Output for [`ClipTextEncodeSdxl`].
#[derive(Clone)]
pub struct ClipTextEncodeSdxlOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    CropW: crate::nodes::Int,
    CropH: crate::nodes::Int,
    TargetWidth: crate::nodes::Int,
    TargetHeight: crate::nodes::Int,
    TextG: crate::nodes::String,
    Clip: crate::nodes::Clip,
    TextL: crate::nodes::String,
> crate::nodes::TypedNode
for ClipTextEncodeSdxl<
    Width,
    Height,
    CropW,
    CropH,
    TargetWidth,
    TargetHeight,
    TextG,
    Clip,
    TextL,
> {
    type Output = ClipTextEncodeSdxlOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSDXL";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXL";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
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
///Output for [`ClipTextEncodeSdxlRefiner`].
#[derive(Clone)]
pub struct ClipTextEncodeSdxlRefinerOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Ascore: crate::nodes::Float,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    Text: crate::nodes::String,
    Clip: crate::nodes::Clip,
> crate::nodes::TypedNode
for ClipTextEncodeSdxlRefiner<Ascore, Width, Height, Text, Clip> {
    type Output = ClipTextEncodeSdxlRefinerOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
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
///Output for [`ConditioningSetTimestepRange`].
#[derive(Clone)]
pub struct ConditioningSetTimestepRangeOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<
    Conditioning: crate::nodes::Conditioning,
    Start: crate::nodes::Float,
    End: crate::nodes::Float,
> crate::nodes::TypedNode for ConditioningSetTimestepRange<Conditioning, Start, End> {
    type Output = ConditioningSetTimestepRangeOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningSetTimestepRange";
    const DISPLAY_NAME: &'static str = "ConditioningSetTimestepRange";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**ConditioningZeroOut**
pub struct ConditioningZeroOut<Conditioning: crate::nodes::Conditioning> {
    ///No documentation.
    pub conditioning: Conditioning,
}
///Output for [`ConditioningZeroOut`].
#[derive(Clone)]
pub struct ConditioningZeroOutOutput {
    ///No documentation.
    pub conditioning: crate::nodes::ConditioningOut,
}
impl<Conditioning: crate::nodes::Conditioning> crate::nodes::TypedNode
for ConditioningZeroOut<Conditioning> {
    type Output = ConditioningZeroOutOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::ConditioningOut(0u32),
        }
    }
    const NAME: &'static str = "ConditioningZeroOut";
    const DISPLAY_NAME: &'static str = "ConditioningZeroOut";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
