use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::path::Path;
use regex::Regex;
use crate::ai::{
    ChatMessage, ChatProvider, ChatResponse,
    openai_compat_adapter::{image_message, text_message, OpenAiCompatibleAdapter},
};

#[async_trait]
impl ChatProvider for OpenAiCompatibleAdapter {
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
        image_uri: Option<String>,
    ) -> Result<ChatResponse, String> {
        let is_vision = image_uri.is_some();
        let system_prompt = crate::ai::build_chat_system_prompt(
            &language,
            &skill_level,
            &context,
            &active_theme,
            &active_subtheme,
            !is_vision,
            false,
            is_vision,
        );
        let mut messages = vec![text_message("system", system_prompt)];
        let last_index = history.len().saturating_sub(1);

        for (index, message) in history.into_iter().enumerate() {
            let role = match message.role.as_str() {
                "user" | "assistant" | "system" => message.role.as_str(),
                _ => "user",
            };
            let wire = if role == "user" && index == last_index {
                if let Some(uri) = image_uri.as_ref() {
                    image_message(role, message.content, resolve_image_uri(uri)?)
                } else {
                    text_message(role, message.content)
                }
            } else {
                text_message(role, message.content)
            };
            messages.push(wire);
        }

        if is_vision {
            let response = self.chat(&model, messages, 2048, 60).await?;
            return Ok(ChatResponse {
                response,
                idealized_correction: None,
                context_summary: None,
            });
        }

        let mut parsed: ChatResponse = self.generate_json(&model, messages, 2048, 60).await?;
        let tags = Regex::new(r"\[([^\]]+)\]").map_err(|e| e.to_string())?;
        parsed.response = tags.replace_all(&parsed.response, |captures: &regex::Captures| {
            let tag = &captures[1];
            if tag.starts_with("anim:") { captures[0].to_string() } else { String::new() }
        }).to_string();
        Ok(parsed)
    }
}

fn resolve_image_uri(uri: &str) -> Result<String, String> {
    if uri.starts_with("data:image/") || uri.starts_with("https://") || uri.starts_with("http://") {
        return Ok(uri.to_string());
    }
    let path = Path::new(uri);
    let bytes = std::fs::read(path).map_err(|error| format!("Could not read image for remote AI provider: {error}"))?;
    let mime = match path.extension().and_then(|value| value.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/jpeg",
    };
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}
