# `rucomfyui`

A Rust client for [ComfyUI](https://github.com/comfyanonymous/ComfyUI), and a few crates built around it.

`rucomfyui` generates a typed Rust interface for every node in ComfyUI's core distribution, so you build workflows with their inputs and outputs checked at compile time. It also covers queueing, live progress (over WebSocket or polling), and the rest of the API, on both native and WebAssembly.

![A screenshot of the node graph showing a SDXL Turbo workflow](https://raw.githubusercontent.com/philpax/rucomfyui/main/crates/rucomfyui_node_graph/rucomfyui_node_graph.png)

## Libraries

- [`rucomfyui`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui) - the client library: typed nodes, workflow construction, queueing, and optional live progress over WebSocket.
- [`rucomfyui_node_graph`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui_node_graph) - the ComfyUI node graph rebuilt in Rust with `egui` and `egui-snarl`.
- [`rucomfyui_workflow_converter`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui_workflow_converter) - turns ComfyUI API workflows (JSON) into Rust or Lua code.
- [`rucomfyui_mlua`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui_mlua) - Lua bindings, for building workflows from scripts.

## Binaries

- [`rucomfyui_node_graph_demo`](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_node_graph_demo) - a demo app for the node graph: connect to ComfyUI, load and save workflows, queue them, and view the results.
- [`rucomfyui_workflow_converter_cli`](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_workflow_converter) - a CLI around `rucomfyui_workflow_converter`.
- [`rucomfyui_lua_script_runner`](https://github.com/philpax/rucomfyui/tree/main/bin/rucomfyui_lua_script_runner) - runs Lua workflow scripts against a server with `rucomfyui_mlua`.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/philpax/rucomfyui/blob/main/LICENSE-APACHE) or [MIT license](https://github.com/philpax/rucomfyui/blob/main/LICENSE-MIT) at your option.
