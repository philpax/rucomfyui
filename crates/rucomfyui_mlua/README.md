# `rucomfyui_mlua`

Lua bindings for [`rucomfyui`](https://crates.io/crates/rucomfyui), for building and running ComfyUI workflows from Lua. You embed it in a Rust program with [`mlua`](https://crates.io/crates/mlua).

## Usage

Add this to your `Cargo.toml`:

```toml
rucomfyui_mlua = "1.1.1"
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

## Building workflows

A graph is created from the server's `object_info` and nodes are constructed by calling `g:NodeName(inputs)` on it. Every node type in the server's `object_info` is available as a method.

**Node inputs** can be passed in two ways:

- **Named (table)**: `g:KSampler { model = c.model, seed = 0, steps = 20, ... }` — each key matches the ComfyUI input name (snake_cased).
- **Positional**: `g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")` — arguments are matched to inputs in declaration order.

Named inputs are recommended for nodes with many fields; positional is convenient for single-input nodes. Required inputs that are omitted raise a Lua error.

**Node outputs** depend on the node:

- Single-output nodes return a handle (userdata) that can be passed as an input to other nodes, or used to index the `result` table from `execute`.
- Multi-output nodes return a table of named handles. The names come from ComfyUI's output definitions, converted to `snake_case`. For example, `CheckpointLoaderSimple` returns `{ model, clip, vae }`.

To see the available inputs and outputs for any node, inspect the `object_info` table returned by `client:get_object_info()`.

**Client methods** available on the `client` object:

| Method | Returns | Description |
|--------|---------|-------------|
| `client:get_object_info()` | table | All node definitions (inputs, outputs, types) |
| `client:get_object_for_name(name)` | table | Node definition for a single node type |
| `client:get_models(category)` | `{string,...}` | Model filenames in a category (e.g. `"checkpoints"`) |
| `client:get_model_categories()` | `{string,...}` | Available model categories |
| `client:execute(graph, opts?)` | table | Queue and run a workflow (see below) |
| `client:queue_prompt(graph)` | table | Queue without waiting (low-level) |
| `client:get_queue()` | table | Current queue state |
| `client:interrupt()` | nil | Cancel the running prompt |
| `client:delete_from_queue(ids)` | nil | Delete specific prompts from the queue |
| `client:clear_queue()` | nil | Clear the queue |
| `client:get_history(max)` | table | Execution history |
| `client:get_history_for_prompt(id)` | table | History for a specific prompt |
| `client:delete_from_history(ids)` | nil | Delete specific prompts from history |
| `client:clear_history()` | nil | Clear all history |
| `client:system_stats()` | table | Server system stats |
| `client:free(unload_models, free_memory)` | nil | Free model/memory resources |
| `client:upload_image(name, data, type?, overwrite?)` | table | Upload an image to the server |

## Return value of `client:execute`

`execute` returns a table mapping node IDs to their outputs. Node IDs are 1-based integers assigned in the order nodes are added to the graph (first node = 1, second = 2, etc.). Each entry has:

| Field | Type | Description |
|-------|------|-------------|
| `images` | `{string,...}` | Output images as raw byte strings (may be empty) |
| `texts` | `{string,...}` | Output texts as Lua strings (may be empty) |

The table can be indexed by a **number** (the node ID) or by a **node handle** (the userdata returned from a node constructor like `g:CheckpointLoaderSimple(...)`):

```lua
local result = client:execute(g)
-- Index by handle (recommended):
local images = result[checkpoint_handle].images
local texts = result[checkpoint_handle].texts
-- Index by numeric ID:
local images = result[1].images
```

## Observing progress with `on_event`

Pass an `on_event` callback to `client:execute` to observe streaming progress events (progress, executing, preview, etc.) without changing the return value:

```lua
local result = client:execute(g, {
    on_event = function(event)
        if event.type == "progress" then
            print(string.format("  progress: %d/%d", event.value, event.max))
        elseif event.type == "preview" then
            -- event.data is raw image bytes; event.format is "jpeg"|"png"|"unknown"
            local f = io.open("output_preview.png", "wb")
            f:write(event.data)
            f:close()
        elseif event.type == "executed" then
            print(string.format("  node %d done (%d images)", event.node, #event.images))
        end
    end,
})
-- result is the same outputs table as without on_event
for i, image in ipairs(result[preview].images) do
    -- image is raw bytes
end
```

The event `type` field is one of `"status"`, `"execution_start"`, `"executing"`, `"progress"`, `"executed"`, or `"preview"`. The `error` and `completed` events are handled internally — errors raise a Lua error and completion stops the stream — so neither is delivered to the callback.

Each event type has the following fields:

| `event.type`     | Fields                                                                 |
|------------------|------------------------------------------------------------------------|
| `"status"`       | `queue_remaining: number`                                              |
| `"execution_start"` | `prompt_id: string`                                                |
| `"executing"`    | `prompt_id: string`, `node: number\|nil` (nil = all nodes finished)    |
| `"progress"`     | `prompt_id: string`, `node: number\|nil`, `value: number`, `max: number` |
| `"executed"`     | `prompt_id: string`, `node: number`, `images: {string,...}`, `texts: {string,...}` |
| `"preview"`      | `prompt_id: string`, `format: string` (`"jpeg"`\|`"png"`\|`"unknown"`), `data: string` (raw bytes) |

The `node` field in `executing` and `progress` events is `nil` when the prompt has finished executing (all nodes done). The `images` and `texts` arrays in `executed` may be empty if the node produced no output of that type. Byte fields (`images`, `data`) are Lua strings containing raw binary data, not text.

**Error handling**: if the workflow fails during execution, `execute` raises a Lua error (catchable with `pcall`). The error message includes the failing node when known. Events received before the error are still delivered to `on_event`, but no partial results are returned — `execute` does not return on failure.

**Callback contract**: `on_event` is called once per event with a single `event` table argument. The callback's return value is ignored. If the callback itself raises an error, execution is aborted and the error propagates to the `execute` caller.

## Related crates

The rest of the [`rucomfyui`](https://github.com/philpax/rucomfyui) workspace:

- [`rucomfyui`](https://crates.io/crates/rucomfyui) - the ComfyUI client.
- [`rucomfyui_node_graph`](https://crates.io/crates/rucomfyui_node_graph) - the node graph editor.
- [`rucomfyui_workflow_converter`](https://crates.io/crates/rucomfyui_workflow_converter) - workflow-to-code conversion.
- [`rucomfyui_mlua`](https://crates.io/crates/rucomfyui_mlua) - Lua bindings (this crate).
