//!model_specific
///**ModelMergeFlux1**
pub struct ModelMergeFlux1<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    ImgIn: crate::nodes::Float,
    TimeIn: crate::nodes::Float,
    GuidanceIn: crate::nodes::Float,
    VectorIn: crate::nodes::Float,
    TxtIn: crate::nodes::Float,
    DoubleBlocks0: crate::nodes::Float,
    DoubleBlocks1: crate::nodes::Float,
    DoubleBlocks2: crate::nodes::Float,
    DoubleBlocks3: crate::nodes::Float,
    DoubleBlocks4: crate::nodes::Float,
    DoubleBlocks5: crate::nodes::Float,
    DoubleBlocks6: crate::nodes::Float,
    DoubleBlocks7: crate::nodes::Float,
    DoubleBlocks8: crate::nodes::Float,
    DoubleBlocks9: crate::nodes::Float,
    DoubleBlocks10: crate::nodes::Float,
    DoubleBlocks11: crate::nodes::Float,
    DoubleBlocks12: crate::nodes::Float,
    DoubleBlocks13: crate::nodes::Float,
    DoubleBlocks14: crate::nodes::Float,
    DoubleBlocks15: crate::nodes::Float,
    DoubleBlocks16: crate::nodes::Float,
    DoubleBlocks17: crate::nodes::Float,
    DoubleBlocks18: crate::nodes::Float,
    SingleBlocks0: crate::nodes::Float,
    SingleBlocks1: crate::nodes::Float,
    SingleBlocks2: crate::nodes::Float,
    SingleBlocks3: crate::nodes::Float,
    SingleBlocks4: crate::nodes::Float,
    SingleBlocks5: crate::nodes::Float,
    SingleBlocks6: crate::nodes::Float,
    SingleBlocks7: crate::nodes::Float,
    SingleBlocks8: crate::nodes::Float,
    SingleBlocks9: crate::nodes::Float,
    SingleBlocks10: crate::nodes::Float,
    SingleBlocks11: crate::nodes::Float,
    SingleBlocks12: crate::nodes::Float,
    SingleBlocks13: crate::nodes::Float,
    SingleBlocks14: crate::nodes::Float,
    SingleBlocks15: crate::nodes::Float,
    SingleBlocks16: crate::nodes::Float,
    SingleBlocks17: crate::nodes::Float,
    SingleBlocks18: crate::nodes::Float,
    SingleBlocks19: crate::nodes::Float,
    SingleBlocks20: crate::nodes::Float,
    SingleBlocks21: crate::nodes::Float,
    SingleBlocks22: crate::nodes::Float,
    SingleBlocks23: crate::nodes::Float,
    SingleBlocks24: crate::nodes::Float,
    SingleBlocks25: crate::nodes::Float,
    SingleBlocks26: crate::nodes::Float,
    SingleBlocks27: crate::nodes::Float,
    SingleBlocks28: crate::nodes::Float,
    SingleBlocks29: crate::nodes::Float,
    SingleBlocks30: crate::nodes::Float,
    SingleBlocks31: crate::nodes::Float,
    SingleBlocks32: crate::nodes::Float,
    SingleBlocks33: crate::nodes::Float,
    SingleBlocks34: crate::nodes::Float,
    SingleBlocks35: crate::nodes::Float,
    SingleBlocks36: crate::nodes::Float,
    SingleBlocks37: crate::nodes::Float,
    FinalLayer: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub img_in_: ImgIn,
    ///No documentation.
    pub time_in_: TimeIn,
    ///No documentation.
    pub guidance_in: GuidanceIn,
    ///No documentation.
    pub vector_in_: VectorIn,
    ///No documentation.
    pub txt_in_: TxtIn,
    ///No documentation.
    pub double_blocks_0_: DoubleBlocks0,
    ///No documentation.
    pub double_blocks_1_: DoubleBlocks1,
    ///No documentation.
    pub double_blocks_2_: DoubleBlocks2,
    ///No documentation.
    pub double_blocks_3_: DoubleBlocks3,
    ///No documentation.
    pub double_blocks_4_: DoubleBlocks4,
    ///No documentation.
    pub double_blocks_5_: DoubleBlocks5,
    ///No documentation.
    pub double_blocks_6_: DoubleBlocks6,
    ///No documentation.
    pub double_blocks_7_: DoubleBlocks7,
    ///No documentation.
    pub double_blocks_8_: DoubleBlocks8,
    ///No documentation.
    pub double_blocks_9_: DoubleBlocks9,
    ///No documentation.
    pub double_blocks_10_: DoubleBlocks10,
    ///No documentation.
    pub double_blocks_11_: DoubleBlocks11,
    ///No documentation.
    pub double_blocks_12_: DoubleBlocks12,
    ///No documentation.
    pub double_blocks_13_: DoubleBlocks13,
    ///No documentation.
    pub double_blocks_14_: DoubleBlocks14,
    ///No documentation.
    pub double_blocks_15_: DoubleBlocks15,
    ///No documentation.
    pub double_blocks_16_: DoubleBlocks16,
    ///No documentation.
    pub double_blocks_17_: DoubleBlocks17,
    ///No documentation.
    pub double_blocks_18_: DoubleBlocks18,
    ///No documentation.
    pub single_blocks_0_: SingleBlocks0,
    ///No documentation.
    pub single_blocks_1_: SingleBlocks1,
    ///No documentation.
    pub single_blocks_2_: SingleBlocks2,
    ///No documentation.
    pub single_blocks_3_: SingleBlocks3,
    ///No documentation.
    pub single_blocks_4_: SingleBlocks4,
    ///No documentation.
    pub single_blocks_5_: SingleBlocks5,
    ///No documentation.
    pub single_blocks_6_: SingleBlocks6,
    ///No documentation.
    pub single_blocks_7_: SingleBlocks7,
    ///No documentation.
    pub single_blocks_8_: SingleBlocks8,
    ///No documentation.
    pub single_blocks_9_: SingleBlocks9,
    ///No documentation.
    pub single_blocks_10_: SingleBlocks10,
    ///No documentation.
    pub single_blocks_11_: SingleBlocks11,
    ///No documentation.
    pub single_blocks_12_: SingleBlocks12,
    ///No documentation.
    pub single_blocks_13_: SingleBlocks13,
    ///No documentation.
    pub single_blocks_14_: SingleBlocks14,
    ///No documentation.
    pub single_blocks_15_: SingleBlocks15,
    ///No documentation.
    pub single_blocks_16_: SingleBlocks16,
    ///No documentation.
    pub single_blocks_17_: SingleBlocks17,
    ///No documentation.
    pub single_blocks_18_: SingleBlocks18,
    ///No documentation.
    pub single_blocks_19_: SingleBlocks19,
    ///No documentation.
    pub single_blocks_20_: SingleBlocks20,
    ///No documentation.
    pub single_blocks_21_: SingleBlocks21,
    ///No documentation.
    pub single_blocks_22_: SingleBlocks22,
    ///No documentation.
    pub single_blocks_23_: SingleBlocks23,
    ///No documentation.
    pub single_blocks_24_: SingleBlocks24,
    ///No documentation.
    pub single_blocks_25_: SingleBlocks25,
    ///No documentation.
    pub single_blocks_26_: SingleBlocks26,
    ///No documentation.
    pub single_blocks_27_: SingleBlocks27,
    ///No documentation.
    pub single_blocks_28_: SingleBlocks28,
    ///No documentation.
    pub single_blocks_29_: SingleBlocks29,
    ///No documentation.
    pub single_blocks_30_: SingleBlocks30,
    ///No documentation.
    pub single_blocks_31_: SingleBlocks31,
    ///No documentation.
    pub single_blocks_32_: SingleBlocks32,
    ///No documentation.
    pub single_blocks_33_: SingleBlocks33,
    ///No documentation.
    pub single_blocks_34_: SingleBlocks34,
    ///No documentation.
    pub single_blocks_35_: SingleBlocks35,
    ///No documentation.
    pub single_blocks_36_: SingleBlocks36,
    ///No documentation.
    pub single_blocks_37_: SingleBlocks37,
    ///No documentation.
    pub final_layer_: FinalLayer,
}
///**ModelMergeSD1**
pub struct ModelMergeSd1<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    TimeEmbed: crate::nodes::Float,
    LabelEmb: crate::nodes::Float,
    InputBlocks0: crate::nodes::Float,
    InputBlocks1: crate::nodes::Float,
    InputBlocks2: crate::nodes::Float,
    InputBlocks3: crate::nodes::Float,
    InputBlocks4: crate::nodes::Float,
    InputBlocks5: crate::nodes::Float,
    InputBlocks6: crate::nodes::Float,
    InputBlocks7: crate::nodes::Float,
    InputBlocks8: crate::nodes::Float,
    InputBlocks9: crate::nodes::Float,
    InputBlocks10: crate::nodes::Float,
    InputBlocks11: crate::nodes::Float,
    MiddleBlock0: crate::nodes::Float,
    MiddleBlock1: crate::nodes::Float,
    MiddleBlock2: crate::nodes::Float,
    OutputBlocks0: crate::nodes::Float,
    OutputBlocks1: crate::nodes::Float,
    OutputBlocks2: crate::nodes::Float,
    OutputBlocks3: crate::nodes::Float,
    OutputBlocks4: crate::nodes::Float,
    OutputBlocks5: crate::nodes::Float,
    OutputBlocks6: crate::nodes::Float,
    OutputBlocks7: crate::nodes::Float,
    OutputBlocks8: crate::nodes::Float,
    OutputBlocks9: crate::nodes::Float,
    OutputBlocks10: crate::nodes::Float,
    OutputBlocks11: crate::nodes::Float,
    Out: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub time_embed_: TimeEmbed,
    ///No documentation.
    pub label_emb_: LabelEmb,
    ///No documentation.
    pub input_blocks_0_: InputBlocks0,
    ///No documentation.
    pub input_blocks_1_: InputBlocks1,
    ///No documentation.
    pub input_blocks_2_: InputBlocks2,
    ///No documentation.
    pub input_blocks_3_: InputBlocks3,
    ///No documentation.
    pub input_blocks_4_: InputBlocks4,
    ///No documentation.
    pub input_blocks_5_: InputBlocks5,
    ///No documentation.
    pub input_blocks_6_: InputBlocks6,
    ///No documentation.
    pub input_blocks_7_: InputBlocks7,
    ///No documentation.
    pub input_blocks_8_: InputBlocks8,
    ///No documentation.
    pub input_blocks_9_: InputBlocks9,
    ///No documentation.
    pub input_blocks_10_: InputBlocks10,
    ///No documentation.
    pub input_blocks_11_: InputBlocks11,
    ///No documentation.
    pub middle_block_0_: MiddleBlock0,
    ///No documentation.
    pub middle_block_1_: MiddleBlock1,
    ///No documentation.
    pub middle_block_2_: MiddleBlock2,
    ///No documentation.
    pub output_blocks_0_: OutputBlocks0,
    ///No documentation.
    pub output_blocks_1_: OutputBlocks1,
    ///No documentation.
    pub output_blocks_2_: OutputBlocks2,
    ///No documentation.
    pub output_blocks_3_: OutputBlocks3,
    ///No documentation.
    pub output_blocks_4_: OutputBlocks4,
    ///No documentation.
    pub output_blocks_5_: OutputBlocks5,
    ///No documentation.
    pub output_blocks_6_: OutputBlocks6,
    ///No documentation.
    pub output_blocks_7_: OutputBlocks7,
    ///No documentation.
    pub output_blocks_8_: OutputBlocks8,
    ///No documentation.
    pub output_blocks_9_: OutputBlocks9,
    ///No documentation.
    pub output_blocks_10_: OutputBlocks10,
    ///No documentation.
    pub output_blocks_11_: OutputBlocks11,
    ///No documentation.
    pub out_: Out,
}
///**ModelMergeSD2**
pub struct ModelMergeSd2<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    TimeEmbed: crate::nodes::Float,
    LabelEmb: crate::nodes::Float,
    InputBlocks0: crate::nodes::Float,
    InputBlocks1: crate::nodes::Float,
    InputBlocks2: crate::nodes::Float,
    InputBlocks3: crate::nodes::Float,
    InputBlocks4: crate::nodes::Float,
    InputBlocks5: crate::nodes::Float,
    InputBlocks6: crate::nodes::Float,
    InputBlocks7: crate::nodes::Float,
    InputBlocks8: crate::nodes::Float,
    InputBlocks9: crate::nodes::Float,
    InputBlocks10: crate::nodes::Float,
    InputBlocks11: crate::nodes::Float,
    MiddleBlock0: crate::nodes::Float,
    MiddleBlock1: crate::nodes::Float,
    MiddleBlock2: crate::nodes::Float,
    OutputBlocks0: crate::nodes::Float,
    OutputBlocks1: crate::nodes::Float,
    OutputBlocks2: crate::nodes::Float,
    OutputBlocks3: crate::nodes::Float,
    OutputBlocks4: crate::nodes::Float,
    OutputBlocks5: crate::nodes::Float,
    OutputBlocks6: crate::nodes::Float,
    OutputBlocks7: crate::nodes::Float,
    OutputBlocks8: crate::nodes::Float,
    OutputBlocks9: crate::nodes::Float,
    OutputBlocks10: crate::nodes::Float,
    OutputBlocks11: crate::nodes::Float,
    Out: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub time_embed_: TimeEmbed,
    ///No documentation.
    pub label_emb_: LabelEmb,
    ///No documentation.
    pub input_blocks_0_: InputBlocks0,
    ///No documentation.
    pub input_blocks_1_: InputBlocks1,
    ///No documentation.
    pub input_blocks_2_: InputBlocks2,
    ///No documentation.
    pub input_blocks_3_: InputBlocks3,
    ///No documentation.
    pub input_blocks_4_: InputBlocks4,
    ///No documentation.
    pub input_blocks_5_: InputBlocks5,
    ///No documentation.
    pub input_blocks_6_: InputBlocks6,
    ///No documentation.
    pub input_blocks_7_: InputBlocks7,
    ///No documentation.
    pub input_blocks_8_: InputBlocks8,
    ///No documentation.
    pub input_blocks_9_: InputBlocks9,
    ///No documentation.
    pub input_blocks_10_: InputBlocks10,
    ///No documentation.
    pub input_blocks_11_: InputBlocks11,
    ///No documentation.
    pub middle_block_0_: MiddleBlock0,
    ///No documentation.
    pub middle_block_1_: MiddleBlock1,
    ///No documentation.
    pub middle_block_2_: MiddleBlock2,
    ///No documentation.
    pub output_blocks_0_: OutputBlocks0,
    ///No documentation.
    pub output_blocks_1_: OutputBlocks1,
    ///No documentation.
    pub output_blocks_2_: OutputBlocks2,
    ///No documentation.
    pub output_blocks_3_: OutputBlocks3,
    ///No documentation.
    pub output_blocks_4_: OutputBlocks4,
    ///No documentation.
    pub output_blocks_5_: OutputBlocks5,
    ///No documentation.
    pub output_blocks_6_: OutputBlocks6,
    ///No documentation.
    pub output_blocks_7_: OutputBlocks7,
    ///No documentation.
    pub output_blocks_8_: OutputBlocks8,
    ///No documentation.
    pub output_blocks_9_: OutputBlocks9,
    ///No documentation.
    pub output_blocks_10_: OutputBlocks10,
    ///No documentation.
    pub output_blocks_11_: OutputBlocks11,
    ///No documentation.
    pub out_: Out,
}
///**ModelMergeSD3_2B**
pub struct ModelMergeSd32B<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    PosEmbed: crate::nodes::Float,
    XEmbedder: crate::nodes::Float,
    ContextEmbedder: crate::nodes::Float,
    YEmbedder: crate::nodes::Float,
    TEmbedder: crate::nodes::Float,
    JointBlocks0: crate::nodes::Float,
    JointBlocks1: crate::nodes::Float,
    JointBlocks2: crate::nodes::Float,
    JointBlocks3: crate::nodes::Float,
    JointBlocks4: crate::nodes::Float,
    JointBlocks5: crate::nodes::Float,
    JointBlocks6: crate::nodes::Float,
    JointBlocks7: crate::nodes::Float,
    JointBlocks8: crate::nodes::Float,
    JointBlocks9: crate::nodes::Float,
    JointBlocks10: crate::nodes::Float,
    JointBlocks11: crate::nodes::Float,
    JointBlocks12: crate::nodes::Float,
    JointBlocks13: crate::nodes::Float,
    JointBlocks14: crate::nodes::Float,
    JointBlocks15: crate::nodes::Float,
    JointBlocks16: crate::nodes::Float,
    JointBlocks17: crate::nodes::Float,
    JointBlocks18: crate::nodes::Float,
    JointBlocks19: crate::nodes::Float,
    JointBlocks20: crate::nodes::Float,
    JointBlocks21: crate::nodes::Float,
    JointBlocks22: crate::nodes::Float,
    JointBlocks23: crate::nodes::Float,
    FinalLayer: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub pos_embed_: PosEmbed,
    ///No documentation.
    pub x_embedder_: XEmbedder,
    ///No documentation.
    pub context_embedder_: ContextEmbedder,
    ///No documentation.
    pub y_embedder_: YEmbedder,
    ///No documentation.
    pub t_embedder_: TEmbedder,
    ///No documentation.
    pub joint_blocks_0_: JointBlocks0,
    ///No documentation.
    pub joint_blocks_1_: JointBlocks1,
    ///No documentation.
    pub joint_blocks_2_: JointBlocks2,
    ///No documentation.
    pub joint_blocks_3_: JointBlocks3,
    ///No documentation.
    pub joint_blocks_4_: JointBlocks4,
    ///No documentation.
    pub joint_blocks_5_: JointBlocks5,
    ///No documentation.
    pub joint_blocks_6_: JointBlocks6,
    ///No documentation.
    pub joint_blocks_7_: JointBlocks7,
    ///No documentation.
    pub joint_blocks_8_: JointBlocks8,
    ///No documentation.
    pub joint_blocks_9_: JointBlocks9,
    ///No documentation.
    pub joint_blocks_10_: JointBlocks10,
    ///No documentation.
    pub joint_blocks_11_: JointBlocks11,
    ///No documentation.
    pub joint_blocks_12_: JointBlocks12,
    ///No documentation.
    pub joint_blocks_13_: JointBlocks13,
    ///No documentation.
    pub joint_blocks_14_: JointBlocks14,
    ///No documentation.
    pub joint_blocks_15_: JointBlocks15,
    ///No documentation.
    pub joint_blocks_16_: JointBlocks16,
    ///No documentation.
    pub joint_blocks_17_: JointBlocks17,
    ///No documentation.
    pub joint_blocks_18_: JointBlocks18,
    ///No documentation.
    pub joint_blocks_19_: JointBlocks19,
    ///No documentation.
    pub joint_blocks_20_: JointBlocks20,
    ///No documentation.
    pub joint_blocks_21_: JointBlocks21,
    ///No documentation.
    pub joint_blocks_22_: JointBlocks22,
    ///No documentation.
    pub joint_blocks_23_: JointBlocks23,
    ///No documentation.
    pub final_layer_: FinalLayer,
}
///**ModelMergeSDXL**
pub struct ModelMergeSdxl<
    Model1: crate::nodes::Model,
    Model2: crate::nodes::Model,
    TimeEmbed: crate::nodes::Float,
    LabelEmb: crate::nodes::Float,
    InputBlocks0: crate::nodes::Float,
    InputBlocks1: crate::nodes::Float,
    InputBlocks2: crate::nodes::Float,
    InputBlocks3: crate::nodes::Float,
    InputBlocks4: crate::nodes::Float,
    InputBlocks5: crate::nodes::Float,
    InputBlocks6: crate::nodes::Float,
    InputBlocks7: crate::nodes::Float,
    InputBlocks8: crate::nodes::Float,
    MiddleBlock0: crate::nodes::Float,
    MiddleBlock1: crate::nodes::Float,
    MiddleBlock2: crate::nodes::Float,
    OutputBlocks0: crate::nodes::Float,
    OutputBlocks1: crate::nodes::Float,
    OutputBlocks2: crate::nodes::Float,
    OutputBlocks3: crate::nodes::Float,
    OutputBlocks4: crate::nodes::Float,
    OutputBlocks5: crate::nodes::Float,
    OutputBlocks6: crate::nodes::Float,
    OutputBlocks7: crate::nodes::Float,
    OutputBlocks8: crate::nodes::Float,
    Out: crate::nodes::Float,
