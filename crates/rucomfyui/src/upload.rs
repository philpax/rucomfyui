//! Upload functionality for the ComfyUI API.

use std::{fmt, str::FromStr};

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{BorrowedBytes, Client, Result};

/// The type of upload directory.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UploadType {
    /// The input directory.
    #[default]
    Input,
    /// The temp directory.
    Temp,
    /// The output directory.
    Output,
}

impl fmt::Display for UploadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UploadType::Input => write!(f, "input"),
            UploadType::Temp => write!(f, "temp"),
            UploadType::Output => write!(f, "output"),
        }
    }
}

impl FromStr for UploadType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "input" => Ok(UploadType::Input),
            "temp" => Ok(UploadType::Temp),
            "output" => Ok(UploadType::Output),
            _ => Err(format!(
                "invalid upload type: {s}, expected 'input', 'temp', or 'output'"
            )),
        }
    }
}

/// Response from uploading an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageResponse {
    /// The filename of the uploaded image.
    pub name: String,
    /// The subfolder the image was uploaded to.
    pub subfolder: String,
    /// The type of upload directory.
    #[serde(rename = "type")]
    pub upload_type: UploadType,
}

impl Client {
    /// Upload an image to the ComfyUI API.
    pub async fn upload_image(
        &self,
        filename: &str,
        bytes: impl Into<BorrowedBytes<'static>>,
        upload_type: UploadType,
        overwrite: bool,
    ) -> Result<UploadImageResponse> {
        let form = Form::new()
            .part("image", Part::bytes(bytes).file_name(filename.to_string()))
            .text("type", upload_type.to_string())
            .text("overwrite", if overwrite { "true" } else { "false" });

        self.post_multipart::<(), _>("upload/image", form).await
    }
}
