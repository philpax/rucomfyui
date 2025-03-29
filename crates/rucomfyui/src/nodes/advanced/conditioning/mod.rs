//!`conditioning` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod flux;
///**CLIPTextEncodeHunyuanDiT**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeHunyuanDiT<
    ClipParam: crate::nodes::types::Clip,
    BertParam: crate::nodes::types::String,
    Mt5XlParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub bert: BertParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub mt_5_xl: Mt5XlParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    BertParam: crate::nodes::types::String,
    Mt5XlParam: crate::nodes::types::String,
> ClipTextEncodeHunyuanDiT<ClipParam, BertParam, Mt5XlParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam, bert: BertParam, mt_5_xl: Mt5XlParam) -> Self {
        Self { clip, bert, mt_5_xl }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    BertParam: crate::nodes::types::String,
    Mt5XlParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ClipTextEncodeHunyuanDiT<ClipParam, BertParam, Mt5XlParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
///**CLIPTextEncodePixArtAlpha**: Encodes text and sets the resolution conditioning for PixArt Alpha. Does not apply to PixArt Sigma.
#[derive(Clone)]
pub struct ClipTextEncodePixArtAlpha<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> {
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: TextParam,
    ///No documentation.
    pub clip: ClipParam,
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> ClipTextEncodePixArtAlpha<WidthParam, HeightParam, TextParam, ClipParam> {
    /// Create a new node.
    pub fn new(
        width: WidthParam,
        height: HeightParam,
        text: TextParam,
        clip: ClipParam,
    ) -> Self {
        Self { width, height, text, clip }
    }
}
impl<
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> crate::nodes::TypedNode
for ClipTextEncodePixArtAlpha<WidthParam, HeightParam, TextParam, ClipParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output
    }
    const NAME: &'static str = "CLIPTextEncodePixArtAlpha";
    const DISPLAY_NAME: &'static str = "CLIPTextEncodePixArtAlpha";
    const DESCRIPTION: &'static str = "Encodes text and sets the resolution conditioning for PixArt Alpha. Does not apply to PixArt Sigma.";
    const CATEGORY: &'static str = "advanced/conditioning";
}
///**CLIPTextEncodeSD3**: No description.
#[derive(Clone)]
pub struct ClipTextEncodeSd3<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    ClipGParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    EmptyPaddingParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_l: ClipLParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub clip_g: ClipGParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub t_5_xxl: T5XxlParam,
    ///No documentation.
    pub empty_padding: EmptyPaddingParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    ClipGParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    EmptyPaddingParam: crate::nodes::types::String,
> ClipTextEncodeSd3<ClipParam, ClipLParam, ClipGParam, T5XxlParam, EmptyPaddingParam> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        clip_l: ClipLParam,
        clip_g: ClipGParam,
        t_5_xxl: T5XxlParam,
        empty_padding: EmptyPaddingParam,
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
    ClipParam: crate::nodes::types::Clip,
    ClipLParam: crate::nodes::types::String,
    ClipGParam: crate::nodes::types::String,
    T5XxlParam: crate::nodes::types::String,
    EmptyPaddingParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ClipTextEncodeSd3<ClipParam, ClipLParam, ClipGParam, T5XxlParam, EmptyPaddingParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ClipParam: crate::nodes::types::Clip,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropWParam: crate::nodes::types::Int,
    CropHParam: crate::nodes::types::Int,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    TextGParam: crate::nodes::types::String,
    TextLParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
*/
    pub crop_w: CropWParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
*/
    pub crop_h: CropHParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub target_width: TargetWidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub target_height: TargetHeightParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text_g: TextGParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text_l: TextLParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropWParam: crate::nodes::types::Int,
    CropHParam: crate::nodes::types::Int,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    TextGParam: crate::nodes::types::String,
    TextLParam: crate::nodes::types::String,
> ClipTextEncodeSdxl<
    ClipParam,
    WidthParam,
    HeightParam,
    CropWParam,
    CropHParam,
    TargetWidthParam,
    TargetHeightParam,
    TextGParam,
    TextLParam,
> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        width: WidthParam,
        height: HeightParam,
        crop_w: CropWParam,
        crop_h: CropHParam,
        target_width: TargetWidthParam,
        target_height: TargetHeightParam,
        text_g: TextGParam,
        text_l: TextLParam,
    ) -> Self {
        Self {
            clip,
            width,
            height,
            crop_w,
            crop_h,
            target_width,
            target_height,
            text_g,
            text_l,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    CropWParam: crate::nodes::types::Int,
    CropHParam: crate::nodes::types::Int,
    TargetWidthParam: crate::nodes::types::Int,
    TargetHeightParam: crate::nodes::types::Int,
    TextGParam: crate::nodes::types::String,
    TextLParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for ClipTextEncodeSdxl<
    ClipParam,
    WidthParam,
    HeightParam,
    CropWParam,
    CropHParam,
    TargetWidthParam,
    TargetHeightParam,
    TextGParam,
    TextLParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("width".to_string(), self.width.clone().into());
        output.insert("height".to_string(), self.height.clone().into());
        output.insert("crop_w".to_string(), self.crop_w.clone().into());
        output.insert("crop_h".to_string(), self.crop_h.clone().into());
        output.insert("target_width".to_string(), self.target_width.clone().into());
        output.insert("target_height".to_string(), self.target_height.clone().into());
        output.insert("text_g".to_string(), self.text_g.clone().into());
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
    AscoreParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> {
    /**No documentation.

**Metadata**:
  - Default: 6
  - Max: 1000
  - Min: 0
  - Step: 0.01
*/
    pub ascore: AscoreParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub width: WidthParam,
    /**No documentation.

**Metadata**:
  - Default: 1024
  - Max: 16384
  - Min: 0
*/
    pub height: HeightParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub text: TextParam,
    ///No documentation.
    pub clip: ClipParam,
}
impl<
    AscoreParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> ClipTextEncodeSdxlRefiner<AscoreParam, WidthParam, HeightParam, TextParam, ClipParam> {
    /// Create a new node.
    pub fn new(
        ascore: AscoreParam,
        width: WidthParam,
        height: HeightParam,
        text: TextParam,
        clip: ClipParam,
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
    AscoreParam: crate::nodes::types::Float,
    WidthParam: crate::nodes::types::Int,
    HeightParam: crate::nodes::types::Int,
    TextParam: crate::nodes::types::String,
    ClipParam: crate::nodes::types::Clip,
> crate::nodes::TypedNode
for ClipTextEncodeSdxlRefiner<
    AscoreParam,
    WidthParam,
    HeightParam,
    TextParam,
    ClipParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
    ConditioningParam: crate::nodes::types::Conditioning,
    StartParam: crate::nodes::types::Float,
    EndParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub conditioning: ConditioningParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub start: StartParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.001
*/
    pub end: EndParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    StartParam: crate::nodes::types::Float,
    EndParam: crate::nodes::types::Float,
> ConditioningSetTimestepRange<ConditioningParam, StartParam, EndParam> {
    /// Create a new node.
    pub fn new(
        conditioning: ConditioningParam,
        start: StartParam,
        end: EndParam,
    ) -> Self {
        Self { conditioning, start, end }
    }
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
    StartParam: crate::nodes::types::Float,
    EndParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ConditioningSetTimestepRange<ConditioningParam, StartParam, EndParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
pub struct ConditioningZeroOut<ConditioningParam: crate::nodes::types::Conditioning> {
    ///No documentation.
    pub conditioning: ConditioningParam,
}
impl<
    ConditioningParam: crate::nodes::types::Conditioning,
> ConditioningZeroOut<ConditioningParam> {
    /// Create a new node.
    pub fn new(conditioning: ConditioningParam) -> Self {
        Self { conditioning }
    }
}
impl<ConditioningParam: crate::nodes::types::Conditioning> crate::nodes::TypedNode
for ConditioningZeroOut<ConditioningParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
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
///**TextEncodeHunyuanVideo_ImageToVideo**: No description.
#[derive(Clone)]
pub struct TextEncodeHunyuanVideoImageToVideo<
    ClipParam: crate::nodes::types::Clip,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    PromptParam: crate::nodes::types::String,
    ImageInterleaveParam: crate::nodes::types::Int,
> {
    ///No documentation.
    pub clip: ClipParam,
    ///No documentation.
    pub clip_vision_output: ClipVisionOutputParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**How much the image influences things vs the text prompt. Higher number means more influence from the text prompt.

**Metadata**:
  - Default: 2
  - Max: 512
  - Min: 1
*/
    pub image_interleave: ImageInterleaveParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    PromptParam: crate::nodes::types::String,
    ImageInterleaveParam: crate::nodes::types::Int,
> TextEncodeHunyuanVideoImageToVideo<
    ClipParam,
    ClipVisionOutputParam,
    PromptParam,
    ImageInterleaveParam,
> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        clip_vision_output: ClipVisionOutputParam,
        prompt: PromptParam,
        image_interleave: ImageInterleaveParam,
    ) -> Self {
        Self {
            clip,
            clip_vision_output,
            prompt,
            image_interleave,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    ClipVisionOutputParam: crate::nodes::types::ClipVisionOutput,
    PromptParam: crate::nodes::types::String,
    ImageInterleaveParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for TextEncodeHunyuanVideoImageToVideo<
    ClipParam,
    ClipVisionOutputParam,
    PromptParam,
    ImageInterleaveParam,
> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output
            .insert(
                "clip_vision_output".to_string(),
                self.clip_vision_output.clone().into(),
            );
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert(
                "image_interleave".to_string(),
                self.image_interleave.clone().into(),
            );
        output
    }
    const NAME: &'static str = "TextEncodeHunyuanVideo_ImageToVideo";
    const DISPLAY_NAME: &'static str = "TextEncodeHunyuanVideo_ImageToVideo";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/conditioning";
}
