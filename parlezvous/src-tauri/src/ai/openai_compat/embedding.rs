use async_trait::async_trait;
use crate::ai::{EmbeddingProvider, openai_compat_adapter::OpenAiCompatibleAdapter};

#[async_trait]
impl EmbeddingProvider for OpenAiCompatibleAdapter {
    async fn generate_embedding(&self, text: String, model: String) -> Result<Vec<f64>, String> {
        self.embedding(text, model).await
    }
}
