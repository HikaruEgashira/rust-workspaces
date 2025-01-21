//! Rust Linter
//! 
//! OpenAIのAPIを使用してRustコードの品質とセキュリティを分析するLSPベースのlinter

mod backend;
pub use backend::Backend;

use tower_lsp::jsonrpc::client::Client;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

pub use serde_json::{json, Value};
pub use log::error;
