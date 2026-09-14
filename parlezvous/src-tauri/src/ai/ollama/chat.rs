use async_trait::async_trait;
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage as OllamaChatMessage};
use crate::ai::{ChatMessage, ChatProvider, ChatResponse, ollama_adapter::OllamaAdapter};

#[async_trait]
impl ChatProvider for OllamaAdapter {
    async fn generate_chat_response(
        &self,
        history: Vec<ChatMessage>,
        model: String,
        language: String,
        skill_level: String,
        context: String,
        active_theme: Option<String>,
        active_subtheme: Option<String>,
        _audio_base64: Option<String>,
        _image_uri: Option<String>,
    ) -> Result<ChatResponse, String> {
        let is_vision = _image_uri.is_some();
        let system_prompt_str = crate::ai::build_chat_system_prompt(
            &language,
            &skill_level,
            &context,
            &active_theme,
            &active_subtheme,
            !is_vision,
            false,
            is_vision,
        );

        let mut messages = Vec::new();

        let system_prompt = OllamaChatMessage::system(system_prompt_str.clone());
        messages.push(system_prompt);

        let history_len = history.len();

        for (i, msg) in history.into_iter().enumerate() {
            let mut o_msg = if msg.role == "user" {
                OllamaChatMessage::user(msg.content)
            } else if msg.role == "system" {
                OllamaChatMessage::system(msg.content)
            } else {
                OllamaChatMessage::assistant(msg.content)
            };

            // Add image to the last user message if vision is enabled
            if is_vision && i == history_len - 1 && msg.role == "user" {
                if let Some(uri) = &_image_uri {
                    if uri.starts_with("data:image/") {
                        let base64_data = uri.splitn(2, ',').nth(1).unwrap_or("").to_string();
                        if !base64_data.is_empty() {
                            o_msg = o_msg.add_image(ollama_rs::generation::images::Image::from_base64(&base64_data));
                        }
                    }
                }
            }
            messages.push(o_msg);
        }

        let max_gen_attempts = 3;
        let mut gen_attempts = 0;

        loop {
            let request = ChatMessageRequest::new(model.clone(), messages.clone());
            let client = self.get_client()?;
            let response = client
                .send_chat_messages(request)
                .await
                .map_err(|e| e.to_string())?;
            let content = response.message.content.clone();

            if is_vision {
                return Ok(ChatResponse {
                    response: content.clone(),
                    idealized_correction: None,
                    context_summary: None,
                });
            }

            let json_content =
                if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
                    &content[start..=end]
                } else {
                    &content
                };

            match serde_json::from_str::<ChatResponse>(json_content) {
                Ok(mut parsed) => {
                    // Strip hallucinated bracket tags like [expression:smile] that aren't [anim:...]
                    let re = regex::Regex::new(r"\[([^\]]+)\]").unwrap();
                    parsed.response = re.replace_all(&parsed.response, |caps: &regex::Captures| {
                        let tag = &caps[1];
                        if tag.starts_with("anim:") {
                            caps[0].to_string()
                        } else {
                            "".to_string()
                        }
                    }).to_string();

                    return Ok(parsed);
                },
                Err(e) => {
                    gen_attempts += 1;
                    println!(
                        "LLM Chat JSON Parse Error on attempt {}: {} \nRaw output: {}",
                        gen_attempts, e, content
                    );
                    if gen_attempts >= max_gen_attempts {
                        return Err(format!(
                            "The AI model failed to produce valid JSON after {} attempts.",
                            gen_attempts
                        ));
                    }
                }
            }
        }
    }
}
