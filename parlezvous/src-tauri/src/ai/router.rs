use super::{EmbeddingProvider, LlmProvider};
use super::openai_compat_adapter::OpenAiCompatibleAdapter;
#[cfg(target_os = "android")]
use super::litert_adapter::LiteRtAdapter;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "android")]
#[derive(Clone, Debug, PartialEq, Eq)]
struct LiteRtConfig {
    model_path: String,
    accelerator: String,
    max_tokens: u32,
}

#[cfg(target_os = "android")]
struct LiteRtSlot {
    config: LiteRtConfig,
    provider: Arc<LiteRtAdapter>,
}

/// Routes native on-device models separately from remote OpenAI-compatible models.
/// Remote provider selection is data-driven by the saved endpoint, not hard-coded by vendor.
pub struct AiRouter {
    remote: Arc<OpenAiCompatibleAdapter>,
    #[cfg(target_os = "android")]
    db: Arc<Mutex<Connection>>,
    #[cfg(target_os = "android")]
    app_handle: tauri::AppHandle,
    #[cfg(target_os = "android")]
    litert: Mutex<Option<LiteRtSlot>>,
}

impl AiRouter {
    pub fn new(db: Arc<Mutex<Connection>>, app_handle: tauri::AppHandle) -> Self {
        #[cfg(not(target_os = "android"))]
        let _ = app_handle;
        Self {
            remote: Arc::new(OpenAiCompatibleAdapter::new(db.clone())),
            #[cfg(target_os = "android")]
            db,
            #[cfg(target_os = "android")]
            app_handle,
            #[cfg(target_os = "android")]
            litert: Mutex::new(None),
        }
    }

    pub fn remote(&self) -> Arc<OpenAiCompatibleAdapter> { self.remote.clone() }

    pub fn embeddings(&self) -> Arc<dyn EmbeddingProvider + Send + Sync> {
        self.remote.clone()
    }

    pub fn provider_for(&self, model: &str) -> Result<Arc<dyn LlmProvider + Send + Sync>, String> {
        #[cfg(not(target_os = "android"))]
        let _ = model;

        #[cfg(target_os = "android")]
        if is_litert_model(model) {
            let settings = crate::services::settings::get_settings(self.db.clone())?;
            let config = LiteRtConfig {
                model_path: model.trim().to_string(),
                accelerator: settings.litert_accelerator,
                max_tokens: settings.litert_max_tokens,
            };
            let mut slot = self.litert.lock().map_err(|_| "Failed to lock LiteRT provider slot".to_string())?;
            if let Some(existing) = slot.as_ref() {
                if existing.config == config { return Ok(existing.provider.clone()); }
            }
            let provider = Arc::new(LiteRtAdapter::new(
                self.app_handle.clone(),
                config.model_path.clone(),
                config.accelerator.clone(),
                config.max_tokens,
            )?);
            *slot = Some(LiteRtSlot { config, provider: provider.clone() });
            return Ok(provider);
        }

        Ok(self.remote.clone())
    }
}

pub fn is_litert_model(model: &str) -> bool {
    model.trim().to_ascii_lowercase().ends_with(".litertlm")
}

#[cfg(test)]
mod tests {
    use super::is_litert_model;

    #[test]
    fn detects_litert_by_model_format() {
        assert!(is_litert_model("gemma-4-E2B-it.litertlm"));
        assert!(is_litert_model("MODEL.LITERTLM"));
        assert!(!is_litert_model("qwen3:latest"));
        assert!(!is_litert_model("my-litert-experiment"));
    }
}
