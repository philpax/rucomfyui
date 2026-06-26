# `rucomfyui_lua_script_runner`

Runs Lua workflow scripts against a ComfyUI server with [`rucomfyui_mlua`](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui_mlua). A worked example of embedding the Lua bindings in a Rust program.

## Usage

```bash
cargo run -p rucomfyui_lua_script_runner -- <url> [script] [prompt]
```

- `url` - the ComfyUI server, e.g. `http://127.0.0.1:8188`.
- `script` - a Lua script (defaults to the bundled [`workflow.lua`](https://github.com/philpax/rucomfyui/blob/main/bin/rucomfyui_lua_script_runner/workflow.lua)).
- `prompt` - an optional positive prompt for the script.

For example:

```bash
cargo run -p rucomfyui_lua_script_runner -- http://127.0.0.1:8188
```
