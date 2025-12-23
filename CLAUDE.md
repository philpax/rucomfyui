## Build Commands

**Always use `clippy` instead of `build` or `run`** - this project requires a running ComfyUI server to execute, and clippy provides better feedback.

```bash
# Full check (use this)
cargo clippy --no-default-features -F rustls-tls -F typed_nodes --all --workspace --examples

# Format code
cargo fmt
```

## Project Structure

### Core Libraries (`crates/`)

- **`rucomfyui`**: The main Rust client for ComfyUI. Provides typed API access, workflow construction, and queue management. Features include `typed_nodes` for generated node types and TLS backend selection (`default-tls`, `native-tls`, `rustls-tls`).

- **`rucomfyui_node_graph`**: An egui-based visual node graph editor that recreates the ComfyUI interface in Rust using `egui-snarl`.

- **`rucomfyui_workflow_converter`**: Converts ComfyUI API workflows (JSON) to typed Rust or Lua code.

- **`rucomfyui_mlua`**: Lua bindings for rucomfyui via mlua, enabling workflow construction from Lua scripts.

- **`rucomfyui/generate_nodes`**: Internal tool that generates typed Rust node definitions from ComfyUI's ObjectInfo API.

### Binaries (`bin/`)

- **`rucomfyui_node_graph_demo`**: Demo application for the node graph editor. Connects to ComfyUI, allows loading/saving workflows, queuing, and viewing results.

- **`rucomfyui_workflow_converter_cli`**: CLI tool wrapping `rucomfyui_workflow_converter`.

- **`rucomfyui_lua_script_runner`**: Runs Lua workflow scripts against a ComfyUI server using `rucomfyui_mlua`.

## Key Conventions

- Use `rustls-tls` feature and disable default features for consistent cross-platform TLS
- The `typed_nodes` feature enables generated node types in `rucomfyui::nodes`
- Workflows are represented as `WorkflowGraph` (builder) or `Workflow` (serializable JSON)
