# Rig Playground

このワークスペースは、[rig](https://github.com/0xPlaygrounds/rig)ライブラリを使用したLLM（Large Language Model）アプリケーションの開発例を提供します。

## Rigとは

Rigは、LLMを活用したアプリケーションを構築するためのRustライブラリです。以下の特徴を持っています：

- LLM completion と embedding workflowsの完全サポート
- OpenAI、Cohereなどの主要なLLMプロバイダーに対応
- MongoDB、LanceDBなどのベクトルストアとの統合
- 型安全な開発とエラーハンドリング

## 主な機能

### 1. 統一的なAPI

異なるLLMプロバイダー間で一貫したインターフェースを提供します：

```rust
// OpenAIの場合
let openai_client = openai::Client::from_env();
let gpt4 = openai_client.model("gpt-4o-mini").build();
let response = gpt4.prompt("Hello, GPT-4!").await?;

// Cohereの場合
let cohere_client = cohere::Client::from_env();
let command = cohere_client.model("command").build();
let response = command.prompt("Hello, Cohere!").await?;
```

### 2. RAG (Retrieval-Augmented Generation)

ドキュメントの埋め込みと検索を活用した高度な生成が可能です：

```rust
let rag_agent = openai_client.context_rag_agent("gpt-4")
    .preamble("You are a helpful assistant.")
    .dynamic_context(2, vector_store.index(embedding_model))
    .build();

let response = rag_agent.prompt("What is Rig?").await?;
```

### 3. 型安全な開発

Rustの型システムを活用した安全な開発が可能です：

```rust
#[derive(serde::Deserialize, JsonSchema)]
struct Person {
    name: String,
    age: u8,
}

let extractor = openai_client.extractor::<Person>("gpt-4").build();
let person: Person = extractor.extract("John Doe is 30 years old").await?;
```

## 使用方法

1. 依存関係の追加：

```toml
[dependencies]
rig-core = "0.0.6"
tokio = { version = "1.34.0", features = ["full"] }
```

2. 環境変数の設定：

```bash
export OPENAI_API_KEY=your_api_key
```

3. 基本的な使用例：

```rust
use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let model = openai_client.model("gpt-4o-mini").build();

    let response = model.prompt("Explain quantum computing in one sentence.").await?;
    println!("Response: {}", response);

    Ok(())
}
```

## サンプルコード

このワークスペースには以下のサンプルが含まれています：

1. `src/main.rs`: OpenAIを使用した基本的なLLM completion
2. `src/main_rag.rs`: RAGを使用した高度な例
3. `src/main_tool.rs`: Tool機能の実装例
4. `src/main_extractor.rs`: JSON構造体の抽出例

各機能の特徴：
- OpenAI: コスト効率の良いgpt-4o-miniモデルを使用
- RAG: OpenAIのtext-embedding-3-smallを使用した効率的な埋め込み
- Tool: カスタム機能の拡張が可能
- Extractor: 構造化データの抽出をサポート

注: 現在のバージョン（0.0.6）ではAnthropicのサポートは含まれていません。

## Tool機能

RigのTool機能を使用すると、LLMエージェントに追加の機能を実装できます。以下は文字列の長さを計算するツールの例です：

```rust
use rig::tool::{Tool, ToolSet};

// 文字列の長さを計算するツール
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

// ツールをAgentで使用
let tool_set = ToolSet::new().with_tool(StringLengthTool);
let agent = openai_client
    .agent("gpt-4o-mini")
    .with_tools(tool_set)
    .build();
```

## JSON構造体の抽出

rigライブラリは、LLMを使用してテキストから構造化データを抽出する機能を提供します：

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 抽出したい構造体の定義
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct Person {
    name: Option<String>,
    age: Option<u8>,
    occupation: Option<String>,
}

// Extractorの使用
let extractor = openai_client
    .extractor::<Person>("gpt-4o-mini")
    .build();

let text = "山田太郎さんは32歳のソフトウェアエンジニアです。";
let person = extractor.extract(text).await?;
```

## 注意事項

- 現在のバージョン（0.0.6）は開発中であり、将来的に破壊的な変更が含まれる可能性があります
- APIキーの管理には十分注意してください
- コスト効率を考慮し、適切なモデル（例：gpt-4o-mini）を選択してください
