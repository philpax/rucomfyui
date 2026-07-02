# Changelog

## [1.1.1]

### Changed

- Switched `on_event` callback invocations from async to synchronous for better compatibility with Lua environments that don't support async callbacks.

## [1.1.0]

### Added

- `client:execute` now accepts an optional `opts` table with an `on_event` callback that receives streaming progress events (status, execution_start, executing, progress, executed, preview) during workflow execution. The return value is unchanged. Errors raise a Lua error catchable with `pcall`.
- Forwarded `websocket` feature (enabled by default) for live streaming events.
- Expanded documentation: building workflows, client methods reference, return value shape, event field reference, callback contract.

### Changed

- Event-to-Lua conversion uses mlua's serde serializer with `serde_bytes` for binary fields.
- The example script now uses `on_event`, includes an interactive checkpoint picker, and writes output directly to disk.
- `futures` and `serde_bytes` normalized to workspace dependencies.

## [1.0.0]

Initial stable release of `rucomfyui_mlua`, Lua bindings for building and running ComfyUI workflows via `mlua`.

### Added

- Lua bindings for building and running ComfyUI workflows via `mlua`, enabling workflow construction from Lua scripts.
- Exposes `rucomfyui`'s `WorkflowGraph` and `Client` APIs to Lua, including typed nodes when the `typed_nodes` feature is enabled on the underlying `rucomfyui` dependency.
- The `rucomfyui_lua_script_runner` binary (separately, non-published) demonstrates running Lua workflow scripts against a ComfyUI server.

### Lua-version feature forwarding

`rucomfyui_mlua` forwards `mlua`'s mutually-exclusive Lua-version features (`lua54`, `luajit`, `luajit52`, `lua53`, `lua52`, `lua51`, `luau`) with `lua54` as the default. Consumers can select a Lua version via `default-features = false, features = ["luajit"]`.

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
