use async_trait::async_trait;
use crate::ai::{ModelProvider, litert_adapter::LiteRtAdapter};

#[async_trait]
impl ModelProvider for LiteRtAdapter {
    async fn check_health(&self) -> bool {
        true
    }

    async fn list_models(&self) -> Result<Vec<String>, String> {
        Ok(vec!["gemma-4-E2B-it.litertlm".to_string()])
    }
}
