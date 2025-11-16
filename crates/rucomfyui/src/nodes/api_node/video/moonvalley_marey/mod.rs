//!`Moonvalley Marey` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Moonvalley Marey Image to Video**: Moonvalley Marey Image to Video Node
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoonvalleyImg2VideoNode<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> {
    ///The reference image used to generate the video
    pub image: ImageParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative prompt text

**Metadata**:
  - Multiline: true
  - Default: <synthetic> <scene cut> gopro, bright, contrast, static, overexposed, vignette, artifacts, still, noise, texture, scanlines, videogame, 360 camera, VR, transition, flare, saturation, distorted, warped, wide angle, saturated, vibrant, glowing, cross dissolve, cheesy, ugly hands, mutated hands, mutant, disfigured, extra fingers, blown out, horrible, blurry, worst quality, bad, dissolve, melt, fade in, fade out, wobbly, weird, low quality, plastic, stock footage, video camera, boring
*/
    pub negative_prompt: NegativePromptParam,
    /**Guidance scale for generation control

**Metadata**:
  - Default: 4.5
  - Max: 20
  - Min: 1
  - Step: 1
*/
    pub prompt_adherence: PromptAdherenceParam,
    /**Random seed value

**Metadata**:
  - Default: 9
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Number of denoising steps

**Metadata**:
  - Default: 33
  - Max: 100
  - Min: 1
  - Step: 1
*/
    pub steps: StepsParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> MoonvalleyImg2VideoNode<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    PromptAdherenceParam,
    SeedParam,
    StepsParam,
> {
    /// Create a new node.
    pub fn new(
        image: ImageParam,
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        prompt_adherence: PromptAdherenceParam,
        seed: SeedParam,
        steps: StepsParam,
    ) -> Self {
        Self {
            image,
            prompt,
            negative_prompt,
            prompt_adherence,
            seed,
            steps,
        }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MoonvalleyImg2VideoNode<
    ImageParam,
    PromptParam,
    NegativePromptParam,
    PromptAdherenceParam,
    SeedParam,
    StepsParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output
            .insert(
                "prompt_adherence".to_string(),
                self.prompt_adherence.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output
    }
    const NAME: &'static str = "MoonvalleyImg2VideoNode";
    const DISPLAY_NAME: &'static str = "Moonvalley Marey Image to Video";
    const DESCRIPTION: &'static str = "Moonvalley Marey Image to Video Node";
    const CATEGORY: &'static str = "api node/video/Moonvalley Marey";
}
///**Moonvalley Marey Text to Video**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoonvalleyTxt2VideoNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative prompt text

**Metadata**:
  - Multiline: true
  - Default: <synthetic> <scene cut> gopro, bright, contrast, static, overexposed, vignette, artifacts, still, noise, texture, scanlines, videogame, 360 camera, VR, transition, flare, saturation, distorted, warped, wide angle, saturated, vibrant, glowing, cross dissolve, cheesy, ugly hands, mutated hands, mutant, disfigured, extra fingers, blown out, horrible, blurry, worst quality, bad, dissolve, melt, fade in, fade out, wobbly, weird, low quality, plastic, stock footage, video camera, boring
*/
    pub negative_prompt: NegativePromptParam,
    /**Guidance scale for generation control

**Metadata**:
  - Default: 4
  - Max: 20
  - Min: 1
  - Step: 1
*/
    pub prompt_adherence: PromptAdherenceParam,
    /**Random seed value

**Metadata**:
  - Default: 9
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    /**Inference steps

**Metadata**:
  - Default: 33
  - Max: 100
  - Min: 1
  - Step: 1
*/
    pub steps: StepsParam,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> MoonvalleyTxt2VideoNode<
    PromptParam,
    NegativePromptParam,
    PromptAdherenceParam,
    SeedParam,
    StepsParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        prompt_adherence: PromptAdherenceParam,
        seed: SeedParam,
        steps: StepsParam,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            prompt_adherence,
            seed,
            steps,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    PromptAdherenceParam: crate::nodes::types::Float,
    SeedParam: crate::nodes::types::Int,
    StepsParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MoonvalleyTxt2VideoNode<
    PromptParam,
    NegativePromptParam,
    PromptAdherenceParam,
    SeedParam,
    StepsParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output
            .insert(
                "prompt_adherence".to_string(),
                self.prompt_adherence.clone().into(),
            );
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        output
    }
    const NAME: &'static str = "MoonvalleyTxt2VideoNode";
    const DISPLAY_NAME: &'static str = "Moonvalley Marey Text to Video";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/video/Moonvalley Marey";
}
///**Moonvalley Marey Video to Video**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MoonvalleyVideo2VideoNode<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    VideoParam: crate::nodes::types::Video,
    StepsParam: crate::nodes::types::Int,
    MotionIntensityParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**Describes the video to generate

