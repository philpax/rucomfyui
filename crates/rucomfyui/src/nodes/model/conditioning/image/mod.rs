//!`image` definitions/categories.
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
    ///Output for [`HiDreamO1ReferenceImages`](super::HiDreamO1ReferenceImages).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct HiDreamO1ReferenceImagesOutput {
        ///No documentation.
        pub positive: crate::nodes::types::ConditioningOut,
        ///No documentation.
        pub negative: crate::nodes::types::ConditioningOut,
    }
}
///**HiDream-O1 Reference Images**: Attach 1-10 reference images to conditioning, one for edit instructionor multiple for subject-driven personalization.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct HiDreamO1ReferenceImages<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
> {
    ///No documentation.
    pub positive: PositiveParam,
    ///No documentation.
    pub negative: NegativeParam,
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
> HiDreamO1ReferenceImages<PositiveParam, NegativeParam> {
    /// Create a new node.
    pub fn new(positive: PositiveParam, negative: NegativeParam) -> Self {
        Self { positive, negative }
    }
}
impl<
    PositiveParam: crate::nodes::types::Conditioning,
    NegativeParam: crate::nodes::types::Conditioning,
> crate::nodes::TypedNode for HiDreamO1ReferenceImages<PositiveParam, NegativeParam> {
    type Output = out::HiDreamO1ReferenceImagesOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            positive: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 0u32),
            negative: crate::nodes::types::ConditioningOut::from_dynamic(node_id, 1u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("positive".to_string(), self.positive.clone().into());
        output.insert("negative".to_string(), self.negative.clone().into());
        output
    }
    const NAME: &'static str = "HiDreamO1ReferenceImages";
    const DISPLAY_NAME: &'static str = "HiDream-O1 Reference Images";
    const DESCRIPTION: &'static str = "Attach 1-10 reference images to conditioning, one for edit instructionor multiple for subject-driven personalization.";
    const CATEGORY: &'static str = "model/conditioning/image";
}
