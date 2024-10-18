use anyhow::Context;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let url = std::env::args()
        .nth(1)
        .context("ComfyUI URL not provided")?;
    let client = rucomfyui::Client::new(url);

    let workflow = rucomfyui::Workflow::from_json(include_str!("existing_workflow.json"))?;
    let result = client.easy_queue(&workflow).await?;

    for (idx, image) in result
        .iter()
        .flat_map(|(_, output)| output.images.iter())
        .enumerate()
    {
        std::fs::write(format!("output_{idx}.png"), image)?;
    }

    Ok(())
}
