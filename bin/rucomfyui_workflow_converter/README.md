# `rucomfyui_workflow_converter_cli`

A command-line wrapper around [`rucomfyui_workflow_converter`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui_workflow_converter) that turns ComfyUI API workflows (JSON) into Rust or Lua code.

## Usage

Export a workflow from ComfyUI with "Save (API Format)", then:

```bash
# Convert to Rust (the default), printed to stdout
cargo run -p rucomfyui_workflow_converter_cli -- workflow.json

# Convert to Lua
cargo run -p rucomfyui_workflow_converter_cli -- workflow.json --format lua

# Write to a file
cargo run -p rucomfyui_workflow_converter_cli -- workflow.json -o workflow.rs

# Use a custom object_info.json (a bundled copy is used by default)
cargo run -p rucomfyui_workflow_converter_cli -- workflow.json --object-info custom_object_info.json
```

It uses ComfyUI's object info to produce the typed code; a bundled `object_info.json` is used unless you pass `--object-info`.
