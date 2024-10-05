# `rucomfyui_generate_nodes`

Generates typed node definitions from ComfyUI's object info.

## Usage

In the repository root, run:

```bash
cargo run -p rucomfyui_generate_nodes
```

This will generate `crates/rucomfyui/src/nodes` with typed node definitions.

To update the node definitions, delete `crates/rucomfyui/generate_nodes/object_info.json`
and run the command with a ComfyUI instance:

```bash
cargo run -p rucomfyui_generate_nodes -- http://127.0.0.1:8188 # or your ComfyUI URL
```