//!`background removal` definitions/categories.
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
///**Remove Background**: Generates a foreground mask to remove the background from an image using a background removal model.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RemoveBackground<
    BgRemovalModelParam: crate::nodes::types::BackgroundRemoval,
    ImageParam: crate::nodes::types::Image,
> {
    ///Background removal model used to generate the mask
    pub bg_removal_model: BgRemovalModelParam,
    ///Input image to remove the background from
    pub image: ImageParam,
}
impl<
    BgRemovalModelParam: crate::nodes::types::BackgroundRemoval,
    ImageParam: crate::nodes::types::Image,
> RemoveBackground<BgRemovalModelParam, ImageParam> {
    /// Create a new node.
    pub fn new(bg_removal_model: BgRemovalModelParam, image: ImageParam) -> Self {
        Self { bg_removal_model, image }
    }
}
impl<
    BgRemovalModelParam: crate::nodes::types::BackgroundRemoval,
    ImageParam: crate::nodes::types::Image,
> crate::nodes::TypedNode for RemoveBackground<BgRemovalModelParam, ImageParam> {
    type Output = crate::nodes::types::MaskOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert(
                "bg_removal_model".to_string(),
                self.bg_removal_model.clone().into(),
            );
        output.insert("image".to_string(), self.image.clone().into());
        output
    }
    const NAME: &'static str = "RemoveBackground";
    const DISPLAY_NAME: &'static str = "Remove Background";
    const DESCRIPTION: &'static str = "Generates a foreground mask to remove the background from an image using a background removal model.";
    const CATEGORY: &'static str = "image/background removal";
}
