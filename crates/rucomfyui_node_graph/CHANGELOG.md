# Changelog

## [1.0.0]

Initial stable release of `rucomfyui_node_graph`, the ComfyUI node graph rebuilt in Rust with `egui` and `egui-snarl`.

### Added

- The ComfyUI node graph rebuilt in Rust with `egui` and `egui-snarl`, mirroring the ComfyUI visual editor.
- Connects to a ComfyUI server, loads and saves workflows, queues prompts, and views results within an `eframe` application.
- Supports both typed and dynamic nodes via `rucomfyui` (typed nodes enabled through feature unification when used alongside the rest of the workspace).
- The `rucomfyui_node_graph_demo` binary (separately, non-published) provides a demo application showcasing the node graph editor.

## [1.0.0-rc3]

### Added

- `rucomfyui_generate_nodes` is now a publishable crate, enabling external crates to generate typed node bindings via `base_crate_path`.

## [1.0.0-rc2]

### Changed

- READMEs now reference crates.io versions, automatically synced from `Cargo.toml` by the release workflow.

## [1.0.0-rc1]

### Added

- The ComfyUI node graph rebuilt in Rust with `egui` and `egui-snarl`, mirroring the ComfyUI visual editor.
- Connects to a ComfyUI server, loads and saves workflows, queues prompts, and views results within an `eframe` application.
- Supports both typed and dynamic nodes via `rucomfyui` (typed nodes enabled through feature unification when used alongside the rest of the workspace).
- The `rucomfyui_node_graph_demo` binary (separately, non-published) provides a demo application showcasing the node graph editor.
