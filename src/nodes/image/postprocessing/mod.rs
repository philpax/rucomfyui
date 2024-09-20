//!`postprocessing` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`ImageBlend`](super::ImageBlend).
    #[derive(Clone)]
    pub struct ImageBlendOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageBlur`](super::ImageBlur).
    #[derive(Clone)]
    pub struct ImageBlurOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageQuantize`](super::ImageQuantize).
    #[derive(Clone)]
    pub struct ImageQuantizeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`ImageSharpen`](super::ImageSharpen).
    #[derive(Clone)]
    pub struct ImageSharpenOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
    ///Output for [`Morphology`](super::Morphology).
    #[derive(Clone)]
    pub struct MorphologyOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
    }
}
///**ImageBlend**: No description.
pub struct ImageBlend<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> {
    ///No documentation.
    pub image_1: Image1,
    ///No documentation.
    pub image_2: Image2,
    ///No documentation.
    pub blend_factor: BlendFactor,
    ///No documentation.
    pub blend_mode: BlendMode,
}
impl<
    Image1: crate::nodes::types::Image,
    Image2: crate::nodes::types::Image,
    BlendFactor: crate::nodes::types::Float,
    BlendMode: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageBlend<Image1, Image2, BlendFactor, BlendMode> {
    type Output = out::ImageBlendOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image_1".to_string(), self.image_1.to_workflow_input());
        output.insert("image_2".to_string(), self.image_2.to_workflow_input());
        output.insert("blend_factor".to_string(), self.blend_factor.to_workflow_input());
        output.insert("blend_mode".to_string(), self.blend_mode.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageBlend";
    const DISPLAY_NAME: &'static str = "ImageBlend";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageBlur**: No description.
pub struct ImageBlur<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub blur_radius: BlurRadius,
    ///No documentation.
    pub sigma: Sigma,
}
impl<
    Image: crate::nodes::types::Image,
    BlurRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageBlur<Image, BlurRadius, Sigma> {
    type Output = out::ImageBlurOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("blur_radius".to_string(), self.blur_radius.to_workflow_input());
        output.insert("sigma".to_string(), self.sigma.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageBlur";
    const DISPLAY_NAME: &'static str = "ImageBlur";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageQuantize**: No description.
pub struct ImageQuantize<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub colors: Colors,
    ///No documentation.
    pub dither: Dither,
}
impl<
    Image: crate::nodes::types::Image,
    Colors: crate::nodes::types::Int,
    Dither: crate::nodes::types::String,
> crate::nodes::TypedNode for ImageQuantize<Image, Colors, Dither> {
    type Output = out::ImageQuantizeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("colors".to_string(), self.colors.to_workflow_input());
        output.insert("dither".to_string(), self.dither.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageQuantize";
    const DISPLAY_NAME: &'static str = "ImageQuantize";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageSharpen**: No description.
pub struct ImageSharpen<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub sharpen_radius: SharpenRadius,
    ///No documentation.
    pub sigma: Sigma,
    ///No documentation.
    pub alpha: Alpha,
}
impl<
    Image: crate::nodes::types::Image,
    SharpenRadius: crate::nodes::types::Int,
    Sigma: crate::nodes::types::Float,
    Alpha: crate::nodes::types::Float,
> crate::nodes::TypedNode for ImageSharpen<Image, SharpenRadius, Sigma, Alpha> {
    type Output = out::ImageSharpenOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
            .insert(
                "sharpen_radius".to_string(),
                self.sharpen_radius.to_workflow_input(),
            );
        output.insert("sigma".to_string(), self.sigma.to_workflow_input());
        output.insert("alpha".to_string(), self.alpha.to_workflow_input());
        output
    }
    const NAME: &'static str = "ImageSharpen";
    const DISPLAY_NAME: &'static str = "ImageSharpen";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
///**ImageMorphology**: No description.
pub struct Morphology<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub operation: Operation,
    ///No documentation.
    pub kernel_size: KernelSize,
}
impl<
    Image: crate::nodes::types::Image,
    Operation: crate::nodes::types::String,
    KernelSize: crate::nodes::types::Int,
> crate::nodes::TypedNode for Morphology<Image, Operation, KernelSize> {
    type Output = out::MorphologyOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("operation".to_string(), self.operation.to_workflow_input());
        output.insert("kernel_size".to_string(), self.kernel_size.to_workflow_input());
        output
    }
    const NAME: &'static str = "Morphology";
    const DISPLAY_NAME: &'static str = "ImageMorphology";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/postprocessing";
}
