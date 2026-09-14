use async_trait::async_trait;
use crate::ai::{ModelProvider, ollama_adapter::OllamaAdapter};

#[async_trait]
impl ModelProvider for OllamaAdapter {
    async fn check_health(&self) -> bool {
        if let Ok(client) = self.get_client() {
            client.list_local_models().await.is_ok()
        } else {
            false
        }
    }

    async fn list_models(&self) -> Result<Vec<String>, String> {
        let client = self.get_client()?;
        let models = client
            .list_local_models()
            .await
            .map_err(|e| e.to_string())?;
        Ok(models.into_iter().map(|m| m.name).collect())
    }
}
