use anyhow::Result;
use rig::{
    completion::{Prompt, ToolDefinition},
    providers::openai,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("String length error")]
struct StringLengthError;

#[derive(Debug, Serialize, Deserialize)]
struct StringLengthArgs {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StringLengthTool;

impl Tool for StringLengthTool {
    const NAME: &'static str = "string_length";

    type Error = StringLengthError;
    type Args = StringLengthArgs;
    type Output = usize;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Calculates the length of the input string".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The text to calculate the length of"
                    }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.text.len())
    }
}

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
