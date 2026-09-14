use async_trait::async_trait;
use crate::ai::{EmbeddingProvider, ollama_adapter::OllamaAdapter};

#[async_trait]
impl EmbeddingProvider for OllamaAdapter {
    async fn generate_embedding(&self, text: String, model: String) -> Result<Vec<f64>, String> {
        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            // Note: ollama-rs 0.3 handles embeddings via generate_embeddings.
            let request =
                ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest::new(
                    model.clone(),
                    ollama_rs::generation::embeddings::request::EmbeddingsInput::Single(
                        text.clone(),
                    ),
                );

            let client = self.get_client()?;
            match client.generate_embeddings(request).await {
                Ok(response) => {
                    return Ok(response
                        .embeddings
                        .into_iter()
                        .flatten()
                        .map(|v| v as f64)
                        .collect());
                }
                Err(e) => {
                    attempts += 1;
                    println!("LLM Embedding Error on attempt {}: {}", attempts, e);
                    if attempts >= max_attempts {
                        return Err(format!(
                            "The AI model failed to generate embeddings after {} attempts.",
                            attempts
                        ));
                    }
                }
            }
        }

        Err("Unknown error in embedding generation loop".to_string())
    }
}
