//!`conditioning` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod flux;
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("bert".to_string(), self.bert.to_workflow_input());
        output.insert("mt5xl".to_string(), self.mt_5_xl.to_workflow_input());
        output
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("clip_l".to_string(), self.clip_l.to_workflow_input());
        output.insert("clip_g".to_string(), self.clip_g.to_workflow_input());
        output.insert("t5xxl".to_string(), self.t_5_xxl.to_workflow_input());
        output
            .insert("empty_padding".to_string(), self.empty_padding.to_workflow_input());
        output
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("crop_w".to_string(), self.crop_w.to_workflow_input());
        output.insert("crop_h".to_string(), self.crop_h.to_workflow_input());
        output.insert("target_width".to_string(), self.target_width.to_workflow_input());
        output
            .insert("target_height".to_string(), self.target_height.to_workflow_input());
        output.insert("text_g".to_string(), self.text_g.to_workflow_input());
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output.insert("text_l".to_string(), self.text_l.to_workflow_input());
        output
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("ascore".to_string(), self.ascore.to_workflow_input());
        output.insert("width".to_string(), self.width.to_workflow_input());
        output.insert("height".to_string(), self.height.to_workflow_input());
        output.insert("text".to_string(), self.text.to_workflow_input());
        output.insert("clip".to_string(), self.clip.to_workflow_input());
        output
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output.insert("start".to_string(), self.start.to_workflow_input());
        output.insert("end".to_string(), self.end.to_workflow_input());
        output
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
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("conditioning".to_string(), self.conditioning.to_workflow_input());
        output
    }
    const NAME: &'static str = "ConditioningZeroOut";
    const DISPLAY_NAME: &'static str = "ConditioningZeroOut";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
