# `rucomfyui_workflow_converter`

Converts ComfyUI API workflows (JSON) to typed Rust or Lua code.

## Usage

Add this to your `Cargo.toml`'s `[dependencies]`:

```toml
rucomfyui = { git = "https://github.com/philpax/rucomfyui" }
rucomfyui_workflow_converter = { git = "https://github.com/philpax/rucomfyui" }
```

Given an API workflow (as exported by ComfyUI's "Save (API Format)") and the
server's object info, convert it to Rust or Lua:

```rust,ignore
let client = rucomfyui::Client::new("http://127.0.0.1:8188");
let object_info = client.get_object_info().await?;
let workflow_json = std::fs::read_to_string("workflow.json")?;

let rust = rucomfyui_workflow_converter::convert_to_rust(&workflow_json, &object_info)?;
let lua = rucomfyui_workflow_converter::convert_to_lua(&workflow_json, &object_info)?;
```

The `rust` and `lua` features (both enabled by default) gate the respective
converters. See [`rucomfyui_workflow_converter_cli`](../../bin/rucomfyui_workflow_converter)
for a command-line wrapper.
