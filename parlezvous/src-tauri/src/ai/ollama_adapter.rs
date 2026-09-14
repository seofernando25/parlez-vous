use super::JournalResponse;
use ollama_rs::{
    generation::{
        completion::request::GenerationRequest,
    },
    models::ModelOptions,
    Ollama,
};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct OllamaAdapter {
    db: Arc<Mutex<Connection>>,
}

impl OllamaAdapter {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }

    pub(crate) fn get_client(&self) -> Result<Ollama, String> {
        let conn = self.db.lock().map_err(|_| "Failed to lock DB")?;
        let url: String = conn
            .query_row(
                "SELECT ollama_server_url FROM settings WHERE id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "http://localhost".to_string());

        let url = url.trim_end_matches('/');
        let mut host = url;
        let mut port = if url.starts_with("https://") { 443 } else { 11434 };

        // Very basic host/port parsing. Ollama::new expects (host, port)
        if url.starts_with("http://") || url.starts_with("https://") {
            if let Some((h, p)) = url.rsplit_once(':') {
                if let Ok(parsed_port) = p.parse::<u16>() {
                    // Make sure the split wasn't the https:// protocol part
                    if h != "http" && h != "https" {
                        host = h;
                        port = parsed_port;
                    }
                }
            }
        }

        Ok(Ollama::new(host.to_string(), port))
    }

    pub(crate) async fn execute_journal_generation(
        &self,
        model: &str,
        prompt: String,
    ) -> Result<JournalResponse, String> {
        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            let options = ModelOptions::default().num_predict(300);
            let request =
                GenerationRequest::new(model.to_string(), prompt.clone()).options(options);

            let client = self.get_client()?;
            let response = tokio::time::timeout(std::time::Duration::from_secs(30), client.generate(request))
                .await
                .map_err(|_| "Ollama generation timed out after 30 seconds.".to_string())?
                .map_err(|e| e.to_string())?;
            let content = response.response;

            let json_content =
                if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
                    &content[start..=end]
                } else {
                    &content
                };

            match serde_json::from_str::<JournalResponse>(json_content) {
                Ok(parsed) => return Ok(parsed),
                Err(e) => {
                    attempts += 1;
                    println!(
                        "LLM JSON Parse Error on attempt {}: {} \nRaw output: {}",
                        attempts, e, content
                    );
                    if attempts >= max_attempts {
                        return Err(format!(
                            "The AI model failed to produce valid JSON after {} attempts.",
                            max_attempts
                        ));
                    }
                }
            }
        }

        Err("Unknown error in generation loop".to_string())
    }
}

