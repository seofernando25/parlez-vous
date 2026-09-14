use async_trait::async_trait;
use tauri_plugin_litert::LitertExt;
use crate::ai::{ChatMessage, ChatProvider, ChatResponse, litert_adapter::LiteRtAdapter};

#[async_trait]
impl ChatProvider for LiteRtAdapter {
    async fn generate_chat_response(
        &self,
        history: Vec<ChatMessage>,
        _model: String,
        language: String,
        skill_level: String,
        context: String,
        active_theme: Option<String>,
        active_subtheme: Option<String>,
        audio_base64: Option<String>,
        image_uri: Option<String>,
    ) -> Result<ChatResponse, String> {
        self.ensure_initialized().await?;
        let system_prompt_str = crate::ai::build_chat_system_prompt(
            &language,
            &skill_level,
            &context,
            &active_theme,
            &active_subtheme,
            false,
            true, // use_expression_tags
            image_uri.is_some(),
        );

        self.ensure_tokenizer_loaded();

        let mut context_summary: Option<String> = None;
        let mut truncated_history = {
            let t = self.tokenizer.lock().unwrap();
            if let Some(tokenizer) = &*t {
                crate::ai::truncate_history_exact(&system_prompt_str, &history, self.max_tokens as usize, tokenizer)
            } else {
                crate::ai::truncate_history_by_tokens(&system_prompt_str, &history, self.max_tokens as usize)
            }
        };

        let compression_needed = truncated_history.len() < history.len();

        let mut is_continuation = false;

        if compression_needed {
            println!("[LiteRT] Context maxed or loops predicted. Compressing history...");
            let mut summary_prompt = format!(
                "You are an internal system compressing memory for a language tutor avatar. Provide a detailed, comprehensive summary of the following conversation in English. Focus on the core context, the topics discussed, the user's intent, and note any specific {} vocabulary or concepts the user was practicing. Ensure the summary contains enough detail so the AI can seamlessly resume the conversation without losing track of the subject:\n\n",
                language
            );

            let keep_count = std::cmp::min(2, history.len());
            // Summarize everything except the very last messages we keep
            let history_to_summarize = &history[0..history.len().saturating_sub(keep_count)];
            for msg in history_to_summarize {
                summary_prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            summary_prompt.push_str("\n\nSummary:");

            let gemma_summary_prompt = format!("<start_of_turn>user\n{}<end_of_turn>\n<start_of_turn>model\n", summary_prompt);
            let payload = tauri_plugin_litert::GenerateChatRequest { prompt: gemma_summary_prompt, reset: true, audio_base64: None, image_uri: None, system_instruction: None };

            let summary_text = match self.app_handle.litert().generate_chat(payload) {
                Ok(res) => res.response.trim().to_string(),
                Err(_) => format!("(Context compressed due to length)")
            };

            context_summary = Some(summary_text.clone());

            // Replace the truncated history with just the summary injected as a system note
            let mut new_history = Vec::new();
            new_history.push(ChatMessage {
                role: "system".to_string(),
                content: format!("[System Note: The conversation was compressed due to length. Previous context: {} \nCRITICAL INSTRUCTION: Do NOT greet the user or introduce yourself again. Just naturally continue the conversation.]", summary_text),
                audio_base64: None,
            });

            for msg in &history[history.len() - keep_count .. history.len()] {
                new_history.push(msg.clone());
            }
            truncated_history = new_history;
            is_continuation = false; // Force a full reset with the new summarized context

            // Also reset last_history so we don't think it's a continuation next time if it doesn't match
            if let Ok(mut last_hist_guard) = self.last_history.lock() {
                *last_hist_guard = None;
            }
        } else {
            // Normal continuation logic
            let mut last_hist_guard = self.last_history.lock().unwrap();
            if let Some(last_hist) = &*last_hist_guard {
                if !history.is_empty() && history.len() >= last_hist.len() && truncated_history.len() == history.len() {
                    let mut matches = true;
                    for (i, msg) in last_hist.iter().enumerate() {
                        if history[i].role != msg.role || history[i].content != msg.content {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        is_continuation = true;
                    }
                }
            }
            *last_hist_guard = Some(truncated_history.clone());
        }

        let is_multimodal = audio_base64.is_some() || image_uri.is_some();
        let mut multimodal_system_instruction: Option<String> = None;
        let gemma_prompt = if is_multimodal {
            // MULTIMODAL: Do not wrap in <start_of_turn>. Let Kotlin/LiteRT handle it natively.
            let mut text = truncated_history.last().unwrap_or(history.last().unwrap()).content.clone();
            if text.contains("🎤 [Audio Message]") || text.trim().is_empty() {
                text = text.replace("🎤 [Audio Message]", "Please evaluate this media and respond appropriately.");
                if text.trim().is_empty() {
                    text = "Please evaluate this media and respond appropriately.".to_string();
                }
            }

            // Gemma 4 vision requires the <image> token in the prompt text
            if image_uri.is_some() {
                text = format!("<image>\n{}", text);
            }

            if !is_continuation {
                let mut system_notes = String::new();
                for (i, msg) in truncated_history.iter().enumerate() {
                    if i == truncated_history.len().saturating_sub(1) {
                        continue; // Skip the last message, handled as `text` below
                    }
                    if msg.role == "system" {
                        system_notes.push_str(&msg.content);
                        system_notes.push_str("\n\n");
                    } else {
                        let role_name = if msg.role == "assistant" || msg.role == "model" { "Assistant" } else { "User" };
                        system_notes.push_str(&format!("[{}'s previous message: \"{}\"]\n\n", role_name, msg.content));
                    }
                }
                multimodal_system_instruction = Some(format!("{}\n\n{}", system_prompt_str, system_notes));
                text
            } else {
                text
            }
        } else if is_continuation {
            let last_msg = history.last().unwrap();
            format!("<start_of_turn>{}\n{}<end_of_turn>\n", last_msg.role, last_msg.content)
        } else {
            let mut prompt = String::new();

            if truncated_history.is_empty() {
                prompt.push_str(&format!("<start_of_turn>user\n{}<end_of_turn>\n", system_prompt_str));
            } else {
                let mut first_user_found = false;
                let mut buffered_system = String::new();
                for msg in truncated_history {
                    let mut role = msg.role.as_str();
                    if role == "assistant" { role = "model"; }
                    if role == "system" {
                        buffered_system.push_str(&msg.content);
                        buffered_system.push_str("\n\n");
                        continue;
                    }

                    prompt.push_str(&format!("<start_of_turn>{}\n", role));
                    if !first_user_found && role == "user" {
                        prompt.push_str(&format!("{}\n\n", system_prompt_str));
                        first_user_found = true;
                    }
                    if role == "user" && !buffered_system.is_empty() {
                        prompt.push_str(&buffered_system);
                        buffered_system.clear();
                    }
                    prompt.push_str(&msg.content);
                    prompt.push_str("<end_of_turn>\n");
                }
            }
            prompt
        };

        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            let payload = tauri_plugin_litert::GenerateChatRequest {
                prompt: if is_multimodal { gemma_prompt.clone() } else { gemma_prompt.clone() + "<start_of_turn>model\n" },
                reset: !is_continuation,
                audio_base64: audio_base64.clone(),
                image_uri: image_uri.clone(),
                system_instruction: multimodal_system_instruction.clone(),
            };

            let response_text = match self.app_handle.litert().generate_chat(payload) {
                Ok(res) => res.response,
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(format!("Android inference error: {}", e));
                    }
                    continue;
                }
            };

            return Ok(ChatResponse {
                response: response_text,
                idealized_correction: None,
                context_summary: context_summary.clone(),
            });
        }
        Err("Unknown error in generation loop".to_string())
    }
}
