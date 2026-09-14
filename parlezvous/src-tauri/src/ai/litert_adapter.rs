use tauri::{AppHandle, Manager};
use tauri_plugin_litert::LitertExt;
use tokenizers::Tokenizer;

pub struct LiteRtAdapter {
    pub(crate) app_handle: AppHandle,
    pub(crate) max_tokens: u32,
    pub(crate) model_path: String,
    pub(crate) accelerator: String,
    pub(crate) last_history: std::sync::Mutex<Option<Vec<crate::ai::ChatMessage>>>,
    pub(crate) is_initialized: std::sync::Arc<tokio::sync::Mutex<bool>>,
    pub(crate) tokenizer: std::sync::Mutex<Option<Tokenizer>>,
}

impl LiteRtAdapter {
    pub fn new(app_handle: AppHandle, model_path: String, accelerator: String, max_tokens: u32) -> Result<Self, String> {
        let is_initialized = std::sync::Arc::new(tokio::sync::Mutex::new(false));
        let instance = Self {
            app_handle: app_handle.clone(),
            max_tokens,
            model_path: model_path.clone(),
            accelerator: accelerator.clone(),
            last_history: std::sync::Mutex::new(None),
            is_initialized: is_initialized.clone(),
            tokenizer: std::sync::Mutex::new(None),
        };

        // PRE-LOAD: Fire off the initialization immediately in the background 
        // to bypass the ANR while still having the model ready for the first chat!
        let payload = tauri_plugin_litert::InitModelRequest {
            model_path,
            accelerator,
            max_tokens,
        };

        let is_initialized_clone = is_initialized.clone();
        tauri::async_runtime::spawn(async move {
            // Lock the mutex so that if the user chats before this finishes, 
            // ensure_initialized() will wait for this lock instead of spawning a duplicate!
            let mut init_lock = is_initialized_clone.lock().await;
            if *init_lock {
                return;
            }

            let success = tokio::task::spawn_blocking(move || {
                match app_handle.litert().init_model(payload) {
                    Ok(response) => response.success,
                    Err(e) => {
                        println!("[LiteRT Preload] Init Error: {}", e);
                        false
                    }
                }
            }).await.unwrap_or(false);

            if success {
                *init_lock = true;
            }
        });

        Ok(instance)
    }

    pub(crate) fn ensure_tokenizer_loaded(&self) {
        let mut t = self.tokenizer.lock().unwrap();
        if t.is_none() {
            if let Ok(app_dir) = self.app_handle.path().app_data_dir() {
                let tokenizer_path = app_dir.join("tokenizer.json");
                if tokenizer_path.exists() {
                    if let Ok(tokenizer) = Tokenizer::from_file(tokenizer_path) {
                        *t = Some(tokenizer);
                    }
                }
            }
        }
    }

    pub(crate) async fn ensure_initialized(&self) -> Result<(), String> {
        let mut initialized = self.is_initialized.lock().await;
        if *initialized {
            return Ok(());
        }

        let payload = tauri_plugin_litert::InitModelRequest {
            model_path: self.model_path.clone(),
            accelerator: self.accelerator.clone(),
            max_tokens: self.max_tokens,
        };

        // Call init_model in spawn_blocking to prevent run_mobile_plugin from blocking the tokio worker thread
        let app_handle = self.app_handle.clone();
        let success_or_err = tokio::task::spawn_blocking(move || {
            match app_handle.litert().init_model(payload) {
                Ok(response) => {
                    if response.success {
                        Ok(())
                    } else {
                        Err("Plugin returned success=false".to_string())
                    }
                },
                Err(e) => {
                    println!("[LiteRT] Init Error: {}", e);
                    Err(e.to_string())
                }
            }
        })
        .await
        .map_err(|e| format!("Task failed: {}", e))?;

        match success_or_err {
            Ok(_) => {
                *initialized = true;
                Ok(())
            }
            Err(e) => {
                Err(format!("LiteRT Init Error: {}", e))
            }
        }
    }
    
    pub(crate) fn execute_json_generation<T: serde::de::DeserializeOwned>(
        &self,
        prompt: String,
        error_prefix: &str,
    ) -> Result<T, String> {
        // Clear chat history memory so returning to chat doesn't hallucinate a continuation
        // on top of this single-shot conversation
        if let Ok(mut last_hist_guard) = self.last_history.lock() {
            *last_hist_guard = None;
        }

        let max_attempts = 3;
        let mut attempts = 0;

        // Wrap the prompt in Gemma tags
        let gemma_prompt = format!("<start_of_turn>user\n{}<end_of_turn>\n<start_of_turn>model\n", prompt);

        while attempts < max_attempts {
            let payload = tauri_plugin_litert::GenerateChatRequest { prompt: gemma_prompt.clone(), reset: true, audio_base64: None, image_uri: None, system_instruction: None };
            
            let response = match self.app_handle.litert().generate_chat(payload) {
                Ok(res) => res.response,
                Err(e) => {
                    attempts += 1;
                    println!("{} Native Error on attempt {}: {}", error_prefix, attempts, e);
                    if attempts >= max_attempts {
                        return Err(format!("Android inference error: {}", e));
                    }
                    continue;
                }
            };
            
            let content = response;
            let json_content = if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
                &content[start..=end]
            } else {
                &content
            };
            
            match serde_json::from_str::<T>(json_content) {
                Ok(parsed) => return Ok(parsed),
                Err(e) => {
                    attempts += 1;
                    println!(
                        "{} JSON Parse Error on attempt {}: {} \nRaw output: {}",
                        error_prefix, attempts, e, content
                    );
                    if attempts >= max_attempts {
                        return Err(format!("The AI model failed to produce valid JSON after {} attempts.", max_attempts));
                    }
                }
            }
        }
        Err(format!("The AI model failed to produce valid JSON after {} attempts.", max_attempts))
    }
}
