# Changelog

## 1.0.0

First release of `rucomfyui`, a Rust client for [ComfyUI](https://github.com/comfyanonymous/ComfyUI), along with a few crates built around it.

`rucomfyui` itself:

- A typed Rust interface for every node in ComfyUI's core distribution (the `typed_nodes` feature, on by default), so workflows are built with their inputs and outputs checked at compile time.
- Building and saving workflows with `WorkflowGraph`, mixing typed and dynamic nodes, or loading and saving ComfyUI's API-format JSON.
- Queueing prompts and following them with `Client::execute`, which streams progress events (the running node, sampler progress, previews, outputs). It polls over HTTP by default, or uses a WebSocket connection with the `websocket` feature.
- The rest of the API: object info, history, the queue, models, system stats, and image upload.
- Native and WebAssembly support.

The other crates:

- `rucomfyui_node_graph` - the ComfyUI node graph rebuilt with `egui`/`egui-snarl`.
- `rucomfyui_workflow_converter` - converts API workflows to Rust or Lua code.
- `rucomfyui_mlua` - Lua bindings for building and running workflows.
