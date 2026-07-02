# Changelog

## [1.1.0]

### Changed

- Updated `rucomfyui` dependency to 1.1.0.

## [1.0.0]

Initial stable release of `rucomfyui_generate_nodes`, a library for generating typed Rust node definitions from ComfyUI ObjectInfo.

### Added

- Library for generating typed Rust node definitions from ComfyUI ObjectInfo, enabling external crates to generate type-safe node bindings.
- `GenerateConfig` with `base_crate_path` field: when set to `"rucomfyui"`, generated code references `rucomfyui::` instead of `crate::`, enabling full bidirectional interoperability between custom and built-in typed nodes.
- Support for use in build scripts (`build.rs`) for compile-time node generation.
- Integration test validating external generation with custom→builtin, builtin→custom, and custom→custom type flow.

## [1.0.0-rc3]

### Added

- `rucomfyui_generate_nodes` is now a publishable crate, enabling external crates to generate typed node bindings via `base_crate_path`.

## [1.0.0-rc2]

### Added

- Library for generating typed Rust node definitions from ComfyUI ObjectInfo, enabling external crates to generate type-safe node bindings via `base_crate_path` configuration.
- `GenerateConfig` with `base_crate_path` field: when set to `"rucomfyui"`, generated code references `rucomfyui::` instead of `crate::`, enabling full bidirectional interoperability between custom and built-in typed nodes.
- Support for use in build scripts (`build.rs`) for compile-time node generation.
- Integration test validating external generation with custom→builtin, builtin→custom, and custom→custom type flow.

## [1.0.0-rc1]

### Added

- Initial crate structure, factored out of `rucomfyui` for standalone use.
- Basic node generation from ComfyUI ObjectInfo into typed Rust code.
