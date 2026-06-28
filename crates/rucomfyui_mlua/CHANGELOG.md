# Changelog

## [1.0.0-rc1]

### Added

- Lua bindings for building and running ComfyUI workflows via `mlua`, enabling workflow construction from Lua scripts.
- Exposes `rucomfyui`'s `WorkflowGraph` and `Client` APIs to Lua, including typed nodes when the `typed_nodes` feature is enabled on the underlying `rucomfyui` dependency.
- The `rucomfyui_lua_script_runner` binary (separately, non-published) demonstrates running Lua workflow scripts against a ComfyUI server.
