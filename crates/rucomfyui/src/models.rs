//! Retrieve model categories and the models within them.

use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

use crate::{Client, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// A category of models.
pub enum ModelCategory {
    /// Checkpoints.
    Checkpoints,
    /// Configs.
    Configs,
    /// LoRAs.
    Loras,
    /// VAE models.
    Vae,
    /// Text encoders.
    TextEncoders,
    /// Diffusion models.
    DiffusionModels,
    /// Clip vision.
    ClipVision,
    /// Style models.
    StyleModels,
    /// Embeddings.
    Embeddings,
    /// Diffusers.
    Diffusers,
    /// VAE approximations.
    VaeApprox,
    /// Controlnet.
    Controlnet,
    /// Gligen.
    Gligen,
    /// Upscale models.
    UpscaleModels,
    /// Custom nodes.
    CustomNodes,
    /// Hypernetworks.
    Hypernetworks,
    /// Photomaker.
    Photomaker,
    /// Classifiers.
    Classifiers,
    /// Inpaint.
    Inpaint,
    /// Other.
    #[serde(untagged)]
    Other(String),
}
impl fmt::Display for ModelCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // I don't love this, but it'll do the trick
        let json = serde_json::to_string(self).unwrap_or_default();
        let unquoted = json.trim_start_matches('"').trim_end_matches('"');
        write!(f, "{unquoted}")
    }
}

impl Client {
    /// Get all model categories.
    pub async fn model_categories(&self) -> Result<Vec<ModelCategory>> {
        self.get("models").await
    }

    /// Get all models in a category.
    pub async fn models(&self, category: ModelCategory) -> Result<Vec<String>> {
        self.get(&format!("models/{category}")).await
    }
}
