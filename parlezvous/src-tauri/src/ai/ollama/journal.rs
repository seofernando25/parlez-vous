use async_trait::async_trait;
use crate::ai::{GradingVariables, JournalProvider, JournalResponse, JournalVariables, ollama_adapter::OllamaAdapter};

#[async_trait]
impl JournalProvider for OllamaAdapter {
    async fn generate_guided_journal(
        &self,
        variables: JournalVariables,
    ) -> Result<JournalResponse, String> {
        let prompt = crate::ai::build_journal_prompt(
            &variables.language,
            &variables.skill_level,
            &variables.mood,
            &variables.weather,
            &variables.activity,
            &variables.active_theme,
        );

        self.execute_journal_generation(&variables.model, prompt)
            .await
    }

    async fn grade_custom_journal(
        &self,
        variables: GradingVariables,
    ) -> Result<JournalResponse, String> {
        let prompt = format!(
            "You are a strict language teacher. Evaluate the following custom journal entry written in {lang} by a {skill_level} learner:\n\n\
            ENTRY: \"{custom}\"\n\n\
            1. Place your corrected, idealized version of the user's text into the `generated_target_text` field.\n\
            2. Provide a native English translation of your CORRECTED text in the `native_translation` field.\n\
            3. Place your grammatical explanations and constructive feedback into the `feedback` field.\n\
            4. Extract 3-5 useful dictionary-form vocabulary words from your CORRECTED text.\n\
            Output STRICTLY valid JSON with the following structure, nothing else: \
            {{\"generated_target_text\": \"...\", \"native_translation\": \"...\", \"vocabulary\": [{{\"target_text\": \"...\", \"native_text\": \"...\"}}], \"feedback\": \"...\"}}",
            lang=variables.language, skill_level=variables.skill_level, custom=variables.entry
        );

        self.execute_journal_generation(&variables.model, prompt)
            .await
    }
}
