//!`photomaker` definitions/categories.
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
///**PhotoMakerEncode**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PhotoMakerEncode<
    PhotomakerParam: crate::nodes::types::Photomaker,
    ImageParam: crate::nodes::types::Image,
    ClipParam: crate::nodes::types::Clip,
    TextParam: crate::nodes::types::String,
> {
    ///No documentation.
    pub photomaker: PhotomakerParam,
    ///No documentation.
    pub image: ImageParam,
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
  - Default: photograph of photomaker
*/
    pub text: TextParam,
}
impl<
    PhotomakerParam: crate::nodes::types::Photomaker,
    ImageParam: crate::nodes::types::Image,
    ClipParam: crate::nodes::types::Clip,
    TextParam: crate::nodes::types::String,
> PhotoMakerEncode<PhotomakerParam, ImageParam, ClipParam, TextParam> {
    /// Create a new node.
    pub fn new(
        photomaker: PhotomakerParam,
        image: ImageParam,
        clip: ClipParam,
        text: TextParam,
    ) -> Self {
        Self {
            photomaker,
            image,
            clip,
            text,
        }
    }
}
impl<
    PhotomakerParam: crate::nodes::types::Photomaker,
    ImageParam: crate::nodes::types::Image,
    ClipParam: crate::nodes::types::Clip,
    TextParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for PhotoMakerEncode<PhotomakerParam, ImageParam, ClipParam, TextParam> {
    type Output = crate::nodes::types::ConditioningOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("photomaker".to_string(), self.photomaker.clone().into());
        output.insert("image".to_string(), self.image.clone().into());
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("text".to_string(), self.text.clone().into());
        output
    }
    const NAME: &'static str = "PhotoMakerEncode";
    const DISPLAY_NAME: &'static str = "PhotoMakerEncode";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/photomaker";
}
///**PhotoMakerLoader**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PhotoMakerLoader {}
impl PhotoMakerLoader {
    /// Create a new node.
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::nodes::TypedNode for PhotoMakerLoader {
    type Output = crate::nodes::types::PhotomakerOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        HashMap::default()
    }
    const NAME: &'static str = "PhotoMakerLoader";
    const DISPLAY_NAME: &'static str = "PhotoMakerLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/photomaker";
}
