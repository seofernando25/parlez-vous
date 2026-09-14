use async_trait::async_trait;
use crate::ai::{PuzzleProvider, litert_adapter::LiteRtAdapter};

#[async_trait]
impl PuzzleProvider for LiteRtAdapter {
    async fn generate_coding_puzzle(
        &self,
        language: String,
        _model: String,
        theme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String> {
        self.ensure_initialized().await?;
        let (system_prompt, user_prompt) = crate::ai::build_coding_puzzle_prompt(&language, &theme, &puzzle_type, &previously_used);
        let prompt = format!("{}\n{}", system_prompt, user_prompt);
        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            let mut json_value: serde_json::Value = match self.execute_json_generation(prompt.clone(), "Coding Puzzle") {
                Ok(v) => v,
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts { return Err(e); }
                    continue;
                }
            };

            if puzzle_type == "keystone" {
                if let Some(code) = json_value.get("code_with_blank").and_then(|v| v.as_str()) {
                    if !code.contains("___BLANK___") {
                        let mut recovered = false;
                        if let Some(exact) = json_value.get("exact_answer").and_then(|v| v.as_str()) {
                            if !exact.trim().is_empty() && code.contains(exact) {
                                let new_code = code.replacen(exact, "___BLANK___", 1);
                                if let Some(obj) = json_value.as_object_mut() {
                                    obj.insert("code_with_blank".to_string(), serde_json::Value::String(new_code));
                                    recovered = true;
                                    println!("[LiteRT] Recovered missing ___BLANK___ using exact_answer");
                                }
                            }
                        }

                        if !recovered {
                            println!("[LiteRT] Rejection: Model failed to include ___BLANK___ in keystone puzzle. Retrying...");
                            attempts += 1;
                            if attempts >= max_attempts {
                                return Err(format!("The AI model failed to include ___BLANK___ after {} attempts.", max_attempts));
                            }
                            continue;
                        }
                    }
                }
            }
            return Ok(json_value.to_string());
        }
        Err("Failed to generate coding puzzle".to_string())
    }

    async fn generate_language_puzzle(
        &self,
        language: String,
        _model: String,
        skill_level: String,
        active_theme: String,
        active_subtheme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String> {
        self.ensure_initialized().await?;
        let system_prompt = crate::ai::build_language_puzzle_prompt(&language, &skill_level, &active_theme, &active_subtheme, &puzzle_type, &previously_used);
        let user_prompt = format!("Generate a {} puzzle.", puzzle_type);
        let prompt = format!("{}\n{}", system_prompt, user_prompt);
        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            let mut json_value: serde_json::Value = match self.execute_json_generation(prompt.clone(), "Language Puzzle") {
                Ok(v) => v,
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts { return Err(e); }
                    continue;
                }
            };

            if puzzle_type == "keystone" {
                if let Some(code) = json_value.get("code_with_blank").and_then(|v| v.as_str()) {
                    if !code.contains("___BLANK___") {
                        let mut recovered = false;
                        if let Some(exact) = json_value.get("exact_answer").and_then(|v| v.as_str()) {
                            if !exact.trim().is_empty() && code.contains(exact) {
                                let new_code = code.replacen(exact, "___BLANK___", 1);
                                if let Some(obj) = json_value.as_object_mut() {
                                    obj.insert("code_with_blank".to_string(), serde_json::Value::String(new_code));
                                    recovered = true;
                                }
                            }
                        }
                        if !recovered {
                            attempts += 1;
                            if attempts >= max_attempts {
                                return Err(format!("The AI model failed to include ___BLANK___ after {} attempts.", max_attempts));
                            }
                            continue;
                        }
                    }
                }
            }
            return Ok(json_value.to_string());
        }
        Err("Failed to generate language puzzle".to_string())
    }
}
