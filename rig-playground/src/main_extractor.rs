use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use anyhow::Result;

// JSONとして抽出したい構造体の定義
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct Person {
    name: Option<String>,
    age: Option<u8>,
    occupation: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // OpenAI clientの初期化
    let openai_client = openai::Client::from_env();
    
    // Extractorの作成（gpt-4o-miniモデルを使用）
    let extractor = openai_client
        .extractor::<Person>("gpt-4o-mini")
        .build();

    // テキストからJSONデータを抽出
    let text = "山田太郎さんは32歳のソフトウェアエンジニアです。";
    let person = extractor.extract(text).await?;
    
    // 抽出結果の表示
    println!("Extracted person: {:?}", person);

    Ok(())
}
