use anyhow::Result;
use rig::{completion::Prompt, providers::openai};

// Import from lib.rs
use rig_playground::StringLengthTool;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // OpenAI clientの初期化
    let openai_client = openai::Client::from_env();
    
    // Agentの作成（gpt-4o-miniモデルを使用）
    let string_length_agent = openai_client
        .agent("gpt-4o-mini")
        .preamble("You are a string length calculator. Use the provided tool to calculate string lengths.")
        .tool(StringLengthTool)
        .build();

    // Agentを使用してツールを呼び出す
    println!("Calculate the length of 'Hello, Rig!'");
    println!(
        "Agent response: {}",
        string_length_agent.prompt("Calculate the length of 'Hello, Rig!'").await?
    );

    Ok(())
}
