use rig::tool::{Tool, ToolSet};
use rig::providers::openai;
use anyhow::Result;

// An example tool that calculates the length of a string
struct StringLengthTool;

impl Tool for StringLengthTool {
    fn name(&self) -> &str {
        "string_length_tool"
    }

    fn description(&self) -> &str {
        "Calculates the length of the input string"
    }

    fn invoke(&self, input: &str) -> String {
        let length = input.len();
        format!("The length of the input string is: {}", length)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // OpenAI clientの初期化
    let openai_client = openai::Client::from_env();
    
    // ToolSetの作成とツールの追加
    let tool_set = ToolSet::new().with_tool(StringLengthTool);
    
    // Agentの作成（gpt-4o-miniモデルを使用）
    let agent = openai_client
        .agent("gpt-4o-mini")
        .with_tools(tool_set)
        .build();

    // Agentを使用してツールを呼び出す
    let response = agent
        .prompt("Calculate the length of this text: 'Hello, Rig!'")
        .await?;
    
    println!("Agent response: {}", response);

    Ok(())
}
