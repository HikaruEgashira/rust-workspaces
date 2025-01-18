use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct Person {
    name: Option<String>,
    age: Option<u8>,
    occupation: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let extractor = openai_client
        .extractor::<Person>("gpt-4o-mini")
        .build();

    let text = "山田太郎さんは32歳のソフトウェアエンジニアです。";
    let person = extractor.extract(text).await?;
    println!("Extracted person: {:?}", person);

    Ok(())
}
