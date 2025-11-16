# `rucomfyui_generate_nodes`

A library for generating typed Rust node definitions from ComfyUI ObjectInfo.

This library converts ComfyUI's dynamic node definitions into statically-typed Rust code, enabling type-safe usage of ComfyUI nodes in your Rust applications.

## Features

- **Type-safe node generation**: Convert ComfyUI's JSON node definitions into strongly-typed Rust structs
- **Support for custom nodes**: Generate bindings for any ComfyUI nodes, including custom ones
- **Flexible integration**: Use in build scripts, procedural macros, or standalone tools
- **Configurable output**: Control generation behavior with configuration options

## Command-Line Usage

In the repository root, run:

```bash
# Use cached object info
cargo run -p rucomfyui_generate_nodes

# Fetch fresh from ComfyUI instance
cargo run -p rucomfyui_generate_nodes -- http://127.0.0.1:8188
```

This will generate `crates/rucomfyui/src/nodes` with typed node definitions.

## Library Usage

### Basic Example

```rust
use std::path::Path;
use rucomfyui_generate_nodes::generate_nodes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch object info from a ComfyUI instance
    let client = rucomfyui::Client::new("http://127.0.0.1:8188");
    let object_info = client.get_object_info().await?;

    // Filter to only standard nodes (exclude custom nodes)
    let mut nodes: Vec<_> = object_info
        .values()
        .filter(|n| !n.python_module.starts_with("custom_nodes"))
        .cloned()
        .collect();
    nodes.sort_by(|a, b| a.name.cmp(&b.name));

    // Generate the Rust code
    let output_dir = Path::new("src/nodes");
    generate_nodes(&nodes, output_dir)?;

    Ok(())
}
```

### Generating Custom Nodes

To generate bindings for custom nodes, simply include them in the node list:

```rust
use std::path::Path;
use rucomfyui_generate_nodes::generate_nodes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = rucomfyui::Client::new("http://127.0.0.1:8188");
    let object_info = client.get_object_info().await?;

    // Include ALL nodes (including custom nodes)
    let mut nodes: Vec<_> = object_info.values().cloned().collect();
    nodes.sort_by(|a, b| a.name.cmp(&b.name));

    // Generate the Rust code
    let output_dir = Path::new("src/custom_nodes");
    generate_nodes(&nodes, output_dir)?;

    Ok(())
}
```

### Advanced Configuration

You can customize the generation behavior:

```rust
use std::path::Path;
use rucomfyui_generate_nodes::{generate_nodes_with_config, GenerateConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = rucomfyui::Client::new("http://127.0.0.1:8188");
    let object_info = client.get_object_info().await?;

    let nodes: Vec<_> = object_info.values().cloned().collect();

    let config = GenerateConfig {
        scrub_array_values: false, // Keep array values from inputs
    };

    generate_nodes_with_config(&nodes, Path::new("src/nodes"), config)?;

    Ok(())
}
```

### Using in a Build Script

You can use this library in a `build.rs` script to generate nodes at build time:

```rust
// build.rs
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = rucomfyui::Client::new(
        std::env::var("COMFYUI_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8188".to_string())
    );

    let object_info = client.get_object_info().await?;
    let nodes: Vec<_> = object_info.values().cloned().collect();

    let out_dir = std::env::var("OUT_DIR")?;
    let output_dir = Path::new(&out_dir).join("nodes");

    rucomfyui_generate_nodes::generate_nodes(&nodes, &output_dir)?;

    Ok(())
}
```

Then in your `Cargo.toml`:

```toml
[build-dependencies]
rucomfyui_generate_nodes = "0.1"
rucomfyui = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

## Output Structure

The generator creates a hierarchical module structure matching ComfyUI's category system:

```
src/nodes/
├── mod.rs              # Root module with TypedNode/TypedOutputNode traits
├── types.rs            # Type definitions for all ComfyUI types
├── all.rs              # Re-export all nodes
├── advanced/
│   ├── mod.rs
│   ├── loaders/
│   └── conditioning/
├── image/
├── latent/
└── sampling/
```

## How It Works

1. **Input**: Accepts a collection of `Object` structs representing ComfyUI nodes
2. **Categorization**: Organizes nodes into a hierarchical category tree
3. **Code Generation**: Uses `proc_macro2`, `quote`, and `syn` to generate type-safe Rust code
4. **Type System**: Creates traits for each ComfyUI type (e.g., `Clip`, `Model`, `Image`)
5. **Output Structs**: Generates wrapper structs for node outputs with slot information
6. **Generic Nodes**: Each node is generic over its input types for maximum flexibility

## Custom Types

When working with custom nodes that introduce new types, the library will generate `ObjectType::Other(String)` variants. You can handle these by:

1. Extending the `ObjectType` enum in your fork of `rucomfyui`
2. Using the generated code as-is (the library handles unknown types gracefully)
3. Creating wrapper types in your own code