> {
    ///No documentation.
    pub model1: Model1,
    ///No documentation.
    pub model2: Model2,
    ///No documentation.
    pub time_embed_: TimeEmbed,
    ///No documentation.
    pub label_emb_: LabelEmb,
    ///No documentation.
    pub input_blocks_0: InputBlocks0,
    ///No documentation.
    pub input_blocks_1: InputBlocks1,
    ///No documentation.
    pub input_blocks_2: InputBlocks2,
    ///No documentation.
    pub input_blocks_3: InputBlocks3,
    ///No documentation.
    pub input_blocks_4: InputBlocks4,
    ///No documentation.
    pub input_blocks_5: InputBlocks5,
    ///No documentation.
    pub input_blocks_6: InputBlocks6,
    ///No documentation.
    pub input_blocks_7: InputBlocks7,
    ///No documentation.
    pub input_blocks_8: InputBlocks8,
    ///No documentation.
    pub middle_block_0: MiddleBlock0,
    ///No documentation.
    pub middle_block_1: MiddleBlock1,
    ///No documentation.
    pub middle_block_2: MiddleBlock2,
    ///No documentation.
    pub output_blocks_0: OutputBlocks0,
    ///No documentation.
    pub output_blocks_1: OutputBlocks1,
    ///No documentation.
    pub output_blocks_2: OutputBlocks2,
    ///No documentation.
    pub output_blocks_3: OutputBlocks3,
    ///No documentation.
    pub output_blocks_4: OutputBlocks4,
    ///No documentation.
    pub output_blocks_5: OutputBlocks5,
    ///No documentation.
    pub output_blocks_6: OutputBlocks6,
    ///No documentation.
    pub output_blocks_7: OutputBlocks7,
    ///No documentation.
    pub output_blocks_8: OutputBlocks8,
    ///No documentation.
    pub out_: Out,
}
