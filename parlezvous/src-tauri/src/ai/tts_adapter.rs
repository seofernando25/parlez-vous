use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tauri::{AppHandle, Manager, Runtime};
use std::time::Duration;

#[async_trait]
pub trait TtsProvider: Send + Sync {
    async fn generate_tts(&self, text: &str, language: &str, voice: &str, speed: f32) -> Result<Vec<u8>, String>;
}

pub struct OpenAiTtsAdapter {
    client: Client,
    server_url: String,
}

impl OpenAiTtsAdapter {
    pub fn new(server_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            server_url,
        }
    }
}

#[async_trait]
impl TtsProvider for OpenAiTtsAdapter {
    async fn generate_tts(&self, text: &str, language: &str, voice: &str, speed: f32) -> Result<Vec<u8>, String> {
        let payload = json!({
            "model": "tts-1",
            "input": text,
            "voice": voice,
            "response_format": "wav",
            "language": language,
            "speed": speed
        });

        let res = self.client.post(&self.server_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to send TTS request: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("TTS request failed with status: {} - {}", status, text));
        }

        let bytes = res.bytes().await.map_err(|e| format!("Failed to read TTS audio bytes: {}", e))?;
        Ok(bytes.to_vec())
    }
}

pub struct SupertonicTtsAdapter<R: Runtime> {
    app_handle: AppHandle<R>,
}

impl<R: Runtime> SupertonicTtsAdapter<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self { app_handle }
    }
}

#[async_trait]
impl<R: Runtime> TtsProvider for SupertonicTtsAdapter<R> {
    async fn generate_tts(&self, text: &str, language: &str, _voice: &str, speed: f32) -> Result<Vec<u8>, String> {
        let state = self.app_handle.state::<tauri_plugin_supertonic::commands::SupertonicState>();
        let app_state = self.app_handle.state::<crate::AppState>();
        
        let settings = crate::services::settings::get_settings(app_state.db.clone())
            .map_err(|e| format!("Failed to get settings: {}", e))?;
            
        let payload = tauri_plugin_supertonic::GenerateTtsRequest {
            text: text.to_string(),
            lang: language.to_string(),
            speed,
            steps: 6,
            voice_style: settings.supertonic_voice_style,
        };

        // We can call the command directly
        let response = tauri_plugin_supertonic::commands::generate_supertonic_tts(
            self.app_handle.clone(),
            state,
            payload
        ).await.map_err(|e| format!("Supertonic TTS generation failed: {:?}", e))?;

        Ok(response.audio_bytes)
    }
}
