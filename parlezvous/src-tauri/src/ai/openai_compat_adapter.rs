use reqwest::{Client, Method, RequestBuilder, StatusCode};
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{sync::{Arc, Mutex}, time::Duration};

#[derive(Clone, Debug)]
pub(crate) struct EndpointConfig {
    pub provider: String,
    pub base_url: String,
    pub api_key: String,
}

pub struct OpenAiCompatibleAdapter {
    db: Arc<Mutex<Connection>>,
    client: Client,
}

impl OpenAiCompatibleAdapter {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db, client: Client::new() }
    }

    pub(crate) fn config(&self) -> Result<EndpointConfig, String> {
        let settings = crate::services::settings::get_settings(self.db.clone())?;
        Ok(EndpointConfig {
            provider: settings.ai_provider,
            base_url: normalize_base_url(&settings.ai_base_url),
            api_key: settings.ai_api_key,
        })
    }

    pub(crate) fn app_settings(&self) -> Result<crate::services::settings::AppSettings, String> {
        crate::services::settings::get_settings(self.db.clone())
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, String> {
        let config = self.config()?;
        let url = format!("{}/{}", config.base_url, path.trim_start_matches('/'));
        let mut request = self.client.request(method, url).header("Content-Type", "application/json");
        if !config.api_key.trim().is_empty() {
            request = request.bearer_auth(config.api_key.trim());
        }
        if config.provider == "openrouter" {
            request = request.header("X-OpenRouter-Title", "ParlezVous");
        }
        Ok(request)
    }

    pub(crate) async fn check_health(&self) -> bool {
        match self.request(Method::GET, "models") {
            Ok(request) => match request.timeout(Duration::from_secs(8)).send().await {
                Ok(response) => response.status().is_success()
                    || matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::METHOD_NOT_ALLOWED),
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    pub(crate) async fn list_models(&self) -> Result<Vec<String>, String> {
        let response = self.request(Method::GET, "models")?
            .timeout(Duration::from_secs(12)).send().await.map_err(|e| e.to_string())?;
        if matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::METHOD_NOT_ALLOWED) {
            return Ok(Vec::new());
        }
        let value = response_json(response).await?;
        let mut models: Vec<String> = value.get("data").and_then(Value::as_array).into_iter().flatten()
            .filter_map(|item| item.get("id").and_then(Value::as_str).map(str::to_owned)).collect();
        models.sort();
        Ok(models)
    }

    pub(crate) async fn chat(
        &self, model: &str, messages: Vec<Value>, max_tokens: u32, timeout_secs: u64,
    ) -> Result<String, String> {
        self.chat_tuned(model, messages, max_tokens, timeout_secs, None).await
    }

    pub(crate) async fn chat_tuned(
        &self, model: &str, messages: Vec<Value>, max_tokens: u32, timeout_secs: u64, temperature: Option<f32>,
    ) -> Result<String, String> {
        let mut body = json!({ "model": model, "messages": messages, "max_tokens": max_tokens });
        if let Some(value) = temperature { body["temperature"] = json!(value); }
        if self.config()?.provider == "managed" { body["chat_template_kwargs"] = json!({ "enable_thinking": false }); }
        let response = self.request(Method::POST, "chat/completions")?
            .json(&body).timeout(Duration::from_secs(timeout_secs)).send().await.map_err(|e| e.to_string())?;
        let value = response_json(response).await?;
        value.pointer("/choices/0/message/content").and_then(Value::as_str).map(str::to_owned)
            .ok_or_else(|| format!("AI endpoint returned no message content: {}", compact_json(&value)))
    }

    pub(crate) async fn generate_json<T: DeserializeOwned>(
        &self, model: &str, messages: Vec<Value>, max_tokens: u32, timeout_secs: u64,
    ) -> Result<T, String> {
        self.generate_json_tuned(model, messages, max_tokens, timeout_secs, Some(0.1)).await
    }

    pub(crate) async fn generate_json_tuned<T: DeserializeOwned>(
        &self, model: &str, messages: Vec<Value>, max_tokens: u32, timeout_secs: u64, temperature: Option<f32>,
    ) -> Result<T, String> {
        let mut last_error = String::new();
        for attempt in 1..=3 {
            match self.chat_tuned(model, messages.clone(), max_tokens, timeout_secs, temperature).await {
                Ok(content) => {
                    let cleaned = strip_thinking(&content);
                    match serde_json::from_str::<T>(extract_json(&cleaned)) {
                        Ok(value) => return Ok(value),
                        Err(error) => last_error = format!("attempt {attempt}: {error}; output={}", truncate(&cleaned, 500)),
                    }
                }
                Err(error) => last_error = format!("attempt {attempt}: {error}"),
            }
            if attempt < 3 { tokio::time::sleep(Duration::from_millis(250 * attempt)).await; }
        }
        Err(format!("The AI model did not return valid structured output after 3 attempts ({last_error})"))
    }

    pub(crate) async fn embedding(&self, text: String, model: String) -> Result<Vec<f64>, String> {
        let body = json!({ "model": model, "input": text, "encoding_format": "float" });
        let response = self.request(Method::POST, "embeddings")?
            .json(&body).timeout(Duration::from_secs(30)).send().await.map_err(|e| e.to_string())?;
        let value = response_json(response).await?;
        let embedding: Vec<f64> = value.pointer("/data/0/embedding").and_then(Value::as_array)
            .ok_or_else(|| "AI endpoint returned no embedding".to_string())?
            .iter().map(|number| number.as_f64().ok_or_else(|| "Embedding contained a non-number".to_string())).collect::<Result<_, _>>()?;
        if embedding.is_empty() {
            return Err("AI endpoint returned an empty embedding".to_string());
        }
        Ok(embedding)
    }
}

pub(crate) fn text_message(role: &str, content: impl Into<String>) -> Value {
    json!({ "role": role, "content": content.into() })
}

pub(crate) fn image_message(role: &str, text: String, image_url: String) -> Value {
    json!({ "role": role, "content": [
        { "type": "text", "text": text },
        { "type": "image_url", "image_url": { "url": image_url } }
    ] })
}

pub(crate) fn strip_thinking(content: &str) -> String {
    let mut output = content.to_string();
    while let (Some(start), Some(end)) = (output.find("<think>"), output.find("</think>")) {
        if start >= end { break; }
        output.replace_range(start..end + "</think>".len(), "");
    }
    output.trim().to_string()
}

pub(crate) fn extract_json(content: &str) -> &str {
    match (content.find('{'), content.rfind('}')) {
        (Some(start), Some(end)) if start <= end => &content[start..=end],
        _ => content,
    }
}

fn normalize_base_url(input: &str) -> String {
    let mut base = input.trim().trim_end_matches('/').to_string();
    for suffix in ["/chat/completions", "/embeddings", "/models"] {
        if base.ends_with(suffix) { base.truncate(base.len() - suffix.len()); break; }
    }
    if base.is_empty() { "http://localhost:11434/v1".to_string() } else { base }
}

async fn response_json(response: reqwest::Response) -> Result<Value, String> {
    let status = response.status();
    let text = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("AI endpoint returned {status}: {}", truncate(&text, 700)));
    }
    serde_json::from_str(&text).map_err(|e| format!("Invalid AI endpoint response: {e}; body={}", truncate(&text, 700)))
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max { return value.to_string(); }
    value.chars().take(max).collect::<String>() + "…"
}

