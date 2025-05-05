//! System statistics from ComfyUI.
use serde::{Deserialize, Serialize};

use crate::{Client, Result};

/// Functions for interacting with the system.
impl Client {
    /// Get system statistics.
    pub async fn system_stats(&self) -> Result<SystemStats> {
        self.get("system_stats").await
    }

    /// Free memory and/or unload models.
    pub async fn free(&self, unload_models: bool, free_memory: bool) -> Result<()> {
        self.post_json_without_parse(
            "free",
            &serde_json::json!({
                "unload_models": unload_models,
                "free_memory": free_memory,
            }),
        )
        .await
        .map(|_| ())
    }
}

/// System statistics from ComfyUI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// System information.
    pub system: SystemInfo,
    /// Available devices.
    pub devices: Vec<DeviceInfo>,
}
/// System information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system type.
    pub os: String,
    /// Total RAM in bytes.
    pub ram_total: u64,
    /// Free RAM in bytes.
    pub ram_free: u64,
    /// ComfyUI version.
    pub comfyui_version: String,
    /// Python version.
    pub python_version: String,
    /// PyTorch version.
    pub pytorch_version: String,
    /// Whether Python is embedded.
    pub embedded_python: bool,
    /// Command line arguments.
    pub argv: Vec<String>,
}
/// Device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device name.
    pub name: String,
    /// Device type (e.g., "cuda").
    #[serde(rename = "type")]
    pub device_type: String,
    /// Device index.
    pub index: u32,
    /// Total VRAM in bytes.
    pub vram_total: u64,
    /// Free VRAM in bytes.
    pub vram_free: u64,
    /// Total VRAM reported by torch in bytes.
    pub torch_vram_total: u64,
    /// Free VRAM reported by torch in bytes.
    pub torch_vram_free: u64,
}
