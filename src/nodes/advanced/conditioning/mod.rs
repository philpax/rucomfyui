//!`conditioning` definitions/categories.
#![allow(unused_imports)]
use crate::workflow::WorkflowNodeId;
pub mod flux;
/// Output types for nodes.
pub mod out {
    ///Output for [`ClipTextEncodeHunyuanDiT`](super::ClipTextEncodeHunyuanDiT).
    #[derive(Clone)]
    pub struct ClipTextEncodeHunyuanDiTOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ClipTextEncodeSd3`](super::ClipTextEncodeSd3).
    #[derive(Clone)]
    pub struct ClipTextEncodeSd3Output {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ClipTextEncodeSdxl`](super::ClipTextEncodeSdxl).
    #[derive(Clone)]
    pub struct ClipTextEncodeSdxlOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ClipTextEncodeSdxlRefiner`](super::ClipTextEncodeSdxlRefiner).
    #[derive(Clone)]
    pub struct ClipTextEncodeSdxlRefinerOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningSetTimestepRange`](super::ConditioningSetTimestepRange).
    #[derive(Clone)]
    pub struct ConditioningSetTimestepRangeOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
    ///Output for [`ConditioningZeroOut`](super::ConditioningZeroOut).
    #[derive(Clone)]
    pub struct ConditioningZeroOutOutput {
        ///No documentation.
        pub conditioning: crate::nodes::types::ConditioningOut,
    }
}
///**CLIPTextEncodeHunyuanDiT**: No description.
pub struct ClipTextEncodeHunyuanDiT<
    Clip: crate::nodes::types::Clip,
    Bert: crate::nodes::types::String,
    Mt5Xl: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub bert: Bert,
    ///No documentation.
    pub mt_5_xl: Mt5Xl,
}
impl<
    Clip: crate::nodes::types::Clip,
    Bert: crate::nodes::types::String,
    Mt5Xl: crate::nodes::types::String,
> crate::nodes::TypedNode for ClipTextEncodeHunyuanDiT<Clip, Bert, Mt5Xl> {
    type Output = out::ClipTextEncodeHunyuanDiTOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSD3**: No description.
pub struct ClipTextEncodeSd3<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    ClipG: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    EmptyPadding: crate::nodes::types::String,
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
impl<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    ClipG: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    EmptyPadding: crate::nodes::types::String,
> crate::nodes::TypedNode
for ClipTextEncodeSd3<Clip, ClipL, ClipG, T5Xxl, EmptyPadding> {
    type Output = out::ClipTextEncodeSd3Output;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSD3";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSDXL**: No description.
pub struct ClipTextEncodeSdxl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CropW: crate::nodes::types::Int,
    CropH: crate::nodes::types::Int,
    TargetWidth: crate::nodes::types::Int,
    TargetHeight: crate::nodes::types::Int,
    TextG: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
    TextL: crate::nodes::types::String,
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
impl<
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    CropW: crate::nodes::types::Int,
    CropH: crate::nodes::types::Int,
    TargetWidth: crate::nodes::types::Int,
    TargetHeight: crate::nodes::types::Int,
    TextG: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
    TextL: crate::nodes::types::String,
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
    type Output = out::ClipTextEncodeSdxlOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSDXL";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXL";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSDXLRefiner**: No description.
pub struct ClipTextEncodeSdxlRefiner<
    Ascore: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
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
impl<
    Ascore: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> crate::nodes::TypedNode
for ClipTextEncodeSdxlRefiner<Ascore, Width, Height, Text, Clip> {
    type Output = out::ClipTextEncodeSdxlRefinerOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**ConditioningSetTimestepRange**: No description.
pub struct ConditioningSetTimestepRange<
    Conditioning: crate::nodes::types::Conditioning,
    Start: crate::nodes::types::Float,
    End: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    ///No documentation.
    pub start: Start,
    ///No documentation.
    pub end: End,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Start: crate::nodes::types::Float,
    End: crate::nodes::types::Float,
> crate::nodes::TypedNode for ConditioningSetTimestepRange<Conditioning, Start, End> {
    type Output = out::ConditioningSetTimestepRangeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ConditioningSetTimestepRange";
    const DISPLAY_NAME: &'static str = "ConditioningSetTimestepRange";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**ConditioningZeroOut**: No description.
pub struct ConditioningZeroOut<Conditioning: crate::nodes::types::Conditioning> {
    ///No documentation.
    pub conditioning: Conditioning,
}
impl<Conditioning: crate::nodes::types::Conditioning> crate::nodes::TypedNode
for ConditioningZeroOut<Conditioning> {
    type Output = out::ConditioningZeroOutOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            conditioning: crate::nodes::types::ConditioningOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "ConditioningZeroOut";
    const DISPLAY_NAME: &'static str = "ConditioningZeroOut";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
