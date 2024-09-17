//!video_models
///**Image Only Checkpoint Loader (img2vid model)**
pub struct ImageOnlyCheckpointLoader {}
///Output for [`ImageOnlyCheckpointLoader`].
pub struct ImageOnlyCheckpointLoaderOutput {
    ///No documentation.
    pub model: crate::nodes::ModelOut,
    ///No documentation.
    pub clip_vision: crate::nodes::ClipVisionOut,
    ///No documentation.
    pub vae: crate::nodes::VaeOut,
}
impl crate::nodes::TypedNode for ImageOnlyCheckpointLoader {
    type Output = ImageOnlyCheckpointLoaderOutput;
    fn output(&self) -> Self::Output {
        Self::Output {
            model: crate::nodes::ModelOut(0usize),
            clip_vision: crate::nodes::ClipVisionOut(1usize),
            vae: crate::nodes::VaeOut(2usize),
        }
    }
    const NAME: &'static str = "ImageOnlyCheckpointLoader";
    const DISPLAY_NAME: &'static str = "Image Only Checkpoint Loader (img2vid model)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "loaders/video_models";
}
