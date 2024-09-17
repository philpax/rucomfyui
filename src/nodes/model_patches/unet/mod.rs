//!unet
///**FreeU**
pub struct FreeU<
    Model: crate::nodes::Model,
    B1: crate::nodes::Float,
    B2: crate::nodes::Float,
    S1: crate::nodes::Float,
    S2: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub b1: B1,
    ///No documentation.
    pub b2: B2,
    ///No documentation.
    pub s1: S1,
    ///No documentation.
    pub s2: S2,
}
///**FreeU_V2**
pub struct FreeUV2<
    Model: crate::nodes::Model,
    B1: crate::nodes::Float,
    B2: crate::nodes::Float,
    S1: crate::nodes::Float,
    S2: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub b1: B1,
    ///No documentation.
    pub b2: B2,
    ///No documentation.
    pub s1: S1,
    ///No documentation.
    pub s2: S2,
}
///**HyperTile**
pub struct HyperTile<
    Model: crate::nodes::Model,
    TileSize: crate::nodes::Int,
    SwapSize: crate::nodes::Int,
    MaxDepth: crate::nodes::Int,
    ScaleDepth: crate::nodes::Boolean,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub tile_size: TileSize,
    ///No documentation.
    pub swap_size: SwapSize,
    ///No documentation.
    pub max_depth: MaxDepth,
    ///No documentation.
    pub scale_depth: ScaleDepth,
}
///**PerturbedAttentionGuidance**
pub struct PerturbedAttentionGuidance<
    Model: crate::nodes::Model,
    Scale: crate::nodes::Float,
> {
    ///No documentation.
    pub model: Model,
    ///No documentation.
    pub scale: Scale,
}
