//!deprecated
///**DiffusersLoader**
pub struct DiffusersLoader {}
///Output for [`DiffusersLoader`].
pub struct DiffusersLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip: crate::nodes::ClipOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl crate::nodes::TypedNode for DiffusersLoader {
    type Output = DiffusersLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0u32),
            clip: crate::nodes::ClipOut(1u32),
            vae: crate::nodes::VaeOut(2u32),
        }
    }
    const NAME: &'static str = "DiffusersLoader";
    const DISPLAY_NAME: &'static str = "DiffusersLoader";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "advanced/loaders/deprecated";
}