**Metadata**:
  - Multiline: true
*/
    pub prompt: PromptParam,
    /**Negative prompt text

**Metadata**:
  - Multiline: true
  - Default: <synthetic> <scene cut> gopro, bright, contrast, static, overexposed, vignette, artifacts, still, noise, texture, scanlines, videogame, 360 camera, VR, transition, flare, saturation, distorted, warped, wide angle, saturated, vibrant, glowing, cross dissolve, cheesy, ugly hands, mutated hands, mutant, disfigured, extra fingers, blown out, horrible, blurry, worst quality, bad, dissolve, melt, fade in, fade out, wobbly, weird, low quality, plastic, stock footage, video camera, boring
*/
    pub negative_prompt: NegativePromptParam,
    /**Random seed value

**Metadata**:
  - Default: 9
  - Display: number
  - Max: 4294967295
  - Min: 0
  - Step: 1
*/
    pub seed: SeedParam,
    ///The reference video used to generate the output video. Must be at least 5 seconds long. Videos longer than 5s will be automatically trimmed. Only MP4 format supported.
    pub video: VideoParam,
    /**Number of inference steps

**Metadata**:
  - Default: 33
  - Display: number
  - Max: 100
  - Min: 1
  - Step: 1
*/
    pub steps: StepsParam,
    /**Only used if control_type is 'Motion Transfer'

**Metadata**:
  - Default: 100
  - Max: 100
  - Min: 0
  - Step: 1
*/
    pub motion_intensity: Option<MotionIntensityParam>,
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    VideoParam: crate::nodes::types::Video,
    StepsParam: crate::nodes::types::Int,
    MotionIntensityParam: crate::nodes::types::Int,
> MoonvalleyVideo2VideoNode<
    PromptParam,
    NegativePromptParam,
    SeedParam,
    VideoParam,
    StepsParam,
    MotionIntensityParam,
> {
    /// Create a new node.
    pub fn new(
        prompt: PromptParam,
        negative_prompt: NegativePromptParam,
        seed: SeedParam,
        video: VideoParam,
        steps: StepsParam,
        motion_intensity: Option<MotionIntensityParam>,
    ) -> Self {
        Self {
            prompt,
            negative_prompt,
            seed,
            video,
            steps,
            motion_intensity,
        }
    }
}
impl<
    PromptParam: crate::nodes::types::String,
    NegativePromptParam: crate::nodes::types::String,
    SeedParam: crate::nodes::types::Int,
    VideoParam: crate::nodes::types::Video,
    StepsParam: crate::nodes::types::Int,
    MotionIntensityParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for MoonvalleyVideo2VideoNode<
    PromptParam,
    NegativePromptParam,
    SeedParam,
    VideoParam,
    StepsParam,
    MotionIntensityParam,
> {
    type Output = crate::nodes::types::VideoOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output
            .insert("negative_prompt".to_string(), self.negative_prompt.clone().into());
        output.insert("seed".to_string(), self.seed.clone().into());
        output.insert("video".to_string(), self.video.clone().into());
        output.insert("steps".to_string(), self.steps.clone().into());
        if let Some(v) = &self.motion_intensity {
            output.insert("motion_intensity".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "MoonvalleyVideo2VideoNode";
    const DISPLAY_NAME: &'static str = "Moonvalley Marey Video to Video";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "api node/video/Moonvalley Marey";
}
