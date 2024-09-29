//!`conditioning` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
pub mod flux;
///**CLIPTextEncodeHunyuanDiT**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeHunyuanDiT<
    Clip: crate::nodes::types::Clip,
    Bert: crate::nodes::types::String,
    Mt5Xl: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub bert: Bert,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub mt_5_xl: Mt5Xl,
}
impl<
    Clip: crate::nodes::types::Clip,
    Bert: crate::nodes::types::String,
    Mt5Xl: crate::nodes::types::String,
> ClipTextEncodeHunyuanDiT<Clip, Bert, Mt5Xl> {
    /// Create a new node.
    pub fn new(clip: Clip, bert: Bert, mt_5_xl: Mt5Xl) -> Self {
        Self { clip, bert, mt_5_xl }
    }
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
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("bert".to_string(), self.bert.clone().into());
        output.insert("mt5xl".to_string(), self.mt_5_xl.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeHunyuanDiT";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSD3**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeSd3<
    Clip: crate::nodes::types::Clip,
    ClipL: crate::nodes::types::String,
    ClipG: crate::nodes::types::String,
    T5Xxl: crate::nodes::types::String,
    EmptyPadding: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_l: ClipL,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_g: ClipG,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
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
> ClipTextEncodeSd3<Clip, ClipL, ClipG, T5Xxl, EmptyPadding> {
    /// Create a new node.
    pub fn new(
        clip: Clip,
        clip_l: ClipL,
        clip_g: ClipG,
        t_5_xxl: T5Xxl,
        empty_padding: EmptyPadding,
    ) -> Self {
        Self {
            clip,
            clip_l,
            clip_g,
            t_5_xxl,
            empty_padding,
        }
    }
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
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("clip_l".to_string(), self.clip_l.clone().into());
        output.insert("clip_g".to_string(), self.clip_g.clone().into());
        output.insert("t5xxl".to_string(), self.t_5_xxl.clone().into());
        output.insert("empty_padding".to_string(), self.empty_padding.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeSD3";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSD3";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSDXL**: No description.
#[derive(Clone)]
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
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
*/
    pub crop_w: CropW,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
*/
    pub crop_h: CropH,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub target_width: TargetWidth,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub target_height: TargetHeight,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text_g: TextG,
    ///No documentation.
    pub clip: Clip,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
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
> ClipTextEncodeSdxl<
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
    /// Create a new node.
    pub fn new(
        width: Width,
        height: Height,
        crop_w: CropW,
        crop_h: CropH,
        target_width: TargetWidth,
        target_height: TargetHeight,
        text_g: TextG,
        clip: Clip,
        text_l: TextL,
    ) -> Self {
        Self {
            width,
            height,
            crop_w,
            crop_h,
            target_width,
            target_height,
            text_g,
            clip,
            text_l,
        }
    }
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
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("crop_w".to_string(), self.crop_w.clone().into());
        output.insert("crop_h".to_string(), self.crop_h.clone().into());
        output.insert("target_width".to_string(), self.target_width.clone().into());
        output.insert("target_height".to_string(), self.target_height.clone().into());
        output.insert("text_g".to_string(), self.text_g.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("text_l".to_string(), self.text_l.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeSDXL";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXL";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSDXLRefiner**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeSdxlRefiner<
    Ascore: crate::nodes::types::Float,
    Width: crate::nodes::types::Int,
    Height: crate::nodes::types::Int,
    Text: crate::nodes::types::String,
    Clip: crate::nodes::types::Clip,
> {
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub ascore: Ascore,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub width: Width,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub height: Height,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
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
> ClipTextEncodeSdxlRefiner<Ascore, Width, Height, Text, Clip> {
    /// Create a new node.
    pub fn new(
        ascore: Ascore,
        width: Width,
        height: Height,
        text: Text,
        clip: Clip,
    ) -> Self {
        Self {
            ascore,
            width,
            height,
            text,
            clip,
        }
    }
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
        output.insert("ascore".to_string(), self.ascore.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodeSDXLRefiner";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**ConditioningSetTimestepRange**: No description.
#[derive(Clone)]
pub struct ConditioningSetTimestepRange<
    Conditioning: crate::nodes::types::Conditioning,
    Start: crate::nodes::types::Float,
    End: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: Conditioning,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start: Start,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end: End,
}
impl<
    Conditioning: crate::nodes::types::Conditioning,
    Start: crate::nodes::types::Float,
    End: crate::nodes::types::Float,
> ConditioningSetTimestepRange<Conditioning, Start, End> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning, start: Start, end: End) -> Self {
        Self { conditioning, start, end }
    }
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
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output.insert("start".to_string(), self.start.clone().into());
        output.insert("end".to_string(), self.end.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningSetTimestepRange";
    const DISPLAY_NAME: &'static str = "ConditioningSetTimestepRange";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**ConditioningZeroOut**: No description.
#[derive(Clone)]
pub struct ConditioningZeroOut<Conditioning: crate::nodes::types::Conditioning> {
    ///No documentation.
    pub conditioning: Conditioning,
}
impl<Conditioning: crate::nodes::types::Conditioning> ConditioningZeroOut<Conditioning> {
    /// Create a new node.
    pub fn new(conditioning: Conditioning) -> Self {
        Self { conditioning }
    }
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
        output.insert("conditioning".to_string(), self.conditioning.clone().into());
        output
    }
    const NAME: &'static str = "ConditioningZeroOut";
    const DISPLAY_NAME: &'static str = "ConditioningZeroOut";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
