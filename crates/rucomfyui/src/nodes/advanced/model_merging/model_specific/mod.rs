//!`model_specific` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**ModelMergeAuraflow**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelMergeAuraflow<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InitXLinearParam: crate::nodes::types::Float,
    PositionalEncodingParam: crate::nodes::types::Float,
    CondSeqLinearParam: crate::nodes::types::Float,
    RegisterTokensParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    DoubleLayers0Param: crate::nodes::types::Float,
    DoubleLayers1Param: crate::nodes::types::Float,
    DoubleLayers2Param: crate::nodes::types::Float,
    DoubleLayers3Param: crate::nodes::types::Float,
    SingleLayers0Param: crate::nodes::types::Float,
    SingleLayers1Param: crate::nodes::types::Float,
    SingleLayers2Param: crate::nodes::types::Float,
    SingleLayers3Param: crate::nodes::types::Float,
    SingleLayers4Param: crate::nodes::types::Float,
    SingleLayers5Param: crate::nodes::types::Float,
    SingleLayers6Param: crate::nodes::types::Float,
    SingleLayers7Param: crate::nodes::types::Float,
    SingleLayers8Param: crate::nodes::types::Float,
    SingleLayers9Param: crate::nodes::types::Float,
    SingleLayers10Param: crate::nodes::types::Float,
    SingleLayers11Param: crate::nodes::types::Float,
    SingleLayers12Param: crate::nodes::types::Float,
    SingleLayers13Param: crate::nodes::types::Float,
    SingleLayers14Param: crate::nodes::types::Float,
    SingleLayers15Param: crate::nodes::types::Float,
    SingleLayers16Param: crate::nodes::types::Float,
    SingleLayers17Param: crate::nodes::types::Float,
    SingleLayers18Param: crate::nodes::types::Float,
    SingleLayers19Param: crate::nodes::types::Float,
    SingleLayers20Param: crate::nodes::types::Float,
    SingleLayers21Param: crate::nodes::types::Float,
    SingleLayers22Param: crate::nodes::types::Float,
    SingleLayers23Param: crate::nodes::types::Float,
    SingleLayers24Param: crate::nodes::types::Float,
    SingleLayers25Param: crate::nodes::types::Float,
    SingleLayers26Param: crate::nodes::types::Float,
    SingleLayers27Param: crate::nodes::types::Float,
    SingleLayers28Param: crate::nodes::types::Float,
    SingleLayers29Param: crate::nodes::types::Float,
    SingleLayers30Param: crate::nodes::types::Float,
    SingleLayers31Param: crate::nodes::types::Float,
    ModFParam: crate::nodes::types::Float,
    FinalLinearParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub init_x_linear: InitXLinearParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub positional_encoding: PositionalEncodingParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub cond_seq_linear: CondSeqLinearParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub register_tokens: RegisterTokensParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_0: DoubleLayers0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_1: DoubleLayers1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_2: DoubleLayers2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_layers_3: DoubleLayers3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_0: SingleLayers0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_1: SingleLayers1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_2: SingleLayers2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_3: SingleLayers3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_4: SingleLayers4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_5: SingleLayers5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_6: SingleLayers6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_7: SingleLayers7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_8: SingleLayers8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_9: SingleLayers9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_10: SingleLayers10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_11: SingleLayers11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_12: SingleLayers12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_13: SingleLayers13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_14: SingleLayers14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_15: SingleLayers15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_16: SingleLayers16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_17: SingleLayers17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_18: SingleLayers18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_19: SingleLayers19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_20: SingleLayers20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_21: SingleLayers21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_22: SingleLayers22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_23: SingleLayers23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_24: SingleLayers24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_25: SingleLayers25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_26: SingleLayers26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_27: SingleLayers27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_28: SingleLayers28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_29: SingleLayers29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_30: SingleLayers30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_layers_31: SingleLayers31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub mod_f: ModFParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_linear: FinalLinearParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InitXLinearParam: crate::nodes::types::Float,
    PositionalEncodingParam: crate::nodes::types::Float,
    CondSeqLinearParam: crate::nodes::types::Float,
    RegisterTokensParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    DoubleLayers0Param: crate::nodes::types::Float,
    DoubleLayers1Param: crate::nodes::types::Float,
    DoubleLayers2Param: crate::nodes::types::Float,
    DoubleLayers3Param: crate::nodes::types::Float,
    SingleLayers0Param: crate::nodes::types::Float,
    SingleLayers1Param: crate::nodes::types::Float,
    SingleLayers2Param: crate::nodes::types::Float,
    SingleLayers3Param: crate::nodes::types::Float,
    SingleLayers4Param: crate::nodes::types::Float,
    SingleLayers5Param: crate::nodes::types::Float,
    SingleLayers6Param: crate::nodes::types::Float,
    SingleLayers7Param: crate::nodes::types::Float,
    SingleLayers8Param: crate::nodes::types::Float,
    SingleLayers9Param: crate::nodes::types::Float,
    SingleLayers10Param: crate::nodes::types::Float,
    SingleLayers11Param: crate::nodes::types::Float,
    SingleLayers12Param: crate::nodes::types::Float,
    SingleLayers13Param: crate::nodes::types::Float,
    SingleLayers14Param: crate::nodes::types::Float,
    SingleLayers15Param: crate::nodes::types::Float,
    SingleLayers16Param: crate::nodes::types::Float,
    SingleLayers17Param: crate::nodes::types::Float,
    SingleLayers18Param: crate::nodes::types::Float,
    SingleLayers19Param: crate::nodes::types::Float,
    SingleLayers20Param: crate::nodes::types::Float,
    SingleLayers21Param: crate::nodes::types::Float,
    SingleLayers22Param: crate::nodes::types::Float,
    SingleLayers23Param: crate::nodes::types::Float,
    SingleLayers24Param: crate::nodes::types::Float,
    SingleLayers25Param: crate::nodes::types::Float,
    SingleLayers26Param: crate::nodes::types::Float,
    SingleLayers27Param: crate::nodes::types::Float,
    SingleLayers28Param: crate::nodes::types::Float,
    SingleLayers29Param: crate::nodes::types::Float,
    SingleLayers30Param: crate::nodes::types::Float,
    SingleLayers31Param: crate::nodes::types::Float,
    ModFParam: crate::nodes::types::Float,
    FinalLinearParam: crate::nodes::types::Float,
