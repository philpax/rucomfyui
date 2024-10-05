# `rucomfyui_node_graph`

A recreation of the ComfyUI node graph in Rust using `egui`, `egui_node_graph2` and `rucomfyui`.

![A screenshot of the node graph showing a SDXL Turbo workflow](rucomfyui_node_graph.png)

## Usage

Add this to your `Cargo.toml`'s `[dependencies]`:

```toml
rucomfyui = { git = "https://github.com/philpax/rucomfyui" }
rucomfyui_node_graph = { git = "https://github.com/philpax/rucomfyui" }
```

See the [demo application](../../bin/rucomfyui_node_graph_demo/README.md) for an example of how to use this crate
with a real ComfyUI instance.

This crate does not actually interact with ComfyUI; it just renders a node graph based on the
object information provided to it by `rucomfyui`. This means it can be used without direct
access to a ComfyUI instance, but you will need to wire up the interface layer yourself.