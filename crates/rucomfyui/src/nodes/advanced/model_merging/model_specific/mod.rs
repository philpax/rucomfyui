//!`model_specific` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ModelMergeAuraflow**: No description.
#[derive(Clone)]
pub struct ModelMergeAuraflow<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    InitXLinear: crate::nodes::types::Float,
    PositionalEncoding: crate::nodes::types::Float,
    CondSeqLinear: crate::nodes::types::Float,
    RegisterTokens: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    DoubleLayers0: crate::nodes::types::Float,
    DoubleLayers1: crate::nodes::types::Float,
    DoubleLayers2: crate::nodes::types::Float,
    DoubleLayers3: crate::nodes::types::Float,
    SingleLayers0: crate::nodes::types::Float,
    SingleLayers1: crate::nodes::types::Float,
    SingleLayers2: crate::nodes::types::Float,
    SingleLayers3: crate::nodes::types::Float,
    SingleLayers4: crate::nodes::types::Float,
    SingleLayers5: crate::nodes::types::Float,
    SingleLayers6: crate::nodes::types::Float,
    SingleLayers7: crate::nodes::types::Float,
    SingleLayers8: crate::nodes::types::Float,
    SingleLayers9: crate::nodes::types::Float,
    SingleLayers10: crate::nodes::types::Float,
    SingleLayers11: crate::nodes::types::Float,
    SingleLayers12: crate::nodes::types::Float,
    SingleLayers13: crate::nodes::types::Float,
    SingleLayers14: crate::nodes::types::Float,
    SingleLayers15: crate::nodes::types::Float,
    SingleLayers16: crate::nodes::types::Float,
    SingleLayers17: crate::nodes::types::Float,
    SingleLayers18: crate::nodes::types::Float,
    SingleLayers19: crate::nodes::types::Float,
    SingleLayers20: crate::nodes::types::Float,
    SingleLayers21: crate::nodes::types::Float,
    SingleLayers22: crate::nodes::types::Float,
    SingleLayers23: crate::nodes::types::Float,
    SingleLayers24: crate::nodes::types::Float,
    SingleLayers25: crate::nodes::types::Float,
    SingleLayers26: crate::nodes::types::Float,
    SingleLayers27: crate::nodes::types::Float,
    SingleLayers28: crate::nodes::types::Float,
    SingleLayers29: crate::nodes::types::Float,
    SingleLayers30: crate::nodes::types::Float,
    SingleLayers31: crate::nodes::types::Float,
    ModF: crate::nodes::types::Float,
    FinalLinear: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub init_x_linear: InitXLinear,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub positional_encoding: PositionalEncoding,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cond_seq_linear: CondSeqLinear,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub register_tokens: RegisterTokens,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_0: DoubleLayers0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_1: DoubleLayers1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_2: DoubleLayers2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_3: DoubleLayers3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_0: SingleLayers0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_1: SingleLayers1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_2: SingleLayers2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_3: SingleLayers3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_4: SingleLayers4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_5: SingleLayers5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_6: SingleLayers6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_7: SingleLayers7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_8: SingleLayers8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_9: SingleLayers9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_10: SingleLayers10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_11: SingleLayers11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_12: SingleLayers12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_13: SingleLayers13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_14: SingleLayers14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_15: SingleLayers15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_16: SingleLayers16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_17: SingleLayers17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_18: SingleLayers18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_19: SingleLayers19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_20: SingleLayers20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_21: SingleLayers21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_22: SingleLayers22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_23: SingleLayers23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_24: SingleLayers24,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_25: SingleLayers25,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_26: SingleLayers26,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_27: SingleLayers27,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_28: SingleLayers28,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_29: SingleLayers29,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_30: SingleLayers30,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_31: SingleLayers31,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub mod_f: ModF,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_linear: FinalLinear,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    InitXLinear: crate::nodes::types::Float,
    PositionalEncoding: crate::nodes::types::Float,
    CondSeqLinear: crate::nodes::types::Float,
    RegisterTokens: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    DoubleLayers0: crate::nodes::types::Float,
    DoubleLayers1: crate::nodes::types::Float,
    DoubleLayers2: crate::nodes::types::Float,
    DoubleLayers3: crate::nodes::types::Float,
    SingleLayers0: crate::nodes::types::Float,
    SingleLayers1: crate::nodes::types::Float,
    SingleLayers2: crate::nodes::types::Float,
    SingleLayers3: crate::nodes::types::Float,
    SingleLayers4: crate::nodes::types::Float,
    SingleLayers5: crate::nodes::types::Float,
    SingleLayers6: crate::nodes::types::Float,
    SingleLayers7: crate::nodes::types::Float,
    SingleLayers8: crate::nodes::types::Float,
    SingleLayers9: crate::nodes::types::Float,
    SingleLayers10: crate::nodes::types::Float,
    SingleLayers11: crate::nodes::types::Float,
    SingleLayers12: crate::nodes::types::Float,
    SingleLayers13: crate::nodes::types::Float,
    SingleLayers14: crate::nodes::types::Float,
    SingleLayers15: crate::nodes::types::Float,
    SingleLayers16: crate::nodes::types::Float,
    SingleLayers17: crate::nodes::types::Float,
    SingleLayers18: crate::nodes::types::Float,
    SingleLayers19: crate::nodes::types::Float,
    SingleLayers20: crate::nodes::types::Float,
    SingleLayers21: crate::nodes::types::Float,
    SingleLayers22: crate::nodes::types::Float,
    SingleLayers23: crate::nodes::types::Float,
    SingleLayers24: crate::nodes::types::Float,
    SingleLayers25: crate::nodes::types::Float,
    SingleLayers26: crate::nodes::types::Float,
    SingleLayers27: crate::nodes::types::Float,
    SingleLayers28: crate::nodes::types::Float,
    SingleLayers29: crate::nodes::types::Float,
    SingleLayers30: crate::nodes::types::Float,
    SingleLayers31: crate::nodes::types::Float,
    ModF: crate::nodes::types::Float,
    FinalLinear: crate::nodes::types::Float,
