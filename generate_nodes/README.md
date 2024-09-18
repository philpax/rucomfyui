# `generate_nodes`

Generates typed node definitions from ComfyUI's object info.

## Usage

In the repository root, run:

```bash
cargo run -p generate_nodes -- http://127.0.0.1:8188 # or your ComfyUI URL
```

This will generate `src/nodes` with typed node definitions.