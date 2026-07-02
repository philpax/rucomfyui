# `rucomfyui_node_graph`

The ComfyUI node graph, rebuilt in Rust with `egui`, `egui-snarl` and `rucomfyui`.

![A screenshot of the node graph showing a SDXL Turbo workflow](https://raw.githubusercontent.com/philpax/rucomfyui/main/crates/rucomfyui_node_graph/rucomfyui_node_graph.png)

## Usage

Add this to your `Cargo.toml`:

```toml
rucomfyui = "1.1.0"
rucomfyui_node_graph = "1.1.0"
```

The [demo app](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_node_graph_demo) shows it driving a real ComfyUI instance.

On its own, this crate doesn't talk to ComfyUI - it renders a graph from the object info that `rucomfyui` hands it. You can use it without a running ComfyUI instance, but you'll have to wire up the connection yourself.

## Related crates

The rest of the [`rucomfyui`](https://github.com/philpax/rucomfyui) workspace:

- [`rucomfyui`](https://crates.io/crates/rucomfyui) - the ComfyUI client.
- [`rucomfyui_node_graph`](https://crates.io/crates/rucomfyui_node_graph) - the node graph editor (this crate).
- [`rucomfyui_workflow_converter`](https://crates.io/crates/rucomfyui_workflow_converter) - workflow-to-code conversion.
- [`rucomfyui_mlua`](https://crates.io/crates/rucomfyui_mlua) - Lua bindings.
