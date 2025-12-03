//! Lua bindings for rucomfyui - a fluent ComfyUI workflow builder.
//!
//! This crate provides a high-level Lua API for building and executing ComfyUI workflows.
//! It's designed to be embedded in Rust applications using mlua.
//!
//! # Example
//!
//! ```no_run
//! use mlua::Lua;
//!
//! # fn main() -> mlua::Result<()> {
//! let lua = Lua::new();
//!
//! // Install the module
//! let comfy = rucomfyui_mlua::module(&lua)?;
//! lua.globals().set("comfy", comfy)?;
//!
//! // Use from Lua
//! lua.load(r#"
//!     local client = comfy.client("http://127.0.0.1:8188")
//!     local object_info = client:get_object_info()
//!     local g = comfy.graph(object_info)
//!
//!     local c = g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")
//!     local preview = g:PreviewImage(
//!         g:VAEDecode {
//!             vae = c.vae,
//!             samples = g:KSampler {
//!                 model = c.model,
//!                 seed = 0,
//!                 steps = 20,
//!                 cfg = 8.0,
//!                 sampler_name = "euler",
//!                 scheduler = "normal",
//!                 positive = g:CLIPTextEncode { text = "a cat", clip = c.clip },
//!                 negative = g:CLIPTextEncode { text = "text", clip = c.clip },
//!                 latent_image = g:EmptyLatentImage { width = 1024, height = 1024, batch_size = 1 },
//!                 denoise = 1.0
//!             }
//!         }
//!     )
//!
//!     local result = client:queue(g)
//!     for i, image in ipairs(result[preview].images) do
//!         -- image is raw bytes
//!     end
//! "#).exec()?;
//! # Ok(())
//! # }
//! ```

mod client;
mod conversion;
mod error;
mod graph;
mod node_output;

pub use error::Error;

use mlua::{Lua, Result, Table};

/// Create the rucomfyui Lua module.
///
/// This returns a table containing the module's functions and types.
/// The integrator can place this table wherever they like in the Lua environment.
///
/// # Example
///
/// ```no_run
/// use mlua::Lua;
///
/// let lua = Lua::new();
/// let comfy = rucomfyui_mlua::module(&lua)?;
/// lua.globals().set("comfy", comfy)?;
/// # Ok::<(), mlua::Error>(())
/// ```
pub fn module(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;

    // comfy.client(url) -> Client
    exports.set(
        "client",
        lua.create_function(|lua, url: String| client::Client::new(lua, url))?,
    )?;

    // comfy.graph(object_info) -> Graph
    exports.set(
        "graph",
        lua.create_function(|lua, object_info: Table| graph::Graph::new(lua, object_info))?,
    )?;

    Ok(exports)
}
