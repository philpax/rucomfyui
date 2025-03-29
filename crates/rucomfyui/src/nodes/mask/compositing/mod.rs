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
    ImageParam: crate::nodes::types::Image,
    AlphaParam: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub alpha: AlphaParam,
}
impl<
    ImageParam: crate::nodes::types::Image,
    AlphaParam: crate::nodes::types::Mask,
> JoinImageWithAlpha<ImageParam, AlphaParam> {
    /// Create a new node.
    pub fn new(image: ImageParam, alpha: AlphaParam) -> Self {
        Self { image, alpha }
    }
}
impl<
    ImageParam: crate::nodes::types::Image,
    AlphaParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode for JoinImageWithAlpha<ImageParam, AlphaParam> {
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
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
    ModeParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub source: SourceParam,
    ///No documentation.
    pub source_alpha: SourceAlphaParam,
    ///No documentation.
    pub destination: DestinationParam,
    ///No documentation.
    pub destination_alpha: DestinationAlphaParam,
    /**No documentation.

**Metadata**:
  - Default: DST
*/
    pub mode: ModeParam,
}
impl<
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
    ModeParam: crate::nodes::types::String,
> PorterDuffImageComposite<
    SourceParam,
    SourceAlphaParam,
    DestinationParam,
    DestinationAlphaParam,
    ModeParam,
> {
    /// Create a new node.
    pub fn new(
        source: SourceParam,
        source_alpha: SourceAlphaParam,
        destination: DestinationParam,
        destination_alpha: DestinationAlphaParam,
        mode: ModeParam,
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
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
    ModeParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for PorterDuffImageComposite<
    SourceParam,
    SourceAlphaParam,
    DestinationParam,
    DestinationAlphaParam,
    ModeParam,
> {
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
pub struct SplitImageWithAlpha<ImageParam: crate::nodes::types::Image> {
    ///No documentation.
    pub image: ImageParam,
}
impl<ImageParam: crate::nodes::types::Image> SplitImageWithAlpha<ImageParam> {
    /// Create a new node.
    pub fn new(image: ImageParam) -> Self {
        Self { image }
    }
}
impl<ImageParam: crate::nodes::types::Image> crate::nodes::TypedNode
for SplitImageWithAlpha<ImageParam> {
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
