//!`attention_experiments` definitions/categories.
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
///**CLIPAttentionMultiply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CLIPAttentionMultiply<
    ClipParam: crate::nodes::types::Clip,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub q: QParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub k: KParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub v: VParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> CLIPAttentionMultiply<ClipParam, QParam, KParam, VParam, OutParam> {
    /// Create a new node.
    pub fn new(clip: ClipParam, q: QParam, k: KParam, v: VParam, out: OutParam) -> Self {
        Self { clip, q, k, v, out }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for CLIPAttentionMultiply<ClipParam, QParam, KParam, VParam, OutParam> {
    type Output = crate::nodes::types::ClipOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("q".to_string(), self.q.clone().into());
        output.insert("k".to_string(), self.k.clone().into());
        output.insert("v".to_string(), self.v.clone().into());
        output.insert("out".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "CLIPAttentionMultiply";
    const DISPLAY_NAME: &'static str = "CLIPAttentionMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/attention_experiments";
}
///**UNetCrossAttentionMultiply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct UNetCrossAttentionMultiply<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub q: QParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub k: KParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub v: VParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> UNetCrossAttentionMultiply<ModelParam, QParam, KParam, VParam, OutParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        q: QParam,
        k: KParam,
        v: VParam,
        out: OutParam,
    ) -> Self {
        Self { model, q, k, v, out }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for UNetCrossAttentionMultiply<ModelParam, QParam, KParam, VParam, OutParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("q".to_string(), self.q.clone().into());
        output.insert("k".to_string(), self.k.clone().into());
        output.insert("v".to_string(), self.v.clone().into());
        output.insert("out".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "UNetCrossAttentionMultiply";
    const DISPLAY_NAME: &'static str = "UNetCrossAttentionMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/attention_experiments";
}
///**UNetSelfAttentionMultiply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct UNetSelfAttentionMultiply<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub q: QParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub k: KParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub v: VParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> UNetSelfAttentionMultiply<ModelParam, QParam, KParam, VParam, OutParam> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        q: QParam,
        k: KParam,
        v: VParam,
        out: OutParam,
    ) -> Self {
        Self { model, q, k, v, out }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    QParam: crate::nodes::types::Float,
    KParam: crate::nodes::types::Float,
    VParam: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for UNetSelfAttentionMultiply<ModelParam, QParam, KParam, VParam, OutParam> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output.insert("q".to_string(), self.q.clone().into());
        output.insert("k".to_string(), self.k.clone().into());
        output.insert("v".to_string(), self.v.clone().into());
        output.insert("out".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "UNetSelfAttentionMultiply";
    const DISPLAY_NAME: &'static str = "UNetSelfAttentionMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/attention_experiments";
}
///**UNetTemporalAttentionMultiply**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct UNetTemporalAttentionMultiply<
    ModelParam: crate::nodes::types::Model,
    SelfStructuralParam: crate::nodes::types::Float,
    SelfTemporalParam: crate::nodes::types::Float,
    CrossStructuralParam: crate::nodes::types::Float,
    CrossTemporalParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model: ModelParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub self_structural: SelfStructuralParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub self_temporal: SelfTemporalParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub cross_structural: CrossStructuralParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 10
  - Min: 0
  - Step: 0.01
*/
    pub cross_temporal: CrossTemporalParam,
}
impl<
    ModelParam: crate::nodes::types::Model,
    SelfStructuralParam: crate::nodes::types::Float,
    SelfTemporalParam: crate::nodes::types::Float,
    CrossStructuralParam: crate::nodes::types::Float,
    CrossTemporalParam: crate::nodes::types::Float,
> UNetTemporalAttentionMultiply<
    ModelParam,
    SelfStructuralParam,
    SelfTemporalParam,
    CrossStructuralParam,
    CrossTemporalParam,
> {
    /// Create a new node.
    pub fn new(
        model: ModelParam,
        self_structural: SelfStructuralParam,
        self_temporal: SelfTemporalParam,
        cross_structural: CrossStructuralParam,
        cross_temporal: CrossTemporalParam,
    ) -> Self {
        Self {
            model,
            self_structural,
            self_temporal,
            cross_structural,
            cross_temporal,
        }
    }
}
impl<
    ModelParam: crate::nodes::types::Model,
    SelfStructuralParam: crate::nodes::types::Float,
    SelfTemporalParam: crate::nodes::types::Float,
    CrossStructuralParam: crate::nodes::types::Float,
    CrossTemporalParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for UNetTemporalAttentionMultiply<
    ModelParam,
    SelfStructuralParam,
    SelfTemporalParam,
    CrossStructuralParam,
    CrossTemporalParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model".to_string(), self.model.clone().into());
        output
            .insert("self_structural".to_string(), self.self_structural.clone().into());
        output.insert("self_temporal".to_string(), self.self_temporal.clone().into());
        output
            .insert(
                "cross_structural".to_string(),
                self.cross_structural.clone().into(),
            );
        output.insert("cross_temporal".to_string(), self.cross_temporal.clone().into());
        output
    }
    const NAME: &'static str = "UNetTemporalAttentionMultiply";
    const DISPLAY_NAME: &'static str = "UNetTemporalAttentionMultiply";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "experimental/attention_experiments";
}
