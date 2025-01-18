use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::openai::Client,
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");
    let mut vector_store = InMemoryVectorStore::default();

    // シンプルなテキストを埋め込み
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc1", "Rig is a Rust library for building LLM applications.")
        .simple_document("doc2", "Rig supports OpenAI and Cohere as LLM providers.")
        .build()
        .await?;
    vector_store.add_documents(embeddings).await?;

    // RAG的なエージェント機能
    let rag_agent = openai_client.context_rag_agent("gpt-4o-mini")
        .preamble("You are an assistant that answers questions about Rig.")
        .dynamic_context(1, vector_store.index(embedding_model))
        .build();

    let response = rag_agent.prompt("What is Rig?").await?;
    println!("RAG Agent response: {}", response);

    Ok(())
}
