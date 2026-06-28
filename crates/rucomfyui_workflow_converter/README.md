# `rucomfyui_workflow_converter`

Turns ComfyUI API workflows (JSON) into typed Rust or Lua code.

## Usage

Add this to your `Cargo.toml`:

```toml
rucomfyui = "1.0.0-rc2"
rucomfyui_workflow_converter = "1.0.0-rc2"
```

Take a workflow exported with ComfyUI's "Save (API Format)", along with the server's object info, and convert it:

```rust,ignore
let client = rucomfyui::Client::new("http://127.0.0.1:8188");
let object_info = client.get_object_info().await?;
let workflow_json = std::fs::read_to_string("workflow.json")?;

let rust = rucomfyui_workflow_converter::convert_to_rust(&workflow_json, &object_info)?;
let lua = rucomfyui_workflow_converter::convert_to_lua(&workflow_json, &object_info)?;
```

The `rust` and `lua` features (both on by default) decide which converters are built. There's also a CLI, [`rucomfyui_workflow_converter_cli`](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_workflow_converter).

## Related crates

The rest of the [`rucomfyui`](https://github.com/philpax/rucomfyui) workspace:

- [`rucomfyui`](https://crates.io/crates/rucomfyui) - the ComfyUI client.
- [`rucomfyui_node_graph`](https://crates.io/crates/rucomfyui_node_graph) - the node graph editor.
- [`rucomfyui_workflow_converter`](https://crates.io/crates/rucomfyui_workflow_converter) - workflow-to-code conversion (this crate).
- [`rucomfyui_mlua`](https://crates.io/crates/rucomfyui_mlua) - Lua bindings.
