//!`compositing` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
#[derive(Clone)]
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
> JoinImageWithAlpha<Image, Alpha> {
    /// Create a new node.
    pub fn new(image: Image, alpha: Alpha) -> Self {
        Self { image, alpha }
    }
}
impl<
    Image: crate::nodes::types::Image,
    Alpha: crate::nodes::types::Mask,
> crate::nodes::TypedNode for JoinImageWithAlpha<Image, Alpha> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("alpha".to_string(), self.alpha.clone().into());
        output
    }
    const NAME: &'static str = "JoinImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Join Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Porter-Duff Image Composite**: No description.
#[derive(Clone)]
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
    /**No documentation.

**Metadata**:
  - Default: DST
*/
    pub mode: Mode,
}
impl<
    Source: crate::nodes::types::Image,
    SourceAlpha: crate::nodes::types::Mask,
    Destination: crate::nodes::types::Image,
    DestinationAlpha: crate::nodes::types::Mask,
    Mode: crate::nodes::types::String,
> PorterDuffImageComposite<Source, SourceAlpha, Destination, DestinationAlpha, Mode> {
    /// Create a new node.
    pub fn new(
        source: Source,
        source_alpha: SourceAlpha,
        destination: Destination,
        destination_alpha: DestinationAlpha,
        mode: Mode,
    ) -> Self {
        Self {
            source,
            source_alpha,
            destination,
            destination_alpha,
            mode,
        }
    }
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
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("source".to_string(), self.source.clone().into());
        output.insert("source_alpha".to_string(), self.source_alpha.clone().into());
        output.insert("destination".to_string(), self.destination.clone().into());
        output
            .insert(
                "destination_alpha".to_string(),
                self.destination_alpha.clone().into(),
            );
        output.insert("mode".to_string(), self.mode.clone().into());
        output
    }
    const NAME: &'static str = "PorterDuffImageComposite";
    const DISPLAY_NAME: &'static str = "Porter-Duff Image Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
///**Split Image with Alpha**: No description.
#[derive(Clone)]
pub struct SplitImageWithAlpha<Image: crate::nodes::types::Image> {
    ///No documentation.
    pub image: Image,
}
impl<Image: crate::nodes::types::Image> SplitImageWithAlpha<Image> {
    /// Create a new node.
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}
impl<Image: crate::nodes::types::Image> crate::nodes::TypedNode
for SplitImageWithAlpha<Image> {
    type Output = out::SplitImageWithAlphaOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            mask: crate::nodes::types::MaskOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "SplitImageWithAlpha";
    const DISPLAY_NAME: &'static str = "Split Image with Alpha";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "mask/compositing";
}
