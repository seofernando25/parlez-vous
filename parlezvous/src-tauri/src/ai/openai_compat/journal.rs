use async_trait::async_trait;
use crate::ai::{
    GradingVariables, JournalProvider, JournalResponse, JournalVariables,
    openai_compat_adapter::{text_message, OpenAiCompatibleAdapter},
};

#[async_trait]
impl JournalProvider for OpenAiCompatibleAdapter {
    async fn generate_guided_journal(&self, variables: JournalVariables) -> Result<JournalResponse, String> {
        let prompt = crate::ai::build_journal_prompt(
            &variables.language,
            &variables.skill_level,
            &variables.mood,
            &variables.weather,
            &variables.activity,
            &variables.active_theme,
        );
        self.generate_json(&variables.model, vec![text_message("user", prompt)], 600, 45).await
    }

    async fn grade_custom_journal(&self, variables: GradingVariables) -> Result<JournalResponse, String> {
        let prompt = format!(
            "You are a strict language teacher. Evaluate the following custom journal entry written in {lang} by a {skill} learner:\n\nENTRY: \"{entry}\"\n\n1. Put the corrected version in generated_target_text.\n2. Put the English translation in native_translation.\n3. Put concise grammatical feedback in feedback.\n4. Extract 3-5 useful dictionary-form vocabulary words.\nReturn only valid JSON shaped as {{\"generated_target_text\":\"...\",\"native_translation\":\"...\",\"vocabulary\":[{{\"target_text\":\"...\",\"native_text\":\"...\"}}],\"feedback\":\"...\"}}.",
            lang = variables.language,
            skill = variables.skill_level,
            entry = variables.entry,
        );
        self.generate_json(&variables.model, vec![text_message("user", prompt)], 900, 45).await
    }
}
