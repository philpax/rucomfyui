# Changelog

## [1.0.0-rc3]

### Added

- `rucomfyui_generate_nodes` is now a publishable crate, enabling external crates to generate typed node bindings via `base_crate_path`.

## [1.0.0-rc2]

### Changed

- `rucomfyui_mlua` now forwards `mlua`'s Lua-version features (`lua54`, `luajit`, `luajit52`, `lua53`, `lua52`, `lua51`, `luau`) instead of hard-coding `lua54`. Consumers can select a Lua version via `default-features = false, features = ["luajit"]` etc. `lua54` remains the default for out-of-the-box usability.
- READMEs now reference crates.io versions, automatically synced from `Cargo.toml` by the release workflow.

## [1.0.0-rc1]

### Added

- Lua bindings for building and running ComfyUI workflows via `mlua`, enabling workflow construction from Lua scripts.
- Exposes `rucomfyui`'s `WorkflowGraph` and `Client` APIs to Lua, including typed nodes when the `typed_nodes` feature is enabled on the underlying `rucomfyui` dependency.
- The `rucomfyui_lua_script_runner` binary (separately, non-published) demonstrates running Lua workflow scripts against a ComfyUI server.
