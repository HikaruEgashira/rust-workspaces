use std::env;
use tower_lsp::{LspService, Server, LanguageClient};
use log::info;
use rust_linter::Backend;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

#[tokio::main]
async fn main() {
    env_logger::init();

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", openai_api_key))
            .expect("Failed to create Authorization header"),
    );

    let openai_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client: Box::new(client) as Box<dyn LanguageClient>,
        openai_client,
    });

    info!("Starting Rust linter server...");
    Server::new(stdin, stdout, socket).serve(service).await;
}
