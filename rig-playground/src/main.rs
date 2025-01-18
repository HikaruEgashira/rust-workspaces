use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // OPENAI_API_KEY が環境変数に設定されている必要があります
    let openai_client = openai::Client::from_env();
    let model = openai_client.model("gpt-4o-mini").build();

    let response = model.prompt("Explain quantum computing in one sentence.").await?;
    println!("Response: {}", response);

    Ok(())
}
