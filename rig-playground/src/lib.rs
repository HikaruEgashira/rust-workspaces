use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
#[error("String length error")]
pub struct StringLengthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct StringLengthArgs {
    pub text: String,
}

#[derive(Debug)]
pub struct StringLengthTool;

impl Tool for StringLengthTool {
    const NAME: &'static str = "string_length";

    type Error = StringLengthError;
    type Args = StringLengthArgs;
    type Output = usize;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Calculates the length of the input string".to_string(),
            parameters: serde_json::json!({
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq)]
pub struct Person {
    pub name: Option<String>,
    pub age: Option<u8>,
    pub occupation: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_string_length_tool_definition() {
        let tool = StringLengthTool;
        let definition = tool.definition("".to_string()).await;
        assert_eq!(definition.name, "string_length");
        assert!(definition.description.contains("length"));
    }

    #[tokio::test]
    async fn test_string_length_tool_call() {
        let tool = StringLengthTool;
        let args = StringLengthArgs {
            text: "Hello, Rig!".to_string(),
        };
        let result = tool.call(args).await.unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_person_serialization() {
        let person = Person {
            name: Some("山田太郎".to_string()),
            age: Some(32),
            occupation: Some("ソフトウェアエンジニア".to_string()),
        };
        let json = serde_json::to_string(&person).unwrap();
        let deserialized: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(person, deserialized);
    }
}
