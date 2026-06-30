# `rucomfyui_mlua`

Lua bindings for [`rucomfyui`](https://crates.io/crates/rucomfyui), for building and running ComfyUI workflows from Lua. You embed it in a Rust program with [`mlua`](https://crates.io/crates/mlua).

## Usage

Add this to your `Cargo.toml`:

```toml
rucomfyui_mlua = "1.0.0-rc4"
```

Install the module into a Lua state, then build and run workflows from Lua:

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

`IntegrationConfig` decides which client methods Lua can reach (`ClientConfig::all`, `read_only`, `execute`). For a runnable example, see [`rucomfyui_lua_script_runner`](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_lua_script_runner).

## Related crates

The rest of the [`rucomfyui`](https://github.com/philpax/rucomfyui) workspace:

- [`rucomfyui`](https://crates.io/crates/rucomfyui) - the ComfyUI client.
- [`rucomfyui_node_graph`](https://crates.io/crates/rucomfyui_node_graph) - the node graph editor.
- [`rucomfyui_workflow_converter`](https://crates.io/crates/rucomfyui_workflow_converter) - workflow-to-code conversion.
- [`rucomfyui_mlua`](https://crates.io/crates/rucomfyui_mlua) - Lua bindings (this crate).