> ModelMergeAuraflow<
    Model1Param,
    Model2Param,
    InitXLinearParam,
    PositionalEncodingParam,
    CondSeqLinearParam,
    RegisterTokensParam,
    TEmbedderParam,
    DoubleLayers0Param,
    DoubleLayers1Param,
    DoubleLayers2Param,
    DoubleLayers3Param,
    SingleLayers0Param,
    SingleLayers1Param,
    SingleLayers2Param,
    SingleLayers3Param,
    SingleLayers4Param,
    SingleLayers5Param,
    SingleLayers6Param,
    SingleLayers7Param,
    SingleLayers8Param,
    SingleLayers9Param,
    SingleLayers10Param,
    SingleLayers11Param,
    SingleLayers12Param,
    SingleLayers13Param,
    SingleLayers14Param,
    SingleLayers15Param,
    SingleLayers16Param,
    SingleLayers17Param,
    SingleLayers18Param,
    SingleLayers19Param,
    SingleLayers20Param,
    SingleLayers21Param,
    SingleLayers22Param,
    SingleLayers23Param,
    SingleLayers24Param,
    SingleLayers25Param,
    SingleLayers26Param,
    SingleLayers27Param,
    SingleLayers28Param,
    SingleLayers29Param,
    SingleLayers30Param,
    SingleLayers31Param,
    ModFParam,
    FinalLinearParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        init_x_linear: InitXLinearParam,
        positional_encoding: PositionalEncodingParam,
        cond_seq_linear: CondSeqLinearParam,
        register_tokens: RegisterTokensParam,
        t_embedder: TEmbedderParam,
        double_layers_0: DoubleLayers0Param,
        double_layers_1: DoubleLayers1Param,
        double_layers_2: DoubleLayers2Param,
        double_layers_3: DoubleLayers3Param,
        single_layers_0: SingleLayers0Param,
        single_layers_1: SingleLayers1Param,
        single_layers_2: SingleLayers2Param,
        single_layers_3: SingleLayers3Param,
        single_layers_4: SingleLayers4Param,
        single_layers_5: SingleLayers5Param,
        single_layers_6: SingleLayers6Param,
        single_layers_7: SingleLayers7Param,
        single_layers_8: SingleLayers8Param,
        single_layers_9: SingleLayers9Param,
        single_layers_10: SingleLayers10Param,
        single_layers_11: SingleLayers11Param,
        single_layers_12: SingleLayers12Param,
        single_layers_13: SingleLayers13Param,
        single_layers_14: SingleLayers14Param,
        single_layers_15: SingleLayers15Param,
        single_layers_16: SingleLayers16Param,
        single_layers_17: SingleLayers17Param,
        single_layers_18: SingleLayers18Param,
        single_layers_19: SingleLayers19Param,
        single_layers_20: SingleLayers20Param,
        single_layers_21: SingleLayers21Param,
        single_layers_22: SingleLayers22Param,
        single_layers_23: SingleLayers23Param,
        single_layers_24: SingleLayers24Param,
        single_layers_25: SingleLayers25Param,
        single_layers_26: SingleLayers26Param,
        single_layers_27: SingleLayers27Param,
        single_layers_28: SingleLayers28Param,
        single_layers_29: SingleLayers29Param,
        single_layers_30: SingleLayers30Param,
        single_layers_31: SingleLayers31Param,
        mod_f: ModFParam,
        final_linear: FinalLinearParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    InitXLinearParam: crate::nodes::types::Float,
    PositionalEncodingParam: crate::nodes::types::Float,
    CondSeqLinearParam: crate::nodes::types::Float,
    RegisterTokensParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    DoubleLayers0Param: crate::nodes::types::Float,
    DoubleLayers1Param: crate::nodes::types::Float,
    DoubleLayers2Param: crate::nodes::types::Float,
    DoubleLayers3Param: crate::nodes::types::Float,
    SingleLayers0Param: crate::nodes::types::Float,
    SingleLayers1Param: crate::nodes::types::Float,
    SingleLayers2Param: crate::nodes::types::Float,
    SingleLayers3Param: crate::nodes::types::Float,
    SingleLayers4Param: crate::nodes::types::Float,
    SingleLayers5Param: crate::nodes::types::Float,
    SingleLayers6Param: crate::nodes::types::Float,
    SingleLayers7Param: crate::nodes::types::Float,
    SingleLayers8Param: crate::nodes::types::Float,
    SingleLayers9Param: crate::nodes::types::Float,
    SingleLayers10Param: crate::nodes::types::Float,
    SingleLayers11Param: crate::nodes::types::Float,
    SingleLayers12Param: crate::nodes::types::Float,
    SingleLayers13Param: crate::nodes::types::Float,
    SingleLayers14Param: crate::nodes::types::Float,
    SingleLayers15Param: crate::nodes::types::Float,
    SingleLayers16Param: crate::nodes::types::Float,
    SingleLayers17Param: crate::nodes::types::Float,
    SingleLayers18Param: crate::nodes::types::Float,
    SingleLayers19Param: crate::nodes::types::Float,
    SingleLayers20Param: crate::nodes::types::Float,
    SingleLayers21Param: crate::nodes::types::Float,
    SingleLayers22Param: crate::nodes::types::Float,
    SingleLayers23Param: crate::nodes::types::Float,
    SingleLayers24Param: crate::nodes::types::Float,
    SingleLayers25Param: crate::nodes::types::Float,
    SingleLayers26Param: crate::nodes::types::Float,
    SingleLayers27Param: crate::nodes::types::Float,
    SingleLayers28Param: crate::nodes::types::Float,
    SingleLayers29Param: crate::nodes::types::Float,
    SingleLayers30Param: crate::nodes::types::Float,
    SingleLayers31Param: crate::nodes::types::Float,
    ModFParam: crate::nodes::types::Float,
    FinalLinearParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeAuraflow<
    Model1Param,
    Model2Param,
    InitXLinearParam,
    PositionalEncodingParam,
    CondSeqLinearParam,
    RegisterTokensParam,
    TEmbedderParam,
    DoubleLayers0Param,
    DoubleLayers1Param,
    DoubleLayers2Param,
    DoubleLayers3Param,
    SingleLayers0Param,
    SingleLayers1Param,
    SingleLayers2Param,
    SingleLayers3Param,
    SingleLayers4Param,
    SingleLayers5Param,
    SingleLayers6Param,
    SingleLayers7Param,
    SingleLayers8Param,
    SingleLayers9Param,
    SingleLayers10Param,
    SingleLayers11Param,
    SingleLayers12Param,
    SingleLayers13Param,
    SingleLayers14Param,
    SingleLayers15Param,
    SingleLayers16Param,
    SingleLayers17Param,
    SingleLayers18Param,
    SingleLayers19Param,
    SingleLayers20Param,
    SingleLayers21Param,
    SingleLayers22Param,
    SingleLayers23Param,
    SingleLayers24Param,
    SingleLayers25Param,
    SingleLayers26Param,
    SingleLayers27Param,
    SingleLayers28Param,
    SingleLayers29Param,
    SingleLayers30Param,
    SingleLayers31Param,
    ModFParam,
    FinalLinearParam,
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
///**ModelMergeCosmos14B**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelMergeCosmos14B<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    BlocksBlock28Param: crate::nodes::types::Float,
    BlocksBlock29Param: crate::nodes::types::Float,
    BlocksBlock30Param: crate::nodes::types::Float,
    BlocksBlock31Param: crate::nodes::types::Float,
    BlocksBlock32Param: crate::nodes::types::Float,
    BlocksBlock33Param: crate::nodes::types::Float,
    BlocksBlock34Param: crate::nodes::types::Float,
    BlocksBlock35Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embedder: PosEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub extra_pos_embedder: ExtraPosEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub affline_norm: AfflineNormParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_0: BlocksBlock0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_1: BlocksBlock1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_2: BlocksBlock2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_3: BlocksBlock3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_4: BlocksBlock4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_5: BlocksBlock5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_6: BlocksBlock6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_7: BlocksBlock7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_8: BlocksBlock8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_9: BlocksBlock9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_10: BlocksBlock10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_11: BlocksBlock11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_12: BlocksBlock12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_13: BlocksBlock13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_14: BlocksBlock14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_15: BlocksBlock15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_16: BlocksBlock16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_17: BlocksBlock17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_18: BlocksBlock18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_19: BlocksBlock19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_20: BlocksBlock20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_21: BlocksBlock21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_22: BlocksBlock22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_23: BlocksBlock23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_24: BlocksBlock24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_25: BlocksBlock25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_26: BlocksBlock26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_27: BlocksBlock27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_28: BlocksBlock28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_29: BlocksBlock29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_30: BlocksBlock30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_31: BlocksBlock31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_32: BlocksBlock32Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_33: BlocksBlock33Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_34: BlocksBlock34Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_35: BlocksBlock35Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    BlocksBlock28Param: crate::nodes::types::Float,
    BlocksBlock29Param: crate::nodes::types::Float,
    BlocksBlock30Param: crate::nodes::types::Float,
    BlocksBlock31Param: crate::nodes::types::Float,
    BlocksBlock32Param: crate::nodes::types::Float,
    BlocksBlock33Param: crate::nodes::types::Float,
    BlocksBlock34Param: crate::nodes::types::Float,
    BlocksBlock35Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeCosmos14B<
    Model1Param,
    Model2Param,
    PosEmbedderParam,
    ExtraPosEmbedderParam,
    XEmbedderParam,
    TEmbedderParam,
    AfflineNormParam,
    BlocksBlock0Param,
    BlocksBlock1Param,
    BlocksBlock2Param,
    BlocksBlock3Param,
    BlocksBlock4Param,
    BlocksBlock5Param,
    BlocksBlock6Param,
    BlocksBlock7Param,
    BlocksBlock8Param,
    BlocksBlock9Param,
    BlocksBlock10Param,
    BlocksBlock11Param,
    BlocksBlock12Param,
    BlocksBlock13Param,
    BlocksBlock14Param,
    BlocksBlock15Param,
    BlocksBlock16Param,
    BlocksBlock17Param,
    BlocksBlock18Param,
    BlocksBlock19Param,
    BlocksBlock20Param,
    BlocksBlock21Param,
    BlocksBlock22Param,
    BlocksBlock23Param,
    BlocksBlock24Param,
    BlocksBlock25Param,
    BlocksBlock26Param,
    BlocksBlock27Param,
    BlocksBlock28Param,
    BlocksBlock29Param,
    BlocksBlock30Param,
    BlocksBlock31Param,
    BlocksBlock32Param,
    BlocksBlock33Param,
    BlocksBlock34Param,
    BlocksBlock35Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        pos_embedder: PosEmbedderParam,
        extra_pos_embedder: ExtraPosEmbedderParam,
        x_embedder: XEmbedderParam,
        t_embedder: TEmbedderParam,
        affline_norm: AfflineNormParam,
        blocks_block_0: BlocksBlock0Param,
        blocks_block_1: BlocksBlock1Param,
        blocks_block_2: BlocksBlock2Param,
        blocks_block_3: BlocksBlock3Param,
        blocks_block_4: BlocksBlock4Param,
        blocks_block_5: BlocksBlock5Param,
        blocks_block_6: BlocksBlock6Param,
        blocks_block_7: BlocksBlock7Param,
        blocks_block_8: BlocksBlock8Param,
        blocks_block_9: BlocksBlock9Param,
        blocks_block_10: BlocksBlock10Param,
        blocks_block_11: BlocksBlock11Param,
        blocks_block_12: BlocksBlock12Param,
        blocks_block_13: BlocksBlock13Param,
        blocks_block_14: BlocksBlock14Param,
        blocks_block_15: BlocksBlock15Param,
        blocks_block_16: BlocksBlock16Param,
        blocks_block_17: BlocksBlock17Param,
        blocks_block_18: BlocksBlock18Param,
        blocks_block_19: BlocksBlock19Param,
        blocks_block_20: BlocksBlock20Param,
        blocks_block_21: BlocksBlock21Param,
        blocks_block_22: BlocksBlock22Param,
        blocks_block_23: BlocksBlock23Param,
        blocks_block_24: BlocksBlock24Param,
        blocks_block_25: BlocksBlock25Param,
        blocks_block_26: BlocksBlock26Param,
        blocks_block_27: BlocksBlock27Param,
        blocks_block_28: BlocksBlock28Param,
        blocks_block_29: BlocksBlock29Param,
        blocks_block_30: BlocksBlock30Param,
        blocks_block_31: BlocksBlock31Param,
        blocks_block_32: BlocksBlock32Param,
        blocks_block_33: BlocksBlock33Param,
        blocks_block_34: BlocksBlock34Param,
        blocks_block_35: BlocksBlock35Param,
        final_layer: FinalLayerParam,
    ) -> Self {
        Self {
            model_1,
            model_2,
            pos_embedder,
            extra_pos_embedder,
            x_embedder,
            t_embedder,
            affline_norm,
            blocks_block_0,
            blocks_block_1,
            blocks_block_2,
            blocks_block_3,
            blocks_block_4,
            blocks_block_5,
            blocks_block_6,
            blocks_block_7,
            blocks_block_8,
            blocks_block_9,
            blocks_block_10,
            blocks_block_11,
            blocks_block_12,
            blocks_block_13,
            blocks_block_14,
            blocks_block_15,
            blocks_block_16,
            blocks_block_17,
            blocks_block_18,
            blocks_block_19,
            blocks_block_20,
            blocks_block_21,
            blocks_block_22,
            blocks_block_23,
            blocks_block_24,
            blocks_block_25,
            blocks_block_26,
            blocks_block_27,
            blocks_block_28,
            blocks_block_29,
            blocks_block_30,
            blocks_block_31,
            blocks_block_32,
            blocks_block_33,
            blocks_block_34,
            blocks_block_35,
            final_layer,
        }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    BlocksBlock28Param: crate::nodes::types::Float,
    BlocksBlock29Param: crate::nodes::types::Float,
    BlocksBlock30Param: crate::nodes::types::Float,
    BlocksBlock31Param: crate::nodes::types::Float,
    BlocksBlock32Param: crate::nodes::types::Float,
    BlocksBlock33Param: crate::nodes::types::Float,
    BlocksBlock34Param: crate::nodes::types::Float,
    BlocksBlock35Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeCosmos14B<
    Model1Param,
    Model2Param,
    PosEmbedderParam,
    ExtraPosEmbedderParam,
    XEmbedderParam,
    TEmbedderParam,
    AfflineNormParam,
    BlocksBlock0Param,
    BlocksBlock1Param,
    BlocksBlock2Param,
    BlocksBlock3Param,
    BlocksBlock4Param,
    BlocksBlock5Param,
    BlocksBlock6Param,
    BlocksBlock7Param,
    BlocksBlock8Param,
    BlocksBlock9Param,
    BlocksBlock10Param,
    BlocksBlock11Param,
    BlocksBlock12Param,
    BlocksBlock13Param,
    BlocksBlock14Param,
    BlocksBlock15Param,
    BlocksBlock16Param,
    BlocksBlock17Param,
    BlocksBlock18Param,
    BlocksBlock19Param,
    BlocksBlock20Param,
    BlocksBlock21Param,
    BlocksBlock22Param,
    BlocksBlock23Param,
    BlocksBlock24Param,
    BlocksBlock25Param,
    BlocksBlock26Param,
    BlocksBlock27Param,
    BlocksBlock28Param,
    BlocksBlock29Param,
    BlocksBlock30Param,
    BlocksBlock31Param,
    BlocksBlock32Param,
    BlocksBlock33Param,
    BlocksBlock34Param,
    BlocksBlock35Param,
    FinalLayerParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("pos_embedder.".to_string(), self.pos_embedder.clone().into());
        output
            .insert(
                "extra_pos_embedder.".to_string(),
                self.extra_pos_embedder.clone().into(),
            );
        output.insert("x_embedder.".to_string(), self.x_embedder.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output.insert("affline_norm.".to_string(), self.affline_norm.clone().into());
        output.insert("blocks.block0.".to_string(), self.blocks_block_0.clone().into());
        output.insert("blocks.block1.".to_string(), self.blocks_block_1.clone().into());
        output.insert("blocks.block2.".to_string(), self.blocks_block_2.clone().into());
        output.insert("blocks.block3.".to_string(), self.blocks_block_3.clone().into());
        output.insert("blocks.block4.".to_string(), self.blocks_block_4.clone().into());
        output.insert("blocks.block5.".to_string(), self.blocks_block_5.clone().into());
        output.insert("blocks.block6.".to_string(), self.blocks_block_6.clone().into());
        output.insert("blocks.block7.".to_string(), self.blocks_block_7.clone().into());
        output.insert("blocks.block8.".to_string(), self.blocks_block_8.clone().into());
        output.insert("blocks.block9.".to_string(), self.blocks_block_9.clone().into());
        output
            .insert("blocks.block10.".to_string(), self.blocks_block_10.clone().into());
        output
            .insert("blocks.block11.".to_string(), self.blocks_block_11.clone().into());
        output
            .insert("blocks.block12.".to_string(), self.blocks_block_12.clone().into());
        output
            .insert("blocks.block13.".to_string(), self.blocks_block_13.clone().into());
        output
            .insert("blocks.block14.".to_string(), self.blocks_block_14.clone().into());
        output
            .insert("blocks.block15.".to_string(), self.blocks_block_15.clone().into());
        output
            .insert("blocks.block16.".to_string(), self.blocks_block_16.clone().into());
        output
            .insert("blocks.block17.".to_string(), self.blocks_block_17.clone().into());
        output
            .insert("blocks.block18.".to_string(), self.blocks_block_18.clone().into());
        output
            .insert("blocks.block19.".to_string(), self.blocks_block_19.clone().into());
        output
            .insert("blocks.block20.".to_string(), self.blocks_block_20.clone().into());
        output
            .insert("blocks.block21.".to_string(), self.blocks_block_21.clone().into());
        output
            .insert("blocks.block22.".to_string(), self.blocks_block_22.clone().into());
        output
            .insert("blocks.block23.".to_string(), self.blocks_block_23.clone().into());
        output
            .insert("blocks.block24.".to_string(), self.blocks_block_24.clone().into());
        output
            .insert("blocks.block25.".to_string(), self.blocks_block_25.clone().into());
        output
            .insert("blocks.block26.".to_string(), self.blocks_block_26.clone().into());
        output
            .insert("blocks.block27.".to_string(), self.blocks_block_27.clone().into());
        output
            .insert("blocks.block28.".to_string(), self.blocks_block_28.clone().into());
        output
            .insert("blocks.block29.".to_string(), self.blocks_block_29.clone().into());
        output
            .insert("blocks.block30.".to_string(), self.blocks_block_30.clone().into());
        output
            .insert("blocks.block31.".to_string(), self.blocks_block_31.clone().into());
        output
            .insert("blocks.block32.".to_string(), self.blocks_block_32.clone().into());
        output
            .insert("blocks.block33.".to_string(), self.blocks_block_33.clone().into());
        output
            .insert("blocks.block34.".to_string(), self.blocks_block_34.clone().into());
        output
            .insert("blocks.block35.".to_string(), self.blocks_block_35.clone().into());
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeCosmos14B";
    const DISPLAY_NAME: &'static str = "ModelMergeCosmos14B";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeCosmos7B**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelMergeCosmos7B<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embedder: PosEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub extra_pos_embedder: ExtraPosEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub affline_norm: AfflineNormParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_0: BlocksBlock0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_1: BlocksBlock1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_2: BlocksBlock2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_3: BlocksBlock3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_4: BlocksBlock4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_5: BlocksBlock5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_6: BlocksBlock6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_7: BlocksBlock7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_8: BlocksBlock8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_9: BlocksBlock9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_10: BlocksBlock10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_11: BlocksBlock11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_12: BlocksBlock12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_13: BlocksBlock13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_14: BlocksBlock14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_15: BlocksBlock15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_16: BlocksBlock16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_17: BlocksBlock17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_18: BlocksBlock18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_19: BlocksBlock19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_20: BlocksBlock20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_21: BlocksBlock21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_22: BlocksBlock22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_23: BlocksBlock23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_24: BlocksBlock24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_25: BlocksBlock25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_26: BlocksBlock26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_block_27: BlocksBlock27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeCosmos7B<
    Model1Param,
    Model2Param,
    PosEmbedderParam,
    ExtraPosEmbedderParam,
    XEmbedderParam,
    TEmbedderParam,
    AfflineNormParam,
    BlocksBlock0Param,
    BlocksBlock1Param,
    BlocksBlock2Param,
    BlocksBlock3Param,
    BlocksBlock4Param,
    BlocksBlock5Param,
    BlocksBlock6Param,
    BlocksBlock7Param,
    BlocksBlock8Param,
    BlocksBlock9Param,
    BlocksBlock10Param,
    BlocksBlock11Param,
    BlocksBlock12Param,
    BlocksBlock13Param,
    BlocksBlock14Param,
    BlocksBlock15Param,
    BlocksBlock16Param,
    BlocksBlock17Param,
    BlocksBlock18Param,
    BlocksBlock19Param,
    BlocksBlock20Param,
    BlocksBlock21Param,
    BlocksBlock22Param,
    BlocksBlock23Param,
    BlocksBlock24Param,
    BlocksBlock25Param,
    BlocksBlock26Param,
    BlocksBlock27Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        pos_embedder: PosEmbedderParam,
        extra_pos_embedder: ExtraPosEmbedderParam,
        x_embedder: XEmbedderParam,
        t_embedder: TEmbedderParam,
        affline_norm: AfflineNormParam,
        blocks_block_0: BlocksBlock0Param,
        blocks_block_1: BlocksBlock1Param,
        blocks_block_2: BlocksBlock2Param,
        blocks_block_3: BlocksBlock3Param,
        blocks_block_4: BlocksBlock4Param,
        blocks_block_5: BlocksBlock5Param,
        blocks_block_6: BlocksBlock6Param,
        blocks_block_7: BlocksBlock7Param,
        blocks_block_8: BlocksBlock8Param,
        blocks_block_9: BlocksBlock9Param,
        blocks_block_10: BlocksBlock10Param,
        blocks_block_11: BlocksBlock11Param,
        blocks_block_12: BlocksBlock12Param,
        blocks_block_13: BlocksBlock13Param,
        blocks_block_14: BlocksBlock14Param,
        blocks_block_15: BlocksBlock15Param,
        blocks_block_16: BlocksBlock16Param,
        blocks_block_17: BlocksBlock17Param,
        blocks_block_18: BlocksBlock18Param,
        blocks_block_19: BlocksBlock19Param,
        blocks_block_20: BlocksBlock20Param,
        blocks_block_21: BlocksBlock21Param,
        blocks_block_22: BlocksBlock22Param,
        blocks_block_23: BlocksBlock23Param,
        blocks_block_24: BlocksBlock24Param,
        blocks_block_25: BlocksBlock25Param,
        blocks_block_26: BlocksBlock26Param,
        blocks_block_27: BlocksBlock27Param,
        final_layer: FinalLayerParam,
    ) -> Self {
        Self {
            model_1,
            model_2,
            pos_embedder,
            extra_pos_embedder,
            x_embedder,
            t_embedder,
            affline_norm,
            blocks_block_0,
            blocks_block_1,
            blocks_block_2,
            blocks_block_3,
            blocks_block_4,
            blocks_block_5,
            blocks_block_6,
            blocks_block_7,
            blocks_block_8,
            blocks_block_9,
            blocks_block_10,
            blocks_block_11,
            blocks_block_12,
            blocks_block_13,
            blocks_block_14,
            blocks_block_15,
            blocks_block_16,
            blocks_block_17,
            blocks_block_18,
            blocks_block_19,
            blocks_block_20,
            blocks_block_21,
            blocks_block_22,
            blocks_block_23,
            blocks_block_24,
            blocks_block_25,
            blocks_block_26,
            blocks_block_27,
            final_layer,
        }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedderParam: crate::nodes::types::Float,
    ExtraPosEmbedderParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    AfflineNormParam: crate::nodes::types::Float,
    BlocksBlock0Param: crate::nodes::types::Float,
    BlocksBlock1Param: crate::nodes::types::Float,
    BlocksBlock2Param: crate::nodes::types::Float,
    BlocksBlock3Param: crate::nodes::types::Float,
    BlocksBlock4Param: crate::nodes::types::Float,
    BlocksBlock5Param: crate::nodes::types::Float,
    BlocksBlock6Param: crate::nodes::types::Float,
    BlocksBlock7Param: crate::nodes::types::Float,
    BlocksBlock8Param: crate::nodes::types::Float,
    BlocksBlock9Param: crate::nodes::types::Float,
    BlocksBlock10Param: crate::nodes::types::Float,
    BlocksBlock11Param: crate::nodes::types::Float,
    BlocksBlock12Param: crate::nodes::types::Float,
    BlocksBlock13Param: crate::nodes::types::Float,
    BlocksBlock14Param: crate::nodes::types::Float,
    BlocksBlock15Param: crate::nodes::types::Float,
    BlocksBlock16Param: crate::nodes::types::Float,
    BlocksBlock17Param: crate::nodes::types::Float,
    BlocksBlock18Param: crate::nodes::types::Float,
    BlocksBlock19Param: crate::nodes::types::Float,
    BlocksBlock20Param: crate::nodes::types::Float,
    BlocksBlock21Param: crate::nodes::types::Float,
    BlocksBlock22Param: crate::nodes::types::Float,
    BlocksBlock23Param: crate::nodes::types::Float,
    BlocksBlock24Param: crate::nodes::types::Float,
    BlocksBlock25Param: crate::nodes::types::Float,
    BlocksBlock26Param: crate::nodes::types::Float,
    BlocksBlock27Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeCosmos7B<
    Model1Param,
    Model2Param,
    PosEmbedderParam,
    ExtraPosEmbedderParam,
    XEmbedderParam,
    TEmbedderParam,
    AfflineNormParam,
    BlocksBlock0Param,
    BlocksBlock1Param,
    BlocksBlock2Param,
    BlocksBlock3Param,
    BlocksBlock4Param,
    BlocksBlock5Param,
    BlocksBlock6Param,
    BlocksBlock7Param,
    BlocksBlock8Param,
    BlocksBlock9Param,
    BlocksBlock10Param,
    BlocksBlock11Param,
    BlocksBlock12Param,
    BlocksBlock13Param,
    BlocksBlock14Param,
    BlocksBlock15Param,
    BlocksBlock16Param,
    BlocksBlock17Param,
    BlocksBlock18Param,
    BlocksBlock19Param,
    BlocksBlock20Param,
    BlocksBlock21Param,
    BlocksBlock22Param,
    BlocksBlock23Param,
    BlocksBlock24Param,
    BlocksBlock25Param,
    BlocksBlock26Param,
    BlocksBlock27Param,
    FinalLayerParam,
> {
    type Output = crate::nodes::types::ModelOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("model1".to_string(), self.model_1.clone().into());
        output.insert("model2".to_string(), self.model_2.clone().into());
        output.insert("pos_embedder.".to_string(), self.pos_embedder.clone().into());
        output
            .insert(
                "extra_pos_embedder.".to_string(),
                self.extra_pos_embedder.clone().into(),
            );
        output.insert("x_embedder.".to_string(), self.x_embedder.clone().into());
        output.insert("t_embedder.".to_string(), self.t_embedder.clone().into());
        output.insert("affline_norm.".to_string(), self.affline_norm.clone().into());
        output.insert("blocks.block0.".to_string(), self.blocks_block_0.clone().into());
        output.insert("blocks.block1.".to_string(), self.blocks_block_1.clone().into());
        output.insert("blocks.block2.".to_string(), self.blocks_block_2.clone().into());
        output.insert("blocks.block3.".to_string(), self.blocks_block_3.clone().into());
        output.insert("blocks.block4.".to_string(), self.blocks_block_4.clone().into());
        output.insert("blocks.block5.".to_string(), self.blocks_block_5.clone().into());
        output.insert("blocks.block6.".to_string(), self.blocks_block_6.clone().into());
        output.insert("blocks.block7.".to_string(), self.blocks_block_7.clone().into());
        output.insert("blocks.block8.".to_string(), self.blocks_block_8.clone().into());
        output.insert("blocks.block9.".to_string(), self.blocks_block_9.clone().into());
        output
            .insert("blocks.block10.".to_string(), self.blocks_block_10.clone().into());
        output
            .insert("blocks.block11.".to_string(), self.blocks_block_11.clone().into());
        output
            .insert("blocks.block12.".to_string(), self.blocks_block_12.clone().into());
        output
            .insert("blocks.block13.".to_string(), self.blocks_block_13.clone().into());
        output
            .insert("blocks.block14.".to_string(), self.blocks_block_14.clone().into());
        output
            .insert("blocks.block15.".to_string(), self.blocks_block_15.clone().into());
        output
            .insert("blocks.block16.".to_string(), self.blocks_block_16.clone().into());
        output
            .insert("blocks.block17.".to_string(), self.blocks_block_17.clone().into());
        output
            .insert("blocks.block18.".to_string(), self.blocks_block_18.clone().into());
        output
            .insert("blocks.block19.".to_string(), self.blocks_block_19.clone().into());
        output
            .insert("blocks.block20.".to_string(), self.blocks_block_20.clone().into());
        output
            .insert("blocks.block21.".to_string(), self.blocks_block_21.clone().into());
        output
            .insert("blocks.block22.".to_string(), self.blocks_block_22.clone().into());
        output
            .insert("blocks.block23.".to_string(), self.blocks_block_23.clone().into());
        output
            .insert("blocks.block24.".to_string(), self.blocks_block_24.clone().into());
        output
            .insert("blocks.block25.".to_string(), self.blocks_block_25.clone().into());
        output
            .insert("blocks.block26.".to_string(), self.blocks_block_26.clone().into());
        output
            .insert("blocks.block27.".to_string(), self.blocks_block_27.clone().into());
        output.insert("final_layer.".to_string(), self.final_layer.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeCosmos7B";
    const DISPLAY_NAME: &'static str = "ModelMergeCosmos7B";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
///**ModelMergeFlux1**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelMergeFlux1<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    ImgInParam: crate::nodes::types::Float,
    TimeInParam: crate::nodes::types::Float,
    GuidanceInParam: crate::nodes::types::Float,
    VectorInParam: crate::nodes::types::Float,
    TxtInParam: crate::nodes::types::Float,
    DoubleBlocks0Param: crate::nodes::types::Float,
    DoubleBlocks1Param: crate::nodes::types::Float,
    DoubleBlocks2Param: crate::nodes::types::Float,
    DoubleBlocks3Param: crate::nodes::types::Float,
    DoubleBlocks4Param: crate::nodes::types::Float,
    DoubleBlocks5Param: crate::nodes::types::Float,
    DoubleBlocks6Param: crate::nodes::types::Float,
    DoubleBlocks7Param: crate::nodes::types::Float,
    DoubleBlocks8Param: crate::nodes::types::Float,
    DoubleBlocks9Param: crate::nodes::types::Float,
    DoubleBlocks10Param: crate::nodes::types::Float,
    DoubleBlocks11Param: crate::nodes::types::Float,
    DoubleBlocks12Param: crate::nodes::types::Float,
    DoubleBlocks13Param: crate::nodes::types::Float,
    DoubleBlocks14Param: crate::nodes::types::Float,
    DoubleBlocks15Param: crate::nodes::types::Float,
    DoubleBlocks16Param: crate::nodes::types::Float,
    DoubleBlocks17Param: crate::nodes::types::Float,
    DoubleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks0Param: crate::nodes::types::Float,
    SingleBlocks1Param: crate::nodes::types::Float,
    SingleBlocks2Param: crate::nodes::types::Float,
    SingleBlocks3Param: crate::nodes::types::Float,
    SingleBlocks4Param: crate::nodes::types::Float,
    SingleBlocks5Param: crate::nodes::types::Float,
    SingleBlocks6Param: crate::nodes::types::Float,
    SingleBlocks7Param: crate::nodes::types::Float,
    SingleBlocks8Param: crate::nodes::types::Float,
    SingleBlocks9Param: crate::nodes::types::Float,
    SingleBlocks10Param: crate::nodes::types::Float,
    SingleBlocks11Param: crate::nodes::types::Float,
    SingleBlocks12Param: crate::nodes::types::Float,
    SingleBlocks13Param: crate::nodes::types::Float,
    SingleBlocks14Param: crate::nodes::types::Float,
    SingleBlocks15Param: crate::nodes::types::Float,
    SingleBlocks16Param: crate::nodes::types::Float,
    SingleBlocks17Param: crate::nodes::types::Float,
    SingleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks19Param: crate::nodes::types::Float,
    SingleBlocks20Param: crate::nodes::types::Float,
    SingleBlocks21Param: crate::nodes::types::Float,
    SingleBlocks22Param: crate::nodes::types::Float,
    SingleBlocks23Param: crate::nodes::types::Float,
    SingleBlocks24Param: crate::nodes::types::Float,
    SingleBlocks25Param: crate::nodes::types::Float,
    SingleBlocks26Param: crate::nodes::types::Float,
    SingleBlocks27Param: crate::nodes::types::Float,
    SingleBlocks28Param: crate::nodes::types::Float,
    SingleBlocks29Param: crate::nodes::types::Float,
    SingleBlocks30Param: crate::nodes::types::Float,
    SingleBlocks31Param: crate::nodes::types::Float,
    SingleBlocks32Param: crate::nodes::types::Float,
    SingleBlocks33Param: crate::nodes::types::Float,
    SingleBlocks34Param: crate::nodes::types::Float,
    SingleBlocks35Param: crate::nodes::types::Float,
    SingleBlocks36Param: crate::nodes::types::Float,
    SingleBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub img_in: ImgInParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_in: TimeInParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub guidance_in: GuidanceInParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub vector_in: VectorInParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub txt_in: TxtInParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_0: DoubleBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_1: DoubleBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_2: DoubleBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_3: DoubleBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_4: DoubleBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_5: DoubleBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_6: DoubleBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_7: DoubleBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_8: DoubleBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_9: DoubleBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_10: DoubleBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_11: DoubleBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_12: DoubleBlocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_13: DoubleBlocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_14: DoubleBlocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_15: DoubleBlocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_16: DoubleBlocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_17: DoubleBlocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub double_blocks_18: DoubleBlocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_0: SingleBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_1: SingleBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_2: SingleBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_3: SingleBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_4: SingleBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_5: SingleBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_6: SingleBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_7: SingleBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_8: SingleBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_9: SingleBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_10: SingleBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_11: SingleBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_12: SingleBlocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_13: SingleBlocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_14: SingleBlocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_15: SingleBlocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_16: SingleBlocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_17: SingleBlocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_18: SingleBlocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_19: SingleBlocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_20: SingleBlocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_21: SingleBlocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_22: SingleBlocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_23: SingleBlocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_24: SingleBlocks24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_25: SingleBlocks25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_26: SingleBlocks26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_27: SingleBlocks27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_28: SingleBlocks28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_29: SingleBlocks29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_30: SingleBlocks30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_31: SingleBlocks31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_32: SingleBlocks32Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_33: SingleBlocks33Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_34: SingleBlocks34Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_35: SingleBlocks35Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_36: SingleBlocks36Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub single_blocks_37: SingleBlocks37Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    ImgInParam: crate::nodes::types::Float,
    TimeInParam: crate::nodes::types::Float,
    GuidanceInParam: crate::nodes::types::Float,
    VectorInParam: crate::nodes::types::Float,
    TxtInParam: crate::nodes::types::Float,
    DoubleBlocks0Param: crate::nodes::types::Float,
    DoubleBlocks1Param: crate::nodes::types::Float,
    DoubleBlocks2Param: crate::nodes::types::Float,
    DoubleBlocks3Param: crate::nodes::types::Float,
    DoubleBlocks4Param: crate::nodes::types::Float,
    DoubleBlocks5Param: crate::nodes::types::Float,
    DoubleBlocks6Param: crate::nodes::types::Float,
    DoubleBlocks7Param: crate::nodes::types::Float,
    DoubleBlocks8Param: crate::nodes::types::Float,
    DoubleBlocks9Param: crate::nodes::types::Float,
    DoubleBlocks10Param: crate::nodes::types::Float,
    DoubleBlocks11Param: crate::nodes::types::Float,
    DoubleBlocks12Param: crate::nodes::types::Float,
    DoubleBlocks13Param: crate::nodes::types::Float,
    DoubleBlocks14Param: crate::nodes::types::Float,
    DoubleBlocks15Param: crate::nodes::types::Float,
    DoubleBlocks16Param: crate::nodes::types::Float,
    DoubleBlocks17Param: crate::nodes::types::Float,
    DoubleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks0Param: crate::nodes::types::Float,
    SingleBlocks1Param: crate::nodes::types::Float,
    SingleBlocks2Param: crate::nodes::types::Float,
    SingleBlocks3Param: crate::nodes::types::Float,
    SingleBlocks4Param: crate::nodes::types::Float,
    SingleBlocks5Param: crate::nodes::types::Float,
    SingleBlocks6Param: crate::nodes::types::Float,
    SingleBlocks7Param: crate::nodes::types::Float,
    SingleBlocks8Param: crate::nodes::types::Float,
    SingleBlocks9Param: crate::nodes::types::Float,
    SingleBlocks10Param: crate::nodes::types::Float,
    SingleBlocks11Param: crate::nodes::types::Float,
    SingleBlocks12Param: crate::nodes::types::Float,
    SingleBlocks13Param: crate::nodes::types::Float,
    SingleBlocks14Param: crate::nodes::types::Float,
    SingleBlocks15Param: crate::nodes::types::Float,
    SingleBlocks16Param: crate::nodes::types::Float,
    SingleBlocks17Param: crate::nodes::types::Float,
    SingleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks19Param: crate::nodes::types::Float,
    SingleBlocks20Param: crate::nodes::types::Float,
    SingleBlocks21Param: crate::nodes::types::Float,
    SingleBlocks22Param: crate::nodes::types::Float,
    SingleBlocks23Param: crate::nodes::types::Float,
    SingleBlocks24Param: crate::nodes::types::Float,
    SingleBlocks25Param: crate::nodes::types::Float,
    SingleBlocks26Param: crate::nodes::types::Float,
    SingleBlocks27Param: crate::nodes::types::Float,
    SingleBlocks28Param: crate::nodes::types::Float,
    SingleBlocks29Param: crate::nodes::types::Float,
    SingleBlocks30Param: crate::nodes::types::Float,
    SingleBlocks31Param: crate::nodes::types::Float,
    SingleBlocks32Param: crate::nodes::types::Float,
    SingleBlocks33Param: crate::nodes::types::Float,
    SingleBlocks34Param: crate::nodes::types::Float,
    SingleBlocks35Param: crate::nodes::types::Float,
    SingleBlocks36Param: crate::nodes::types::Float,
    SingleBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeFlux1<
    Model1Param,
    Model2Param,
    ImgInParam,
    TimeInParam,
    GuidanceInParam,
    VectorInParam,
    TxtInParam,
    DoubleBlocks0Param,
    DoubleBlocks1Param,
    DoubleBlocks2Param,
    DoubleBlocks3Param,
    DoubleBlocks4Param,
    DoubleBlocks5Param,
    DoubleBlocks6Param,
    DoubleBlocks7Param,
    DoubleBlocks8Param,
    DoubleBlocks9Param,
    DoubleBlocks10Param,
    DoubleBlocks11Param,
    DoubleBlocks12Param,
    DoubleBlocks13Param,
    DoubleBlocks14Param,
    DoubleBlocks15Param,
    DoubleBlocks16Param,
    DoubleBlocks17Param,
    DoubleBlocks18Param,
    SingleBlocks0Param,
    SingleBlocks1Param,
    SingleBlocks2Param,
    SingleBlocks3Param,
    SingleBlocks4Param,
    SingleBlocks5Param,
    SingleBlocks6Param,
    SingleBlocks7Param,
    SingleBlocks8Param,
    SingleBlocks9Param,
    SingleBlocks10Param,
    SingleBlocks11Param,
    SingleBlocks12Param,
    SingleBlocks13Param,
    SingleBlocks14Param,
    SingleBlocks15Param,
    SingleBlocks16Param,
    SingleBlocks17Param,
    SingleBlocks18Param,
    SingleBlocks19Param,
    SingleBlocks20Param,
    SingleBlocks21Param,
    SingleBlocks22Param,
    SingleBlocks23Param,
    SingleBlocks24Param,
    SingleBlocks25Param,
    SingleBlocks26Param,
    SingleBlocks27Param,
    SingleBlocks28Param,
    SingleBlocks29Param,
    SingleBlocks30Param,
    SingleBlocks31Param,
    SingleBlocks32Param,
    SingleBlocks33Param,
    SingleBlocks34Param,
    SingleBlocks35Param,
    SingleBlocks36Param,
    SingleBlocks37Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        img_in: ImgInParam,
        time_in: TimeInParam,
        guidance_in: GuidanceInParam,
        vector_in: VectorInParam,
        txt_in: TxtInParam,
        double_blocks_0: DoubleBlocks0Param,
        double_blocks_1: DoubleBlocks1Param,
        double_blocks_2: DoubleBlocks2Param,
        double_blocks_3: DoubleBlocks3Param,
        double_blocks_4: DoubleBlocks4Param,
        double_blocks_5: DoubleBlocks5Param,
        double_blocks_6: DoubleBlocks6Param,
        double_blocks_7: DoubleBlocks7Param,
        double_blocks_8: DoubleBlocks8Param,
        double_blocks_9: DoubleBlocks9Param,
        double_blocks_10: DoubleBlocks10Param,
        double_blocks_11: DoubleBlocks11Param,
        double_blocks_12: DoubleBlocks12Param,
        double_blocks_13: DoubleBlocks13Param,
        double_blocks_14: DoubleBlocks14Param,
        double_blocks_15: DoubleBlocks15Param,
        double_blocks_16: DoubleBlocks16Param,
        double_blocks_17: DoubleBlocks17Param,
        double_blocks_18: DoubleBlocks18Param,
        single_blocks_0: SingleBlocks0Param,
        single_blocks_1: SingleBlocks1Param,
        single_blocks_2: SingleBlocks2Param,
        single_blocks_3: SingleBlocks3Param,
        single_blocks_4: SingleBlocks4Param,
        single_blocks_5: SingleBlocks5Param,
        single_blocks_6: SingleBlocks6Param,
        single_blocks_7: SingleBlocks7Param,
        single_blocks_8: SingleBlocks8Param,
        single_blocks_9: SingleBlocks9Param,
        single_blocks_10: SingleBlocks10Param,
        single_blocks_11: SingleBlocks11Param,
        single_blocks_12: SingleBlocks12Param,
        single_blocks_13: SingleBlocks13Param,
        single_blocks_14: SingleBlocks14Param,
        single_blocks_15: SingleBlocks15Param,
        single_blocks_16: SingleBlocks16Param,
        single_blocks_17: SingleBlocks17Param,
        single_blocks_18: SingleBlocks18Param,
        single_blocks_19: SingleBlocks19Param,
        single_blocks_20: SingleBlocks20Param,
        single_blocks_21: SingleBlocks21Param,
        single_blocks_22: SingleBlocks22Param,
        single_blocks_23: SingleBlocks23Param,
        single_blocks_24: SingleBlocks24Param,
        single_blocks_25: SingleBlocks25Param,
        single_blocks_26: SingleBlocks26Param,
        single_blocks_27: SingleBlocks27Param,
        single_blocks_28: SingleBlocks28Param,
        single_blocks_29: SingleBlocks29Param,
        single_blocks_30: SingleBlocks30Param,
        single_blocks_31: SingleBlocks31Param,
        single_blocks_32: SingleBlocks32Param,
        single_blocks_33: SingleBlocks33Param,
        single_blocks_34: SingleBlocks34Param,
        single_blocks_35: SingleBlocks35Param,
        single_blocks_36: SingleBlocks36Param,
        single_blocks_37: SingleBlocks37Param,
        final_layer: FinalLayerParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    ImgInParam: crate::nodes::types::Float,
    TimeInParam: crate::nodes::types::Float,
    GuidanceInParam: crate::nodes::types::Float,
    VectorInParam: crate::nodes::types::Float,
    TxtInParam: crate::nodes::types::Float,
    DoubleBlocks0Param: crate::nodes::types::Float,
    DoubleBlocks1Param: crate::nodes::types::Float,
    DoubleBlocks2Param: crate::nodes::types::Float,
    DoubleBlocks3Param: crate::nodes::types::Float,
    DoubleBlocks4Param: crate::nodes::types::Float,
    DoubleBlocks5Param: crate::nodes::types::Float,
    DoubleBlocks6Param: crate::nodes::types::Float,
    DoubleBlocks7Param: crate::nodes::types::Float,
    DoubleBlocks8Param: crate::nodes::types::Float,
    DoubleBlocks9Param: crate::nodes::types::Float,
    DoubleBlocks10Param: crate::nodes::types::Float,
    DoubleBlocks11Param: crate::nodes::types::Float,
    DoubleBlocks12Param: crate::nodes::types::Float,
    DoubleBlocks13Param: crate::nodes::types::Float,
    DoubleBlocks14Param: crate::nodes::types::Float,
    DoubleBlocks15Param: crate::nodes::types::Float,
    DoubleBlocks16Param: crate::nodes::types::Float,
    DoubleBlocks17Param: crate::nodes::types::Float,
    DoubleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks0Param: crate::nodes::types::Float,
    SingleBlocks1Param: crate::nodes::types::Float,
    SingleBlocks2Param: crate::nodes::types::Float,
    SingleBlocks3Param: crate::nodes::types::Float,
    SingleBlocks4Param: crate::nodes::types::Float,
    SingleBlocks5Param: crate::nodes::types::Float,
    SingleBlocks6Param: crate::nodes::types::Float,
    SingleBlocks7Param: crate::nodes::types::Float,
    SingleBlocks8Param: crate::nodes::types::Float,
    SingleBlocks9Param: crate::nodes::types::Float,
    SingleBlocks10Param: crate::nodes::types::Float,
    SingleBlocks11Param: crate::nodes::types::Float,
    SingleBlocks12Param: crate::nodes::types::Float,
    SingleBlocks13Param: crate::nodes::types::Float,
    SingleBlocks14Param: crate::nodes::types::Float,
    SingleBlocks15Param: crate::nodes::types::Float,
    SingleBlocks16Param: crate::nodes::types::Float,
    SingleBlocks17Param: crate::nodes::types::Float,
    SingleBlocks18Param: crate::nodes::types::Float,
    SingleBlocks19Param: crate::nodes::types::Float,
    SingleBlocks20Param: crate::nodes::types::Float,
    SingleBlocks21Param: crate::nodes::types::Float,
    SingleBlocks22Param: crate::nodes::types::Float,
    SingleBlocks23Param: crate::nodes::types::Float,
    SingleBlocks24Param: crate::nodes::types::Float,
    SingleBlocks25Param: crate::nodes::types::Float,
    SingleBlocks26Param: crate::nodes::types::Float,
    SingleBlocks27Param: crate::nodes::types::Float,
    SingleBlocks28Param: crate::nodes::types::Float,
    SingleBlocks29Param: crate::nodes::types::Float,
    SingleBlocks30Param: crate::nodes::types::Float,
    SingleBlocks31Param: crate::nodes::types::Float,
    SingleBlocks32Param: crate::nodes::types::Float,
    SingleBlocks33Param: crate::nodes::types::Float,
    SingleBlocks34Param: crate::nodes::types::Float,
    SingleBlocks35Param: crate::nodes::types::Float,
    SingleBlocks36Param: crate::nodes::types::Float,
    SingleBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeFlux1<
    Model1Param,
    Model2Param,
    ImgInParam,
    TimeInParam,
    GuidanceInParam,
    VectorInParam,
    TxtInParam,
    DoubleBlocks0Param,
    DoubleBlocks1Param,
    DoubleBlocks2Param,
    DoubleBlocks3Param,
    DoubleBlocks4Param,
    DoubleBlocks5Param,
    DoubleBlocks6Param,
    DoubleBlocks7Param,
    DoubleBlocks8Param,
    DoubleBlocks9Param,
    DoubleBlocks10Param,
    DoubleBlocks11Param,
    DoubleBlocks12Param,
    DoubleBlocks13Param,
    DoubleBlocks14Param,
    DoubleBlocks15Param,
    DoubleBlocks16Param,
    DoubleBlocks17Param,
    DoubleBlocks18Param,
    SingleBlocks0Param,
    SingleBlocks1Param,
    SingleBlocks2Param,
    SingleBlocks3Param,
    SingleBlocks4Param,
    SingleBlocks5Param,
    SingleBlocks6Param,
    SingleBlocks7Param,
    SingleBlocks8Param,
    SingleBlocks9Param,
    SingleBlocks10Param,
    SingleBlocks11Param,
    SingleBlocks12Param,
    SingleBlocks13Param,
    SingleBlocks14Param,
    SingleBlocks15Param,
    SingleBlocks16Param,
    SingleBlocks17Param,
    SingleBlocks18Param,
    SingleBlocks19Param,
    SingleBlocks20Param,
    SingleBlocks21Param,
    SingleBlocks22Param,
    SingleBlocks23Param,
    SingleBlocks24Param,
    SingleBlocks25Param,
    SingleBlocks26Param,
    SingleBlocks27Param,
    SingleBlocks28Param,
    SingleBlocks29Param,
    SingleBlocks30Param,
    SingleBlocks31Param,
    SingleBlocks32Param,
    SingleBlocks33Param,
    SingleBlocks34Param,
    SingleBlocks35Param,
    SingleBlocks36Param,
    SingleBlocks37Param,
    FinalLayerParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeLTXV<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchifyProjParam: crate::nodes::types::Float,
    AdalnSingleParam: crate::nodes::types::Float,
    CaptionProjectionParam: crate::nodes::types::Float,
    TransformerBlocks0Param: crate::nodes::types::Float,
    TransformerBlocks1Param: crate::nodes::types::Float,
    TransformerBlocks2Param: crate::nodes::types::Float,
    TransformerBlocks3Param: crate::nodes::types::Float,
    TransformerBlocks4Param: crate::nodes::types::Float,
    TransformerBlocks5Param: crate::nodes::types::Float,
    TransformerBlocks6Param: crate::nodes::types::Float,
    TransformerBlocks7Param: crate::nodes::types::Float,
    TransformerBlocks8Param: crate::nodes::types::Float,
    TransformerBlocks9Param: crate::nodes::types::Float,
    TransformerBlocks10Param: crate::nodes::types::Float,
    TransformerBlocks11Param: crate::nodes::types::Float,
    TransformerBlocks12Param: crate::nodes::types::Float,
    TransformerBlocks13Param: crate::nodes::types::Float,
    TransformerBlocks14Param: crate::nodes::types::Float,
    TransformerBlocks15Param: crate::nodes::types::Float,
    TransformerBlocks16Param: crate::nodes::types::Float,
    TransformerBlocks17Param: crate::nodes::types::Float,
    TransformerBlocks18Param: crate::nodes::types::Float,
    TransformerBlocks19Param: crate::nodes::types::Float,
    TransformerBlocks20Param: crate::nodes::types::Float,
    TransformerBlocks21Param: crate::nodes::types::Float,
    TransformerBlocks22Param: crate::nodes::types::Float,
    TransformerBlocks23Param: crate::nodes::types::Float,
    TransformerBlocks24Param: crate::nodes::types::Float,
    TransformerBlocks25Param: crate::nodes::types::Float,
    TransformerBlocks26Param: crate::nodes::types::Float,
    TransformerBlocks27Param: crate::nodes::types::Float,
    ScaleShiftTableParam: crate::nodes::types::Float,
    ProjOutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub patchify_proj: PatchifyProjParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub adaln_single: AdalnSingleParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub caption_projection: CaptionProjectionParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_0: TransformerBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_1: TransformerBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_2: TransformerBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_3: TransformerBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_4: TransformerBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_5: TransformerBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_6: TransformerBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_7: TransformerBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_8: TransformerBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_9: TransformerBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_10: TransformerBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_11: TransformerBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_12: TransformerBlocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_13: TransformerBlocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_14: TransformerBlocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_15: TransformerBlocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_16: TransformerBlocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_17: TransformerBlocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_18: TransformerBlocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_19: TransformerBlocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_20: TransformerBlocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_21: TransformerBlocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_22: TransformerBlocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_23: TransformerBlocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_24: TransformerBlocks24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_25: TransformerBlocks25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_26: TransformerBlocks26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub transformer_blocks_27: TransformerBlocks27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub scale_shift_table: ScaleShiftTableParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub proj_out: ProjOutParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchifyProjParam: crate::nodes::types::Float,
    AdalnSingleParam: crate::nodes::types::Float,
    CaptionProjectionParam: crate::nodes::types::Float,
    TransformerBlocks0Param: crate::nodes::types::Float,
    TransformerBlocks1Param: crate::nodes::types::Float,
    TransformerBlocks2Param: crate::nodes::types::Float,
    TransformerBlocks3Param: crate::nodes::types::Float,
    TransformerBlocks4Param: crate::nodes::types::Float,
    TransformerBlocks5Param: crate::nodes::types::Float,
    TransformerBlocks6Param: crate::nodes::types::Float,
    TransformerBlocks7Param: crate::nodes::types::Float,
    TransformerBlocks8Param: crate::nodes::types::Float,
    TransformerBlocks9Param: crate::nodes::types::Float,
    TransformerBlocks10Param: crate::nodes::types::Float,
    TransformerBlocks11Param: crate::nodes::types::Float,
    TransformerBlocks12Param: crate::nodes::types::Float,
    TransformerBlocks13Param: crate::nodes::types::Float,
    TransformerBlocks14Param: crate::nodes::types::Float,
    TransformerBlocks15Param: crate::nodes::types::Float,
    TransformerBlocks16Param: crate::nodes::types::Float,
    TransformerBlocks17Param: crate::nodes::types::Float,
    TransformerBlocks18Param: crate::nodes::types::Float,
    TransformerBlocks19Param: crate::nodes::types::Float,
    TransformerBlocks20Param: crate::nodes::types::Float,
    TransformerBlocks21Param: crate::nodes::types::Float,
    TransformerBlocks22Param: crate::nodes::types::Float,
    TransformerBlocks23Param: crate::nodes::types::Float,
    TransformerBlocks24Param: crate::nodes::types::Float,
    TransformerBlocks25Param: crate::nodes::types::Float,
    TransformerBlocks26Param: crate::nodes::types::Float,
    TransformerBlocks27Param: crate::nodes::types::Float,
    ScaleShiftTableParam: crate::nodes::types::Float,
    ProjOutParam: crate::nodes::types::Float,
> ModelMergeLTXV<
    Model1Param,
    Model2Param,
    PatchifyProjParam,
    AdalnSingleParam,
    CaptionProjectionParam,
    TransformerBlocks0Param,
    TransformerBlocks1Param,
    TransformerBlocks2Param,
    TransformerBlocks3Param,
    TransformerBlocks4Param,
    TransformerBlocks5Param,
    TransformerBlocks6Param,
    TransformerBlocks7Param,
    TransformerBlocks8Param,
    TransformerBlocks9Param,
    TransformerBlocks10Param,
    TransformerBlocks11Param,
    TransformerBlocks12Param,
    TransformerBlocks13Param,
    TransformerBlocks14Param,
    TransformerBlocks15Param,
    TransformerBlocks16Param,
    TransformerBlocks17Param,
    TransformerBlocks18Param,
    TransformerBlocks19Param,
    TransformerBlocks20Param,
    TransformerBlocks21Param,
    TransformerBlocks22Param,
    TransformerBlocks23Param,
    TransformerBlocks24Param,
    TransformerBlocks25Param,
    TransformerBlocks26Param,
    TransformerBlocks27Param,
    ScaleShiftTableParam,
    ProjOutParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        patchify_proj: PatchifyProjParam,
        adaln_single: AdalnSingleParam,
        caption_projection: CaptionProjectionParam,
        transformer_blocks_0: TransformerBlocks0Param,
        transformer_blocks_1: TransformerBlocks1Param,
        transformer_blocks_2: TransformerBlocks2Param,
        transformer_blocks_3: TransformerBlocks3Param,
        transformer_blocks_4: TransformerBlocks4Param,
        transformer_blocks_5: TransformerBlocks5Param,
        transformer_blocks_6: TransformerBlocks6Param,
        transformer_blocks_7: TransformerBlocks7Param,
        transformer_blocks_8: TransformerBlocks8Param,
        transformer_blocks_9: TransformerBlocks9Param,
        transformer_blocks_10: TransformerBlocks10Param,
        transformer_blocks_11: TransformerBlocks11Param,
        transformer_blocks_12: TransformerBlocks12Param,
        transformer_blocks_13: TransformerBlocks13Param,
        transformer_blocks_14: TransformerBlocks14Param,
        transformer_blocks_15: TransformerBlocks15Param,
        transformer_blocks_16: TransformerBlocks16Param,
        transformer_blocks_17: TransformerBlocks17Param,
        transformer_blocks_18: TransformerBlocks18Param,
        transformer_blocks_19: TransformerBlocks19Param,
        transformer_blocks_20: TransformerBlocks20Param,
        transformer_blocks_21: TransformerBlocks21Param,
        transformer_blocks_22: TransformerBlocks22Param,
        transformer_blocks_23: TransformerBlocks23Param,
        transformer_blocks_24: TransformerBlocks24Param,
        transformer_blocks_25: TransformerBlocks25Param,
        transformer_blocks_26: TransformerBlocks26Param,
        transformer_blocks_27: TransformerBlocks27Param,
        scale_shift_table: ScaleShiftTableParam,
        proj_out: ProjOutParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchifyProjParam: crate::nodes::types::Float,
    AdalnSingleParam: crate::nodes::types::Float,
    CaptionProjectionParam: crate::nodes::types::Float,
    TransformerBlocks0Param: crate::nodes::types::Float,
    TransformerBlocks1Param: crate::nodes::types::Float,
    TransformerBlocks2Param: crate::nodes::types::Float,
    TransformerBlocks3Param: crate::nodes::types::Float,
    TransformerBlocks4Param: crate::nodes::types::Float,
    TransformerBlocks5Param: crate::nodes::types::Float,
    TransformerBlocks6Param: crate::nodes::types::Float,
    TransformerBlocks7Param: crate::nodes::types::Float,
    TransformerBlocks8Param: crate::nodes::types::Float,
    TransformerBlocks9Param: crate::nodes::types::Float,
    TransformerBlocks10Param: crate::nodes::types::Float,
    TransformerBlocks11Param: crate::nodes::types::Float,
    TransformerBlocks12Param: crate::nodes::types::Float,
    TransformerBlocks13Param: crate::nodes::types::Float,
    TransformerBlocks14Param: crate::nodes::types::Float,
    TransformerBlocks15Param: crate::nodes::types::Float,
    TransformerBlocks16Param: crate::nodes::types::Float,
    TransformerBlocks17Param: crate::nodes::types::Float,
    TransformerBlocks18Param: crate::nodes::types::Float,
    TransformerBlocks19Param: crate::nodes::types::Float,
    TransformerBlocks20Param: crate::nodes::types::Float,
    TransformerBlocks21Param: crate::nodes::types::Float,
    TransformerBlocks22Param: crate::nodes::types::Float,
    TransformerBlocks23Param: crate::nodes::types::Float,
    TransformerBlocks24Param: crate::nodes::types::Float,
    TransformerBlocks25Param: crate::nodes::types::Float,
    TransformerBlocks26Param: crate::nodes::types::Float,
    TransformerBlocks27Param: crate::nodes::types::Float,
    ScaleShiftTableParam: crate::nodes::types::Float,
    ProjOutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeLTXV<
    Model1Param,
    Model2Param,
    PatchifyProjParam,
    AdalnSingleParam,
    CaptionProjectionParam,
    TransformerBlocks0Param,
    TransformerBlocks1Param,
    TransformerBlocks2Param,
    TransformerBlocks3Param,
    TransformerBlocks4Param,
    TransformerBlocks5Param,
    TransformerBlocks6Param,
    TransformerBlocks7Param,
    TransformerBlocks8Param,
    TransformerBlocks9Param,
    TransformerBlocks10Param,
    TransformerBlocks11Param,
    TransformerBlocks12Param,
    TransformerBlocks13Param,
    TransformerBlocks14Param,
    TransformerBlocks15Param,
    TransformerBlocks16Param,
    TransformerBlocks17Param,
    TransformerBlocks18Param,
    TransformerBlocks19Param,
    TransformerBlocks20Param,
    TransformerBlocks21Param,
    TransformerBlocks22Param,
    TransformerBlocks23Param,
    TransformerBlocks24Param,
    TransformerBlocks25Param,
    TransformerBlocks26Param,
    TransformerBlocks27Param,
    ScaleShiftTableParam,
    ProjOutParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeMochiPreview<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosFrequenciesParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    T5YEmbedderParam: crate::nodes::types::Float,
    T5YprojParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    Blocks40Param: crate::nodes::types::Float,
    Blocks41Param: crate::nodes::types::Float,
    Blocks42Param: crate::nodes::types::Float,
    Blocks43Param: crate::nodes::types::Float,
    Blocks44Param: crate::nodes::types::Float,
    Blocks45Param: crate::nodes::types::Float,
    Blocks46Param: crate::nodes::types::Float,
    Blocks47Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_frequencies: PosFrequenciesParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_5_y_embedder: T5YEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_5_yproj: T5YprojParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_0: Blocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_1: Blocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_2: Blocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_3: Blocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_4: Blocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_5: Blocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_6: Blocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_7: Blocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_8: Blocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_9: Blocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_10: Blocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_11: Blocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_12: Blocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_13: Blocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_14: Blocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_15: Blocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_16: Blocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_17: Blocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_18: Blocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_19: Blocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_20: Blocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_21: Blocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_22: Blocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_23: Blocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_24: Blocks24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_25: Blocks25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_26: Blocks26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_27: Blocks27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_28: Blocks28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_29: Blocks29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_30: Blocks30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_31: Blocks31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_32: Blocks32Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_33: Blocks33Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_34: Blocks34Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_35: Blocks35Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_36: Blocks36Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_37: Blocks37Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_38: Blocks38Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_39: Blocks39Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_40: Blocks40Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_41: Blocks41Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_42: Blocks42Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_43: Blocks43Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_44: Blocks44Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_45: Blocks45Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_46: Blocks46Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_47: Blocks47Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosFrequenciesParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    T5YEmbedderParam: crate::nodes::types::Float,
    T5YprojParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    Blocks40Param: crate::nodes::types::Float,
    Blocks41Param: crate::nodes::types::Float,
    Blocks42Param: crate::nodes::types::Float,
    Blocks43Param: crate::nodes::types::Float,
    Blocks44Param: crate::nodes::types::Float,
    Blocks45Param: crate::nodes::types::Float,
    Blocks46Param: crate::nodes::types::Float,
    Blocks47Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeMochiPreview<
    Model1Param,
    Model2Param,
    PosFrequenciesParam,
    TEmbedderParam,
    T5YEmbedderParam,
    T5YprojParam,
    Blocks0Param,
    Blocks1Param,
    Blocks2Param,
    Blocks3Param,
    Blocks4Param,
    Blocks5Param,
    Blocks6Param,
    Blocks7Param,
    Blocks8Param,
    Blocks9Param,
    Blocks10Param,
    Blocks11Param,
    Blocks12Param,
    Blocks13Param,
    Blocks14Param,
    Blocks15Param,
    Blocks16Param,
    Blocks17Param,
    Blocks18Param,
    Blocks19Param,
    Blocks20Param,
    Blocks21Param,
    Blocks22Param,
    Blocks23Param,
    Blocks24Param,
    Blocks25Param,
    Blocks26Param,
    Blocks27Param,
    Blocks28Param,
    Blocks29Param,
    Blocks30Param,
    Blocks31Param,
    Blocks32Param,
    Blocks33Param,
    Blocks34Param,
    Blocks35Param,
    Blocks36Param,
    Blocks37Param,
    Blocks38Param,
    Blocks39Param,
    Blocks40Param,
    Blocks41Param,
    Blocks42Param,
    Blocks43Param,
    Blocks44Param,
    Blocks45Param,
    Blocks46Param,
    Blocks47Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        pos_frequencies: PosFrequenciesParam,
        t_embedder: TEmbedderParam,
        t_5_y_embedder: T5YEmbedderParam,
        t_5_yproj: T5YprojParam,
        blocks_0: Blocks0Param,
        blocks_1: Blocks1Param,
        blocks_2: Blocks2Param,
        blocks_3: Blocks3Param,
        blocks_4: Blocks4Param,
        blocks_5: Blocks5Param,
        blocks_6: Blocks6Param,
        blocks_7: Blocks7Param,
        blocks_8: Blocks8Param,
        blocks_9: Blocks9Param,
        blocks_10: Blocks10Param,
        blocks_11: Blocks11Param,
        blocks_12: Blocks12Param,
        blocks_13: Blocks13Param,
        blocks_14: Blocks14Param,
        blocks_15: Blocks15Param,
        blocks_16: Blocks16Param,
        blocks_17: Blocks17Param,
        blocks_18: Blocks18Param,
        blocks_19: Blocks19Param,
        blocks_20: Blocks20Param,
        blocks_21: Blocks21Param,
        blocks_22: Blocks22Param,
        blocks_23: Blocks23Param,
        blocks_24: Blocks24Param,
        blocks_25: Blocks25Param,
        blocks_26: Blocks26Param,
        blocks_27: Blocks27Param,
        blocks_28: Blocks28Param,
        blocks_29: Blocks29Param,
        blocks_30: Blocks30Param,
        blocks_31: Blocks31Param,
        blocks_32: Blocks32Param,
        blocks_33: Blocks33Param,
        blocks_34: Blocks34Param,
        blocks_35: Blocks35Param,
        blocks_36: Blocks36Param,
        blocks_37: Blocks37Param,
        blocks_38: Blocks38Param,
        blocks_39: Blocks39Param,
        blocks_40: Blocks40Param,
        blocks_41: Blocks41Param,
        blocks_42: Blocks42Param,
        blocks_43: Blocks43Param,
        blocks_44: Blocks44Param,
        blocks_45: Blocks45Param,
        blocks_46: Blocks46Param,
        blocks_47: Blocks47Param,
        final_layer: FinalLayerParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosFrequenciesParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    T5YEmbedderParam: crate::nodes::types::Float,
    T5YprojParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    Blocks40Param: crate::nodes::types::Float,
    Blocks41Param: crate::nodes::types::Float,
    Blocks42Param: crate::nodes::types::Float,
    Blocks43Param: crate::nodes::types::Float,
    Blocks44Param: crate::nodes::types::Float,
    Blocks45Param: crate::nodes::types::Float,
    Blocks46Param: crate::nodes::types::Float,
    Blocks47Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeMochiPreview<
    Model1Param,
    Model2Param,
    PosFrequenciesParam,
    TEmbedderParam,
    T5YEmbedderParam,
    T5YprojParam,
    Blocks0Param,
    Blocks1Param,
    Blocks2Param,
    Blocks3Param,
    Blocks4Param,
    Blocks5Param,
    Blocks6Param,
    Blocks7Param,
    Blocks8Param,
    Blocks9Param,
    Blocks10Param,
    Blocks11Param,
    Blocks12Param,
    Blocks13Param,
    Blocks14Param,
    Blocks15Param,
    Blocks16Param,
    Blocks17Param,
    Blocks18Param,
    Blocks19Param,
    Blocks20Param,
    Blocks21Param,
    Blocks22Param,
    Blocks23Param,
    Blocks24Param,
    Blocks25Param,
    Blocks26Param,
    Blocks27Param,
    Blocks28Param,
    Blocks29Param,
    Blocks30Param,
    Blocks31Param,
    Blocks32Param,
    Blocks33Param,
    Blocks34Param,
    Blocks35Param,
    Blocks36Param,
    Blocks37Param,
    Blocks38Param,
    Blocks39Param,
    Blocks40Param,
    Blocks41Param,
    Blocks42Param,
    Blocks43Param,
    Blocks44Param,
    Blocks45Param,
    Blocks46Param,
    Blocks47Param,
    FinalLayerParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeSD1<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbedParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmbParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_9: InputBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_10: InputBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_11: InputBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_9: OutputBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_10: OutputBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_11: OutputBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> ModelMergeSD1<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    InputBlocks9Param,
    InputBlocks10Param,
    InputBlocks11Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutputBlocks9Param,
    OutputBlocks10Param,
    OutputBlocks11Param,
    OutParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        time_embed: TimeEmbedParam,
        label_emb: LabelEmbParam,
        input_blocks_0: InputBlocks0Param,
        input_blocks_1: InputBlocks1Param,
        input_blocks_2: InputBlocks2Param,
        input_blocks_3: InputBlocks3Param,
        input_blocks_4: InputBlocks4Param,
        input_blocks_5: InputBlocks5Param,
        input_blocks_6: InputBlocks6Param,
        input_blocks_7: InputBlocks7Param,
        input_blocks_8: InputBlocks8Param,
        input_blocks_9: InputBlocks9Param,
        input_blocks_10: InputBlocks10Param,
        input_blocks_11: InputBlocks11Param,
        middle_block_0: MiddleBlock0Param,
        middle_block_1: MiddleBlock1Param,
        middle_block_2: MiddleBlock2Param,
        output_blocks_0: OutputBlocks0Param,
        output_blocks_1: OutputBlocks1Param,
        output_blocks_2: OutputBlocks2Param,
        output_blocks_3: OutputBlocks3Param,
        output_blocks_4: OutputBlocks4Param,
        output_blocks_5: OutputBlocks5Param,
        output_blocks_6: OutputBlocks6Param,
        output_blocks_7: OutputBlocks7Param,
        output_blocks_8: OutputBlocks8Param,
        output_blocks_9: OutputBlocks9Param,
        output_blocks_10: OutputBlocks10Param,
        output_blocks_11: OutputBlocks11Param,
        out: OutParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSD1<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    InputBlocks9Param,
    InputBlocks10Param,
    InputBlocks11Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutputBlocks9Param,
    OutputBlocks10Param,
    OutputBlocks11Param,
    OutParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeSD2<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbedParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmbParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_9: InputBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_10: InputBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_11: InputBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_9: OutputBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_10: OutputBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_11: OutputBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> ModelMergeSD2<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    InputBlocks9Param,
    InputBlocks10Param,
    InputBlocks11Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutputBlocks9Param,
    OutputBlocks10Param,
    OutputBlocks11Param,
    OutParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        time_embed: TimeEmbedParam,
        label_emb: LabelEmbParam,
        input_blocks_0: InputBlocks0Param,
        input_blocks_1: InputBlocks1Param,
        input_blocks_2: InputBlocks2Param,
        input_blocks_3: InputBlocks3Param,
        input_blocks_4: InputBlocks4Param,
        input_blocks_5: InputBlocks5Param,
        input_blocks_6: InputBlocks6Param,
        input_blocks_7: InputBlocks7Param,
        input_blocks_8: InputBlocks8Param,
        input_blocks_9: InputBlocks9Param,
        input_blocks_10: InputBlocks10Param,
        input_blocks_11: InputBlocks11Param,
        middle_block_0: MiddleBlock0Param,
        middle_block_1: MiddleBlock1Param,
        middle_block_2: MiddleBlock2Param,
        output_blocks_0: OutputBlocks0Param,
        output_blocks_1: OutputBlocks1Param,
        output_blocks_2: OutputBlocks2Param,
        output_blocks_3: OutputBlocks3Param,
        output_blocks_4: OutputBlocks4Param,
        output_blocks_5: OutputBlocks5Param,
        output_blocks_6: OutputBlocks6Param,
        output_blocks_7: OutputBlocks7Param,
        output_blocks_8: OutputBlocks8Param,
        output_blocks_9: OutputBlocks9Param,
        output_blocks_10: OutputBlocks10Param,
        output_blocks_11: OutputBlocks11Param,
        out: OutParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    InputBlocks9Param: crate::nodes::types::Float,
    InputBlocks10Param: crate::nodes::types::Float,
    InputBlocks11Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutputBlocks9Param: crate::nodes::types::Float,
    OutputBlocks10Param: crate::nodes::types::Float,
    OutputBlocks11Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSD2<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    InputBlocks9Param,
    InputBlocks10Param,
    InputBlocks11Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutputBlocks9Param,
    OutputBlocks10Param,
    OutputBlocks11Param,
    OutParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeSD35_Large<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    JointBlocks24Param: crate::nodes::types::Float,
    JointBlocks25Param: crate::nodes::types::Float,
    JointBlocks26Param: crate::nodes::types::Float,
    JointBlocks27Param: crate::nodes::types::Float,
    JointBlocks28Param: crate::nodes::types::Float,
    JointBlocks29Param: crate::nodes::types::Float,
    JointBlocks30Param: crate::nodes::types::Float,
    JointBlocks31Param: crate::nodes::types::Float,
    JointBlocks32Param: crate::nodes::types::Float,
    JointBlocks33Param: crate::nodes::types::Float,
    JointBlocks34Param: crate::nodes::types::Float,
    JointBlocks35Param: crate::nodes::types::Float,
    JointBlocks36Param: crate::nodes::types::Float,
    JointBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embed: PosEmbedParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub context_embedder: ContextEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y_embedder: YEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_0: JointBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_1: JointBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_2: JointBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_3: JointBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_4: JointBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_5: JointBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_6: JointBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_7: JointBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_8: JointBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_9: JointBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_10: JointBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_11: JointBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_12: JointBlocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_13: JointBlocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_14: JointBlocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_15: JointBlocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_16: JointBlocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_17: JointBlocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_18: JointBlocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_19: JointBlocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_20: JointBlocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_21: JointBlocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_22: JointBlocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_23: JointBlocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_24: JointBlocks24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_25: JointBlocks25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_26: JointBlocks26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_27: JointBlocks27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_28: JointBlocks28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_29: JointBlocks29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_30: JointBlocks30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_31: JointBlocks31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_32: JointBlocks32Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_33: JointBlocks33Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_34: JointBlocks34Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_35: JointBlocks35Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_36: JointBlocks36Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_37: JointBlocks37Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    JointBlocks24Param: crate::nodes::types::Float,
    JointBlocks25Param: crate::nodes::types::Float,
    JointBlocks26Param: crate::nodes::types::Float,
    JointBlocks27Param: crate::nodes::types::Float,
    JointBlocks28Param: crate::nodes::types::Float,
    JointBlocks29Param: crate::nodes::types::Float,
    JointBlocks30Param: crate::nodes::types::Float,
    JointBlocks31Param: crate::nodes::types::Float,
    JointBlocks32Param: crate::nodes::types::Float,
    JointBlocks33Param: crate::nodes::types::Float,
    JointBlocks34Param: crate::nodes::types::Float,
    JointBlocks35Param: crate::nodes::types::Float,
    JointBlocks36Param: crate::nodes::types::Float,
    JointBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeSD35_Large<
    Model1Param,
    Model2Param,
    PosEmbedParam,
    XEmbedderParam,
    ContextEmbedderParam,
    YEmbedderParam,
    TEmbedderParam,
    JointBlocks0Param,
    JointBlocks1Param,
    JointBlocks2Param,
    JointBlocks3Param,
    JointBlocks4Param,
    JointBlocks5Param,
    JointBlocks6Param,
    JointBlocks7Param,
    JointBlocks8Param,
    JointBlocks9Param,
    JointBlocks10Param,
    JointBlocks11Param,
    JointBlocks12Param,
    JointBlocks13Param,
    JointBlocks14Param,
    JointBlocks15Param,
    JointBlocks16Param,
    JointBlocks17Param,
    JointBlocks18Param,
    JointBlocks19Param,
    JointBlocks20Param,
    JointBlocks21Param,
    JointBlocks22Param,
    JointBlocks23Param,
    JointBlocks24Param,
    JointBlocks25Param,
    JointBlocks26Param,
    JointBlocks27Param,
    JointBlocks28Param,
    JointBlocks29Param,
    JointBlocks30Param,
    JointBlocks31Param,
    JointBlocks32Param,
    JointBlocks33Param,
    JointBlocks34Param,
    JointBlocks35Param,
    JointBlocks36Param,
    JointBlocks37Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        pos_embed: PosEmbedParam,
        x_embedder: XEmbedderParam,
        context_embedder: ContextEmbedderParam,
        y_embedder: YEmbedderParam,
        t_embedder: TEmbedderParam,
        joint_blocks_0: JointBlocks0Param,
        joint_blocks_1: JointBlocks1Param,
        joint_blocks_2: JointBlocks2Param,
        joint_blocks_3: JointBlocks3Param,
        joint_blocks_4: JointBlocks4Param,
        joint_blocks_5: JointBlocks5Param,
        joint_blocks_6: JointBlocks6Param,
        joint_blocks_7: JointBlocks7Param,
        joint_blocks_8: JointBlocks8Param,
        joint_blocks_9: JointBlocks9Param,
        joint_blocks_10: JointBlocks10Param,
        joint_blocks_11: JointBlocks11Param,
        joint_blocks_12: JointBlocks12Param,
        joint_blocks_13: JointBlocks13Param,
        joint_blocks_14: JointBlocks14Param,
        joint_blocks_15: JointBlocks15Param,
        joint_blocks_16: JointBlocks16Param,
        joint_blocks_17: JointBlocks17Param,
        joint_blocks_18: JointBlocks18Param,
        joint_blocks_19: JointBlocks19Param,
        joint_blocks_20: JointBlocks20Param,
        joint_blocks_21: JointBlocks21Param,
        joint_blocks_22: JointBlocks22Param,
        joint_blocks_23: JointBlocks23Param,
        joint_blocks_24: JointBlocks24Param,
        joint_blocks_25: JointBlocks25Param,
        joint_blocks_26: JointBlocks26Param,
        joint_blocks_27: JointBlocks27Param,
        joint_blocks_28: JointBlocks28Param,
        joint_blocks_29: JointBlocks29Param,
        joint_blocks_30: JointBlocks30Param,
        joint_blocks_31: JointBlocks31Param,
        joint_blocks_32: JointBlocks32Param,
        joint_blocks_33: JointBlocks33Param,
        joint_blocks_34: JointBlocks34Param,
        joint_blocks_35: JointBlocks35Param,
        joint_blocks_36: JointBlocks36Param,
        joint_blocks_37: JointBlocks37Param,
        final_layer: FinalLayerParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    JointBlocks24Param: crate::nodes::types::Float,
    JointBlocks25Param: crate::nodes::types::Float,
    JointBlocks26Param: crate::nodes::types::Float,
    JointBlocks27Param: crate::nodes::types::Float,
    JointBlocks28Param: crate::nodes::types::Float,
    JointBlocks29Param: crate::nodes::types::Float,
    JointBlocks30Param: crate::nodes::types::Float,
    JointBlocks31Param: crate::nodes::types::Float,
    JointBlocks32Param: crate::nodes::types::Float,
    JointBlocks33Param: crate::nodes::types::Float,
    JointBlocks34Param: crate::nodes::types::Float,
    JointBlocks35Param: crate::nodes::types::Float,
    JointBlocks36Param: crate::nodes::types::Float,
    JointBlocks37Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSD35_Large<
    Model1Param,
    Model2Param,
    PosEmbedParam,
    XEmbedderParam,
    ContextEmbedderParam,
    YEmbedderParam,
    TEmbedderParam,
    JointBlocks0Param,
    JointBlocks1Param,
    JointBlocks2Param,
    JointBlocks3Param,
    JointBlocks4Param,
    JointBlocks5Param,
    JointBlocks6Param,
    JointBlocks7Param,
    JointBlocks8Param,
    JointBlocks9Param,
    JointBlocks10Param,
    JointBlocks11Param,
    JointBlocks12Param,
    JointBlocks13Param,
    JointBlocks14Param,
    JointBlocks15Param,
    JointBlocks16Param,
    JointBlocks17Param,
    JointBlocks18Param,
    JointBlocks19Param,
    JointBlocks20Param,
    JointBlocks21Param,
    JointBlocks22Param,
    JointBlocks23Param,
    JointBlocks24Param,
    JointBlocks25Param,
    JointBlocks26Param,
    JointBlocks27Param,
    JointBlocks28Param,
    JointBlocks29Param,
    JointBlocks30Param,
    JointBlocks31Param,
    JointBlocks32Param,
    JointBlocks33Param,
    JointBlocks34Param,
    JointBlocks35Param,
    JointBlocks36Param,
    JointBlocks37Param,
    FinalLayerParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeSD3_2B<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub pos_embed: PosEmbedParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub x_embedder: XEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub context_embedder: ContextEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub y_embedder: YEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub t_embedder: TEmbedderParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_0: JointBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_1: JointBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_2: JointBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_3: JointBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_4: JointBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_5: JointBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_6: JointBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_7: JointBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_8: JointBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_9: JointBlocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_10: JointBlocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_11: JointBlocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_12: JointBlocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_13: JointBlocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_14: JointBlocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_15: JointBlocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_16: JointBlocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_17: JointBlocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_18: JointBlocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_19: JointBlocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_20: JointBlocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_21: JointBlocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_22: JointBlocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub joint_blocks_23: JointBlocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub final_layer: FinalLayerParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> ModelMergeSD3_2B<
    Model1Param,
    Model2Param,
    PosEmbedParam,
    XEmbedderParam,
    ContextEmbedderParam,
    YEmbedderParam,
    TEmbedderParam,
    JointBlocks0Param,
    JointBlocks1Param,
    JointBlocks2Param,
    JointBlocks3Param,
    JointBlocks4Param,
    JointBlocks5Param,
    JointBlocks6Param,
    JointBlocks7Param,
    JointBlocks8Param,
    JointBlocks9Param,
    JointBlocks10Param,
    JointBlocks11Param,
    JointBlocks12Param,
    JointBlocks13Param,
    JointBlocks14Param,
    JointBlocks15Param,
    JointBlocks16Param,
    JointBlocks17Param,
    JointBlocks18Param,
    JointBlocks19Param,
    JointBlocks20Param,
    JointBlocks21Param,
    JointBlocks22Param,
    JointBlocks23Param,
    FinalLayerParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        pos_embed: PosEmbedParam,
        x_embedder: XEmbedderParam,
        context_embedder: ContextEmbedderParam,
        y_embedder: YEmbedderParam,
        t_embedder: TEmbedderParam,
        joint_blocks_0: JointBlocks0Param,
        joint_blocks_1: JointBlocks1Param,
        joint_blocks_2: JointBlocks2Param,
        joint_blocks_3: JointBlocks3Param,
        joint_blocks_4: JointBlocks4Param,
        joint_blocks_5: JointBlocks5Param,
        joint_blocks_6: JointBlocks6Param,
        joint_blocks_7: JointBlocks7Param,
        joint_blocks_8: JointBlocks8Param,
        joint_blocks_9: JointBlocks9Param,
        joint_blocks_10: JointBlocks10Param,
        joint_blocks_11: JointBlocks11Param,
        joint_blocks_12: JointBlocks12Param,
        joint_blocks_13: JointBlocks13Param,
        joint_blocks_14: JointBlocks14Param,
        joint_blocks_15: JointBlocks15Param,
        joint_blocks_16: JointBlocks16Param,
        joint_blocks_17: JointBlocks17Param,
        joint_blocks_18: JointBlocks18Param,
        joint_blocks_19: JointBlocks19Param,
        joint_blocks_20: JointBlocks20Param,
        joint_blocks_21: JointBlocks21Param,
        joint_blocks_22: JointBlocks22Param,
        joint_blocks_23: JointBlocks23Param,
        final_layer: FinalLayerParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PosEmbedParam: crate::nodes::types::Float,
    XEmbedderParam: crate::nodes::types::Float,
    ContextEmbedderParam: crate::nodes::types::Float,
    YEmbedderParam: crate::nodes::types::Float,
    TEmbedderParam: crate::nodes::types::Float,
    JointBlocks0Param: crate::nodes::types::Float,
    JointBlocks1Param: crate::nodes::types::Float,
    JointBlocks2Param: crate::nodes::types::Float,
    JointBlocks3Param: crate::nodes::types::Float,
    JointBlocks4Param: crate::nodes::types::Float,
    JointBlocks5Param: crate::nodes::types::Float,
    JointBlocks6Param: crate::nodes::types::Float,
    JointBlocks7Param: crate::nodes::types::Float,
    JointBlocks8Param: crate::nodes::types::Float,
    JointBlocks9Param: crate::nodes::types::Float,
    JointBlocks10Param: crate::nodes::types::Float,
    JointBlocks11Param: crate::nodes::types::Float,
    JointBlocks12Param: crate::nodes::types::Float,
    JointBlocks13Param: crate::nodes::types::Float,
    JointBlocks14Param: crate::nodes::types::Float,
    JointBlocks15Param: crate::nodes::types::Float,
    JointBlocks16Param: crate::nodes::types::Float,
    JointBlocks17Param: crate::nodes::types::Float,
    JointBlocks18Param: crate::nodes::types::Float,
    JointBlocks19Param: crate::nodes::types::Float,
    JointBlocks20Param: crate::nodes::types::Float,
    JointBlocks21Param: crate::nodes::types::Float,
    JointBlocks22Param: crate::nodes::types::Float,
    JointBlocks23Param: crate::nodes::types::Float,
    FinalLayerParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSD3_2B<
    Model1Param,
    Model2Param,
    PosEmbedParam,
    XEmbedderParam,
    ContextEmbedderParam,
    YEmbedderParam,
    TEmbedderParam,
    JointBlocks0Param,
    JointBlocks1Param,
    JointBlocks2Param,
    JointBlocks3Param,
    JointBlocks4Param,
    JointBlocks5Param,
    JointBlocks6Param,
    JointBlocks7Param,
    JointBlocks8Param,
    JointBlocks9Param,
    JointBlocks10Param,
    JointBlocks11Param,
    JointBlocks12Param,
    JointBlocks13Param,
    JointBlocks14Param,
    JointBlocks15Param,
    JointBlocks16Param,
    JointBlocks17Param,
    JointBlocks18Param,
    JointBlocks19Param,
    JointBlocks20Param,
    JointBlocks21Param,
    JointBlocks22Param,
    JointBlocks23Param,
    FinalLayerParam,
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
#[allow(non_camel_case_types)]
pub struct ModelMergeSDXL<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embed: TimeEmbedParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub label_emb: LabelEmbParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_0: InputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_1: InputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_2: InputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_3: InputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_4: InputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_5: InputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_6: InputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_7: InputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub input_blocks_8: InputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_0: MiddleBlock0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_1: MiddleBlock1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub middle_block_2: MiddleBlock2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_0: OutputBlocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_1: OutputBlocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_2: OutputBlocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_3: OutputBlocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_4: OutputBlocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_5: OutputBlocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_6: OutputBlocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_7: OutputBlocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub output_blocks_8: OutputBlocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub out: OutParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> ModelMergeSDXL<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        time_embed: TimeEmbedParam,
        label_emb: LabelEmbParam,
        input_blocks_0: InputBlocks0Param,
        input_blocks_1: InputBlocks1Param,
        input_blocks_2: InputBlocks2Param,
        input_blocks_3: InputBlocks3Param,
        input_blocks_4: InputBlocks4Param,
        input_blocks_5: InputBlocks5Param,
        input_blocks_6: InputBlocks6Param,
        input_blocks_7: InputBlocks7Param,
        input_blocks_8: InputBlocks8Param,
        middle_block_0: MiddleBlock0Param,
        middle_block_1: MiddleBlock1Param,
        middle_block_2: MiddleBlock2Param,
        output_blocks_0: OutputBlocks0Param,
        output_blocks_1: OutputBlocks1Param,
        output_blocks_2: OutputBlocks2Param,
        output_blocks_3: OutputBlocks3Param,
        output_blocks_4: OutputBlocks4Param,
        output_blocks_5: OutputBlocks5Param,
        output_blocks_6: OutputBlocks6Param,
        output_blocks_7: OutputBlocks7Param,
        output_blocks_8: OutputBlocks8Param,
        out: OutParam,
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
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    TimeEmbedParam: crate::nodes::types::Float,
    LabelEmbParam: crate::nodes::types::Float,
    InputBlocks0Param: crate::nodes::types::Float,
    InputBlocks1Param: crate::nodes::types::Float,
    InputBlocks2Param: crate::nodes::types::Float,
    InputBlocks3Param: crate::nodes::types::Float,
    InputBlocks4Param: crate::nodes::types::Float,
    InputBlocks5Param: crate::nodes::types::Float,
    InputBlocks6Param: crate::nodes::types::Float,
    InputBlocks7Param: crate::nodes::types::Float,
    InputBlocks8Param: crate::nodes::types::Float,
    MiddleBlock0Param: crate::nodes::types::Float,
    MiddleBlock1Param: crate::nodes::types::Float,
    MiddleBlock2Param: crate::nodes::types::Float,
    OutputBlocks0Param: crate::nodes::types::Float,
    OutputBlocks1Param: crate::nodes::types::Float,
    OutputBlocks2Param: crate::nodes::types::Float,
    OutputBlocks3Param: crate::nodes::types::Float,
    OutputBlocks4Param: crate::nodes::types::Float,
    OutputBlocks5Param: crate::nodes::types::Float,
    OutputBlocks6Param: crate::nodes::types::Float,
    OutputBlocks7Param: crate::nodes::types::Float,
    OutputBlocks8Param: crate::nodes::types::Float,
    OutParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeSDXL<
    Model1Param,
    Model2Param,
    TimeEmbedParam,
    LabelEmbParam,
    InputBlocks0Param,
    InputBlocks1Param,
    InputBlocks2Param,
    InputBlocks3Param,
    InputBlocks4Param,
    InputBlocks5Param,
    InputBlocks6Param,
    InputBlocks7Param,
    InputBlocks8Param,
    MiddleBlock0Param,
    MiddleBlock1Param,
    MiddleBlock2Param,
    OutputBlocks0Param,
    OutputBlocks1Param,
    OutputBlocks2Param,
    OutputBlocks3Param,
    OutputBlocks4Param,
    OutputBlocks5Param,
    OutputBlocks6Param,
    OutputBlocks7Param,
    OutputBlocks8Param,
    OutParam,
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
///**ModelMergeWAN2_1**: 1.3B model has 30 blocks, 14B model has 40 blocks. Image to video model has the extra img_emb.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ModelMergeWAN2_1<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchEmbeddingParam: crate::nodes::types::Float,
    TimeEmbeddingParam: crate::nodes::types::Float,
    TimeProjectionParam: crate::nodes::types::Float,
    TextEmbeddingParam: crate::nodes::types::Float,
    ImgEmbParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    HeadParam: crate::nodes::types::Float,
> {
    ///No documentation.
    pub model_1: Model1Param,
    ///No documentation.
    pub model_2: Model2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub patch_embedding: PatchEmbeddingParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_embedding: TimeEmbeddingParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub time_projection: TimeProjectionParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub text_embedding: TextEmbeddingParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub img_emb: ImgEmbParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_0: Blocks0Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_1: Blocks1Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_2: Blocks2Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_3: Blocks3Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_4: Blocks4Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_5: Blocks5Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_6: Blocks6Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_7: Blocks7Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_8: Blocks8Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_9: Blocks9Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_10: Blocks10Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_11: Blocks11Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_12: Blocks12Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_13: Blocks13Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_14: Blocks14Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_15: Blocks15Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_16: Blocks16Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_17: Blocks17Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_18: Blocks18Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_19: Blocks19Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_20: Blocks20Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_21: Blocks21Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_22: Blocks22Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_23: Blocks23Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_24: Blocks24Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_25: Blocks25Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_26: Blocks26Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_27: Blocks27Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_28: Blocks28Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_29: Blocks29Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_30: Blocks30Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_31: Blocks31Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_32: Blocks32Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_33: Blocks33Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_34: Blocks34Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_35: Blocks35Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_36: Blocks36Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_37: Blocks37Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_38: Blocks38Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub blocks_39: Blocks39Param,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 1
  - Min: 0
  - Step: 0.01
*/
    pub head: HeadParam,
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchEmbeddingParam: crate::nodes::types::Float,
    TimeEmbeddingParam: crate::nodes::types::Float,
    TimeProjectionParam: crate::nodes::types::Float,
    TextEmbeddingParam: crate::nodes::types::Float,
    ImgEmbParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    HeadParam: crate::nodes::types::Float,
> ModelMergeWAN2_1<
    Model1Param,
    Model2Param,
    PatchEmbeddingParam,
    TimeEmbeddingParam,
    TimeProjectionParam,
    TextEmbeddingParam,
    ImgEmbParam,
    Blocks0Param,
    Blocks1Param,
    Blocks2Param,
    Blocks3Param,
    Blocks4Param,
    Blocks5Param,
    Blocks6Param,
    Blocks7Param,
    Blocks8Param,
    Blocks9Param,
    Blocks10Param,
    Blocks11Param,
    Blocks12Param,
    Blocks13Param,
    Blocks14Param,
    Blocks15Param,
    Blocks16Param,
    Blocks17Param,
    Blocks18Param,
    Blocks19Param,
    Blocks20Param,
    Blocks21Param,
    Blocks22Param,
    Blocks23Param,
    Blocks24Param,
    Blocks25Param,
    Blocks26Param,
    Blocks27Param,
    Blocks28Param,
    Blocks29Param,
    Blocks30Param,
    Blocks31Param,
    Blocks32Param,
    Blocks33Param,
    Blocks34Param,
    Blocks35Param,
    Blocks36Param,
    Blocks37Param,
    Blocks38Param,
    Blocks39Param,
    HeadParam,
> {
    /// Create a new node.
    pub fn new(
        model_1: Model1Param,
        model_2: Model2Param,
        patch_embedding: PatchEmbeddingParam,
        time_embedding: TimeEmbeddingParam,
        time_projection: TimeProjectionParam,
        text_embedding: TextEmbeddingParam,
        img_emb: ImgEmbParam,
        blocks_0: Blocks0Param,
        blocks_1: Blocks1Param,
        blocks_2: Blocks2Param,
        blocks_3: Blocks3Param,
        blocks_4: Blocks4Param,
        blocks_5: Blocks5Param,
        blocks_6: Blocks6Param,
        blocks_7: Blocks7Param,
        blocks_8: Blocks8Param,
        blocks_9: Blocks9Param,
        blocks_10: Blocks10Param,
        blocks_11: Blocks11Param,
        blocks_12: Blocks12Param,
        blocks_13: Blocks13Param,
        blocks_14: Blocks14Param,
        blocks_15: Blocks15Param,
        blocks_16: Blocks16Param,
        blocks_17: Blocks17Param,
        blocks_18: Blocks18Param,
        blocks_19: Blocks19Param,
        blocks_20: Blocks20Param,
        blocks_21: Blocks21Param,
        blocks_22: Blocks22Param,
        blocks_23: Blocks23Param,
        blocks_24: Blocks24Param,
        blocks_25: Blocks25Param,
        blocks_26: Blocks26Param,
        blocks_27: Blocks27Param,
        blocks_28: Blocks28Param,
        blocks_29: Blocks29Param,
        blocks_30: Blocks30Param,
        blocks_31: Blocks31Param,
        blocks_32: Blocks32Param,
        blocks_33: Blocks33Param,
        blocks_34: Blocks34Param,
        blocks_35: Blocks35Param,
        blocks_36: Blocks36Param,
        blocks_37: Blocks37Param,
        blocks_38: Blocks38Param,
        blocks_39: Blocks39Param,
        head: HeadParam,
    ) -> Self {
        Self {
            model_1,
            model_2,
            patch_embedding,
            time_embedding,
            time_projection,
            text_embedding,
            img_emb,
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
            head,
        }
    }
}
impl<
    Model1Param: crate::nodes::types::Model,
    Model2Param: crate::nodes::types::Model,
    PatchEmbeddingParam: crate::nodes::types::Float,
    TimeEmbeddingParam: crate::nodes::types::Float,
    TimeProjectionParam: crate::nodes::types::Float,
    TextEmbeddingParam: crate::nodes::types::Float,
    ImgEmbParam: crate::nodes::types::Float,
    Blocks0Param: crate::nodes::types::Float,
    Blocks1Param: crate::nodes::types::Float,
    Blocks2Param: crate::nodes::types::Float,
    Blocks3Param: crate::nodes::types::Float,
    Blocks4Param: crate::nodes::types::Float,
    Blocks5Param: crate::nodes::types::Float,
    Blocks6Param: crate::nodes::types::Float,
    Blocks7Param: crate::nodes::types::Float,
    Blocks8Param: crate::nodes::types::Float,
    Blocks9Param: crate::nodes::types::Float,
    Blocks10Param: crate::nodes::types::Float,
    Blocks11Param: crate::nodes::types::Float,
    Blocks12Param: crate::nodes::types::Float,
    Blocks13Param: crate::nodes::types::Float,
    Blocks14Param: crate::nodes::types::Float,
    Blocks15Param: crate::nodes::types::Float,
    Blocks16Param: crate::nodes::types::Float,
    Blocks17Param: crate::nodes::types::Float,
    Blocks18Param: crate::nodes::types::Float,
    Blocks19Param: crate::nodes::types::Float,
    Blocks20Param: crate::nodes::types::Float,
    Blocks21Param: crate::nodes::types::Float,
    Blocks22Param: crate::nodes::types::Float,
    Blocks23Param: crate::nodes::types::Float,
    Blocks24Param: crate::nodes::types::Float,
    Blocks25Param: crate::nodes::types::Float,
    Blocks26Param: crate::nodes::types::Float,
    Blocks27Param: crate::nodes::types::Float,
    Blocks28Param: crate::nodes::types::Float,
    Blocks29Param: crate::nodes::types::Float,
    Blocks30Param: crate::nodes::types::Float,
    Blocks31Param: crate::nodes::types::Float,
    Blocks32Param: crate::nodes::types::Float,
    Blocks33Param: crate::nodes::types::Float,
    Blocks34Param: crate::nodes::types::Float,
    Blocks35Param: crate::nodes::types::Float,
    Blocks36Param: crate::nodes::types::Float,
    Blocks37Param: crate::nodes::types::Float,
    Blocks38Param: crate::nodes::types::Float,
    Blocks39Param: crate::nodes::types::Float,
    HeadParam: crate::nodes::types::Float,
> crate::nodes::TypedNode
for ModelMergeWAN2_1<
    Model1Param,
    Model2Param,
    PatchEmbeddingParam,
    TimeEmbeddingParam,
    TimeProjectionParam,
    TextEmbeddingParam,
    ImgEmbParam,
    Blocks0Param,
    Blocks1Param,
    Blocks2Param,
    Blocks3Param,
    Blocks4Param,
    Blocks5Param,
    Blocks6Param,
    Blocks7Param,
    Blocks8Param,
    Blocks9Param,
    Blocks10Param,
    Blocks11Param,
    Blocks12Param,
    Blocks13Param,
    Blocks14Param,
    Blocks15Param,
    Blocks16Param,
    Blocks17Param,
    Blocks18Param,
    Blocks19Param,
    Blocks20Param,
    Blocks21Param,
    Blocks22Param,
    Blocks23Param,
    Blocks24Param,
    Blocks25Param,
    Blocks26Param,
    Blocks27Param,
    Blocks28Param,
    Blocks29Param,
    Blocks30Param,
    Blocks31Param,
    Blocks32Param,
    Blocks33Param,
    Blocks34Param,
    Blocks35Param,
    Blocks36Param,
    Blocks37Param,
    Blocks38Param,
    Blocks39Param,
    HeadParam,
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
            .insert("patch_embedding.".to_string(), self.patch_embedding.clone().into());
        output.insert("time_embedding.".to_string(), self.time_embedding.clone().into());
        output
            .insert("time_projection.".to_string(), self.time_projection.clone().into());
        output.insert("text_embedding.".to_string(), self.text_embedding.clone().into());
        output.insert("img_emb.".to_string(), self.img_emb.clone().into());
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
        output.insert("head.".to_string(), self.head.clone().into());
        output
    }
    const NAME: &'static str = "ModelMergeWAN2_1";
    const DISPLAY_NAME: &'static str = "ModelMergeWAN2_1";
    const DESCRIPTION: &'static str = "1.3B model has 30 blocks, 14B model has 40 blocks. Image to video model has the extra img_emb.";
    const CATEGORY: &'static str = "advanced/model_merging/model_specific";
}
