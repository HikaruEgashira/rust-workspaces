use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageClient, LanguageServer};
use tokio::sync::mpsc;
use async_trait::async_trait;

#[derive(Debug)]
struct TestClient {
    tx: mpsc::Sender<String>,
}

impl TestClient {
    fn new() -> (Self, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel(1);
        (TestClient { tx }, rx)
    }
}

#[async_trait]
impl LanguageServer for TestClient {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl LanguageClient for TestClient {
    async fn show_message(&self, message: MessageType, msg: &str) -> Result<()> {
        let _ = self.tx.try_send(format!("{:?}: {}", message, msg));
        Ok(())
    }

    async fn log_message(&self, message: MessageType, msg: &str) -> Result<()> {
        let _ = self.tx.try_send(format!("LOG {:?}: {}", message, msg));
        Ok(())
    }

    async fn publish_diagnostics(&self, _: Url, _: Vec<Diagnostic>, _: Option<i32>) -> Result<()> {
        Ok(())
    }

    async fn apply_edit(&self, _: ApplyWorkspaceEditParams) -> Result<ApplyWorkspaceEditResponse> {
        Ok(ApplyWorkspaceEditResponse::default())
    }
}

unsafe impl Send for TestClient {}
unsafe impl Sync for TestClient {}

mod common {
    use std::env;
    use reqwest::Client as ReqwestClient;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    use rust_linter::Backend;
    use super::TestClient;
    use tokio::sync::mpsc;

    pub async fn setup_test_backend() -> (Backend, mpsc::Receiver<String>) {
        let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", openai_api_key))
                .expect("Failed to create Authorization header")
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let openai_client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        let (test_client, rx) = TestClient::new();
        let backend = Backend {
            client: Box::new(test_client) as Box<dyn LanguageClient>,
            openai_client: openai_client.clone(),
        };

        (backend, rx)
    }
}

#[tokio::test]
async fn test_analyze_valid_code() {
    let (backend, mut rx) = common::setup_test_backend().await;
    let test_code = r#"
        fn main() {
            let x = 1;
            println!("Value: {}", x);
        }
    "#;
    
    let result = backend.analyze_code(test_code).await;
    assert!(result.is_ok(), "Code analysis should succeed");
    
    let analysis = result.unwrap();
    assert!(!analysis.is_empty(), "Analysis should not be empty");
    assert!(analysis.contains("コード品質"), "Analysis should contain quality review");
    assert!(analysis.contains("セキュリティ"), "Analysis should contain security review");

    if let Ok(message) = rx.try_recv() {
        assert!(message.contains("INFO"), "Should receive info message");
    }
}

#[tokio::test]
async fn test_analyze_invalid_code() {
    let (backend, mut rx) = common::setup_test_backend().await;
    let invalid_code = "fn main() { let x: i32 = \"string\"; }";
    
    let result = backend.analyze_code(invalid_code).await;
    assert!(result.is_ok(), "Analysis should succeed even for invalid code");
    
    let analysis = result.unwrap();
    assert!(analysis.contains("エラー"), "Analysis should mention errors");

    if let Ok(message) = rx.try_recv() {
        assert!(message.contains("INFO"), "Should receive info message");
    }
}
