//! Run a Lua workflow script against a ComfyUI server.
//!
//! Usage: run_script <url> [script] [prompt]
//!
//! Arguments:
//!   url     - ComfyUI server URL (e.g., http://127.0.0.1:8188)
//!   script  - Path to Lua script (default: examples/workflow.lua)
//!   prompt  - Text prompt to pass to the script (optional)

use anyhow::Context;
use mlua::Lua;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let url = args
        .get(1)
        .context("Usage: run_script <url> [script] [prompt]")?;
    let script_path = args
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("crates/rucomfyui_mlua/examples/workflow.lua");
    let prompt = args.get(3);

    // Read the script
    let script = std::fs::read_to_string(script_path)
        .with_context(|| format!("Failed to read script: {}", script_path))?;

    // Create Lua state
    let lua = Lua::new();

    // Set up the rucomfyui module
    let config = rucomfyui_mlua::IntegrationConfig::all();
    let comfy = rucomfyui_mlua::module(&lua, &config)?;
    lua.globals().set("comfy", comfy)?;

    // Create and set up the client
    let client = rucomfyui_mlua::create_client_userdata(
        &lua,
        rucomfyui::Client::new(url),
        rucomfyui_mlua::ClientConfig::all(),
    )?;
    lua.globals().set("client", client)?;

    // Set the prompt if provided
    if let Some(prompt) = prompt {
        lua.globals().set("prompt", prompt.as_str())?;
    }

    // Run the script
    println!("Running script: {}", script_path);
    if let Some(prompt) = prompt {
        println!("Prompt: {}", prompt);
    }

    let images: Vec<mlua::String> = lua.load(&script).set_name(script_path).eval_async().await?;

    // Save the images
    for (idx, image) in images.iter().enumerate() {
        let filename = format!("output_{}.png", idx);
        std::fs::write(&filename, image.as_bytes())?;
        println!("Saved: {}", filename);
    }

    println!("Done! Generated {} image(s)", images.len());

    Ok(())
}
