//! Rust Linter
//! 
//! OpenAIのAPIを使用してRustコードの品質とセキュリティを分析するLSPベースのlinter

mod backend;
pub use backend::Backend;

pub use serde_json::{json, Value};
pub use log::error;
