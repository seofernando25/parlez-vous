use async_trait::async_trait;
use serde_json::Value;
use crate::ai::{
    PuzzleProvider,
    openai_compat_adapter::{extract_json, strip_thinking, text_message, OpenAiCompatibleAdapter},
};

#[async_trait]
impl PuzzleProvider for OpenAiCompatibleAdapter {
    async fn generate_coding_puzzle(
        &self,
        language: String,
        model: String,
        theme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String> {
        let (system, user) = crate::ai::build_coding_puzzle_prompt(
            &language, &theme, &puzzle_type, &previously_used,
        );
        self.generate_puzzle(&model, &puzzle_type, vec![
            text_message("system", system),
            text_message("user", user),
        ]).await
    }

    async fn generate_language_puzzle(
        &self,
        language: String,
        model: String,
        skill_level: String,
        active_theme: String,
        active_subtheme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String> {
        let system = crate::ai::build_language_puzzle_prompt(
            &language,
            &skill_level,
            &active_theme,
            &active_subtheme,
            &puzzle_type,
            &previously_used,
        );
        self.generate_puzzle(&model, &puzzle_type, vec![
            text_message("system", system),
            text_message("user", format!("Generate a {puzzle_type} puzzle.")),
        ]).await
    }
}

impl OpenAiCompatibleAdapter {
    async fn generate_puzzle(
        &self,
        model: &str,
        puzzle_type: &str,
        messages: Vec<Value>,
    ) -> Result<String, String> {
        let mut last_error = String::new();
        for _ in 0..3 {
            let content = match self.chat(model, messages.clone(), 4096, 60).await {
                Ok(content) => strip_thinking(&content),
                Err(error) => { last_error = error; continue; }
            };
            let parsed = serde_json::from_str::<Value>(extract_json(&content));
            match parsed {
                Ok(mut value) => {
                    if puzzle_type == "keystone" && !ensure_blank(&mut value) {
                        last_error = "keystone puzzle omitted ___BLANK___".into();
                        continue;
                    }
                    return Ok(value.to_string());
                }
                Err(error) => last_error = error.to_string(),
            }
        }
        Err(format!("The AI model failed to produce a valid puzzle after 3 attempts: {last_error}"))
    }
}

fn ensure_blank(value: &mut Value) -> bool {
    let Some(code) = value.get("code_with_blank").and_then(Value::as_str) else { return false; };
    if code.contains("___BLANK___") { return true; }
    let Some(answer) = value.get("exact_answer").and_then(Value::as_str) else { return false; };
    if answer.trim().is_empty() || !code.contains(answer) { return false; }
    let recovered = code.replacen(answer, "___BLANK___", 1);
    if let Some(object) = value.as_object_mut() {
        object.insert("code_with_blank".into(), Value::String(recovered));
        return true;
    }
    false
}
