//!`compositing` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`PorterDuffImageComposite`](super::PorterDuffImageComposite).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct PorterDuffImageCompositeOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
    ///Output for [`SplitImageWithAlpha`](super::SplitImageWithAlpha).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct SplitImageWithAlphaOutput {
        ///No documentation.
        pub image: crate::nodes::types::ImageOut,
        ///No documentation.
        pub mask: crate::nodes::types::MaskOut,
    }
}
///**Image Composite Masked**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ImageCompositeMasked<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask = crate::nodes::types::MaskOut,
> {
    ///No documentation.
    pub destination: DestinationParam,
    ///No documentation.
    pub source: SourceParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub x: XParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Max: 16384
  - Min: 0
  - Step: 1
*/
    pub y: YParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub resize_source: ResizeSourceParam,
    ///No documentation.
    pub mask: Option<MaskParam>,
}
impl<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> ImageCompositeMasked<
    DestinationParam,
    SourceParam,
    XParam,
    YParam,
    ResizeSourceParam,
    MaskParam,
> {
    /// Create a new node.
    pub fn new(
        destination: DestinationParam,
        source: SourceParam,
        x: XParam,
        y: YParam,
        resize_source: ResizeSourceParam,
        mask: Option<MaskParam>,
    ) -> Self {
        Self {
            destination,
            source,
            x,
            y,
            resize_source,
            mask,
        }
    }
}
impl<
    DestinationParam: crate::nodes::types::Image,
    SourceParam: crate::nodes::types::Image,
    XParam: crate::nodes::types::Int,
    YParam: crate::nodes::types::Int,
    ResizeSourceParam: crate::nodes::types::Boolean,
    MaskParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for ImageCompositeMasked<
    DestinationParam,
    SourceParam,
    XParam,
    YParam,
    ResizeSourceParam,
    MaskParam,
> {
    type Output = crate::nodes::types::ImageOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("destination".to_string(), self.destination.clone().into());
        output.insert("source".to_string(), self.source.clone().into());
        output.insert("x".to_string(), self.x.clone().into());
        output.insert("y".to_string(), self.y.clone().into());
        output.insert("resize_source".to_string(), self.resize_source.clone().into());
        if let Some(v) = &self.mask {
            output.insert("mask".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "ImageCompositeMasked";
    const DISPLAY_NAME: &'static str = "Image Composite Masked";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/compositing";
}
///**Join Image with Alpha**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
    const CATEGORY: &'static str = "image/compositing";
}
///**Porter-Duff Image Composite**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PorterDuffImageComposite<
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
> {
    ///No documentation.
    pub source: SourceParam,
    ///No documentation.
    pub source_alpha: SourceAlphaParam,
    ///No documentation.
    pub destination: DestinationParam,
    ///No documentation.
    pub destination_alpha: DestinationAlphaParam,
}
impl<
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
> PorterDuffImageComposite<
    SourceParam,
    SourceAlphaParam,
    DestinationParam,
    DestinationAlphaParam,
> {
    /// Create a new node.
    pub fn new(
        source: SourceParam,
        source_alpha: SourceAlphaParam,
        destination: DestinationParam,
        destination_alpha: DestinationAlphaParam,
    ) -> Self {
        Self {
            source,
            source_alpha,
            destination,
            destination_alpha,
        }
    }
}
impl<
    SourceParam: crate::nodes::types::Image,
    SourceAlphaParam: crate::nodes::types::Mask,
    DestinationParam: crate::nodes::types::Image,
    DestinationAlphaParam: crate::nodes::types::Mask,
> crate::nodes::TypedNode
for PorterDuffImageComposite<
    SourceParam,
    SourceAlphaParam,
    DestinationParam,
    DestinationAlphaParam,
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
        output
    }
    const NAME: &'static str = "PorterDuffImageComposite";
    const DISPLAY_NAME: &'static str = "Porter-Duff Image Composite";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "image/compositing";
}
///**Split Image with Alpha**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
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
    const CATEGORY: &'static str = "image/compositing";
}
