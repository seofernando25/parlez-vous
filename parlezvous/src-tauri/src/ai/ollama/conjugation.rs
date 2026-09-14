use async_trait::async_trait;
use ollama_rs::{generation::completion::request::GenerationRequest, models::ModelOptions};
use crate::ai::{ConjugationProvider, ollama_adapter::OllamaAdapter};

#[async_trait]
impl ConjugationProvider for OllamaAdapter {
    async fn generate_conjugation_exercise(
        &self,
        language: String,
        model: String,
        previously_used: Vec<String>,
        tense_stats: Vec<crate::ai::TenseStat>,
        active_theme: String,
    ) -> Result<crate::ai::ConjugationExercise, String> {
        let prompt = crate::ai::build_conjugation_prompt(
            &language,
            &previously_used,
            &tense_stats,
            &active_theme,
        );

        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            let options = ModelOptions::default().num_predict(300);
            let request = GenerationRequest::new(model.clone(), prompt.clone()).options(options);

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

            match serde_json::from_str::<crate::ai::ConjugationExercise>(json_content) {
                Ok(mut parsed) => {
                    // One-pass revision/verification
                    let verification_prompt = crate::ai::build_conjugation_verification_prompt(&language, &parsed);
                    let verify_options = ModelOptions::default().num_predict(4096);
                    let verify_request = GenerationRequest::new(model.clone(), verification_prompt).options(verify_options);

                    println!("[Conjugation] Running 1-pass verification on candidate...");
                    if let Ok(client) = self.get_client() {
                        if let Ok(response) = client.generate(verify_request).await {
                            let content = response.response;
                            let v_json_content = if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
                                &content[start..=end]
                            } else {
                                &content
                            };

                            if let Ok(revised) = serde_json::from_str::<crate::ai::ConjugationExercise>(v_json_content) {
                                println!("[Conjugation] Verification successfully parsed, unconditionally accepting revised candidate.");
                                parsed = revised;
                            } else {
                                println!("[Conjugation] Verification JSON parse failed, falling back to original candidate.");
                            }
                        } else {
                            println!("[Conjugation] Verification LLM call failed, falling back to original candidate.");
                        }
                    }

                    return Ok(parsed);
                },
                Err(e) => {
                    attempts += 1;
                    println!(
                        "LLM Conjugation JSON Parse Error on attempt {}: {} \nRaw output: {}",
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

        Err("Unknown error in conjugation generation loop".to_string())
    }
}
