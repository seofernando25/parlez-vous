use async_trait::async_trait;
use crate::ai::{ModelProvider, openai_compat_adapter::OpenAiCompatibleAdapter};

#[async_trait]
impl ModelProvider for OpenAiCompatibleAdapter {
    async fn check_health(&self) -> bool { OpenAiCompatibleAdapter::check_health(self).await }

    async fn list_models(&self) -> Result<Vec<String>, String> {
        OpenAiCompatibleAdapter::list_models(self).await
    }
}