fn compact_json(value: &Value) -> String { truncate(&value.to_string(), 700) }

#[cfg(test)]
mod tests {
    use super::{extract_json, normalize_base_url, strip_thinking, OpenAiCompatibleAdapter};
    use reqwest::Client;
    use rusqlite::Connection;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    #[test]
    fn normalizes_compatible_api_roots() {
        assert_eq!(normalize_base_url("http://localhost:11434/v1/"), "http://localhost:11434/v1");
        assert_eq!(normalize_base_url("http://localhost:1234/v1/chat/completions"), "http://localhost:1234/v1");
    }

    #[test]
    fn extracts_structured_output_around_reasoning() {
        let cleaned = strip_thinking("<think>hidden</think> prefix {\"ok\":true} suffix");
        assert_eq!(extract_json(&cleaned), "{\"ok\":true}");
    }

    #[tokio::test]
    async fn speaks_openai_compatible_models_chat_and_embeddings() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || {
            for _ in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 8192];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            request.extend_from_slice(&buffer[..n]);
                            let text = String::from_utf8_lossy(&request);
                            if let Some(header_end) = text.find("\r\n\r\n") {
                                let length = text[..header_end].lines()
                                    .find_map(|line| line.to_ascii_lowercase().strip_prefix("content-length:").map(str::trim).and_then(|value| value.parse::<usize>().ok()))
                                    .unwrap_or(0);
                                if request.len() >= header_end + 4 + length { break; }
                            }
                        }
                        Err(_) => break,
                    }
                }
                let request = String::from_utf8_lossy(&request);
                assert!(request.to_ascii_lowercase().contains("authorization: bearer test-key"));
                let (status, body) = if request.starts_with("GET /v1/models ") {
                    ("200 OK", serde_json::json!({"data":[{"id":"model-a"}]}).to_string())
                } else if request.starts_with("POST /v1/chat/completions ") {
                    assert!(request.contains("\"model\":\"model-a\""));
                    ("200 OK", serde_json::json!({"choices":[{"message":{"content":"hello"}}]}).to_string())
                } else if request.starts_with("POST /v1/embeddings ") {
                    assert!(!request.contains("\"dimensions\""));
                    ("200 OK", serde_json::json!({"data":[{"embedding": vec![0.25; 1024]}]}).to_string())
                } else {
                    ("404 Not Found", "{}".to_string())
                };
                write!(stream, "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
        });

        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(crate::db::SCHEMA_CURRENT).unwrap();
        connection.execute(
            "UPDATE settings SET ai_provider='custom', ai_base_url=?1, ai_api_key='test-key', embedding_model='embed', active_model='model-a' WHERE id=1",
            [format!("http://{address}/v1")],
        ).unwrap();
        let adapter = OpenAiCompatibleAdapter {
            db: Arc::new(Mutex::new(connection)),
            client: Client::new(),
        };

        assert_eq!(adapter.list_models().await.unwrap(), vec!["model-a"]);
        assert_eq!(adapter.chat("model-a", vec![super::text_message("user", "hi")], 32, 3).await.unwrap(), "hello");
        assert_eq!(adapter.embedding("text".into(), "embed".into()).await.unwrap().len(), 1024);
    }
}
