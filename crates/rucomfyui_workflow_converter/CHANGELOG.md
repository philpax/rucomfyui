# Changelog

## [1.0.0-rc2]

### Changed

- READMEs now reference crates.io versions, automatically synced from `Cargo.toml` by the release workflow.

## [1.0.0-rc1]

### Added

- Converts ComfyUI API workflows (JSON) to typed Rust or Lua code.
- Generates typed Rust using `rucomfyui`'s `typed_nodes` definitions (the `typed_nodes` feature is enabled on the `rucomfyui` dependency for full type coverage).
- Generates Lua code targeting `rucomfyui_mlua` for workflow construction from scripts.
- The `rucomfyui_workflow_converter_cli` binary (separately, non-published) wraps the converter as a command-line tool.