> ModelMergeAuraflow<
    Model1,
    Model2,
    InitXLinear,
    PositionalEncoding,
    CondSeqLinear,
    RegisterTokens,
    TEmbedder,
    DoubleLayers0,
    DoubleLayers1,
    DoubleLayers2,
    DoubleLayers3,
    SingleLayers0,
    SingleLayers1,
    SingleLayers2,
    SingleLayers3,
    SingleLayers4,
    SingleLayers5,
    SingleLayers6,
    SingleLayers7,
    SingleLayers8,
    SingleLayers9,
    SingleLayers10,
    SingleLayers11,
    SingleLayers12,
    SingleLayers13,
    SingleLayers14,
    SingleLayers15,
    SingleLayers16,
    SingleLayers17,
    SingleLayers18,
    SingleLayers19,
    SingleLayers20,
    SingleLayers21,
    SingleLayers22,
    SingleLayers23,
    SingleLayers24,
    SingleLayers25,
    SingleLayers26,
    SingleLayers27,
    SingleLayers28,
    SingleLayers29,
    SingleLayers30,
    SingleLayers31,
    ModF,
    FinalLinear,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        init_x_linear: InitXLinear,
        positional_encoding: PositionalEncoding,
        cond_seq_linear: CondSeqLinear,
        register_tokens: RegisterTokens,
        t_embedder: TEmbedder,
        double_layers_0: DoubleLayers0,
        double_layers_1: DoubleLayers1,
        double_layers_2: DoubleLayers2,
        double_layers_3: DoubleLayers3,
        single_layers_0: SingleLayers0,
        single_layers_1: SingleLayers1,
        single_layers_2: SingleLayers2,
        single_layers_3: SingleLayers3,
        single_layers_4: SingleLayers4,
        single_layers_5: SingleLayers5,
        single_layers_6: SingleLayers6,
        single_layers_7: SingleLayers7,
        single_layers_8: SingleLayers8,
        single_layers_9: SingleLayers9,
        single_layers_10: SingleLayers10,
        single_layers_11: SingleLayers11,
        single_layers_12: SingleLayers12,
        single_layers_13: SingleLayers13,
        single_layers_14: SingleLayers14,
        single_layers_15: SingleLayers15,
        single_layers_16: SingleLayers16,
        single_layers_17: SingleLayers17,
        single_layers_18: SingleLayers18,
        single_layers_19: SingleLayers19,
        single_layers_20: SingleLayers20,
        single_layers_21: SingleLayers21,
        single_layers_22: SingleLayers22,
        single_layers_23: SingleLayers23,
        single_layers_24: SingleLayers24,
        single_layers_25: SingleLayers25,
        single_layers_26: SingleLayers26,
        single_layers_27: SingleLayers27,
        single_layers_28: SingleLayers28,
        single_layers_29: SingleLayers29,
        single_layers_30: SingleLayers30,
        single_layers_31: SingleLayers31,
        mod_f: ModF,
        final_linear: FinalLinear,
    ) -> Self {
        Self {
            model_1,
            model_2,
            init_x_linear,
            positional_encoding,
            cond_seq_linear,
            register_tokens,
            t_embedder,
            double_layers_0,
            double_layers_1,
            double_layers_2,
            double_layers_3,
            single_layers_0,
            single_layers_1,
            single_layers_2,
            single_layers_3,
            single_layers_4,
            single_layers_5,
            single_layers_6,
            single_layers_7,
            single_layers_8,
            single_layers_9,
            single_layers_10,
            single_layers_11,
            single_layers_12,
            single_layers_13,
            single_layers_14,
            single_layers_15,
            single_layers_16,
            single_layers_17,
            single_layers_18,
            single_layers_19,
            single_layers_20,
            single_layers_21,
            single_layers_22,
            single_layers_23,
            single_layers_24,
            single_layers_25,
            single_layers_26,
            single_layers_27,
            single_layers_28,
            single_layers_29,
            single_layers_30,
            single_layers_31,
            mod_f,
            final_linear,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    InitXLinear: crate::nodes::types::Float,
    PositionalEncoding: crate::nodes::types::Float,
    CondSeqLinear: crate::nodes::types::Float,
    RegisterTokens: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    DoubleLayers0: crate::nodes::types::Float,
    DoubleLayers1: crate::nodes::types::Float,
    DoubleLayers2: crate::nodes::types::Float,
    DoubleLayers3: crate::nodes::types::Float,
    SingleLayers0: crate::nodes::types::Float,
    SingleLayers1: crate::nodes::types::Float,
    SingleLayers2: crate::nodes::types::Float,
    SingleLayers3: crate::nodes::types::Float,
    SingleLayers4: crate::nodes::types::Float,
    SingleLayers5: crate::nodes::types::Float,
    SingleLayers6: crate::nodes::types::Float,
    SingleLayers7: crate::nodes::types::Float,
    SingleLayers8: crate::nodes::types::Float,
    SingleLayers9: crate::nodes::types::Float,
    SingleLayers10: crate::nodes::types::Float,
    SingleLayers11: crate::nodes::types::Float,
    SingleLayers12: crate::nodes::types::Float,
    SingleLayers13: crate::nodes::types::Float,
    SingleLayers14: crate::nodes::types::Float,
    SingleLayers15: crate::nodes::types::Float,
    SingleLayers16: crate::nodes::types::Float,
    SingleLayers17: crate::nodes::types::Float,
    SingleLayers18: crate::nodes::types::Float,
    SingleLayers19: crate::nodes::types::Float,
    SingleLayers20: crate::nodes::types::Float,
    SingleLayers21: crate::nodes::types::Float,
    SingleLayers22: crate::nodes::types::Float,
    SingleLayers23: crate::nodes::types::Float,
    SingleLayers24: crate::nodes::types::Float,
    SingleLayers25: crate::nodes::types::Float,
    SingleLayers26: crate::nodes::types::Float,
    SingleLayers27: crate::nodes::types::Float,
    SingleLayers28: crate::nodes::types::Float,
    SingleLayers29: crate::nodes::types::Float,
    SingleLayers30: crate::nodes::types::Float,
    SingleLayers31: crate::nodes::types::Float,
    ModF: crate::nodes::types::Float,
    FinalLinear: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeAuraflow<
    Model1,
    Model2,
    InitXLinear,
    PositionalEncoding,
    CondSeqLinear,
    RegisterTokens,
    TEmbedder,
    DoubleLayers0,
    DoubleLayers1,
    DoubleLayers2,
    DoubleLayers3,
    SingleLayers0,
    SingleLayers1,
    SingleLayers2,
    SingleLayers3,
    SingleLayers4,
    SingleLayers5,
    SingleLayers6,
    SingleLayers7,
    SingleLayers8,
    SingleLayers9,
    SingleLayers10,
    SingleLayers11,
    SingleLayers12,
    SingleLayers13,
    SingleLayers14,
    SingleLayers15,
    SingleLayers16,
    SingleLayers17,
    SingleLayers18,
    SingleLayers19,
    SingleLayers20,
    SingleLayers21,
    SingleLayers22,
    SingleLayers23,
    SingleLayers24,
    SingleLayers25,
    SingleLayers26,
    SingleLayers27,
    SingleLayers28,
    SingleLayers29,
    SingleLayers30,
    SingleLayers31,
    ModF,
    FinalLinear,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("init_x_linear.".to_string(), self.init_x_linear.clone().into());
        output
            .insert(
                "positional_encoding".to_string(),
                self.positional_encoding.clone().into(),
            );
        output
            .insert("cond_seq_linear.".to_string(), self.cond_seq_linear.clone().into());
        output
            .insert("register_tokens".to_string(), self.register_tokens.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output
            .insert("double_layers.0.".to_string(), self.double_layers_0.clone().into());
        output
            .insert("double_layers.1.".to_string(), self.double_layers_1.clone().into());
        output
            .insert("double_layers.2.".to_string(), self.double_layers_2.clone().into());
        output
            .insert("double_layers.3.".to_string(), self.double_layers_3.clone().into());
        output
            .insert("single_layers.0.".to_string(), self.single_layers_0.clone().into());
        output
            .insert("single_layers.1.".to_string(), self.single_layers_1.clone().into());
        output
            .insert("single_layers.2.".to_string(), self.single_layers_2.clone().into());
        output
            .insert("single_layers.3.".to_string(), self.single_layers_3.clone().into());
        output
            .insert("single_layers.4.".to_string(), self.single_layers_4.clone().into());
        output
            .insert("single_layers.5.".to_string(), self.single_layers_5.clone().into());
        output
            .insert("single_layers.6.".to_string(), self.single_layers_6.clone().into());
        output
            .insert("single_layers.7.".to_string(), self.single_layers_7.clone().into());
        output
            .insert("single_layers.8.".to_string(), self.single_layers_8.clone().into());
        output
            .insert("single_layers.9.".to_string(), self.single_layers_9.clone().into());
        output
            .insert(
                "single_layers.10.".to_string(),
                self.single_layers_10.clone().into(),
            );
        output
            .insert(
                "single_layers.11.".to_string(),
                self.single_layers_11.clone().into(),
            );
        output
            .insert(
                "single_layers.12.".to_string(),
                self.single_layers_12.clone().into(),
            );
        output
            .insert(
                "single_layers.13.".to_string(),
                self.single_layers_13.clone().into(),
            );
        output
            .insert(
                "single_layers.14.".to_string(),
                self.single_layers_14.clone().into(),
            );
        output
            .insert(
                "single_layers.15.".to_string(),
                self.single_layers_15.clone().into(),
            );
        output
            .insert(
                "single_layers.16.".to_string(),
                self.single_layers_16.clone().into(),
            );
        output
            .insert(
                "single_layers.17.".to_string(),
                self.single_layers_17.clone().into(),
            );
        output
            .insert(
                "single_layers.18.".to_string(),
                self.single_layers_18.clone().into(),
            );
        output
            .insert(
                "single_layers.19.".to_string(),
                self.single_layers_19.clone().into(),
            );
        output
            .insert(
                "single_layers.20.".to_string(),
                self.single_layers_20.clone().into(),
            );
        output
            .insert(
                "single_layers.21.".to_string(),
                self.single_layers_21.clone().into(),
            );
        output
            .insert(
                "single_layers.22.".to_string(),
                self.single_layers_22.clone().into(),
            );
        output
            .insert(
                "single_layers.23.".to_string(),
                self.single_layers_23.clone().into(),
            );
        output
            .insert(
                "single_layers.24.".to_string(),
                self.single_layers_24.clone().into(),
            );
        output
            .insert(
                "single_layers.25.".to_string(),
                self.single_layers_25.clone().into(),
            );
        output
            .insert(
                "single_layers.26.".to_string(),
                self.single_layers_26.clone().into(),
            );
        output
            .insert(
                "single_layers.27.".to_string(),
                self.single_layers_27.clone().into(),
            );
        output
            .insert(
                "single_layers.28.".to_string(),
                self.single_layers_28.clone().into(),
            );
        output
            .insert(
                "single_layers.29.".to_string(),
                self.single_layers_29.clone().into(),
            );
        output
            .insert(
                "single_layers.30.".to_string(),
                self.single_layers_30.clone().into(),
            );
        output
            .insert(
                "single_layers.31.".to_string(),
                self.single_layers_31.clone().into(),
            );
        output.insert("modF.".to_string(), self.mod_f.clone().into());
        output.insert("final_linear.".to_string(), self.final_linear.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeAuraflow";
    const DISPLAY_NAME: &'static str = "ModelMergeAuraflow";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeFlux1**: No description.
#[derive(Clone)]
pub struct ModelMergeFlux1<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    ImgIn: crate::nodes::types::Float,
    TimeIn: crate::nodes::types::Float,
    GuidanceIn: crate::nodes::types::Float,
    VectorIn: crate::nodes::types::Float,
    TxtIn: crate::nodes::types::Float,
    DoubleBlocks0: crate::nodes::types::Float,
    DoubleBlocks1: crate::nodes::types::Float,
    DoubleBlocks2: crate::nodes::types::Float,
    DoubleBlocks3: crate::nodes::types::Float,
    DoubleBlocks4: crate::nodes::types::Float,
    DoubleBlocks5: crate::nodes::types::Float,
    DoubleBlocks6: crate::nodes::types::Float,
    DoubleBlocks7: crate::nodes::types::Float,
    DoubleBlocks8: crate::nodes::types::Float,
    DoubleBlocks9: crate::nodes::types::Float,
    DoubleBlocks10: crate::nodes::types::Float,
    DoubleBlocks11: crate::nodes::types::Float,
    DoubleBlocks12: crate::nodes::types::Float,
    DoubleBlocks13: crate::nodes::types::Float,
    DoubleBlocks14: crate::nodes::types::Float,
    DoubleBlocks15: crate::nodes::types::Float,
    DoubleBlocks16: crate::nodes::types::Float,
    DoubleBlocks17: crate::nodes::types::Float,
    DoubleBlocks18: crate::nodes::types::Float,
    SingleBlocks0: crate::nodes::types::Float,
    SingleBlocks1: crate::nodes::types::Float,
    SingleBlocks2: crate::nodes::types::Float,
    SingleBlocks3: crate::nodes::types::Float,
    SingleBlocks4: crate::nodes::types::Float,
    SingleBlocks5: crate::nodes::types::Float,
    SingleBlocks6: crate::nodes::types::Float,
    SingleBlocks7: crate::nodes::types::Float,
    SingleBlocks8: crate::nodes::types::Float,
    SingleBlocks9: crate::nodes::types::Float,
    SingleBlocks10: crate::nodes::types::Float,
    SingleBlocks11: crate::nodes::types::Float,
    SingleBlocks12: crate::nodes::types::Float,
    SingleBlocks13: crate::nodes::types::Float,
    SingleBlocks14: crate::nodes::types::Float,
    SingleBlocks15: crate::nodes::types::Float,
    SingleBlocks16: crate::nodes::types::Float,
    SingleBlocks17: crate::nodes::types::Float,
    SingleBlocks18: crate::nodes::types::Float,
    SingleBlocks19: crate::nodes::types::Float,
    SingleBlocks20: crate::nodes::types::Float,
    SingleBlocks21: crate::nodes::types::Float,
    SingleBlocks22: crate::nodes::types::Float,
    SingleBlocks23: crate::nodes::types::Float,
    SingleBlocks24: crate::nodes::types::Float,
    SingleBlocks25: crate::nodes::types::Float,
    SingleBlocks26: crate::nodes::types::Float,
    SingleBlocks27: crate::nodes::types::Float,
    SingleBlocks28: crate::nodes::types::Float,
    SingleBlocks29: crate::nodes::types::Float,
    SingleBlocks30: crate::nodes::types::Float,
    SingleBlocks31: crate::nodes::types::Float,
    SingleBlocks32: crate::nodes::types::Float,
    SingleBlocks33: crate::nodes::types::Float,
    SingleBlocks34: crate::nodes::types::Float,
    SingleBlocks35: crate::nodes::types::Float,
    SingleBlocks36: crate::nodes::types::Float,
    SingleBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub img_in: ImgIn,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_in: TimeIn,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub guidance_in: GuidanceIn,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub vector_in: VectorIn,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub txt_in: TxtIn,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_0: DoubleBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_1: DoubleBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_2: DoubleBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_3: DoubleBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_4: DoubleBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_5: DoubleBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_6: DoubleBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_7: DoubleBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_8: DoubleBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_9: DoubleBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_10: DoubleBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_11: DoubleBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_12: DoubleBlocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_13: DoubleBlocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_14: DoubleBlocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_15: DoubleBlocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_16: DoubleBlocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_17: DoubleBlocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_18: DoubleBlocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_0: SingleBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_1: SingleBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_2: SingleBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_3: SingleBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_4: SingleBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_5: SingleBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_6: SingleBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_7: SingleBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_8: SingleBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_9: SingleBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_10: SingleBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_11: SingleBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_12: SingleBlocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_13: SingleBlocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_14: SingleBlocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_15: SingleBlocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_16: SingleBlocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_17: SingleBlocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_18: SingleBlocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_19: SingleBlocks19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_20: SingleBlocks20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_21: SingleBlocks21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_22: SingleBlocks22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_23: SingleBlocks23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_24: SingleBlocks24,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_25: SingleBlocks25,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_26: SingleBlocks26,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_27: SingleBlocks27,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_28: SingleBlocks28,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_29: SingleBlocks29,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_30: SingleBlocks30,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_31: SingleBlocks31,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_32: SingleBlocks32,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_33: SingleBlocks33,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_34: SingleBlocks34,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_35: SingleBlocks35,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_36: SingleBlocks36,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_37: SingleBlocks37,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayer,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    ImgIn: crate::nodes::types::Float,
    TimeIn: crate::nodes::types::Float,
    GuidanceIn: crate::nodes::types::Float,
    VectorIn: crate::nodes::types::Float,
    TxtIn: crate::nodes::types::Float,
    DoubleBlocks0: crate::nodes::types::Float,
    DoubleBlocks1: crate::nodes::types::Float,
    DoubleBlocks2: crate::nodes::types::Float,
    DoubleBlocks3: crate::nodes::types::Float,
    DoubleBlocks4: crate::nodes::types::Float,
    DoubleBlocks5: crate::nodes::types::Float,
    DoubleBlocks6: crate::nodes::types::Float,
    DoubleBlocks7: crate::nodes::types::Float,
    DoubleBlocks8: crate::nodes::types::Float,
    DoubleBlocks9: crate::nodes::types::Float,
    DoubleBlocks10: crate::nodes::types::Float,
    DoubleBlocks11: crate::nodes::types::Float,
    DoubleBlocks12: crate::nodes::types::Float,
    DoubleBlocks13: crate::nodes::types::Float,
    DoubleBlocks14: crate::nodes::types::Float,
    DoubleBlocks15: crate::nodes::types::Float,
    DoubleBlocks16: crate::nodes::types::Float,
    DoubleBlocks17: crate::nodes::types::Float,
    DoubleBlocks18: crate::nodes::types::Float,
    SingleBlocks0: crate::nodes::types::Float,
    SingleBlocks1: crate::nodes::types::Float,
    SingleBlocks2: crate::nodes::types::Float,
    SingleBlocks3: crate::nodes::types::Float,
    SingleBlocks4: crate::nodes::types::Float,
    SingleBlocks5: crate::nodes::types::Float,
    SingleBlocks6: crate::nodes::types::Float,
    SingleBlocks7: crate::nodes::types::Float,
    SingleBlocks8: crate::nodes::types::Float,
    SingleBlocks9: crate::nodes::types::Float,
    SingleBlocks10: crate::nodes::types::Float,
    SingleBlocks11: crate::nodes::types::Float,
    SingleBlocks12: crate::nodes::types::Float,
    SingleBlocks13: crate::nodes::types::Float,
    SingleBlocks14: crate::nodes::types::Float,
    SingleBlocks15: crate::nodes::types::Float,
    SingleBlocks16: crate::nodes::types::Float,
    SingleBlocks17: crate::nodes::types::Float,
    SingleBlocks18: crate::nodes::types::Float,
    SingleBlocks19: crate::nodes::types::Float,
    SingleBlocks20: crate::nodes::types::Float,
    SingleBlocks21: crate::nodes::types::Float,
    SingleBlocks22: crate::nodes::types::Float,
    SingleBlocks23: crate::nodes::types::Float,
    SingleBlocks24: crate::nodes::types::Float,
    SingleBlocks25: crate::nodes::types::Float,
    SingleBlocks26: crate::nodes::types::Float,
    SingleBlocks27: crate::nodes::types::Float,
    SingleBlocks28: crate::nodes::types::Float,
    SingleBlocks29: crate::nodes::types::Float,
    SingleBlocks30: crate::nodes::types::Float,
    SingleBlocks31: crate::nodes::types::Float,
    SingleBlocks32: crate::nodes::types::Float,
    SingleBlocks33: crate::nodes::types::Float,
    SingleBlocks34: crate::nodes::types::Float,
    SingleBlocks35: crate::nodes::types::Float,
    SingleBlocks36: crate::nodes::types::Float,
    SingleBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> ModelMergeFlux1<
    Model1,
    Model2,
    ImgIn,
    TimeIn,
    GuidanceIn,
    VectorIn,
    TxtIn,
    DoubleBlocks0,
    DoubleBlocks1,
    DoubleBlocks2,
    DoubleBlocks3,
    DoubleBlocks4,
    DoubleBlocks5,
    DoubleBlocks6,
    DoubleBlocks7,
    DoubleBlocks8,
    DoubleBlocks9,
    DoubleBlocks10,
    DoubleBlocks11,
    DoubleBlocks12,
    DoubleBlocks13,
    DoubleBlocks14,
    DoubleBlocks15,
    DoubleBlocks16,
    DoubleBlocks17,
    DoubleBlocks18,
    SingleBlocks0,
    SingleBlocks1,
    SingleBlocks2,
    SingleBlocks3,
    SingleBlocks4,
    SingleBlocks5,
    SingleBlocks6,
    SingleBlocks7,
    SingleBlocks8,
    SingleBlocks9,
    SingleBlocks10,
    SingleBlocks11,
    SingleBlocks12,
    SingleBlocks13,
    SingleBlocks14,
    SingleBlocks15,
    SingleBlocks16,
    SingleBlocks17,
    SingleBlocks18,
    SingleBlocks19,
    SingleBlocks20,
    SingleBlocks21,
    SingleBlocks22,
    SingleBlocks23,
    SingleBlocks24,
    SingleBlocks25,
    SingleBlocks26,
    SingleBlocks27,
    SingleBlocks28,
    SingleBlocks29,
    SingleBlocks30,
    SingleBlocks31,
    SingleBlocks32,
    SingleBlocks33,
    SingleBlocks34,
    SingleBlocks35,
    SingleBlocks36,
    SingleBlocks37,
    FinalLayer,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        img_in: ImgIn,
        time_in: TimeIn,
        guidance_in: GuidanceIn,
        vector_in: VectorIn,
        txt_in: TxtIn,
        double_blocks_0: DoubleBlocks0,
        double_blocks_1: DoubleBlocks1,
        double_blocks_2: DoubleBlocks2,
        double_blocks_3: DoubleBlocks3,
        double_blocks_4: DoubleBlocks4,
        double_blocks_5: DoubleBlocks5,
        double_blocks_6: DoubleBlocks6,
        double_blocks_7: DoubleBlocks7,
        double_blocks_8: DoubleBlocks8,
        double_blocks_9: DoubleBlocks9,
        double_blocks_10: DoubleBlocks10,
        double_blocks_11: DoubleBlocks11,
        double_blocks_12: DoubleBlocks12,
        double_blocks_13: DoubleBlocks13,
        double_blocks_14: DoubleBlocks14,
        double_blocks_15: DoubleBlocks15,
        double_blocks_16: DoubleBlocks16,
        double_blocks_17: DoubleBlocks17,
        double_blocks_18: DoubleBlocks18,
        single_blocks_0: SingleBlocks0,
        single_blocks_1: SingleBlocks1,
        single_blocks_2: SingleBlocks2,
        single_blocks_3: SingleBlocks3,
        single_blocks_4: SingleBlocks4,
        single_blocks_5: SingleBlocks5,
        single_blocks_6: SingleBlocks6,
        single_blocks_7: SingleBlocks7,
        single_blocks_8: SingleBlocks8,
        single_blocks_9: SingleBlocks9,
        single_blocks_10: SingleBlocks10,
        single_blocks_11: SingleBlocks11,
        single_blocks_12: SingleBlocks12,
        single_blocks_13: SingleBlocks13,
        single_blocks_14: SingleBlocks14,
        single_blocks_15: SingleBlocks15,
        single_blocks_16: SingleBlocks16,
        single_blocks_17: SingleBlocks17,
        single_blocks_18: SingleBlocks18,
        single_blocks_19: SingleBlocks19,
        single_blocks_20: SingleBlocks20,
        single_blocks_21: SingleBlocks21,
        single_blocks_22: SingleBlocks22,
        single_blocks_23: SingleBlocks23,
        single_blocks_24: SingleBlocks24,
        single_blocks_25: SingleBlocks25,
        single_blocks_26: SingleBlocks26,
        single_blocks_27: SingleBlocks27,
        single_blocks_28: SingleBlocks28,
        single_blocks_29: SingleBlocks29,
        single_blocks_30: SingleBlocks30,
        single_blocks_31: SingleBlocks31,
        single_blocks_32: SingleBlocks32,
        single_blocks_33: SingleBlocks33,
        single_blocks_34: SingleBlocks34,
        single_blocks_35: SingleBlocks35,
        single_blocks_36: SingleBlocks36,
        single_blocks_37: SingleBlocks37,
        final_layer: FinalLayer,
    ) -> Self {
        Self {
            model_1,
            model_2,
            img_in,
            time_in,
            guidance_in,
            vector_in,
            txt_in,
            double_blocks_0,
            double_blocks_1,
            double_blocks_2,
            double_blocks_3,
            double_blocks_4,
            double_blocks_5,
            double_blocks_6,
            double_blocks_7,
            double_blocks_8,
            double_blocks_9,
            double_blocks_10,
            double_blocks_11,
            double_blocks_12,
            double_blocks_13,
            double_blocks_14,
            double_blocks_15,
            double_blocks_16,
            double_blocks_17,
            double_blocks_18,
            single_blocks_0,
            single_blocks_1,
            single_blocks_2,
            single_blocks_3,
            single_blocks_4,
            single_blocks_5,
            single_blocks_6,
            single_blocks_7,
            single_blocks_8,
            single_blocks_9,
            single_blocks_10,
            single_blocks_11,
            single_blocks_12,
            single_blocks_13,
            single_blocks_14,
            single_blocks_15,
            single_blocks_16,
            single_blocks_17,
            single_blocks_18,
            single_blocks_19,
            single_blocks_20,
            single_blocks_21,
            single_blocks_22,
            single_blocks_23,
            single_blocks_24,
            single_blocks_25,
            single_blocks_26,
            single_blocks_27,
            single_blocks_28,
            single_blocks_29,
            single_blocks_30,
            single_blocks_31,
            single_blocks_32,
            single_blocks_33,
            single_blocks_34,
            single_blocks_35,
            single_blocks_36,
            single_blocks_37,
            final_layer,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    ImgIn: crate::nodes::types::Float,
    TimeIn: crate::nodes::types::Float,
    GuidanceIn: crate::nodes::types::Float,
    VectorIn: crate::nodes::types::Float,
    TxtIn: crate::nodes::types::Float,
    DoubleBlocks0: crate::nodes::types::Float,
    DoubleBlocks1: crate::nodes::types::Float,
    DoubleBlocks2: crate::nodes::types::Float,
    DoubleBlocks3: crate::nodes::types::Float,
    DoubleBlocks4: crate::nodes::types::Float,
    DoubleBlocks5: crate::nodes::types::Float,
    DoubleBlocks6: crate::nodes::types::Float,
    DoubleBlocks7: crate::nodes::types::Float,
    DoubleBlocks8: crate::nodes::types::Float,
    DoubleBlocks9: crate::nodes::types::Float,
    DoubleBlocks10: crate::nodes::types::Float,
    DoubleBlocks11: crate::nodes::types::Float,
    DoubleBlocks12: crate::nodes::types::Float,
    DoubleBlocks13: crate::nodes::types::Float,
    DoubleBlocks14: crate::nodes::types::Float,
    DoubleBlocks15: crate::nodes::types::Float,
    DoubleBlocks16: crate::nodes::types::Float,
    DoubleBlocks17: crate::nodes::types::Float,
    DoubleBlocks18: crate::nodes::types::Float,
    SingleBlocks0: crate::nodes::types::Float,
    SingleBlocks1: crate::nodes::types::Float,
    SingleBlocks2: crate::nodes::types::Float,
    SingleBlocks3: crate::nodes::types::Float,
    SingleBlocks4: crate::nodes::types::Float,
    SingleBlocks5: crate::nodes::types::Float,
    SingleBlocks6: crate::nodes::types::Float,
    SingleBlocks7: crate::nodes::types::Float,
    SingleBlocks8: crate::nodes::types::Float,
    SingleBlocks9: crate::nodes::types::Float,
    SingleBlocks10: crate::nodes::types::Float,
    SingleBlocks11: crate::nodes::types::Float,
    SingleBlocks12: crate::nodes::types::Float,
    SingleBlocks13: crate::nodes::types::Float,
    SingleBlocks14: crate::nodes::types::Float,
    SingleBlocks15: crate::nodes::types::Float,
    SingleBlocks16: crate::nodes::types::Float,
    SingleBlocks17: crate::nodes::types::Float,
    SingleBlocks18: crate::nodes::types::Float,
    SingleBlocks19: crate::nodes::types::Float,
    SingleBlocks20: crate::nodes::types::Float,
    SingleBlocks21: crate::nodes::types::Float,
    SingleBlocks22: crate::nodes::types::Float,
    SingleBlocks23: crate::nodes::types::Float,
    SingleBlocks24: crate::nodes::types::Float,
    SingleBlocks25: crate::nodes::types::Float,
    SingleBlocks26: crate::nodes::types::Float,
    SingleBlocks27: crate::nodes::types::Float,
    SingleBlocks28: crate::nodes::types::Float,
    SingleBlocks29: crate::nodes::types::Float,
    SingleBlocks30: crate::nodes::types::Float,
    SingleBlocks31: crate::nodes::types::Float,
    SingleBlocks32: crate::nodes::types::Float,
    SingleBlocks33: crate::nodes::types::Float,
    SingleBlocks34: crate::nodes::types::Float,
    SingleBlocks35: crate::nodes::types::Float,
    SingleBlocks36: crate::nodes::types::Float,
    SingleBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeFlux1<
    Model1,
    Model2,
    ImgIn,
    TimeIn,
    GuidanceIn,
    VectorIn,
    TxtIn,
    DoubleBlocks0,
    DoubleBlocks1,
    DoubleBlocks2,
    DoubleBlocks3,
    DoubleBlocks4,
    DoubleBlocks5,
    DoubleBlocks6,
    DoubleBlocks7,
    DoubleBlocks8,
    DoubleBlocks9,
    DoubleBlocks10,
    DoubleBlocks11,
    DoubleBlocks12,
    DoubleBlocks13,
    DoubleBlocks14,
    DoubleBlocks15,
    DoubleBlocks16,
    DoubleBlocks17,
    DoubleBlocks18,
    SingleBlocks0,
    SingleBlocks1,
    SingleBlocks2,
    SingleBlocks3,
    SingleBlocks4,
    SingleBlocks5,
    SingleBlocks6,
    SingleBlocks7,
    SingleBlocks8,
    SingleBlocks9,
    SingleBlocks10,
    SingleBlocks11,
    SingleBlocks12,
    SingleBlocks13,
    SingleBlocks14,
    SingleBlocks15,
    SingleBlocks16,
    SingleBlocks17,
    SingleBlocks18,
    SingleBlocks19,
    SingleBlocks20,
    SingleBlocks21,
    SingleBlocks22,
    SingleBlocks23,
    SingleBlocks24,
    SingleBlocks25,
    SingleBlocks26,
    SingleBlocks27,
    SingleBlocks28,
    SingleBlocks29,
    SingleBlocks30,
    SingleBlocks31,
    SingleBlocks32,
    SingleBlocks33,
    SingleBlocks34,
    SingleBlocks35,
    SingleBlocks36,
    SingleBlocks37,
    FinalLayer,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("img_in.".to_string(), self.img_in.clone().into());
        output.insert("time_in.".to_string(), self.time_in.clone().into());
        output.insert("guidance_in".to_string(), self.guidance_in.clone().into());
        output.insert("vector_in.".to_string(), self.vector_in.clone().into());
        output.insert("txt_in.".to_string(), self.txt_in.clone().into());
        output
            .insert("double_blocks.0.".to_string(), self.double_blocks_0.clone().into());
        output
            .insert("double_blocks.1.".to_string(), self.double_blocks_1.clone().into());
        output
            .insert("double_blocks.2.".to_string(), self.double_blocks_2.clone().into());
        output
            .insert("double_blocks.3.".to_string(), self.double_blocks_3.clone().into());
        output
            .insert("double_blocks.4.".to_string(), self.double_blocks_4.clone().into());
        output
            .insert("double_blocks.5.".to_string(), self.double_blocks_5.clone().into());
        output
            .insert("double_blocks.6.".to_string(), self.double_blocks_6.clone().into());
        output
            .insert("double_blocks.7.".to_string(), self.double_blocks_7.clone().into());
        output
            .insert("double_blocks.8.".to_string(), self.double_blocks_8.clone().into());
        output
            .insert("double_blocks.9.".to_string(), self.double_blocks_9.clone().into());
        output
            .insert(
                "double_blocks.10.".to_string(),
                self.double_blocks_10.clone().into(),
            );
        output
            .insert(
                "double_blocks.11.".to_string(),
                self.double_blocks_11.clone().into(),
            );
        output
            .insert(
                "double_blocks.12.".to_string(),
                self.double_blocks_12.clone().into(),
            );
        output
            .insert(
                "double_blocks.13.".to_string(),
                self.double_blocks_13.clone().into(),
            );
        output
            .insert(
                "double_blocks.14.".to_string(),
                self.double_blocks_14.clone().into(),
            );
        output
            .insert(
                "double_blocks.15.".to_string(),
                self.double_blocks_15.clone().into(),
            );
        output
            .insert(
                "double_blocks.16.".to_string(),
                self.double_blocks_16.clone().into(),
            );
        output
            .insert(
                "double_blocks.17.".to_string(),
                self.double_blocks_17.clone().into(),
            );
        output
            .insert(
                "double_blocks.18.".to_string(),
                self.double_blocks_18.clone().into(),
            );
        output
            .insert("single_blocks.0.".to_string(), self.single_blocks_0.clone().into());
        output
            .insert("single_blocks.1.".to_string(), self.single_blocks_1.clone().into());
        output
            .insert("single_blocks.2.".to_string(), self.single_blocks_2.clone().into());
        output
            .insert("single_blocks.3.".to_string(), self.single_blocks_3.clone().into());
        output
            .insert("single_blocks.4.".to_string(), self.single_blocks_4.clone().into());
        output
            .insert("single_blocks.5.".to_string(), self.single_blocks_5.clone().into());
        output
            .insert("single_blocks.6.".to_string(), self.single_blocks_6.clone().into());
        output
            .insert("single_blocks.7.".to_string(), self.single_blocks_7.clone().into());
        output
            .insert("single_blocks.8.".to_string(), self.single_blocks_8.clone().into());
        output
            .insert("single_blocks.9.".to_string(), self.single_blocks_9.clone().into());
        output
            .insert(
                "single_blocks.10.".to_string(),
                self.single_blocks_10.clone().into(),
            );
        output
            .insert(
                "single_blocks.11.".to_string(),
                self.single_blocks_11.clone().into(),
            );
        output
            .insert(
                "single_blocks.12.".to_string(),
                self.single_blocks_12.clone().into(),
            );
        output
            .insert(
                "single_blocks.13.".to_string(),
                self.single_blocks_13.clone().into(),
            );
        output
            .insert(
                "single_blocks.14.".to_string(),
                self.single_blocks_14.clone().into(),
            );
        output
            .insert(
                "single_blocks.15.".to_string(),
                self.single_blocks_15.clone().into(),
            );
        output
            .insert(
                "single_blocks.16.".to_string(),
                self.single_blocks_16.clone().into(),
            );
        output
            .insert(
                "single_blocks.17.".to_string(),
                self.single_blocks_17.clone().into(),
            );
        output
            .insert(
                "single_blocks.18.".to_string(),
                self.single_blocks_18.clone().into(),
            );
        output
            .insert(
                "single_blocks.19.".to_string(),
                self.single_blocks_19.clone().into(),
            );
        output
            .insert(
                "single_blocks.20.".to_string(),
                self.single_blocks_20.clone().into(),
            );
        output
            .insert(
                "single_blocks.21.".to_string(),
                self.single_blocks_21.clone().into(),
            );
        output
            .insert(
                "single_blocks.22.".to_string(),
                self.single_blocks_22.clone().into(),
            );
        output
            .insert(
                "single_blocks.23.".to_string(),
                self.single_blocks_23.clone().into(),
            );
        output
            .insert(
                "single_blocks.24.".to_string(),
                self.single_blocks_24.clone().into(),
            );
        output
            .insert(
                "single_blocks.25.".to_string(),
                self.single_blocks_25.clone().into(),
            );
        output
            .insert(
                "single_blocks.26.".to_string(),
                self.single_blocks_26.clone().into(),
            );
        output
            .insert(
                "single_blocks.27.".to_string(),
                self.single_blocks_27.clone().into(),
            );
        output
            .insert(
                "single_blocks.28.".to_string(),
                self.single_blocks_28.clone().into(),
            );
        output
            .insert(
                "single_blocks.29.".to_string(),
                self.single_blocks_29.clone().into(),
            );
        output
            .insert(
                "single_blocks.30.".to_string(),
                self.single_blocks_30.clone().into(),
            );
        output
            .insert(
                "single_blocks.31.".to_string(),
                self.single_blocks_31.clone().into(),
            );
        output
            .insert(
                "single_blocks.32.".to_string(),
                self.single_blocks_32.clone().into(),
            );
        output
            .insert(
                "single_blocks.33.".to_string(),
                self.single_blocks_33.clone().into(),
            );
        output
            .insert(
                "single_blocks.34.".to_string(),
                self.single_blocks_34.clone().into(),
            );
        output
            .insert(
                "single_blocks.35.".to_string(),
                self.single_blocks_35.clone().into(),
            );
        output
            .insert(
                "single_blocks.36.".to_string(),
                self.single_blocks_36.clone().into(),
            );
        output
            .insert(
                "single_blocks.37.".to_string(),
                self.single_blocks_37.clone().into(),
            );
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeFlux1";
    const DISPLAY_NAME: &'static str = "ModelMergeFlux1";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeLTXV**: No description.
#[derive(Clone)]
pub struct ModelMergeLtxv<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PatchifyProj: crate::nodes::types::Float,
    AdalnSingle: crate::nodes::types::Float,
    CaptionProjection: crate::nodes::types::Float,
    TransformerBlocks0: crate::nodes::types::Float,
    TransformerBlocks1: crate::nodes::types::Float,
    TransformerBlocks2: crate::nodes::types::Float,
    TransformerBlocks3: crate::nodes::types::Float,
    TransformerBlocks4: crate::nodes::types::Float,
    TransformerBlocks5: crate::nodes::types::Float,
    TransformerBlocks6: crate::nodes::types::Float,
    TransformerBlocks7: crate::nodes::types::Float,
    TransformerBlocks8: crate::nodes::types::Float,
    TransformerBlocks9: crate::nodes::types::Float,
    TransformerBlocks10: crate::nodes::types::Float,
    TransformerBlocks11: crate::nodes::types::Float,
    TransformerBlocks12: crate::nodes::types::Float,
    TransformerBlocks13: crate::nodes::types::Float,
    TransformerBlocks14: crate::nodes::types::Float,
    TransformerBlocks15: crate::nodes::types::Float,
    TransformerBlocks16: crate::nodes::types::Float,
    TransformerBlocks17: crate::nodes::types::Float,
    TransformerBlocks18: crate::nodes::types::Float,
    TransformerBlocks19: crate::nodes::types::Float,
    TransformerBlocks20: crate::nodes::types::Float,
    TransformerBlocks21: crate::nodes::types::Float,
    TransformerBlocks22: crate::nodes::types::Float,
    TransformerBlocks23: crate::nodes::types::Float,
    TransformerBlocks24: crate::nodes::types::Float,
    TransformerBlocks25: crate::nodes::types::Float,
    TransformerBlocks26: crate::nodes::types::Float,
    TransformerBlocks27: crate::nodes::types::Float,
    ScaleShiftTable: crate::nodes::types::Float,
    ProjOut: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub patchify_proj: PatchifyProj,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub adaln_single: AdalnSingle,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub caption_projection: CaptionProjection,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_0: TransformerBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_1: TransformerBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_2: TransformerBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_3: TransformerBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_4: TransformerBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_5: TransformerBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_6: TransformerBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_7: TransformerBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_8: TransformerBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_9: TransformerBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_10: TransformerBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_11: TransformerBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_12: TransformerBlocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_13: TransformerBlocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_14: TransformerBlocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_15: TransformerBlocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_16: TransformerBlocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_17: TransformerBlocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_18: TransformerBlocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_19: TransformerBlocks19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_20: TransformerBlocks20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_21: TransformerBlocks21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_22: TransformerBlocks22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_23: TransformerBlocks23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_24: TransformerBlocks24,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_25: TransformerBlocks25,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_26: TransformerBlocks26,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_27: TransformerBlocks27,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub scale_shift_table: ScaleShiftTable,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub proj_out: ProjOut,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PatchifyProj: crate::nodes::types::Float,
    AdalnSingle: crate::nodes::types::Float,
    CaptionProjection: crate::nodes::types::Float,
    TransformerBlocks0: crate::nodes::types::Float,
    TransformerBlocks1: crate::nodes::types::Float,
    TransformerBlocks2: crate::nodes::types::Float,
    TransformerBlocks3: crate::nodes::types::Float,
    TransformerBlocks4: crate::nodes::types::Float,
    TransformerBlocks5: crate::nodes::types::Float,
    TransformerBlocks6: crate::nodes::types::Float,
    TransformerBlocks7: crate::nodes::types::Float,
    TransformerBlocks8: crate::nodes::types::Float,
    TransformerBlocks9: crate::nodes::types::Float,
    TransformerBlocks10: crate::nodes::types::Float,
    TransformerBlocks11: crate::nodes::types::Float,
    TransformerBlocks12: crate::nodes::types::Float,
    TransformerBlocks13: crate::nodes::types::Float,
    TransformerBlocks14: crate::nodes::types::Float,
    TransformerBlocks15: crate::nodes::types::Float,
    TransformerBlocks16: crate::nodes::types::Float,
    TransformerBlocks17: crate::nodes::types::Float,
    TransformerBlocks18: crate::nodes::types::Float,
    TransformerBlocks19: crate::nodes::types::Float,
    TransformerBlocks20: crate::nodes::types::Float,
    TransformerBlocks21: crate::nodes::types::Float,
    TransformerBlocks22: crate::nodes::types::Float,
    TransformerBlocks23: crate::nodes::types::Float,
    TransformerBlocks24: crate::nodes::types::Float,
    TransformerBlocks25: crate::nodes::types::Float,
    TransformerBlocks26: crate::nodes::types::Float,
    TransformerBlocks27: crate::nodes::types::Float,
    ScaleShiftTable: crate::nodes::types::Float,
    ProjOut: crate::nodes::types::Float,
> ModelMergeLtxv<
    Model1,
    Model2,
    PatchifyProj,
    AdalnSingle,
    CaptionProjection,
    TransformerBlocks0,
    TransformerBlocks1,
    TransformerBlocks2,
    TransformerBlocks3,
    TransformerBlocks4,
    TransformerBlocks5,
    TransformerBlocks6,
    TransformerBlocks7,
    TransformerBlocks8,
    TransformerBlocks9,
    TransformerBlocks10,
    TransformerBlocks11,
    TransformerBlocks12,
    TransformerBlocks13,
    TransformerBlocks14,
    TransformerBlocks15,
    TransformerBlocks16,
    TransformerBlocks17,
    TransformerBlocks18,
    TransformerBlocks19,
    TransformerBlocks20,
    TransformerBlocks21,
    TransformerBlocks22,
    TransformerBlocks23,
    TransformerBlocks24,
    TransformerBlocks25,
    TransformerBlocks26,
    TransformerBlocks27,
    ScaleShiftTable,
    ProjOut,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        patchify_proj: PatchifyProj,
        adaln_single: AdalnSingle,
        caption_projection: CaptionProjection,
        transformer_blocks_0: TransformerBlocks0,
        transformer_blocks_1: TransformerBlocks1,
        transformer_blocks_2: TransformerBlocks2,
        transformer_blocks_3: TransformerBlocks3,
        transformer_blocks_4: TransformerBlocks4,
        transformer_blocks_5: TransformerBlocks5,
        transformer_blocks_6: TransformerBlocks6,
        transformer_blocks_7: TransformerBlocks7,
        transformer_blocks_8: TransformerBlocks8,
        transformer_blocks_9: TransformerBlocks9,
        transformer_blocks_10: TransformerBlocks10,
        transformer_blocks_11: TransformerBlocks11,
        transformer_blocks_12: TransformerBlocks12,
        transformer_blocks_13: TransformerBlocks13,
        transformer_blocks_14: TransformerBlocks14,
        transformer_blocks_15: TransformerBlocks15,
        transformer_blocks_16: TransformerBlocks16,
        transformer_blocks_17: TransformerBlocks17,
        transformer_blocks_18: TransformerBlocks18,
        transformer_blocks_19: TransformerBlocks19,
        transformer_blocks_20: TransformerBlocks20,
        transformer_blocks_21: TransformerBlocks21,
        transformer_blocks_22: TransformerBlocks22,
        transformer_blocks_23: TransformerBlocks23,
        transformer_blocks_24: TransformerBlocks24,
        transformer_blocks_25: TransformerBlocks25,
        transformer_blocks_26: TransformerBlocks26,
        transformer_blocks_27: TransformerBlocks27,
        scale_shift_table: ScaleShiftTable,
        proj_out: ProjOut,
    ) -> Self {
        Self {
            model_1,
            model_2,
            patchify_proj,
            adaln_single,
            caption_projection,
            transformer_blocks_0,
            transformer_blocks_1,
            transformer_blocks_2,
            transformer_blocks_3,
            transformer_blocks_4,
            transformer_blocks_5,
            transformer_blocks_6,
            transformer_blocks_7,
            transformer_blocks_8,
            transformer_blocks_9,
            transformer_blocks_10,
            transformer_blocks_11,
            transformer_blocks_12,
            transformer_blocks_13,
            transformer_blocks_14,
            transformer_blocks_15,
            transformer_blocks_16,
            transformer_blocks_17,
            transformer_blocks_18,
            transformer_blocks_19,
            transformer_blocks_20,
            transformer_blocks_21,
            transformer_blocks_22,
            transformer_blocks_23,
            transformer_blocks_24,
            transformer_blocks_25,
            transformer_blocks_26,
            transformer_blocks_27,
            scale_shift_table,
            proj_out,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PatchifyProj: crate::nodes::types::Float,
    AdalnSingle: crate::nodes::types::Float,
    CaptionProjection: crate::nodes::types::Float,
    TransformerBlocks0: crate::nodes::types::Float,
    TransformerBlocks1: crate::nodes::types::Float,
    TransformerBlocks2: crate::nodes::types::Float,
    TransformerBlocks3: crate::nodes::types::Float,
    TransformerBlocks4: crate::nodes::types::Float,
    TransformerBlocks5: crate::nodes::types::Float,
    TransformerBlocks6: crate::nodes::types::Float,
    TransformerBlocks7: crate::nodes::types::Float,
    TransformerBlocks8: crate::nodes::types::Float,
    TransformerBlocks9: crate::nodes::types::Float,
    TransformerBlocks10: crate::nodes::types::Float,
    TransformerBlocks11: crate::nodes::types::Float,
    TransformerBlocks12: crate::nodes::types::Float,
    TransformerBlocks13: crate::nodes::types::Float,
    TransformerBlocks14: crate::nodes::types::Float,
    TransformerBlocks15: crate::nodes::types::Float,
    TransformerBlocks16: crate::nodes::types::Float,
    TransformerBlocks17: crate::nodes::types::Float,
    TransformerBlocks18: crate::nodes::types::Float,
    TransformerBlocks19: crate::nodes::types::Float,
    TransformerBlocks20: crate::nodes::types::Float,
    TransformerBlocks21: crate::nodes::types::Float,
    TransformerBlocks22: crate::nodes::types::Float,
    TransformerBlocks23: crate::nodes::types::Float,
    TransformerBlocks24: crate::nodes::types::Float,
    TransformerBlocks25: crate::nodes::types::Float,
    TransformerBlocks26: crate::nodes::types::Float,
    TransformerBlocks27: crate::nodes::types::Float,
    ScaleShiftTable: crate::nodes::types::Float,
    ProjOut: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeLtxv<
    Model1,
    Model2,
    PatchifyProj,
    AdalnSingle,
    CaptionProjection,
    TransformerBlocks0,
    TransformerBlocks1,
    TransformerBlocks2,
    TransformerBlocks3,
    TransformerBlocks4,
    TransformerBlocks5,
    TransformerBlocks6,
    TransformerBlocks7,
    TransformerBlocks8,
    TransformerBlocks9,
    TransformerBlocks10,
    TransformerBlocks11,
    TransformerBlocks12,
    TransformerBlocks13,
    TransformerBlocks14,
    TransformerBlocks15,
    TransformerBlocks16,
    TransformerBlocks17,
    TransformerBlocks18,
    TransformerBlocks19,
    TransformerBlocks20,
    TransformerBlocks21,
    TransformerBlocks22,
    TransformerBlocks23,
    TransformerBlocks24,
    TransformerBlocks25,
    TransformerBlocks26,
    TransformerBlocks27,
    ScaleShiftTable,
    ProjOut,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("patchify_proj.".to_string(), self.patchify_proj.clone().into());
        output.insert("adaln_single.".to_string(), self.adaln_single.clone().into());
        output
            .insert(
                "caption_projection.".to_string(),
                self.caption_projection.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.0.".to_string(),
                self.transformer_blocks_0.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.1.".to_string(),
                self.transformer_blocks_1.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.2.".to_string(),
                self.transformer_blocks_2.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.3.".to_string(),
                self.transformer_blocks_3.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.4.".to_string(),
                self.transformer_blocks_4.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.5.".to_string(),
                self.transformer_blocks_5.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.6.".to_string(),
                self.transformer_blocks_6.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.7.".to_string(),
                self.transformer_blocks_7.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.8.".to_string(),
                self.transformer_blocks_8.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.9.".to_string(),
                self.transformer_blocks_9.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.10.".to_string(),
                self.transformer_blocks_10.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.11.".to_string(),
                self.transformer_blocks_11.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.12.".to_string(),
                self.transformer_blocks_12.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.13.".to_string(),
                self.transformer_blocks_13.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.14.".to_string(),
                self.transformer_blocks_14.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.15.".to_string(),
                self.transformer_blocks_15.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.16.".to_string(),
                self.transformer_blocks_16.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.17.".to_string(),
                self.transformer_blocks_17.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.18.".to_string(),
                self.transformer_blocks_18.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.19.".to_string(),
                self.transformer_blocks_19.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.20.".to_string(),
                self.transformer_blocks_20.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.21.".to_string(),
                self.transformer_blocks_21.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.22.".to_string(),
                self.transformer_blocks_22.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.23.".to_string(),
                self.transformer_blocks_23.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.24.".to_string(),
                self.transformer_blocks_24.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.25.".to_string(),
                self.transformer_blocks_25.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.26.".to_string(),
                self.transformer_blocks_26.clone().into(),
            );
        output
            .insert(
                "transformer_blocks.27.".to_string(),
                self.transformer_blocks_27.clone().into(),
            );
        output
            .insert(
                "scale_shift_table".to_string(),
                self.scale_shift_table.clone().into(),
            );
        output.insert("proj_out.".to_string(), self.proj_out.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeLTXV";
    const DISPLAY_NAME: &'static str = "ModelMergeLTXV";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeMochiPreview**: No description.
#[derive(Clone)]
pub struct ModelMergeMochiPreview<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosFrequencies: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    T5YEmbedder: crate::nodes::types::Float,
    T5Yproj: crate::nodes::types::Float,
    Blocks0: crate::nodes::types::Float,
    Blocks1: crate::nodes::types::Float,
    Blocks2: crate::nodes::types::Float,
    Blocks3: crate::nodes::types::Float,
    Blocks4: crate::nodes::types::Float,
    Blocks5: crate::nodes::types::Float,
    Blocks6: crate::nodes::types::Float,
    Blocks7: crate::nodes::types::Float,
    Blocks8: crate::nodes::types::Float,
    Blocks9: crate::nodes::types::Float,
    Blocks10: crate::nodes::types::Float,
    Blocks11: crate::nodes::types::Float,
    Blocks12: crate::nodes::types::Float,
    Blocks13: crate::nodes::types::Float,
    Blocks14: crate::nodes::types::Float,
    Blocks15: crate::nodes::types::Float,
    Blocks16: crate::nodes::types::Float,
    Blocks17: crate::nodes::types::Float,
    Blocks18: crate::nodes::types::Float,
    Blocks19: crate::nodes::types::Float,
    Blocks20: crate::nodes::types::Float,
    Blocks21: crate::nodes::types::Float,
    Blocks22: crate::nodes::types::Float,
    Blocks23: crate::nodes::types::Float,
    Blocks24: crate::nodes::types::Float,
    Blocks25: crate::nodes::types::Float,
    Blocks26: crate::nodes::types::Float,
    Blocks27: crate::nodes::types::Float,
    Blocks28: crate::nodes::types::Float,
    Blocks29: crate::nodes::types::Float,
    Blocks30: crate::nodes::types::Float,
    Blocks31: crate::nodes::types::Float,
    Blocks32: crate::nodes::types::Float,
    Blocks33: crate::nodes::types::Float,
    Blocks34: crate::nodes::types::Float,
    Blocks35: crate::nodes::types::Float,
    Blocks36: crate::nodes::types::Float,
    Blocks37: crate::nodes::types::Float,
    Blocks38: crate::nodes::types::Float,
    Blocks39: crate::nodes::types::Float,
    Blocks40: crate::nodes::types::Float,
    Blocks41: crate::nodes::types::Float,
    Blocks42: crate::nodes::types::Float,
    Blocks43: crate::nodes::types::Float,
    Blocks44: crate::nodes::types::Float,
    Blocks45: crate::nodes::types::Float,
    Blocks46: crate::nodes::types::Float,
    Blocks47: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_frequencies: PosFrequencies,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_5_y_embedder: T5YEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_5_yproj: T5Yproj,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_0: Blocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_1: Blocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_2: Blocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_3: Blocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_4: Blocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_5: Blocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_6: Blocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_7: Blocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_8: Blocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_9: Blocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_10: Blocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_11: Blocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_12: Blocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_13: Blocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_14: Blocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_15: Blocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_16: Blocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_17: Blocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_18: Blocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_19: Blocks19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_20: Blocks20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_21: Blocks21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_22: Blocks22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_23: Blocks23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_24: Blocks24,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_25: Blocks25,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_26: Blocks26,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_27: Blocks27,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_28: Blocks28,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_29: Blocks29,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_30: Blocks30,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_31: Blocks31,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_32: Blocks32,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_33: Blocks33,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_34: Blocks34,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_35: Blocks35,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_36: Blocks36,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_37: Blocks37,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_38: Blocks38,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_39: Blocks39,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_40: Blocks40,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_41: Blocks41,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_42: Blocks42,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_43: Blocks43,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_44: Blocks44,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_45: Blocks45,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_46: Blocks46,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_47: Blocks47,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayer,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosFrequencies: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    T5YEmbedder: crate::nodes::types::Float,
    T5Yproj: crate::nodes::types::Float,
    Blocks0: crate::nodes::types::Float,
    Blocks1: crate::nodes::types::Float,
    Blocks2: crate::nodes::types::Float,
    Blocks3: crate::nodes::types::Float,
    Blocks4: crate::nodes::types::Float,
    Blocks5: crate::nodes::types::Float,
    Blocks6: crate::nodes::types::Float,
    Blocks7: crate::nodes::types::Float,
    Blocks8: crate::nodes::types::Float,
    Blocks9: crate::nodes::types::Float,
    Blocks10: crate::nodes::types::Float,
    Blocks11: crate::nodes::types::Float,
    Blocks12: crate::nodes::types::Float,
    Blocks13: crate::nodes::types::Float,
    Blocks14: crate::nodes::types::Float,
    Blocks15: crate::nodes::types::Float,
    Blocks16: crate::nodes::types::Float,
    Blocks17: crate::nodes::types::Float,
    Blocks18: crate::nodes::types::Float,
    Blocks19: crate::nodes::types::Float,
    Blocks20: crate::nodes::types::Float,
    Blocks21: crate::nodes::types::Float,
    Blocks22: crate::nodes::types::Float,
    Blocks23: crate::nodes::types::Float,
    Blocks24: crate::nodes::types::Float,
    Blocks25: crate::nodes::types::Float,
    Blocks26: crate::nodes::types::Float,
    Blocks27: crate::nodes::types::Float,
    Blocks28: crate::nodes::types::Float,
    Blocks29: crate::nodes::types::Float,
    Blocks30: crate::nodes::types::Float,
    Blocks31: crate::nodes::types::Float,
    Blocks32: crate::nodes::types::Float,
    Blocks33: crate::nodes::types::Float,
    Blocks34: crate::nodes::types::Float,
    Blocks35: crate::nodes::types::Float,
    Blocks36: crate::nodes::types::Float,
    Blocks37: crate::nodes::types::Float,
    Blocks38: crate::nodes::types::Float,
    Blocks39: crate::nodes::types::Float,
    Blocks40: crate::nodes::types::Float,
    Blocks41: crate::nodes::types::Float,
    Blocks42: crate::nodes::types::Float,
    Blocks43: crate::nodes::types::Float,
    Blocks44: crate::nodes::types::Float,
    Blocks45: crate::nodes::types::Float,
    Blocks46: crate::nodes::types::Float,
    Blocks47: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> ModelMergeMochiPreview<
    Model1,
    Model2,
    PosFrequencies,
    TEmbedder,
    T5YEmbedder,
    T5Yproj,
    Blocks0,
    Blocks1,
    Blocks2,
    Blocks3,
    Blocks4,
    Blocks5,
    Blocks6,
    Blocks7,
    Blocks8,
    Blocks9,
    Blocks10,
    Blocks11,
    Blocks12,
    Blocks13,
    Blocks14,
    Blocks15,
    Blocks16,
    Blocks17,
    Blocks18,
    Blocks19,
    Blocks20,
    Blocks21,
    Blocks22,
    Blocks23,
    Blocks24,
    Blocks25,
    Blocks26,
    Blocks27,
    Blocks28,
    Blocks29,
    Blocks30,
    Blocks31,
    Blocks32,
    Blocks33,
    Blocks34,
    Blocks35,
    Blocks36,
    Blocks37,
    Blocks38,
    Blocks39,
    Blocks40,
    Blocks41,
    Blocks42,
    Blocks43,
    Blocks44,
    Blocks45,
    Blocks46,
    Blocks47,
    FinalLayer,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        pos_frequencies: PosFrequencies,
        t_embedder: TEmbedder,
        t_5_y_embedder: T5YEmbedder,
        t_5_yproj: T5Yproj,
        blocks_0: Blocks0,
        blocks_1: Blocks1,
        blocks_2: Blocks2,
        blocks_3: Blocks3,
        blocks_4: Blocks4,
        blocks_5: Blocks5,
        blocks_6: Blocks6,
        blocks_7: Blocks7,
        blocks_8: Blocks8,
        blocks_9: Blocks9,
        blocks_10: Blocks10,
        blocks_11: Blocks11,
        blocks_12: Blocks12,
        blocks_13: Blocks13,
        blocks_14: Blocks14,
        blocks_15: Blocks15,
        blocks_16: Blocks16,
        blocks_17: Blocks17,
        blocks_18: Blocks18,
        blocks_19: Blocks19,
        blocks_20: Blocks20,
        blocks_21: Blocks21,
        blocks_22: Blocks22,
        blocks_23: Blocks23,
        blocks_24: Blocks24,
        blocks_25: Blocks25,
        blocks_26: Blocks26,
        blocks_27: Blocks27,
        blocks_28: Blocks28,
        blocks_29: Blocks29,
        blocks_30: Blocks30,
        blocks_31: Blocks31,
        blocks_32: Blocks32,
        blocks_33: Blocks33,
        blocks_34: Blocks34,
        blocks_35: Blocks35,
        blocks_36: Blocks36,
        blocks_37: Blocks37,
        blocks_38: Blocks38,
        blocks_39: Blocks39,
        blocks_40: Blocks40,
        blocks_41: Blocks41,
        blocks_42: Blocks42,
        blocks_43: Blocks43,
        blocks_44: Blocks44,
        blocks_45: Blocks45,
        blocks_46: Blocks46,
        blocks_47: Blocks47,
        final_layer: FinalLayer,
    ) -> Self {
        Self {
            model_1,
            model_2,
            pos_frequencies,
            t_embedder,
            t_5_y_embedder,
            t_5_yproj,
            blocks_0,
            blocks_1,
            blocks_2,
            blocks_3,
            blocks_4,
            blocks_5,
            blocks_6,
            blocks_7,
            blocks_8,
            blocks_9,
            blocks_10,
            blocks_11,
            blocks_12,
            blocks_13,
            blocks_14,
            blocks_15,
            blocks_16,
            blocks_17,
            blocks_18,
            blocks_19,
            blocks_20,
            blocks_21,
            blocks_22,
            blocks_23,
            blocks_24,
            blocks_25,
            blocks_26,
            blocks_27,
            blocks_28,
            blocks_29,
            blocks_30,
            blocks_31,
            blocks_32,
            blocks_33,
            blocks_34,
            blocks_35,
            blocks_36,
            blocks_37,
            blocks_38,
            blocks_39,
            blocks_40,
            blocks_41,
            blocks_42,
            blocks_43,
            blocks_44,
            blocks_45,
            blocks_46,
            blocks_47,
            final_layer,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosFrequencies: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    T5YEmbedder: crate::nodes::types::Float,
    T5Yproj: crate::nodes::types::Float,
    Blocks0: crate::nodes::types::Float,
    Blocks1: crate::nodes::types::Float,
    Blocks2: crate::nodes::types::Float,
    Blocks3: crate::nodes::types::Float,
    Blocks4: crate::nodes::types::Float,
    Blocks5: crate::nodes::types::Float,
    Blocks6: crate::nodes::types::Float,
    Blocks7: crate::nodes::types::Float,
    Blocks8: crate::nodes::types::Float,
    Blocks9: crate::nodes::types::Float,
    Blocks10: crate::nodes::types::Float,
    Blocks11: crate::nodes::types::Float,
    Blocks12: crate::nodes::types::Float,
    Blocks13: crate::nodes::types::Float,
    Blocks14: crate::nodes::types::Float,
    Blocks15: crate::nodes::types::Float,
    Blocks16: crate::nodes::types::Float,
    Blocks17: crate::nodes::types::Float,
    Blocks18: crate::nodes::types::Float,
    Blocks19: crate::nodes::types::Float,
    Blocks20: crate::nodes::types::Float,
    Blocks21: crate::nodes::types::Float,
    Blocks22: crate::nodes::types::Float,
    Blocks23: crate::nodes::types::Float,
    Blocks24: crate::nodes::types::Float,
    Blocks25: crate::nodes::types::Float,
    Blocks26: crate::nodes::types::Float,
    Blocks27: crate::nodes::types::Float,
    Blocks28: crate::nodes::types::Float,
    Blocks29: crate::nodes::types::Float,
    Blocks30: crate::nodes::types::Float,
    Blocks31: crate::nodes::types::Float,
    Blocks32: crate::nodes::types::Float,
    Blocks33: crate::nodes::types::Float,
    Blocks34: crate::nodes::types::Float,
    Blocks35: crate::nodes::types::Float,
    Blocks36: crate::nodes::types::Float,
    Blocks37: crate::nodes::types::Float,
    Blocks38: crate::nodes::types::Float,
    Blocks39: crate::nodes::types::Float,
    Blocks40: crate::nodes::types::Float,
    Blocks41: crate::nodes::types::Float,
    Blocks42: crate::nodes::types::Float,
    Blocks43: crate::nodes::types::Float,
    Blocks44: crate::nodes::types::Float,
    Blocks45: crate::nodes::types::Float,
    Blocks46: crate::nodes::types::Float,
    Blocks47: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeMochiPreview<
    Model1,
    Model2,
    PosFrequencies,
    TEmbedder,
    T5YEmbedder,
    T5Yproj,
    Blocks0,
    Blocks1,
    Blocks2,
    Blocks3,
    Blocks4,
    Blocks5,
    Blocks6,
    Blocks7,
    Blocks8,
    Blocks9,
    Blocks10,
    Blocks11,
    Blocks12,
    Blocks13,
    Blocks14,
    Blocks15,
    Blocks16,
    Blocks17,
    Blocks18,
    Blocks19,
    Blocks20,
    Blocks21,
    Blocks22,
    Blocks23,
    Blocks24,
    Blocks25,
    Blocks26,
    Blocks27,
    Blocks28,
    Blocks29,
    Blocks30,
    Blocks31,
    Blocks32,
    Blocks33,
    Blocks34,
    Blocks35,
    Blocks36,
    Blocks37,
    Blocks38,
    Blocks39,
    Blocks40,
    Blocks41,
    Blocks42,
    Blocks43,
    Blocks44,
    Blocks45,
    Blocks46,
    Blocks47,
    FinalLayer,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output
            .insert("pos_frequencies.".to_string(), self.pos_frequencies.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output.insert("t5_y_embedder.".to_string(), self.t_5_y_embedder.clone().into());
        output.insert("t5_yproj.".to_string(), self.t_5_yproj.clone().into());
        output.insert("blocks.0.".to_string(), self.blocks_0.clone().into());
        output.insert("blocks.1.".to_string(), self.blocks_1.clone().into());
        output.insert("blocks.2.".to_string(), self.blocks_2.clone().into());
        output.insert("blocks.3.".to_string(), self.blocks_3.clone().into());
        output.insert("blocks.4.".to_string(), self.blocks_4.clone().into());
        output.insert("blocks.5.".to_string(), self.blocks_5.clone().into());
        output.insert("blocks.6.".to_string(), self.blocks_6.clone().into());
        output.insert("blocks.7.".to_string(), self.blocks_7.clone().into());
        output.insert("blocks.8.".to_string(), self.blocks_8.clone().into());
        output.insert("blocks.9.".to_string(), self.blocks_9.clone().into());
        output.insert("blocks.10.".to_string(), self.blocks_10.clone().into());
        output.insert("blocks.11.".to_string(), self.blocks_11.clone().into());
        output.insert("blocks.12.".to_string(), self.blocks_12.clone().into());
        output.insert("blocks.13.".to_string(), self.blocks_13.clone().into());
        output.insert("blocks.14.".to_string(), self.blocks_14.clone().into());
        output.insert("blocks.15.".to_string(), self.blocks_15.clone().into());
        output.insert("blocks.16.".to_string(), self.blocks_16.clone().into());
        output.insert("blocks.17.".to_string(), self.blocks_17.clone().into());
        output.insert("blocks.18.".to_string(), self.blocks_18.clone().into());
        output.insert("blocks.19.".to_string(), self.blocks_19.clone().into());
        output.insert("blocks.20.".to_string(), self.blocks_20.clone().into());
        output.insert("blocks.21.".to_string(), self.blocks_21.clone().into());
        output.insert("blocks.22.".to_string(), self.blocks_22.clone().into());
        output.insert("blocks.23.".to_string(), self.blocks_23.clone().into());
        output.insert("blocks.24.".to_string(), self.blocks_24.clone().into());
        output.insert("blocks.25.".to_string(), self.blocks_25.clone().into());
        output.insert("blocks.26.".to_string(), self.blocks_26.clone().into());
        output.insert("blocks.27.".to_string(), self.blocks_27.clone().into());
        output.insert("blocks.28.".to_string(), self.blocks_28.clone().into());
        output.insert("blocks.29.".to_string(), self.blocks_29.clone().into());
        output.insert("blocks.30.".to_string(), self.blocks_30.clone().into());
        output.insert("blocks.31.".to_string(), self.blocks_31.clone().into());
        output.insert("blocks.32.".to_string(), self.blocks_32.clone().into());
        output.insert("blocks.33.".to_string(), self.blocks_33.clone().into());
        output.insert("blocks.34.".to_string(), self.blocks_34.clone().into());
        output.insert("blocks.35.".to_string(), self.blocks_35.clone().into());
        output.insert("blocks.36.".to_string(), self.blocks_36.clone().into());
        output.insert("blocks.37.".to_string(), self.blocks_37.clone().into());
        output.insert("blocks.38.".to_string(), self.blocks_38.clone().into());
        output.insert("blocks.39.".to_string(), self.blocks_39.clone().into());
        output.insert("blocks.40.".to_string(), self.blocks_40.clone().into());
        output.insert("blocks.41.".to_string(), self.blocks_41.clone().into());
        output.insert("blocks.42.".to_string(), self.blocks_42.clone().into());
        output.insert("blocks.43.".to_string(), self.blocks_43.clone().into());
        output.insert("blocks.44.".to_string(), self.blocks_44.clone().into());
        output.insert("blocks.45.".to_string(), self.blocks_45.clone().into());
        output.insert("blocks.46.".to_string(), self.blocks_46.clone().into());
        output.insert("blocks.47.".to_string(), self.blocks_47.clone().into());
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeMochiPreview";
    const DISPLAY_NAME: &'static str = "ModelMergeMochiPreview";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeSD1**: No description.
#[derive(Clone)]
pub struct ModelMergeSd1<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbed,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmb,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_9: InputBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_10: InputBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_11: InputBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_9: OutputBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_10: OutputBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_11: OutputBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: Out,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> ModelMergeSd1<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    InputBlocks9,
    InputBlocks10,
    InputBlocks11,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    OutputBlocks9,
    OutputBlocks10,
    OutputBlocks11,
    Out,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        time_embed: TimeEmbed,
        label_emb: LabelEmb,
        input_blocks_0: InputBlocks0,
        input_blocks_1: InputBlocks1,
        input_blocks_2: InputBlocks2,
        input_blocks_3: InputBlocks3,
        input_blocks_4: InputBlocks4,
        input_blocks_5: InputBlocks5,
        input_blocks_6: InputBlocks6,
        input_blocks_7: InputBlocks7,
        input_blocks_8: InputBlocks8,
        input_blocks_9: InputBlocks9,
        input_blocks_10: InputBlocks10,
        input_blocks_11: InputBlocks11,
        middle_block_0: MiddleBlock0,
        middle_block_1: MiddleBlock1,
        middle_block_2: MiddleBlock2,
        output_blocks_0: OutputBlocks0,
        output_blocks_1: OutputBlocks1,
        output_blocks_2: OutputBlocks2,
        output_blocks_3: OutputBlocks3,
        output_blocks_4: OutputBlocks4,
        output_blocks_5: OutputBlocks5,
        output_blocks_6: OutputBlocks6,
        output_blocks_7: OutputBlocks7,
        output_blocks_8: OutputBlocks8,
        output_blocks_9: OutputBlocks9,
        output_blocks_10: OutputBlocks10,
        output_blocks_11: OutputBlocks11,
        out: Out,
    ) -> Self {
        Self {
            model_1,
            model_2,
            time_embed,
            label_emb,
            input_blocks_0,
            input_blocks_1,
            input_blocks_2,
            input_blocks_3,
            input_blocks_4,
            input_blocks_5,
            input_blocks_6,
            input_blocks_7,
            input_blocks_8,
            input_blocks_9,
            input_blocks_10,
            input_blocks_11,
            middle_block_0,
            middle_block_1,
            middle_block_2,
            output_blocks_0,
            output_blocks_1,
            output_blocks_2,
            output_blocks_3,
            output_blocks_4,
            output_blocks_5,
            output_blocks_6,
            output_blocks_7,
            output_blocks_8,
            output_blocks_9,
            output_blocks_10,
            output_blocks_11,
            out,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSd1<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    InputBlocks9,
    InputBlocks10,
    InputBlocks11,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    OutputBlocks9,
    OutputBlocks10,
    OutputBlocks11,
    Out,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("time_embed.".to_string(), self.time_embed.clone().into());
        output.insert("label_emb.".to_string(), self.label_emb.clone().into());
        output.insert("input_blocks.0.".to_string(), self.input_blocks_0.clone().into());
        output.insert("input_blocks.1.".to_string(), self.input_blocks_1.clone().into());
        output.insert("input_blocks.2.".to_string(), self.input_blocks_2.clone().into());
        output.insert("input_blocks.3.".to_string(), self.input_blocks_3.clone().into());
        output.insert("input_blocks.4.".to_string(), self.input_blocks_4.clone().into());
        output.insert("input_blocks.5.".to_string(), self.input_blocks_5.clone().into());
        output.insert("input_blocks.6.".to_string(), self.input_blocks_6.clone().into());
        output.insert("input_blocks.7.".to_string(), self.input_blocks_7.clone().into());
        output.insert("input_blocks.8.".to_string(), self.input_blocks_8.clone().into());
        output.insert("input_blocks.9.".to_string(), self.input_blocks_9.clone().into());
        output
            .insert("input_blocks.10.".to_string(), self.input_blocks_10.clone().into());
        output
            .insert("input_blocks.11.".to_string(), self.input_blocks_11.clone().into());
        output.insert("middle_block.0.".to_string(), self.middle_block_0.clone().into());
        output.insert("middle_block.1.".to_string(), self.middle_block_1.clone().into());
        output.insert("middle_block.2.".to_string(), self.middle_block_2.clone().into());
        output
            .insert("output_blocks.0.".to_string(), self.output_blocks_0.clone().into());
        output
            .insert("output_blocks.1.".to_string(), self.output_blocks_1.clone().into());
        output
            .insert("output_blocks.2.".to_string(), self.output_blocks_2.clone().into());
        output
            .insert("output_blocks.3.".to_string(), self.output_blocks_3.clone().into());
        output
            .insert("output_blocks.4.".to_string(), self.output_blocks_4.clone().into());
        output
            .insert("output_blocks.5.".to_string(), self.output_blocks_5.clone().into());
        output
            .insert("output_blocks.6.".to_string(), self.output_blocks_6.clone().into());
        output
            .insert("output_blocks.7.".to_string(), self.output_blocks_7.clone().into());
        output
            .insert("output_blocks.8.".to_string(), self.output_blocks_8.clone().into());
        output
            .insert("output_blocks.9.".to_string(), self.output_blocks_9.clone().into());
        output
            .insert(
                "output_blocks.10.".to_string(),
                self.output_blocks_10.clone().into(),
            );
        output
            .insert(
                "output_blocks.11.".to_string(),
                self.output_blocks_11.clone().into(),
            );
        output.insert("out.".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSD1";
    const DISPLAY_NAME: &'static str = "ModelMergeSD1";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeSD2**: No description.
#[derive(Clone)]
pub struct ModelMergeSd2<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbed,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmb,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_9: InputBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_10: InputBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_11: InputBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_9: OutputBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_10: OutputBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_11: OutputBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: Out,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> ModelMergeSd2<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    InputBlocks9,
    InputBlocks10,
    InputBlocks11,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    OutputBlocks9,
    OutputBlocks10,
    OutputBlocks11,
    Out,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        time_embed: TimeEmbed,
        label_emb: LabelEmb,
        input_blocks_0: InputBlocks0,
        input_blocks_1: InputBlocks1,
        input_blocks_2: InputBlocks2,
        input_blocks_3: InputBlocks3,
        input_blocks_4: InputBlocks4,
        input_blocks_5: InputBlocks5,
        input_blocks_6: InputBlocks6,
        input_blocks_7: InputBlocks7,
        input_blocks_8: InputBlocks8,
        input_blocks_9: InputBlocks9,
        input_blocks_10: InputBlocks10,
        input_blocks_11: InputBlocks11,
        middle_block_0: MiddleBlock0,
        middle_block_1: MiddleBlock1,
        middle_block_2: MiddleBlock2,
        output_blocks_0: OutputBlocks0,
        output_blocks_1: OutputBlocks1,
        output_blocks_2: OutputBlocks2,
        output_blocks_3: OutputBlocks3,
        output_blocks_4: OutputBlocks4,
        output_blocks_5: OutputBlocks5,
        output_blocks_6: OutputBlocks6,
        output_blocks_7: OutputBlocks7,
        output_blocks_8: OutputBlocks8,
        output_blocks_9: OutputBlocks9,
        output_blocks_10: OutputBlocks10,
        output_blocks_11: OutputBlocks11,
        out: Out,
    ) -> Self {
        Self {
            model_1,
            model_2,
            time_embed,
            label_emb,
            input_blocks_0,
            input_blocks_1,
            input_blocks_2,
            input_blocks_3,
            input_blocks_4,
            input_blocks_5,
            input_blocks_6,
            input_blocks_7,
            input_blocks_8,
            input_blocks_9,
            input_blocks_10,
            input_blocks_11,
            middle_block_0,
            middle_block_1,
            middle_block_2,
            output_blocks_0,
            output_blocks_1,
            output_blocks_2,
            output_blocks_3,
            output_blocks_4,
            output_blocks_5,
            output_blocks_6,
            output_blocks_7,
            output_blocks_8,
            output_blocks_9,
            output_blocks_10,
            output_blocks_11,
            out,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    InputBlocks9: crate::nodes::types::Float,
    InputBlocks10: crate::nodes::types::Float,
    InputBlocks11: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    OutputBlocks9: crate::nodes::types::Float,
    OutputBlocks10: crate::nodes::types::Float,
    OutputBlocks11: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSd2<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    InputBlocks9,
    InputBlocks10,
    InputBlocks11,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    OutputBlocks9,
    OutputBlocks10,
    OutputBlocks11,
    Out,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("time_embed.".to_string(), self.time_embed.clone().into());
        output.insert("label_emb.".to_string(), self.label_emb.clone().into());
        output.insert("input_blocks.0.".to_string(), self.input_blocks_0.clone().into());
        output.insert("input_blocks.1.".to_string(), self.input_blocks_1.clone().into());
        output.insert("input_blocks.2.".to_string(), self.input_blocks_2.clone().into());
        output.insert("input_blocks.3.".to_string(), self.input_blocks_3.clone().into());
        output.insert("input_blocks.4.".to_string(), self.input_blocks_4.clone().into());
        output.insert("input_blocks.5.".to_string(), self.input_blocks_5.clone().into());
        output.insert("input_blocks.6.".to_string(), self.input_blocks_6.clone().into());
        output.insert("input_blocks.7.".to_string(), self.input_blocks_7.clone().into());
        output.insert("input_blocks.8.".to_string(), self.input_blocks_8.clone().into());
        output.insert("input_blocks.9.".to_string(), self.input_blocks_9.clone().into());
        output
            .insert("input_blocks.10.".to_string(), self.input_blocks_10.clone().into());
        output
            .insert("input_blocks.11.".to_string(), self.input_blocks_11.clone().into());
        output.insert("middle_block.0.".to_string(), self.middle_block_0.clone().into());
        output.insert("middle_block.1.".to_string(), self.middle_block_1.clone().into());
        output.insert("middle_block.2.".to_string(), self.middle_block_2.clone().into());
        output
            .insert("output_blocks.0.".to_string(), self.output_blocks_0.clone().into());
        output
            .insert("output_blocks.1.".to_string(), self.output_blocks_1.clone().into());
        output
            .insert("output_blocks.2.".to_string(), self.output_blocks_2.clone().into());
        output
            .insert("output_blocks.3.".to_string(), self.output_blocks_3.clone().into());
        output
            .insert("output_blocks.4.".to_string(), self.output_blocks_4.clone().into());
        output
            .insert("output_blocks.5.".to_string(), self.output_blocks_5.clone().into());
        output
            .insert("output_blocks.6.".to_string(), self.output_blocks_6.clone().into());
        output
            .insert("output_blocks.7.".to_string(), self.output_blocks_7.clone().into());
        output
            .insert("output_blocks.8.".to_string(), self.output_blocks_8.clone().into());
        output
            .insert("output_blocks.9.".to_string(), self.output_blocks_9.clone().into());
        output
            .insert(
                "output_blocks.10.".to_string(),
                self.output_blocks_10.clone().into(),
            );
        output
            .insert(
                "output_blocks.11.".to_string(),
                self.output_blocks_11.clone().into(),
            );
        output.insert("out.".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSD2";
    const DISPLAY_NAME: &'static str = "ModelMergeSD2";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeSD35_Large**: No description.
#[derive(Clone)]
pub struct ModelMergeSd35Large<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    JointBlocks24: crate::nodes::types::Float,
    JointBlocks25: crate::nodes::types::Float,
    JointBlocks26: crate::nodes::types::Float,
    JointBlocks27: crate::nodes::types::Float,
    JointBlocks28: crate::nodes::types::Float,
    JointBlocks29: crate::nodes::types::Float,
    JointBlocks30: crate::nodes::types::Float,
    JointBlocks31: crate::nodes::types::Float,
    JointBlocks32: crate::nodes::types::Float,
    JointBlocks33: crate::nodes::types::Float,
    JointBlocks34: crate::nodes::types::Float,
    JointBlocks35: crate::nodes::types::Float,
    JointBlocks36: crate::nodes::types::Float,
    JointBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embed: PosEmbed,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub context_embedder: ContextEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y_embedder: YEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_0: JointBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_1: JointBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_2: JointBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_3: JointBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_4: JointBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_5: JointBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_6: JointBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_7: JointBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_8: JointBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_9: JointBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_10: JointBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_11: JointBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_12: JointBlocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_13: JointBlocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_14: JointBlocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_15: JointBlocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_16: JointBlocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_17: JointBlocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_18: JointBlocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_19: JointBlocks19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_20: JointBlocks20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_21: JointBlocks21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_22: JointBlocks22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_23: JointBlocks23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_24: JointBlocks24,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_25: JointBlocks25,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_26: JointBlocks26,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_27: JointBlocks27,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_28: JointBlocks28,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_29: JointBlocks29,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_30: JointBlocks30,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_31: JointBlocks31,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_32: JointBlocks32,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_33: JointBlocks33,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_34: JointBlocks34,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_35: JointBlocks35,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_36: JointBlocks36,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_37: JointBlocks37,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayer,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    JointBlocks24: crate::nodes::types::Float,
    JointBlocks25: crate::nodes::types::Float,
    JointBlocks26: crate::nodes::types::Float,
    JointBlocks27: crate::nodes::types::Float,
    JointBlocks28: crate::nodes::types::Float,
    JointBlocks29: crate::nodes::types::Float,
    JointBlocks30: crate::nodes::types::Float,
    JointBlocks31: crate::nodes::types::Float,
    JointBlocks32: crate::nodes::types::Float,
    JointBlocks33: crate::nodes::types::Float,
    JointBlocks34: crate::nodes::types::Float,
    JointBlocks35: crate::nodes::types::Float,
    JointBlocks36: crate::nodes::types::Float,
    JointBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> ModelMergeSd35Large<
    Model1,
    Model2,
    PosEmbed,
    XEmbedder,
    ContextEmbedder,
    YEmbedder,
    TEmbedder,
    JointBlocks0,
    JointBlocks1,
    JointBlocks2,
    JointBlocks3,
    JointBlocks4,
    JointBlocks5,
    JointBlocks6,
    JointBlocks7,
    JointBlocks8,
    JointBlocks9,
    JointBlocks10,
    JointBlocks11,
    JointBlocks12,
    JointBlocks13,
    JointBlocks14,
    JointBlocks15,
    JointBlocks16,
    JointBlocks17,
    JointBlocks18,
    JointBlocks19,
    JointBlocks20,
    JointBlocks21,
    JointBlocks22,
    JointBlocks23,
    JointBlocks24,
    JointBlocks25,
    JointBlocks26,
    JointBlocks27,
    JointBlocks28,
    JointBlocks29,
    JointBlocks30,
    JointBlocks31,
    JointBlocks32,
    JointBlocks33,
    JointBlocks34,
    JointBlocks35,
    JointBlocks36,
    JointBlocks37,
    FinalLayer,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        pos_embed: PosEmbed,
        x_embedder: XEmbedder,
        context_embedder: ContextEmbedder,
        y_embedder: YEmbedder,
        t_embedder: TEmbedder,
        joint_blocks_0: JointBlocks0,
        joint_blocks_1: JointBlocks1,
        joint_blocks_2: JointBlocks2,
        joint_blocks_3: JointBlocks3,
        joint_blocks_4: JointBlocks4,
        joint_blocks_5: JointBlocks5,
        joint_blocks_6: JointBlocks6,
        joint_blocks_7: JointBlocks7,
        joint_blocks_8: JointBlocks8,
        joint_blocks_9: JointBlocks9,
        joint_blocks_10: JointBlocks10,
        joint_blocks_11: JointBlocks11,
        joint_blocks_12: JointBlocks12,
        joint_blocks_13: JointBlocks13,
        joint_blocks_14: JointBlocks14,
        joint_blocks_15: JointBlocks15,
        joint_blocks_16: JointBlocks16,
        joint_blocks_17: JointBlocks17,
        joint_blocks_18: JointBlocks18,
        joint_blocks_19: JointBlocks19,
        joint_blocks_20: JointBlocks20,
        joint_blocks_21: JointBlocks21,
        joint_blocks_22: JointBlocks22,
        joint_blocks_23: JointBlocks23,
        joint_blocks_24: JointBlocks24,
        joint_blocks_25: JointBlocks25,
        joint_blocks_26: JointBlocks26,
        joint_blocks_27: JointBlocks27,
        joint_blocks_28: JointBlocks28,
        joint_blocks_29: JointBlocks29,
        joint_blocks_30: JointBlocks30,
        joint_blocks_31: JointBlocks31,
        joint_blocks_32: JointBlocks32,
        joint_blocks_33: JointBlocks33,
        joint_blocks_34: JointBlocks34,
        joint_blocks_35: JointBlocks35,
        joint_blocks_36: JointBlocks36,
        joint_blocks_37: JointBlocks37,
        final_layer: FinalLayer,
    ) -> Self {
        Self {
            model_1,
            model_2,
            pos_embed,
            x_embedder,
            context_embedder,
            y_embedder,
            t_embedder,
            joint_blocks_0,
            joint_blocks_1,
            joint_blocks_2,
            joint_blocks_3,
            joint_blocks_4,
            joint_blocks_5,
            joint_blocks_6,
            joint_blocks_7,
            joint_blocks_8,
            joint_blocks_9,
            joint_blocks_10,
            joint_blocks_11,
            joint_blocks_12,
            joint_blocks_13,
            joint_blocks_14,
            joint_blocks_15,
            joint_blocks_16,
            joint_blocks_17,
            joint_blocks_18,
            joint_blocks_19,
            joint_blocks_20,
            joint_blocks_21,
            joint_blocks_22,
            joint_blocks_23,
            joint_blocks_24,
            joint_blocks_25,
            joint_blocks_26,
            joint_blocks_27,
            joint_blocks_28,
            joint_blocks_29,
            joint_blocks_30,
            joint_blocks_31,
            joint_blocks_32,
            joint_blocks_33,
            joint_blocks_34,
            joint_blocks_35,
            joint_blocks_36,
            joint_blocks_37,
            final_layer,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    JointBlocks24: crate::nodes::types::Float,
    JointBlocks25: crate::nodes::types::Float,
    JointBlocks26: crate::nodes::types::Float,
    JointBlocks27: crate::nodes::types::Float,
    JointBlocks28: crate::nodes::types::Float,
    JointBlocks29: crate::nodes::types::Float,
    JointBlocks30: crate::nodes::types::Float,
    JointBlocks31: crate::nodes::types::Float,
    JointBlocks32: crate::nodes::types::Float,
    JointBlocks33: crate::nodes::types::Float,
    JointBlocks34: crate::nodes::types::Float,
    JointBlocks35: crate::nodes::types::Float,
    JointBlocks36: crate::nodes::types::Float,
    JointBlocks37: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSd35Large<
    Model1,
    Model2,
    PosEmbed,
    XEmbedder,
    ContextEmbedder,
    YEmbedder,
    TEmbedder,
    JointBlocks0,
    JointBlocks1,
    JointBlocks2,
    JointBlocks3,
    JointBlocks4,
    JointBlocks5,
    JointBlocks6,
    JointBlocks7,
    JointBlocks8,
    JointBlocks9,
    JointBlocks10,
    JointBlocks11,
    JointBlocks12,
    JointBlocks13,
    JointBlocks14,
    JointBlocks15,
    JointBlocks16,
    JointBlocks17,
    JointBlocks18,
    JointBlocks19,
    JointBlocks20,
    JointBlocks21,
    JointBlocks22,
    JointBlocks23,
    JointBlocks24,
    JointBlocks25,
    JointBlocks26,
    JointBlocks27,
    JointBlocks28,
    JointBlocks29,
    JointBlocks30,
    JointBlocks31,
    JointBlocks32,
    JointBlocks33,
    JointBlocks34,
    JointBlocks35,
    JointBlocks36,
    JointBlocks37,
    FinalLayer,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("pos_embed.".to_string(), self.pos_embed.clone().into());
        output.insert("x_embedder.".to_string(), self.x_embedder.clone().into());
        output
            .insert(
                "context_embedder.".to_string(),
                self.context_embedder.clone().into(),
            );
        output.insert("y_embedder.".to_string(), self.y_embedder.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output.insert("joint_blocks.0.".to_string(), self.joint_blocks_0.clone().into());
        output.insert("joint_blocks.1.".to_string(), self.joint_blocks_1.clone().into());
        output.insert("joint_blocks.2.".to_string(), self.joint_blocks_2.clone().into());
        output.insert("joint_blocks.3.".to_string(), self.joint_blocks_3.clone().into());
        output.insert("joint_blocks.4.".to_string(), self.joint_blocks_4.clone().into());
        output.insert("joint_blocks.5.".to_string(), self.joint_blocks_5.clone().into());
        output.insert("joint_blocks.6.".to_string(), self.joint_blocks_6.clone().into());
        output.insert("joint_blocks.7.".to_string(), self.joint_blocks_7.clone().into());
        output.insert("joint_blocks.8.".to_string(), self.joint_blocks_8.clone().into());
        output.insert("joint_blocks.9.".to_string(), self.joint_blocks_9.clone().into());
        output
            .insert("joint_blocks.10.".to_string(), self.joint_blocks_10.clone().into());
        output
            .insert("joint_blocks.11.".to_string(), self.joint_blocks_11.clone().into());
        output
            .insert("joint_blocks.12.".to_string(), self.joint_blocks_12.clone().into());
        output
            .insert("joint_blocks.13.".to_string(), self.joint_blocks_13.clone().into());
        output
            .insert("joint_blocks.14.".to_string(), self.joint_blocks_14.clone().into());
        output
            .insert("joint_blocks.15.".to_string(), self.joint_blocks_15.clone().into());
        output
            .insert("joint_blocks.16.".to_string(), self.joint_blocks_16.clone().into());
        output
            .insert("joint_blocks.17.".to_string(), self.joint_blocks_17.clone().into());
        output
            .insert("joint_blocks.18.".to_string(), self.joint_blocks_18.clone().into());
        output
            .insert("joint_blocks.19.".to_string(), self.joint_blocks_19.clone().into());
        output
            .insert("joint_blocks.20.".to_string(), self.joint_blocks_20.clone().into());
        output
            .insert("joint_blocks.21.".to_string(), self.joint_blocks_21.clone().into());
        output
            .insert("joint_blocks.22.".to_string(), self.joint_blocks_22.clone().into());
        output
            .insert("joint_blocks.23.".to_string(), self.joint_blocks_23.clone().into());
        output
            .insert("joint_blocks.24.".to_string(), self.joint_blocks_24.clone().into());
        output
            .insert("joint_blocks.25.".to_string(), self.joint_blocks_25.clone().into());
        output
            .insert("joint_blocks.26.".to_string(), self.joint_blocks_26.clone().into());
        output
            .insert("joint_blocks.27.".to_string(), self.joint_blocks_27.clone().into());
        output
            .insert("joint_blocks.28.".to_string(), self.joint_blocks_28.clone().into());
        output
            .insert("joint_blocks.29.".to_string(), self.joint_blocks_29.clone().into());
        output
            .insert("joint_blocks.30.".to_string(), self.joint_blocks_30.clone().into());
        output
            .insert("joint_blocks.31.".to_string(), self.joint_blocks_31.clone().into());
        output
            .insert("joint_blocks.32.".to_string(), self.joint_blocks_32.clone().into());
        output
            .insert("joint_blocks.33.".to_string(), self.joint_blocks_33.clone().into());
        output
            .insert("joint_blocks.34.".to_string(), self.joint_blocks_34.clone().into());
        output
            .insert("joint_blocks.35.".to_string(), self.joint_blocks_35.clone().into());
        output
            .insert("joint_blocks.36.".to_string(), self.joint_blocks_36.clone().into());
        output
            .insert("joint_blocks.37.".to_string(), self.joint_blocks_37.clone().into());
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSD35_Large";
    const DISPLAY_NAME: &'static str = "ModelMergeSD35_Large";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeSD3_2B**: No description.
#[derive(Clone)]
pub struct ModelMergeSd32B<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embed: PosEmbed,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub context_embedder: ContextEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y_embedder: YEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedder,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_0: JointBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_1: JointBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_2: JointBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_3: JointBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_4: JointBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_5: JointBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_6: JointBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_7: JointBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_8: JointBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_9: JointBlocks9,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_10: JointBlocks10,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_11: JointBlocks11,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_12: JointBlocks12,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_13: JointBlocks13,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_14: JointBlocks14,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_15: JointBlocks15,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_16: JointBlocks16,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_17: JointBlocks17,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_18: JointBlocks18,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_19: JointBlocks19,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_20: JointBlocks20,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_21: JointBlocks21,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_22: JointBlocks22,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_23: JointBlocks23,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayer,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> ModelMergeSd32B<
    Model1,
    Model2,
    PosEmbed,
    XEmbedder,
    ContextEmbedder,
    YEmbedder,
    TEmbedder,
    JointBlocks0,
    JointBlocks1,
    JointBlocks2,
    JointBlocks3,
    JointBlocks4,
    JointBlocks5,
    JointBlocks6,
    JointBlocks7,
    JointBlocks8,
    JointBlocks9,
    JointBlocks10,
    JointBlocks11,
    JointBlocks12,
    JointBlocks13,
    JointBlocks14,
    JointBlocks15,
    JointBlocks16,
    JointBlocks17,
    JointBlocks18,
    JointBlocks19,
    JointBlocks20,
    JointBlocks21,
    JointBlocks22,
    JointBlocks23,
    FinalLayer,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        pos_embed: PosEmbed,
        x_embedder: XEmbedder,
        context_embedder: ContextEmbedder,
        y_embedder: YEmbedder,
        t_embedder: TEmbedder,
        joint_blocks_0: JointBlocks0,
        joint_blocks_1: JointBlocks1,
        joint_blocks_2: JointBlocks2,
        joint_blocks_3: JointBlocks3,
        joint_blocks_4: JointBlocks4,
        joint_blocks_5: JointBlocks5,
        joint_blocks_6: JointBlocks6,
        joint_blocks_7: JointBlocks7,
        joint_blocks_8: JointBlocks8,
        joint_blocks_9: JointBlocks9,
        joint_blocks_10: JointBlocks10,
        joint_blocks_11: JointBlocks11,
        joint_blocks_12: JointBlocks12,
        joint_blocks_13: JointBlocks13,
        joint_blocks_14: JointBlocks14,
        joint_blocks_15: JointBlocks15,
        joint_blocks_16: JointBlocks16,
        joint_blocks_17: JointBlocks17,
        joint_blocks_18: JointBlocks18,
        joint_blocks_19: JointBlocks19,
        joint_blocks_20: JointBlocks20,
        joint_blocks_21: JointBlocks21,
        joint_blocks_22: JointBlocks22,
        joint_blocks_23: JointBlocks23,
        final_layer: FinalLayer,
    ) -> Self {
        Self {
            model_1,
            model_2,
            pos_embed,
            x_embedder,
            context_embedder,
            y_embedder,
            t_embedder,
            joint_blocks_0,
            joint_blocks_1,
            joint_blocks_2,
            joint_blocks_3,
            joint_blocks_4,
            joint_blocks_5,
            joint_blocks_6,
            joint_blocks_7,
            joint_blocks_8,
            joint_blocks_9,
            joint_blocks_10,
            joint_blocks_11,
            joint_blocks_12,
            joint_blocks_13,
            joint_blocks_14,
            joint_blocks_15,
            joint_blocks_16,
            joint_blocks_17,
            joint_blocks_18,
            joint_blocks_19,
            joint_blocks_20,
            joint_blocks_21,
            joint_blocks_22,
            joint_blocks_23,
            final_layer,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    PosEmbed: crate::nodes::types::Float,
    XEmbedder: crate::nodes::types::Float,
    ContextEmbedder: crate::nodes::types::Float,
    YEmbedder: crate::nodes::types::Float,
    TEmbedder: crate::nodes::types::Float,
    JointBlocks0: crate::nodes::types::Float,
    JointBlocks1: crate::nodes::types::Float,
    JointBlocks2: crate::nodes::types::Float,
    JointBlocks3: crate::nodes::types::Float,
    JointBlocks4: crate::nodes::types::Float,
    JointBlocks5: crate::nodes::types::Float,
    JointBlocks6: crate::nodes::types::Float,
    JointBlocks7: crate::nodes::types::Float,
    JointBlocks8: crate::nodes::types::Float,
    JointBlocks9: crate::nodes::types::Float,
    JointBlocks10: crate::nodes::types::Float,
    JointBlocks11: crate::nodes::types::Float,
    JointBlocks12: crate::nodes::types::Float,
    JointBlocks13: crate::nodes::types::Float,
    JointBlocks14: crate::nodes::types::Float,
    JointBlocks15: crate::nodes::types::Float,
    JointBlocks16: crate::nodes::types::Float,
    JointBlocks17: crate::nodes::types::Float,
    JointBlocks18: crate::nodes::types::Float,
    JointBlocks19: crate::nodes::types::Float,
    JointBlocks20: crate::nodes::types::Float,
    JointBlocks21: crate::nodes::types::Float,
    JointBlocks22: crate::nodes::types::Float,
    JointBlocks23: crate::nodes::types::Float,
    FinalLayer: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSd32B<
    Model1,
    Model2,
    PosEmbed,
    XEmbedder,
    ContextEmbedder,
    YEmbedder,
    TEmbedder,
    JointBlocks0,
    JointBlocks1,
    JointBlocks2,
    JointBlocks3,
    JointBlocks4,
    JointBlocks5,
    JointBlocks6,
    JointBlocks7,
    JointBlocks8,
    JointBlocks9,
    JointBlocks10,
    JointBlocks11,
    JointBlocks12,
    JointBlocks13,
    JointBlocks14,
    JointBlocks15,
    JointBlocks16,
    JointBlocks17,
    JointBlocks18,
    JointBlocks19,
    JointBlocks20,
    JointBlocks21,
    JointBlocks22,
    JointBlocks23,
    FinalLayer,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("pos_embed.".to_string(), self.pos_embed.clone().into());
        output.insert("x_embedder.".to_string(), self.x_embedder.clone().into());
        output
            .insert(
                "context_embedder.".to_string(),
                self.context_embedder.clone().into(),
            );
        output.insert("y_embedder.".to_string(), self.y_embedder.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output.insert("joint_blocks.0.".to_string(), self.joint_blocks_0.clone().into());
        output.insert("joint_blocks.1.".to_string(), self.joint_blocks_1.clone().into());
        output.insert("joint_blocks.2.".to_string(), self.joint_blocks_2.clone().into());
        output.insert("joint_blocks.3.".to_string(), self.joint_blocks_3.clone().into());
        output.insert("joint_blocks.4.".to_string(), self.joint_blocks_4.clone().into());
        output.insert("joint_blocks.5.".to_string(), self.joint_blocks_5.clone().into());
        output.insert("joint_blocks.6.".to_string(), self.joint_blocks_6.clone().into());
        output.insert("joint_blocks.7.".to_string(), self.joint_blocks_7.clone().into());
        output.insert("joint_blocks.8.".to_string(), self.joint_blocks_8.clone().into());
        output.insert("joint_blocks.9.".to_string(), self.joint_blocks_9.clone().into());
        output
            .insert("joint_blocks.10.".to_string(), self.joint_blocks_10.clone().into());
        output
            .insert("joint_blocks.11.".to_string(), self.joint_blocks_11.clone().into());
        output
            .insert("joint_blocks.12.".to_string(), self.joint_blocks_12.clone().into());
        output
            .insert("joint_blocks.13.".to_string(), self.joint_blocks_13.clone().into());
        output
            .insert("joint_blocks.14.".to_string(), self.joint_blocks_14.clone().into());
        output
            .insert("joint_blocks.15.".to_string(), self.joint_blocks_15.clone().into());
        output
            .insert("joint_blocks.16.".to_string(), self.joint_blocks_16.clone().into());
        output
            .insert("joint_blocks.17.".to_string(), self.joint_blocks_17.clone().into());
        output
            .insert("joint_blocks.18.".to_string(), self.joint_blocks_18.clone().into());
        output
            .insert("joint_blocks.19.".to_string(), self.joint_blocks_19.clone().into());
        output
            .insert("joint_blocks.20.".to_string(), self.joint_blocks_20.clone().into());
        output
            .insert("joint_blocks.21.".to_string(), self.joint_blocks_21.clone().into());
        output
            .insert("joint_blocks.22.".to_string(), self.joint_blocks_22.clone().into());
        output
            .insert("joint_blocks.23.".to_string(), self.joint_blocks_23.clone().into());
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSD3_2B";
    const DISPLAY_NAME: &'static str = "ModelMergeSD3_2B";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeSDXL**: No description.
#[derive(Clone)]
pub struct ModelMergeSdxl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1,
    ///No documentation.
    pub model_2: Model2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbed,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmb,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: Out,
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> ModelMergeSdxl<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    Out,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1,
        model_2: Model2,
        time_embed: TimeEmbed,
        label_emb: LabelEmb,
        input_blocks_0: InputBlocks0,
        input_blocks_1: InputBlocks1,
        input_blocks_2: InputBlocks2,
        input_blocks_3: InputBlocks3,
        input_blocks_4: InputBlocks4,
        input_blocks_5: InputBlocks5,
        input_blocks_6: InputBlocks6,
        input_blocks_7: InputBlocks7,
        input_blocks_8: InputBlocks8,
        middle_block_0: MiddleBlock0,
        middle_block_1: MiddleBlock1,
        middle_block_2: MiddleBlock2,
        output_blocks_0: OutputBlocks0,
        output_blocks_1: OutputBlocks1,
        output_blocks_2: OutputBlocks2,
        output_blocks_3: OutputBlocks3,
        output_blocks_4: OutputBlocks4,
        output_blocks_5: OutputBlocks5,
        output_blocks_6: OutputBlocks6,
        output_blocks_7: OutputBlocks7,
        output_blocks_8: OutputBlocks8,
        out: Out,
    ) -> Self {
        Self {
            model_1,
            model_2,
            time_embed,
            label_emb,
            input_blocks_0,
            input_blocks_1,
            input_blocks_2,
            input_blocks_3,
            input_blocks_4,
            input_blocks_5,
            input_blocks_6,
            input_blocks_7,
            input_blocks_8,
            middle_block_0,
            middle_block_1,
            middle_block_2,
            output_blocks_0,
            output_blocks_1,
            output_blocks_2,
            output_blocks_3,
            output_blocks_4,
            output_blocks_5,
            output_blocks_6,
            output_blocks_7,
            output_blocks_8,
            out,
        }
    }
}
impl<
    Model1: crate::nodes::types::Model,
    Model2: crate::nodes::types::Model,
    TimeEmbed: crate::nodes::types::Float,
    LabelEmb: crate::nodes::types::Float,
    InputBlocks0: crate::nodes::types::Float,
    InputBlocks1: crate::nodes::types::Float,
    InputBlocks2: crate::nodes::types::Float,
    InputBlocks3: crate::nodes::types::Float,
    InputBlocks4: crate::nodes::types::Float,
    InputBlocks5: crate::nodes::types::Float,
    InputBlocks6: crate::nodes::types::Float,
    InputBlocks7: crate::nodes::types::Float,
    InputBlocks8: crate::nodes::types::Float,
    MiddleBlock0: crate::nodes::types::Float,
    MiddleBlock1: crate::nodes::types::Float,
    MiddleBlock2: crate::nodes::types::Float,
    OutputBlocks0: crate::nodes::types::Float,
    OutputBlocks1: crate::nodes::types::Float,
    OutputBlocks2: crate::nodes::types::Float,
    OutputBlocks3: crate::nodes::types::Float,
    OutputBlocks4: crate::nodes::types::Float,
    OutputBlocks5: crate::nodes::types::Float,
    OutputBlocks6: crate::nodes::types::Float,
    OutputBlocks7: crate::nodes::types::Float,
    OutputBlocks8: crate::nodes::types::Float,
    Out: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSdxl<
    Model1,
    Model2,
    TimeEmbed,
    LabelEmb,
    InputBlocks0,
    InputBlocks1,
    InputBlocks2,
    InputBlocks3,
    InputBlocks4,
    InputBlocks5,
    InputBlocks6,
    InputBlocks7,
    InputBlocks8,
    MiddleBlock0,
    MiddleBlock1,
    MiddleBlock2,
    OutputBlocks0,
    OutputBlocks1,
    OutputBlocks2,
    OutputBlocks3,
    OutputBlocks4,
    OutputBlocks5,
    OutputBlocks6,
    OutputBlocks7,
    OutputBlocks8,
    Out,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("time_embed.".to_string(), self.time_embed.clone().into());
        output.insert("label_emb.".to_string(), self.label_emb.clone().into());
        output.insert("input_blocks.0".to_string(), self.input_blocks_0.clone().into());
        output.insert("input_blocks.1".to_string(), self.input_blocks_1.clone().into());
        output.insert("input_blocks.2".to_string(), self.input_blocks_2.clone().into());
        output.insert("input_blocks.3".to_string(), self.input_blocks_3.clone().into());
        output.insert("input_blocks.4".to_string(), self.input_blocks_4.clone().into());
        output.insert("input_blocks.5".to_string(), self.input_blocks_5.clone().into());
        output.insert("input_blocks.6".to_string(), self.input_blocks_6.clone().into());
        output.insert("input_blocks.7".to_string(), self.input_blocks_7.clone().into());
        output.insert("input_blocks.8".to_string(), self.input_blocks_8.clone().into());
        output.insert("middle_block.0".to_string(), self.middle_block_0.clone().into());
        output.insert("middle_block.1".to_string(), self.middle_block_1.clone().into());
        output.insert("middle_block.2".to_string(), self.middle_block_2.clone().into());
        output
            .insert("output_blocks.0".to_string(), self.output_blocks_0.clone().into());
        output
            .insert("output_blocks.1".to_string(), self.output_blocks_1.clone().into());
        output
            .insert("output_blocks.2".to_string(), self.output_blocks_2.clone().into());
        output
            .insert("output_blocks.3".to_string(), self.output_blocks_3.clone().into());
        output
            .insert("output_blocks.4".to_string(), self.output_blocks_4.clone().into());
        output
            .insert("output_blocks.5".to_string(), self.output_blocks_5.clone().into());
        output
            .insert("output_blocks.6".to_string(), self.output_blocks_6.clone().into());
        output
            .insert("output_blocks.7".to_string(), self.output_blocks_7.clone().into());
        output
            .insert("output_blocks.8".to_string(), self.output_blocks_8.clone().into());
        output.insert("out.".to_string(), self.out.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeSDXL";
    const DISPLAY_NAME: &'static str = "ModelMergeSDXL";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
