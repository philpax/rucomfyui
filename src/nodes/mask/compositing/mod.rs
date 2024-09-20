//!compositing
#![allow(unused_imports)]
use crate::WorkflowNodeId;
///**Join Image with Alpha**
pub struct JoinImageWithAlpha<Image: crate::nodes::Image, Alpha: crate::nodes::Mask> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub alpha: Alpha,
}
///Output for [`JoinImageWithAlpha`].
#[derive(Clone)]
pub struct JoinImageWithAlphaOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
}
impl<Image: crate::nodes::Image, Alpha: crate::nodes::Mask> crate::nodes::TypedNode
for JoinImageWithAlpha<Image, Alpha> {
    type Output = JoinImageWithAlphaOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut {
                node_id,
                slot: 0u32,
            },
        }
    }
    const NAME: &'static str = "JoinImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Join Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Porter-Duff Image Composite**
pub struct PorterDuffImageComposite<
    Source: crate::nodes::Image,
    SourceAlpha: crate::nodes::Mask,
    Destination: crate::nodes::Image,
    DestinationAlpha: crate::nodes::Mask,
    Mode: crate::nodes::String,
> {
    ///No documentation.
    pub source: Source,
    ///No documentation.
    pub source_alpha: SourceAlpha,
    ///No documentation.
    pub destination: Destination,
    ///No documentation.
    pub destination_alpha: DestinationAlpha,
    ///No documentation.
    pub mode: Mode,
}
///Output for [`PorterDuffImageComposite`].
#[derive(Clone)]
pub struct PorterDuffImageCompositeOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<
    Source: crate::nodes::Image,
    SourceAlpha: crate::nodes::Mask,
    Destination: crate::nodes::Image,
    DestinationAlpha: crate::nodes::Mask,
    Mode: crate::nodes::String,
> crate::nodes::TypedNode
for PorterDuffImageComposite<Source, SourceAlpha, Destination, DestinationAlpha, Mode> {
    type Output = PorterDuffImageCompositeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut {
                node_id,
                slot: 0u32,
            },
            mask: crate::nodes::MaskOut {
                node_id,
                slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "PorterDuffImageComposite";
    const DISPLAY_NAME: &'static str = "Porter-Duff Image Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Split Image with Alpha**
pub struct SplitImageWithAlpha<Image: crate::nodes::Image> {
    ///No documentation.
    pub image: Image,
}
///Output for [`SplitImageWithAlpha`].
#[derive(Clone)]
pub struct SplitImageWithAlphaOutput {
    ///No documentation.
    pub image: crate::nodes::ImageOut,
    ///No documentation.
    pub mask: crate::nodes::MaskOut,
}
impl<Image: crate::nodes::Image> crate::nodes::TypedNode for SplitImageWithAlpha<Image> {
    type Output = SplitImageWithAlphaOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::ImageOut {
                node_id,
                slot: 0u32,
            },
            mask: crate::nodes::MaskOut {
                node_id,
                slot: 1u32,
            },
        }
    }
    const NAME: &'static str = "SplitImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Split Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
