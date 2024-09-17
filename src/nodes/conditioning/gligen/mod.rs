//!gligen
///**GLIGENTextBoxApply**
pub struct GligenTextBoxApply<
    ConditioningTo: crate::nodes::Conditioning,
    Clip: crate::nodes::Clip,
    GligenTextboxModel: crate::nodes::Gligen,
    Text: crate::nodes::String,
    Width: crate::nodes::Int,
    Height: crate::nodes::Int,
    X: crate::nodes::Int,
    Y: crate::nodes::Int,
> {
    ///No documentation.
    pub conditioning_to: ConditioningTo,
    ///No documentation.
    pub clip: Clip,
    ///No documentation.
    pub gligen_textbox_model: GligenTextboxModel,
    ///No documentation.
    pub text: Text,
    ///No documentation.
    pub width: Width,
    ///No documentation.
    pub height: Height,
    ///No documentation.
    pub x: X,
    ///No documentation.
    pub y: Y,
}
