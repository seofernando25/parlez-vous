use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::path::Path;
use crate::ai::{
    tutor::{analysis_prompt, conversation_prompt, correction_prompt, false_friend_challenge, false_friend_feedback, normalize_parts, solver_prompt, CorrectionCheck, TaskSolution, TurnAnalysis, TutorDraft},
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
        if let Some(uri) = image_uri {
            let vision_model = if self.app_settings()?.ai_provider == "managed" {
                crate::services::managed_ai::manifest::VISION_ALIAS.to_string()
            } else {
                model
            };
            return self.vision_response(history, vision_model, language, skill_level, context, uri).await;
        }
        self.tutor_response(history, model, language, skill_level, context, active_theme, active_subtheme).await
    }
}

impl OpenAiCompatibleAdapter {
    async fn tutor_response(
        &self,
        history: Vec<ChatMessage>,
        model: String,
        language: String,
        skill_level: String,
        context: String,
        active_theme: Option<String>,
        active_subtheme: Option<String>,
    ) -> Result<ChatResponse, String> {
        let latest = history.iter().rev().find(|message| message.role == "user")
            .map(|message| message.content.as_str()).unwrap_or("");
        let recent = recent_context(&history, active_theme.as_deref(), active_subtheme.as_deref());
        let tone = self.app_settings()?.tutor_tone;

        if let Some(parts) = literal_bubble_request(latest) {
            return Ok(ChatResponse {
                response: parts.join(" "), response_parts: parts,
                idealized_correction: None, context_summary: None,
            });
        }

        if let Some(parts) = false_friend_feedback(&language, &recent, latest) {
            return Ok(ChatResponse {
                response: parts.join(" "), response_parts: parts,
                idealized_correction: None, context_summary: None,
            });
        }

        let analysis = if let Some(analysis) = fast_analysis(latest) {
            analysis
        } else {
            let analysis: TurnAnalysis = self.generate_json_tuned(
                &model, vec![text_message("user", analysis_prompt(&language, &skill_level, &recent, latest))],
                120, 20, Some(0.0),
            ).await?;
            stabilize_analysis(analysis, latest)
        };
        #[cfg(test)]
        eprintln!("[Tutor trace] analysis={analysis:?}");

        if analysis.intent == "practice" && is_false_friend_task(&analysis.task) {
            if let Some(challenge) = false_friend_challenge(&language, &recent) {
                let parts = vec![challenge];
                return Ok(ChatResponse {
                    response: parts.join(" "), response_parts: parts,
                    idealized_correction: None, context_summary: None,
                });
            }
        }

        if matches!(analysis.intent.as_str(), "conversation" | "other") {
            let temperature = match tone.as_str() { "chill" => 0.45, "focused" => 0.0, _ => 0.2 };
            let draft_future = self.generate_json_tuned::<TutorDraft>(
                &model, vec![text_message("user", conversation_prompt(&language, &skill_level, &tone, &recent, latest))],
                220, 25, Some(temperature),
            );
            let correction_future = self.correction_if_needed(&model, &language, latest, &analysis);
            let (draft, correction) = tokio::try_join!(draft_future, correction_future)?;
            let mut parts = normalize_parts(draft.response_parts);
            if parts.is_empty() { parts.push("Could you say that another way?".to_string()); }
            return Ok(ChatResponse {
                response: parts.join(" "), response_parts: parts,
                idealized_correction: correction, context_summary: None,
            });
        }

        let solution_future = self.generate_json_tuned(
            &model, vec![text_message("user", solver_prompt(&language, &skill_level, &analysis, latest, &context))],
            260, 30, Some(0.0),
        );
        let correction_future = self.correction_if_needed(&model, &language, latest, &analysis);
        let (solution, quiet_correction): (TaskSolution, Option<String>) = tokio::try_join!(solution_future, correction_future)?;
        #[cfg(test)]
        eprintln!("[Tutor trace] solution={solution:?}");
        let correction = solution.correction.clone().or(quiet_correction);
        let parts = vec![solution.answer.trim().to_string()];
        Ok(ChatResponse {
            response: parts.join(" "), response_parts: parts,
            idealized_correction: correction, context_summary: None,
        })
    }

    async fn correction_if_needed(
        &self, model: &str, language: &str, user: &str, analysis: &TurnAnalysis,
    ) -> Result<Option<String>, String> {
        if !analysis.check_correction && analysis.intent != "correct" { return Ok(None); }
        let check: CorrectionCheck = self.generate_json_tuned(
            model, vec![text_message("user", correction_prompt(language, user))], 120, 20, Some(0.0),
        ).await?;
        Ok(check.correction.filter(|value| !value.trim().is_empty()))
    }

    async fn vision_response(
        &self,
        history: Vec<ChatMessage>,
        model: String,
        language: String,
        skill_level: String,
        context: String,
        image_uri: String,
    ) -> Result<ChatResponse, String> {
        let system = crate::ai::build_chat_system_prompt(
            &language, &skill_level, &context, &None, &None, false, false, true,
        );
        let latest = history.iter().rev().find(|message| message.role == "user")
            .map(|message| message.content.clone()).unwrap_or_default();
        let messages = vec![
            text_message("system", system),
            image_message("user", latest, resolve_image_uri(&image_uri)?),
        ];
        let response = self.chat(&model, messages, 1024, 60).await?;
        Ok(ChatResponse {
            response: response.clone(),
            response_parts: vec![response],
            idealized_correction: None,
            context_summary: None,
        })
    }
}


