//!`model_specific` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
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
