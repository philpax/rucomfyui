use std::path::Path;

use anyhow::Result;
use rucomfyui::object_info::Object;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<()> {
    let our_directory = Path::new(file!()).parent().unwrap().parent().unwrap();
    let rucomfyui_directory = our_directory.parent().unwrap();

    let object_info = load_or_get_object_info(our_directory).await?;

    let out_dir = rucomfyui_directory.join("src").join("nodes");

    // Use the library function to generate nodes
    rucomfyui_generate_nodes::generate_nodes(&object_info, &out_dir)?;

    Ok(())
}

async fn load_or_get_object_info(
    our_directory: &Path,
) -> Result<Vec<rucomfyui::object_info::Object>> {
    let path = our_directory.join("object_info.json");

    let Some(url) = std::env::args().nth(1) else {
        // Fall back to existing object info if no URL is provided
        match std::fs::read_to_string(&path) {
            Ok(existing) => {
                println!("Using existing object info from {}", path.display());
                return Ok(serde_json::from_str(&existing)?);
            }
            Err(e) => {
                anyhow::bail!("No ComfyUI URL provided and couldn't read existing object info: {e}")
            }
        }
    };

    println!("Loading object info from ComfyUI...");
    let client = rucomfyui::Client::new(url);

    let mut object_info: Vec<Object> = client
        .get_object_info()
        .await?
        .values()
        .filter(|n| !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_")))
        .cloned()
        .collect();
    object_info.sort_by(|a, b| a.name.cmp(&b.name));
    // Scrub all array type values to improve determinism of results.
    for object in &mut object_info {
        for input in object.input.required.values_mut().chain(
            object
                .input
                .optional
                .iter_mut()
                .flat_map(|v| v.values_mut()),
        ) {
            if let Some(array) = input.as_input_type_mut().as_array_mut() {
                array.clear();
            }
        }
    }
    std::fs::write(&path, serde_json::to_string_pretty(&object_info)?)?;
    Ok(object_info)
}
