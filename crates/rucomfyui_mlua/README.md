# `rucomfyui_mlua`

Lua bindings for [`rucomfyui`](../rucomfyui), exposing a fluent ComfyUI workflow
builder to Lua. Designed to be embedded in Rust applications using
[`mlua`](https://crates.io/crates/mlua).

## Usage

Add this to your `Cargo.toml`'s `[dependencies]`:

```toml
rucomfyui_mlua = { git = "https://github.com/philpax/rucomfyui" }
```

Install the module into a Lua state, then build and execute workflows from Lua:

```rust,ignore
use mlua::Lua;
use rucomfyui_mlua::{IntegrationConfig, module};

let lua = Lua::new();
let comfy = module(&lua, &IntegrationConfig::all())?;
lua.globals().set("comfy", comfy)?;

lua.load(r#"
    local client = comfy.client("http://127.0.0.1:8188")
    local g = comfy.graph(client:get_object_info())
    local c = g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")
    local preview = g:PreviewImage(g:VAEDecode {
        vae = c.vae,
        samples = g:KSampler {
            model = c.model, seed = 0, steps = 20, cfg = 8.0,
            sampler_name = "euler", scheduler = "normal",
            positive = g:CLIPTextEncode { text = "a cat", clip = c.clip },
            negative = g:CLIPTextEncode { text = "", clip = c.clip },
            latent_image = g:EmptyLatentImage { width = 1024, height = 1024, batch_size = 1 },
            denoise = 1.0,
        }
    })
    local result = client:execute(g)
    for i, image in ipairs(result[preview].images) do
        -- image is raw bytes
    end
"#).exec()?;
```

[`IntegrationConfig`] controls which client methods are exposed to Lua (see
`ClientConfig::all`/`read_only`/`execute`). See
[`rucomfyui_lua_script_runner`](../../bin/rucomfyui_lua_script_runner) for a
runnable example.
