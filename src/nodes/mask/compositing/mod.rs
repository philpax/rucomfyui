//!`compositing` definitions/categories.
#![allow(unused_imports)]
use std::collections::HashMap;
use crate::workflow::{WorkflowNodeId, WorkflowInput};
/// Output types for nodes.
pub mod out {
    ///Output for [`PorterDuffImageComposite`](super::PorterDuffImageComposite).
    #[derive(Clone)]
    pub struct PorterDuffImageCompositeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`SplitImageWithAlpha`](super::SplitImageWithAlpha).
    #[derive(Clone)]
    pub struct SplitImageWithAlphaOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Join Image with Alpha**: No description.
pub struct JoinImageWithAlpha<
    Image: crate::nodes::types::Image,
    Alpha: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub image: Image,
    ///No documentation.
    pub alpha: Alpha,
}
impl<
    Image: crate::nodes::types::Image,
    Alpha: crate::nodes::types::Mask,
> crate::nodes::TypedNode for JoinImageWithAlpha<Image, Alpha> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            node_id,
            node_slot: 0u32,
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output.insert("alpha".to_string(), self.alpha.to_workflow_input());
        output
    }
    const NAME: &'static str = "JoinImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Join Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Porter-Duff Image Composite**: No description.
pub struct PorterDuffImageComposite<
    Source: crate::nodes::types::Image,
    SourceAlpha: crate::nodes::types::Mask,
    Destination: crate::nodes::types::Image,
    DestinationAlpha: crate::nodes::types::Mask,
    Mode: crate::nodes::types::String,
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
impl<
    Source: crate::nodes::types::Image,
    SourceAlpha: crate::nodes::types::Mask,
    Destination: crate::nodes::types::Image,
    DestinationAlpha: crate::nodes::types::Mask,
    Mode: crate::nodes::types::String,
> crate::nodes::TypedNode
for PorterDuffImageComposite<Source, SourceAlpha, Destination, DestinationAlpha, Mode> {
    type Output = out::PorterDuffImageCompositeOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("source".to_string(), self.source.to_workflow_input());
        output.insert("source_alpha".to_string(), self.source_alpha.to_workflow_input());
        output.insert("destination".to_string(), self.destination.to_workflow_input());
        output
            .insert(
                "destination_alpha".to_string(),
                self.destination_alpha.to_workflow_input(),
            );
        output.insert("mode".to_string(), self.mode.to_workflow_input());
        output
    }
    const NAME: &'static str = "PorterDuffImageComposite";
    const DISPLAY_NAME: &'static str = "Porter-Duff Image Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Split Image with Alpha**: No description.
pub struct SplitImageWithAlpha<Image: crate::nodes::types::Image> {
    ///No documentation.
    pub image: Image,
}
impl<Image: crate::nodes::types::Image> crate::nodes::TypedNode
for SplitImageWithAlpha<Image> {
    type Output = out::SplitImageWithAlphaOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut {
                node_id,
                node_slot: 0u32,
            },
            mask: crate::nodes::types::MaskOut {
                node_id,
                node_slot: 1u32,
            },
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.to_workflow_input());
        output
    }
    const NAME: &'static str = "SplitImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Split Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
