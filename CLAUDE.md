## Build Commands

**Always use `clippy` instead of `build` or `run`** - this project requires a running ComfyUI server to execute, and clippy provides better feedback.

```bash
# Full check (use this)
cargo clippy --no-default-features -F typed_nodes --all --workspace --examples

# Format code
cargo fmt
```

## Project Structure

### Core Libraries (`crates/`)

- **`rucomfyui`**: The main Rust client for ComfyUI. Provides typed API access, workflow construction, and queue management. Features include `typed_nodes` for generated node types.

- **`rucomfyui_node_graph`**: An egui-based visual node graph editor that recreates the ComfyUI interface in Rust using `egui-snarl`.

- **`rucomfyui_workflow_converter`**: Converts ComfyUI API workflows (JSON) to typed Rust or Lua code.

- **`rucomfyui_mlua`**: Lua bindings for rucomfyui via mlua, enabling workflow construction from Lua scripts.

- **`rucomfyui/generate_nodes`**: Internal tool that generates typed Rust node definitions from ComfyUI's ObjectInfo API.

### Binaries (`bin/`)

- **`rucomfyui_node_graph_demo`**: Demo application for the node graph editor. Connects to ComfyUI, allows loading/saving workflows, queuing, and viewing results.

- **`rucomfyui_workflow_converter_cli`**: CLI tool wrapping `rucomfyui_workflow_converter`.

- **`rucomfyui_lua_script_runner`**: Runs Lua workflow scripts against a ComfyUI server using `rucomfyui_mlua`.

## Key Conventions

- The `typed_nodes` feature enables generated node types in `rucomfyui::nodes`
- Workflows are represented as `WorkflowGraph` (builder) or `Workflow` (serializable JSON)

## Releasing

Releases use [cargo-release](https://github.com/crate-ci/cargo-release) with a local-prep + CI-publish model. All five publishable crates (`rucomfyui`, `rucomfyui_mlua`, `rucomfyui_node_graph`, `rucomfyui_workflow_converter`, `rucomfyui_generate_nodes`) share a single version via `version.workspace = true`.

### Prerequisites

- `cargo install cargo-release`
- `CARGO_REGISTRY_TOKEN` secret configured in the GitHub repo (crates.io token with `publish-new` + `publish-update` scopes)
- GitHub Actions workflow permissions allow PR creation (Settings → Actions → General → Workflow permissions)

### Cutting a release

All work happens locally first. You inspect the exact artifacts before anything hits crates.io.

```bash
# 1. Run cargo-release locally (dry-run by default; preview the changes)
cargo release <version> --no-publish --workspace
# e.g. cargo release 1.0.0-rc2 --no-publish --workspace

# 2. If the preview looks good, execute for real
cargo release <version> --no-publish --workspace --execute

# 3. Inspect the result
git diff HEAD~1              # version bumps, README syncs, changelog updates
git log --oneline -3         # the release commit
git tag --list 'rucomfyui*'  # per-crate tags created

# 4. Verify the publishable crates build from the exact committed tree
cargo publish --dry-run -p rucomfyui
cargo publish --dry-run -p rucomfyui_generate_nodes
cargo publish --dry-run -p rucomfyui_mlua
cargo publish --dry-run -p rucomfyui_node_graph
cargo publish --dry-run -p rucomfyui_workflow_converter

# 5. Push the commit and tags to trigger CI publish
git push origin main
git push --tags
```

### What cargo-release does automatically

- Bumps the version in `Cargo.toml` (workspace) and all internal dep pins
- Runs `pre-release-replacements` to sync `rucomfyui* = "<version>"` lines in each crate's README via `{{version}}` templates
- Creates a single consolidated commit (`consolidate-commits = true`)
- Creates per-crate tags: `rucomfyui-v<version>`, `rucomfyui_mlua-v<version>`, etc.
- Does **not** publish (`--no-publish`); CI handles that

### What CI does

When tags are pushed, `.github/workflows/release.yml` triggers and:
1. Checks out the tagged commit
2. Publishes all five crates in dependency order (`rucomfyui` → `rucomfyui_generate_nodes` → `rucomfyui_mlua` → `rucomfyui_node_graph` → `rucomfyui_workflow_converter`)
3. Creates a GitHub release with auto-generated notes

### Pre-release → final transition

cargo-release doesn't have built-in support for moving from `1.0.0-rc1` to `1.0.0` (the version on crates.io is already `rc1`). To cut the final `1.0.0`:

1. Manually edit the workspace version in `Cargo.toml` from `1.0.0-rcN` to `1.0.0`
2. Manually update the internal dep pins from `version = "1.0.0-rcN"` to `version = "1.0.0"`
3. Update per-crate CHANGELOG.md files with a `## [1.0.0]` section
4. Commit, then run `cargo release 1.0.0 --no-publish --workspace --execute` and push tags

### Configuration

- `release.toml` — workspace-level config (`consolidate-commits`)
- `[package.metadata.release]` in each crate's `Cargo.toml` — per-crate `pre-release-replacements` for README sync, and `release = false` for non-publishable crates
