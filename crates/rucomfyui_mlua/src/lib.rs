//! Lua bindings for rucomfyui - a fluent ComfyUI workflow builder.
//!
//! This crate provides a high-level Lua API for building and executing ComfyUI workflows.
//! It's designed to be embedded in Rust applications using mlua.
//!
//! # Example
//!
//! ```no_run
//! use mlua::Lua;
//! use rucomfyui_mlua::{IntegrationConfig, module};
//!
//! # fn main() -> mlua::Result<()> {
//! let lua = Lua::new();
//!
//! // Install the module with all features enabled
//! let config = IntegrationConfig::all();
//! let comfy = module(&lua, &config)?;
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
//!     local result = client:easy_queue(g)
//!     for i, image in ipairs(result[preview].images) do
//!         -- image is raw bytes
//!     end
//! "#).exec()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! Use [`IntegrationConfig`] to control which functionality is exposed to Lua:
//!
//! ```no_run
//! use rucomfyui_mlua::{ClientConfig, IntegrationConfig};
//!
//! // All methods enabled (default for IntegrationConfig::all())
//! let full_config = IntegrationConfig::all();
//!
//! // Read-only access
//! let readonly_config = IntegrationConfig {
//!     client: ClientConfig::read_only(),
//! };
//!
//! // Custom configuration
//! let custom_config = IntegrationConfig {
//!     client: ClientConfig {
//!         get_object_info: true,
//!         queue: true,
//!         easy_queue: true,
//!         ..ClientConfig::none()
//!     },
//! };
//! ```
//!
//! # Creating Clients from Existing rucomfyui::Client
//!
//! If you already have a configured [`rucomfyui::Client`], you can wrap it:
//!
//! ```no_run
//! use mlua::Lua;
//! use rucomfyui_mlua::{ClientConfig, create_client_userdata};
//!
//! # fn main() -> mlua::Result<()> {
//! let lua = Lua::new();
//!
//! // Create a pre-configured rucomfyui client
//! let rust_client = rucomfyui::Client::new("http://127.0.0.1:8188");
//!
//! // Wrap it as a Lua userdata
//! let lua_client = create_client_userdata(&lua, rust_client, ClientConfig::all())?;
//! lua.globals().set("client", lua_client)?;
//! # Ok(())
//! # }
//! ```

mod client;
mod config;
mod conversion;
mod error;
mod graph;
mod node_output;

pub use config::{ClientConfig, IntegrationConfig};
pub use error::Error;

use mlua::{Lua, Result, Table};

/// Create the rucomfyui Lua module.
///
/// This returns a table containing the module's functions and types.
/// The integrator can place this table wherever they like in the Lua environment.
///
/// # Arguments
///
/// * `lua` - The Lua state
/// * `config` - Configuration controlling which features are available
///
/// # Example
///
/// ```no_run
/// use mlua::Lua;
/// use rucomfyui_mlua::{IntegrationConfig, module};
///
/// let lua = Lua::new();
/// let config = IntegrationConfig::all();
/// let comfy = module(&lua, &config)?;
/// lua.globals().set("comfy", comfy)?;
/// # Ok::<(), mlua::Error>(())
/// ```
pub fn module(lua: &Lua, config: &IntegrationConfig) -> Result<Table> {
    let exports = lua.create_table()?;
    let client_config = config.client.clone();

    // comfy.client(url) -> Client
    exports.set(
        "client",
        lua.create_function(move |lua, url: String| {
            client::Client::new(lua, url, client_config.clone())
        })?,
    )?;

    // comfy.graph(object_info) -> Graph
    exports.set(
        "graph",
        lua.create_function(|lua, object_info: Table| graph::Graph::new(lua, object_info))?,
    )?;

    Ok(exports)
}

/// Create a Lua userdata from an existing rucomfyui::Client.
///
/// This allows Rust integrators to pass in a pre-configured client
/// rather than having Lua create one from a URL.
///
/// # Arguments
///
/// * `lua` - The Lua state
/// * `client` - An existing rucomfyui::Client
/// * `config` - Configuration controlling which methods are available
///
/// # Example
///
/// ```no_run
/// use mlua::Lua;
/// use rucomfyui_mlua::{ClientConfig, create_client_userdata};
///
/// let lua = Lua::new();
/// let rust_client = rucomfyui::Client::new("http://127.0.0.1:8188");
/// let lua_client = create_client_userdata(&lua, rust_client, ClientConfig::execute())?;
/// lua.globals().set("my_client", lua_client)?;
/// # Ok::<(), mlua::Error>(())
/// ```
pub fn create_client_userdata(
    lua: &Lua,
    client: rucomfyui::Client,
    config: ClientConfig,
) -> Result<mlua::AnyUserData> {
    lua.create_userdata(client::Client::from_existing(client, config))
}