fn is_false_friend_task(task: &str) -> bool {
    let lower = task.to_ascii_lowercase();
    lower.contains("false friend") || lower.contains("false-friend")
}

fn literal_bubble_request(user: &str) -> Option<Vec<String>> {
    let lower = user.to_ascii_lowercase();
    if !(lower.contains("separate") || lower.contains("bubble")) { return None; }
    let start = lower.find("say:")? + 4;
    let rest = user.get(start..)?.trim();
    let literal = rest.split('.').next()?.trim();
    let parts: Vec<String> = literal.split(',')
        .map(|part| part.trim().trim_matches(|c: char| c == '"' || c == '\'' || c == '`').to_string())
        .filter(|part| !part.is_empty())
        .collect();
    if (1..=5).contains(&parts.len()) { Some(parts) } else { None }
}

fn fast_analysis(user: &str) -> Option<TurnAnalysis> {
    let lower = user.trim().to_ascii_lowercase();
    if let Some(term) = explicit_meaning_term(user, &lower) {
        return Some(TurnAnalysis {
            intent: "translate".to_string(),
            task: format!("Translate the target-language term '{term}' into English and give only its natural meaning."),
            factual: false, check_correction: false,
        });
    }
    if lower.starts_with("conjugate ") || lower.starts_with("please conjugate ") {
        return Some(TurnAnalysis {
            intent: "conjugate".to_string(), task: user.trim().to_string(),
            factual: false, check_correction: false,
        });
    }
    if lower.starts_with("translate ") || lower.starts_with("how do i say ") || lower.starts_with("how would i say ") {
        return Some(TurnAnalysis {
            intent: "translate".to_string(), task: user.trim().to_string(),
            factual: false, check_correction: false,
        });
    }
    if (lower.contains("false-friend") || lower.contains("false friend"))
        && ["challenge", "quiz", "exercise", "practice"].iter().any(|cue| lower.contains(cue))
    {
        return Some(TurnAnalysis {
            intent: "practice".to_string(), task: user.trim().to_string(),
            factual: false, check_correction: false,
        });
    }
    None
}

fn stabilize_analysis(mut analysis: TurnAnalysis, user: &str) -> TurnAnalysis {
    let lower = user.to_ascii_lowercase();
    if let Some(term) = explicit_meaning_term(user, &lower) {
        analysis.intent = "translate".to_string();
        analysis.task = format!("Translate the target-language term '{term}' into English and give only its natural meaning.");
        analysis.factual = false;
        analysis.check_correction = false;
        return analysis;
    }
    let format_request = ["message bubble", "separate message", "separate tiny", "chunked answer"]
        .iter().any(|cue| lower.contains(cue));
    let practice_cue = ["quiz", "challenge", "exercise", "practice", "drill"]
        .iter().any(|cue| lower.contains(cue));
    if format_request || (analysis.intent == "practice" && !practice_cue) {
        analysis.intent = "conversation".to_string();
        analysis.task = "Respond directly to the learner's latest message.".to_string();
    }
    if looks_like_real_world_fact(user) {
        analysis.intent = "fact".to_string();
        analysis.factual = true;
    }
    analysis
}

fn explicit_meaning_term(user: &str, lower: &str) -> Option<String> {
    let start = lower.find("what does ")? + "what does ".len();
    let tail = lower.get(start..)?;
    let end = tail.find(" mean")?;
    let raw = user.get(start..start + end)?.trim();
    let term = raw.trim_matches(|c: char| c == '"' || c == '\'' || c == '`' || c == '“' || c == '”');
    if term.is_empty() || term.split_whitespace().count() > 4 { None } else { Some(term.to_string()) }
}

fn looks_like_real_world_fact(user: &str) -> bool {
    let lower = user.to_ascii_lowercase();
    if ["history", "historical", "etymology", "etymological", "origin of", "when did", "who was", "where did"]
        .iter().any(|cue| lower.contains(cue)) { return true; }
    if !(lower.starts_with("did ") || lower.starts_with("was ") || lower.starts_with("were ")) { return false; }
    user.split_whitespace().skip(1).take(4).any(|raw| {
        let word = raw.trim_matches(|c: char| !c.is_alphanumeric());
        word != "I" && word.chars().next().is_some_and(char::is_uppercase)
    })
}

fn recent_context(history: &[ChatMessage], theme: Option<&str>, subtheme: Option<&str>) -> String {
    let mut lines = Vec::new();
    if let Some(theme) = theme { lines.push(format!("Practice theme: {theme}")); }
    if let Some(subtheme) = subtheme { lines.push(format!("Current topic: {subtheme}")); }
    for message in history.iter().rev().take(6).collect::<Vec<_>>().into_iter().rev() {
        if message.role == "system" { continue; }
        lines.push(format!("{}: {}", message.role, message.content));
    }
    lines.join("\n")
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
