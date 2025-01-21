use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageClient, LanguageServer};
use serde_json::{json, Value};
use log::error;

#[derive(Debug)]
pub struct Backend {
    pub client: Box<dyn LanguageClient>,
    pub openai_client: reqwest::Client,
}

impl Backend {
    pub async fn analyze_code(&self, code: &str) -> Result<String> {
        let messages = vec![
            json!({
                "role": "system",
                "content": "あなたはRustコードの品質とセキュリティを分析する専門家です。以下の観点でコードをレビューし、Markdown形式でレポートを作成してください：\n\n1. コード品質と標準\n2. セキュリティと脆弱性\n3. エラー処理と耐障害性\n4. パフォーマンスとリソース管理\n5. ドキュメンテーション"
            }),
            json!({
                "role": "user",
                "content": code
            })
        ];

        let response = self.openai_client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": messages
            }))
            .send()
            .await
            .map_err(|e| {
                error!("OpenAI API error: {}", e);
                tower_lsp::jsonrpc::Error::internal_error()
            })?;

        let response_json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse OpenAI response: {}", e);
            tower_lsp::jsonrpc::Error::internal_error()
        })?;

        response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                error!("Invalid response format from OpenAI");
                tower_lsp::jsonrpc::Error::internal_error()
            })
    }
}

#[async_trait::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: None,
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: Default::default(),
                    },
                )),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Rust linter initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        match self.analyze_code(&text).await {
            Ok(analysis) => {
                self.client
                    .show_message(MessageType::INFO, &analysis)
                    .await;
            }
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, &format!("Analysis failed: {}", e))
                    .await;
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.last() {
            match self.analyze_code(&change.text).await {
                Ok(analysis) => {
                    self.client
                        .show_message(MessageType::INFO, &analysis)
                        .await;
                }
                Err(e) => {
                    self.client
                        .show_message(MessageType::ERROR, &format!("Analysis failed: {}", e))
                        .await;
                }
            }
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
