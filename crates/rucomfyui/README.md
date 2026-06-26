# `rucomfyui`

A Rust client for [ComfyUI](https://github.com/comfyanonymous/ComfyUI).

The main thing it gives you is a typed Rust interface for every node in ComfyUI's core distribution, so workflows are built with their inputs and outputs checked at compile time (the `typed_nodes` feature, on by default). It also handles building and saving workflows, queueing them, following their progress, and the rest of the HTTP API, and it runs on native and WebAssembly.

## Usage

Add this to your `Cargo.toml`:

```toml
rucomfyui = { git = "https://github.com/philpax/rucomfyui" }
```

### Building a workflow with typed nodes

```rust
use rucomfyui::nodes::all::{
    CLIPTextEncode, CheckpointLoaderSimple, EmptyLatentImage, KSampler, PreviewImage, VAEDecode,
};

let g = rucomfyui::WorkflowGraph::new();
let c = g.add(CheckpointLoaderSimple::new("sd_xl_base_1.0.safetensors"));
let preview = g.add(PreviewImage::new(g.add(VAEDecode {
    vae: c.vae,
    samples: g.add(KSampler {
        model: c.model,
        seed: 0,
        steps: 20,
        cfg: 8.0,
        sampler_name: "euler",
        scheduler: "normal",
        positive: g.add(CLIPTextEncode::new("a cat sleeping on a red chair", c.clip)),
        negative: g.add(CLIPTextEncode::new("text", c.clip)),
        latent_image: g.add(EmptyLatentImage { width: 1024, height: 1024, batch_size: 1 }),
        denoise: 1.0,
    }),
})));
let workflow = g.into_workflow();
```

`c.vae`, `c.model` and `c.clip` are typed node outputs, so wiring up incompatible nodes won't compile.

### Queueing and collecting results

```rust
let client = rucomfyui::Client::new("http://127.0.0.1:8188");
let outputs = client.execute(&workflow).await?.outputs().await?;
```

`execute` hands back an `Execution`, which is a stream of progress events. `.outputs()` runs it to the end and collects the results; if you'd rather follow along, consume the stream yourself and watch for `Event::Executing`, `Event::Progress`, `Event::Preview`, and friends.

The [examples](https://github.com/philpax/rucomfyui/tree/main/crates/rucomfyui/examples) show this end to end.

## Feature flags

- `typed_nodes` (default): the generated node definitions under `rucomfyui::nodes`. Turn it off for a smaller build if you only need dynamic workflows.
- `websocket`: lets `Client::execute` use a WebSocket connection for live progress and previews, falling back to polling if the socket isn't available. It's off by default because ComfyUI's WebSocket API can be flaky.

## Related crates

The rest of the [`rucomfyui`](https://github.com/philpax/rucomfyui) workspace:

- [`rucomfyui`](https://crates.io/crates/rucomfyui) - the ComfyUI client (this crate).
- [`rucomfyui_node_graph`](https://crates.io/crates/rucomfyui_node_graph) - the node graph editor.
- [`rucomfyui_workflow_converter`](https://crates.io/crates/rucomfyui_workflow_converter) - workflow-to-code conversion.
- [`rucomfyui_mlua`](https://crates.io/crates/rucomfyui_mlua) - Lua bindings.
